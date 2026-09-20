use crate::{
    model::{error::Error, OuDiaFile},
    opt::{deserialize::deserialize_node, directory::Directory, node::Node},
};

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

#[cfg(test)]
mod tests {
    use super::{deserialize_oudia, serialize_oudia};
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
}
