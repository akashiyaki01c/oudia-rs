use crate::{
    model::{error::Error, OuDiaFile},
    opt::{
        deserialize::{deserialize_node, deserialize_node_with_locations},
        directory::Directory,
        node::Node,
        source::{
            find_location, find_location_by_name, find_parent_location_for_missing,
            root_location, LocatedNode, NodePath,
        },
    },
};
use std::fmt;

/// OuDiaファイルの読み込みに失敗した位置付きエラー。
#[derive(Debug)]
pub struct DeserializeError {
    pub error: Error,
    /// 1-based の入力行番号。
    pub line: Option<usize>,
    /// エラーが発生したノードのパス。モデル変換エラーへの接続時に設定されます。
    pub path: Option<NodePath>,
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.error)?;
        if let Some(line) = self.line {
            write!(formatter, " (line {line})")?;
        }
        if let Some(path) = &self.path {
            write!(formatter, " at {path}")?;
        }
        Ok(())
    }
}

impl std::error::Error for DeserializeError {}

/// OuDiaFile構造体のデータをOuPropertiesText形式として出力する関数
pub fn serialize_oudia(data: &OuDiaFile) -> Vec<u8> {
    match data {
        OuDiaFile::OuDia102(rosen_file_data) => rosen_file_data.to_oudia_bytes(),
    }
}

fn decode_oudia(value: &[u8]) -> Result<std::borrow::Cow<'_, str>, Error> {
    let value_without_utf8_bom = value.strip_prefix(b"\xef\xbb\xbf").unwrap_or(value);
    let first_line = value_without_utf8_bom
        .split(|byte| *byte == b'\r' || *byte == b'\n')
        .next()
        .unwrap_or_default();

    if first_line.starts_with(b"FileType=OuDiaSecond") {
        return std::str::from_utf8(value_without_utf8_bom)
            .map(std::borrow::Cow::Borrowed)
            .map_err(|_| Error::InvalidFileFormat(crate::opt::error::Error::UndecodedText));
    }

    if first_line.starts_with(b"FileType=OuDia.") {
        let (decoded, _, had_errors) = encoding_rs::SHIFT_JIS.decode(value);
        return if had_errors {
            Err(Error::InvalidFileFormat(
                crate::opt::error::Error::UndecodedText,
            ))
        } else {
            Ok(decoded)
        };
    }

    if let Ok(decoded) = std::str::from_utf8(value_without_utf8_bom) {
        return Ok(std::borrow::Cow::Borrowed(decoded));
    }

    let (decoded, _, had_errors) = encoding_rs::SHIFT_JIS.decode(value);
    if had_errors {
        Err(Error::InvalidFileFormat(
            crate::opt::error::Error::UndecodedText,
        ))
    } else {
        Ok(decoded)
    }
}

/// OuDiaファイルのバイト列を解釈し、OuDiaFile構造体に変換する関数
pub fn deserialize_oudia(value: &[u8]) -> Result<OuDiaFile, Error> {
    let value = decode_oudia(value)?;
    let node = deserialize_node(&value)?;
    let file = Directory::new_with_value("ROOT", node);

    let file_version = file.find("FileType").ok_or(Error::InvalidVersion)?;
    let Node::Property(file_version) = file_version else {
        return Err(Error::InvalidVersion);
    };
    let file_version: &str = &file_version.value.to_string();

    let file = Node::Directory(file);
    match file_version {
        "OuDia.1.02" => Ok(OuDiaFile::OuDia102(
            crate::model::oudia102::RosenFileData::from_node(&file)?,
        )),
        _ => Err(Error::InvalidVersion),
    }
}

/// OuDiaファイルを位置情報付きで解釈し、OuDiaFile構造体に変換します。
pub fn deserialize_oudia_with_diagnostics(
    value: &[u8],
) -> Result<OuDiaFile, DeserializeError> {
    let value = decode_oudia(value).map_err(|error| DeserializeError {
        error,
        line: None,
        path: None,
    })?;
    let located_nodes = deserialize_node_with_locations(&value).map_err(|error| DeserializeError {
        error: Error::InvalidFileFormat(error.error),
        line: Some(error.line),
        path: None,
    })?;
    let node = located_nodes
        .iter()
        .map(|located| located.node.clone())
        .collect();
    let file = Directory::new_with_value("ROOT", node);

    let file_version = file.find("FileType").ok_or_else(|| {
        let location = root_location(&located_nodes);
        DeserializeError {
            error: Error::InvalidVersion,
            line: location.as_ref().map(|location| location.line),
            path: location.map(|location| location.path),
        }
    })?;
    let Node::Property(file_version) = file_version else {
        let location = find_location_by_name(&located_nodes, "FileType")
            .or_else(|| root_location(&located_nodes));
        return Err(DeserializeError {
            error: Error::InvalidVersion,
            line: location.as_ref().map(|location| location.line),
            path: location.map(|location| location.path),
        });
    };
    let file_version = file_version.value.clone();
    let file = Node::Directory(file);
    let result = match file_version.as_str() {
        "OuDia.1.02" => crate::model::oudia102::RosenFileData::from_node(&file)
            .map(OuDiaFile::OuDia102),
        _ => Err(Error::InvalidVersion),
    };
    result.map_err(|error| {
        let location = find_model_error_location(&located_nodes, &error);
        DeserializeError {
            error,
            line: location.as_ref().map(|location| location.line),
            path: location.map(|location| location.path),
        }
    })
}

