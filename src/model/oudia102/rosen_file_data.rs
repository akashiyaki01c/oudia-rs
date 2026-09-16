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
                return Err(Error::RosenIsNotFound);
            }

            // DispProp
            if let Some(disp_prop) = dir.find("DispProp") {
                result.disp_prop = DispProp::from_node(disp_prop)?;
            } else {
                return Err(Error::RosenIsNotFound);
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
        root.values
            .iter()
            .map(serialize_node)
            .collect::<Vec<_>>()
            .join("\r\n")
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
        let data = include_bytes!("../../../test_data/keio.oud");
        let (text, _, _) = encoding_rs::SHIFT_JIS.decode(data);
        let nodes = deserialize_node(&text).unwrap();
        let file =
            RosenFileData::from_node(&Node::Directory(Directory::new_with_value("ROOT", nodes)))
                .unwrap();

        let written = file.to_oudia_string();
        let written_nodes = deserialize_node(&written).unwrap();
        RosenFileData::from_node(&Node::Directory(Directory::new_with_value(
            "ROOT",
            written_nodes,
        )))
        .unwrap();
    }
}
