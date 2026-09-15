use crate::{
    model::{disp_prop::DispProp, error::Error, rosen::Rosen},
    opt::node::Node,
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
            for node in &dir.values {
                println!("{},{:?}", node.get_name(), node.get_name().as_bytes());
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    OuDia02,
    OuDia03,
    OuDia05,
    /// `OuDia.6`
    OuDia06,
    /// `OuDia.1.01`
    OuDia101,
    /// `OuDia.1.02`
    OuDia102,
}