fn find_model_error_location(
    nodes: &[LocatedNode],
    error: &Error,
) -> Option<crate::opt::source::NodeLocation> {
    match error {
        Error::InvalidValue(name, value) => find_location(nodes, |node| {
            matches!(node, Node::Property(property) if property.name == *name && property.value == *value)
        }),
        Error::EmptyValue(name) => find_location(nodes, |node| {
            matches!(node, Node::Property(property) if property.name == *name && property.value.is_empty())
        }),
        Error::InvalidNumber { field, value } | Error::InvalidEnum { field, value } => {
            let name = field.split('.').next().unwrap_or(field);
            find_location(nodes, |node| {
                matches!(node, Node::Property(property) if property.name == name && property.value == *value)
            })
            .or_else(|| find_location_by_name(nodes, name))
        }
        Error::ExpectedArray(name) => find_location_by_name(nodes, name),
        Error::KeyIsNotFound(name) => {
            let parent_name = match name.as_str() {
                "DiaName" | "Kudari" | "Nobori" => "Dia",
                "Ekimei" | "Ekijikokukeisiki" | "Ekikibo" => "Eki",
                "Syubetsumei" => "Ressyasyubetsu",
                "Houkou" => "Ressya",
                "Rosen" | "DispProp" => return root_location(nodes),
                _ => return None,
            };
            find_parent_location_for_missing(nodes, name, parent_name)
        }
        Error::NodeTypeError | Error::InvalidVersion => root_location(nodes),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::{deserialize_oudia, deserialize_oudia_with_diagnostics, serialize_oudia};
    use crate::model::{error::Error, OuDiaFile};

    #[test]
    fn deserialize_oudia_accepts_shift_jis_bytes() {
        let input = include_bytes!("../test_data/keio.oud");
        let result = deserialize_oudia(input);

        assert!(matches!(result, Ok(OuDiaFile::OuDia102(_))));
    }

    #[test]
    fn serialize_oudia_returns_shift_jis_bytes() {
        let input = include_bytes!("../test_data/keio.oud");
        let data = deserialize_oudia(input).unwrap();
        let output = serialize_oudia(&data);

        assert_eq!(output, input);
    }

    #[test]
    fn decode_oudia_detects_oudia_second_as_utf8() {
        let input = include_bytes!("../test_data/tohoku_hokkaido.oud2");
        let result = super::decode_oudia(input).unwrap();

        assert!(result.starts_with("FileType=OuDiaSecond."));
    }

    #[test]
    fn deserialize_oudia_accepts_a_headerless_utf8_file() {
        let input = include_bytes!("../test_data/kh.oud2");
        let result = deserialize_oudia(input);

        assert!(matches!(result, Err(Error::InvalidVersion)));
    }

    #[test]
    fn diagnostics_include_the_line_for_an_unclosed_directory() {
        let input = b"FileType=OuDia.1.02\r\nRosen.\r\n";
        let error = match deserialize_oudia_with_diagnostics(input) {
            Err(error) => error,
            Ok(_) => panic!("expected an unclosed directory error"),
        };

        assert_eq!(error.line, Some(2));
        assert!(matches!(error.error, Error::InvalidFileFormat(_)));
    }

    #[test]
    fn diagnostics_include_the_location_for_a_model_error() {
        let input = b"FileType=OuDia.1.02\r\nRosen.\r\nRosenmei=Rosen\r\nDiagramDgrYZahyouKyoriDefault=bad\r\n.\r\n";
        let error = match deserialize_oudia_with_diagnostics(input) {
            Err(error) => error,
            Ok(_) => panic!("expected an invalid number error"),
        };

        assert_eq!(error.line, Some(4));
        assert_eq!(
            error.path.as_ref().map(ToString::to_string),
            Some("ROOT.Rosen.DiagramDgrYZahyouKyoriDefault".to_string())
        );
        assert!(matches!(error.error, Error::InvalidNumber { .. }));
    }

    #[test]
    fn diagnostics_choose_the_matching_repeated_node() {
        let input = b"FileType=OuDia.1.02\r\nRosen.\r\nRosenmei=Rosen\r\nEki.\r\nEkimei=First\r\n.\r\nEki.\r\nEkimei=Second\r\nEkikibo=invalid\r\n.\r\n.\r\n";
        let error = match deserialize_oudia_with_diagnostics(input) {
            Err(error) => error,
            Ok(_) => panic!("expected an invalid enum error"),
        };

        assert_eq!(error.line, Some(9));
        assert_eq!(
            error.path.as_ref().map(ToString::to_string),
            Some("ROOT.Rosen.Eki[1].Ekikibo".to_string())
        );
        assert!(matches!(error.error, Error::InvalidEnum { .. }));
    }

    #[test]
    fn diagnostics_point_to_the_parent_for_a_missing_property() {
        let input = b"FileType=OuDia.1.02\r\nRosen.\r\nRosenmei=Rosen\r\nDia.\r\n.\r\n.\r\n";
        let error = match deserialize_oudia_with_diagnostics(input) {
            Err(error) => error,
            Ok(_) => panic!("expected a missing property error"),
        };

        assert_eq!(error.line, Some(4));
        assert_eq!(
            error.path.as_ref().map(ToString::to_string),
            Some("ROOT.Rosen.Dia".to_string())
        );
        assert!(matches!(error.error, Error::KeyIsNotFound(_)));
    }

    #[test]
    fn diagnostics_point_to_root_for_a_missing_top_level_directory() {
        let input = b"FileType=OuDia.1.02\r\n";
        let error = match deserialize_oudia_with_diagnostics(input) {
            Err(error) => error,
            Ok(_) => panic!("expected a missing top-level directory error"),
        };

        assert_eq!(error.line, Some(1));
        assert_eq!(
            error.path.as_ref().map(ToString::to_string),
            Some("ROOT".to_string())
        );
        assert!(matches!(error.error, Error::KeyIsNotFound(_)));
    }
}
