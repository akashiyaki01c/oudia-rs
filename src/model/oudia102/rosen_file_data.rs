use crate::{
    model::{
        error::Error,
        oudia102::{disp_prop::DisplayProperties, rosen::Rosen},
    },
    opt::{directory::Directory, node::Node, property::Property, serialize::serialize_node},
};

const FILE_TYPE_VERSION: &str = "OuDia.1.02";
const KEY_FILE_TYPE: &str = "FileType";
const KEY_ROSEN: &str = "Rosen";
const KEY_DISP_PROP: &str = "DispProp";
const KEY_FILE_TYPE_APP_COMMENT: &str = "FileTypeAppComment";

/// 1つのOuDiaファイルを表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RosenFileData {
    /// 路線
    rosen: Rosen,
    /// ダイヤグラムファイルの表示設定
    disp_prop: DisplayProperties,
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
            let Some(Node::Property(version)) = dir.find(KEY_FILE_TYPE) else {
                return Err(Error::InvalidVersion);
            };
            if version.value != FILE_TYPE_VERSION {
                return Err(Error::InvalidVersion);
            }

            // Rosen
            if let Some(rosen) = dir.find(KEY_ROSEN) {
                result.rosen = Rosen::from_node(rosen)?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_ROSEN.to_string()));
            }

            // DispProp
            if let Some(disp_prop) = dir.find(KEY_DISP_PROP) {
                result.disp_prop = DisplayProperties::from_node(disp_prop)?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_DISP_PROP.to_string()));
            }

            // FileTypeAppComment
            if let Some(Node::Property(file_type_app_comment)) = dir.find(KEY_FILE_TYPE_APP_COMMENT)
            {
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
                    KEY_FILE_TYPE,
                    FILE_TYPE_VERSION.to_string(),
                )),
                self.rosen.to_node(),
                self.disp_prop.to_node(),
                Node::Property(Property::new_with_value(
                    KEY_FILE_TYPE_APP_COMMENT,
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

    #[test]
    fn oudia_text_can_be_read_after_writing() {
        let data = include_bytes!("../../../test_data/OuDia102.oud");
        let (text, _, _) = encoding_rs::SHIFT_JIS.decode(data);
        let nodes = deserialize_node(&text).unwrap();
        let file =
            RosenFileData::from_node(&Node::Directory(Directory::new_with_value("ROOT", nodes)))
                .unwrap();

        let written = file.to_oudia_string();
        assert_eq!(text, written);
        let written_nodes = deserialize_node(&written).unwrap();
        RosenFileData::from_node(&Node::Directory(Directory::new_with_value(
            "ROOT",
            written_nodes,
        )))
        .unwrap();
    }
}
