use crate::{
    model::oudia102::{disp_prop::DispProp, error::Error, rosen::Rosen},
    opt::{directory::Directory, node::Node, property::Property, serialize::serialize_node},
};

/// 1つのOuDiaファイルを表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RosenFileData {
    /// 路線
    rosen: Rosen,
    /// ダイヤグラムファイルの表示設定
    disp_prop: DispProp,
    /// 作成されたアプリ名
    file_type_app_comment: String,
}
impl RosenFileData {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // FileType
            let Some(Node::Property(version)) = dir.find("FileType") else {
                return Err(Error::InvalidVersion);
            };
            if version.value != "OuDia.1.02" {
                return Err(Error::InvalidVersion);
            }

            // Rosen
            if let Some(rosen) = dir.find("Rosen") {
                result.rosen = Rosen::from_node(rosen)?;
            } else {
                return Err(Error::KeyIsNotFound("Rosen".to_string()));
            }

            // DispProp
            if let Some(disp_prop) = dir.find("DispProp") {
                result.disp_prop = DispProp::from_node(disp_prop)?;
            } else {
                return Err(Error::KeyIsNotFound("DispProp".to_string()));
            }

            // FileTypeAppComment
            if let Some(Node::Property(file_type_app_comment)) = dir.find("FileTypeAppComment") {
                result.file_type_app_comment = file_type_app_comment.value.clone();
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    /// OuDiaのプロパティツリーへ変換します。
    pub fn to_node(&self) -> Node {
        Node::Directory(Directory::new_with_value(
            "ROOT",
            vec![
                Node::Property(Property::new_with_value(
                    "FileType",
                    "OuDia.1.02".to_string(),
                )),
                self.rosen.to_node(),
                self.disp_prop.to_node(),
                Node::Property(Property::new_with_value(
                    "FileTypeAppComment",
                    self.file_type_app_comment.clone(),
                )),
            ],
        ))
    }

    /// OuDiaテキスト形式で書き出します。改行コードはCRLFです。
    pub fn to_oudia_string(&self) -> String {
        let Node::Directory(root) = self.to_node() else {
            unreachable!();
        };
        let mut result = String::new();
        for node in &root.values {
            if !result.is_empty() && !result.ends_with("\r\n") {
                result.push_str("\r\n");
            }
            result.push_str(&serialize_node(node));
            if !result.ends_with("\r\n") {
                result.push_str("\r\n");
            }
        }
        result
    }

    /// OuDiaファイル用のShift-JISバイト列で書き出します。
    pub fn to_oudia_bytes(&self) -> Vec<u8> {
        let text = self.to_oudia_string();
        let (encoded, _, _) = encoding_rs::SHIFT_JIS.encode(&text);
        encoded.into_owned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    /// `OuDia.2`
    OuDia02,
    /// `OuDia.3`
    OuDia03,
    /// `OuDia.5`
    OuDia05,
    /// `OuDia.6`
    OuDia06,
    /// `OuDia.1.01`
    OuDia101,
    /// `OuDia.1.02`
    OuDia102,
}

#[cfg(test)]
mod tests {
    use super::RosenFileData;
    use crate::opt::{deserialize::deserialize_node, directory::Directory, node::Node};

    fn first_difference(left: &Node, right: &Node, path: &str) -> Option<String> {
        match (left, right) {
            (Node::Property(left), Node::Property(right)) => {
                if left.name != right.name {
                    return Some(format!("{path}.name: {:?} != {:?}", left.name, right.name));
                }
                if left.value != right.value {
                    return Some(format!(
                        "{path}.value: {:?} != {:?}",
                        left.value, right.value
                    ));
                }
            }
            (Node::Directory(left), Node::Directory(right)) => {
                if left.name != right.name {
                    return Some(format!("{path}.name: {:?} != {:?}", left.name, right.name));
                }
                if left.values.len() != right.values.len() {
                    return Some(format!(
                        "{path}({}).len: {} != {}",
                        left.name,
                        left.values.len(),
                        right.values.len()
                    ));
                }
                for (index, (left, right)) in left.values.iter().zip(&right.values).enumerate() {
                    if let Some(difference) = first_difference(left, right, &format!("{path}[{index}]")) {
                        return Some(difference);
                    }
                }
            }
            _ => return Some(format!("{path}: node types differ")),
        }
        None
    }

    #[test]
    fn oudia_text_can_be_read_after_writing() {
        let data = include_bytes!("../../../test_data/keio.oud");
        let (text, _, _) = encoding_rs::SHIFT_JIS.decode(data);
        let nodes = deserialize_node(&text).unwrap();
        let file =
            RosenFileData::from_node(&Node::Directory(Directory::new_with_value(
                "ROOT",
                nodes.clone(),
            )))
            .unwrap();

        let written = file.to_oudia_string();
        if text != written {
            let difference = text
                .bytes()
                .zip(written.bytes())
                .position(|(left, right)| left != right)
                .unwrap_or_else(|| text.len().min(written.len()));
            let input_context: String = text
                .chars()
                .skip(difference.saturating_sub(20))
                .take(40)
                .collect();
            let output_context: String = written
                .chars()
                .skip(difference.saturating_sub(20))
                .take(40)
                .collect();
            panic!(
                "serialized text differs at byte {difference}: input_len={}, output_len={}, input={input_context:?}, output={output_context:?}",
                text.len(),
                written.len(),
            );
        }
        let written_nodes = deserialize_node(&written).unwrap();
        RosenFileData::from_node(&Node::Directory(Directory::new_with_value(
            "ROOT",
            written_nodes.clone(),
        )))
        .unwrap();
        assert_eq!(nodes.len(), written_nodes.len());
        for (index, (input, output)) in nodes.iter().zip(&written_nodes).enumerate() {
            assert_eq!(
                first_difference(input, output, &format!("root[{index}]")),
                None
            );
        }
    }
}
