use crate::opt::{directory::Directory, property::Property};

// OuPropertiesText における PropertyとDirectotyを表す列挙体
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Property(Property),
    Directory(Directory),
}

impl Node {
    /// 名前を取得する関数
    pub fn get_name(&self) -> &str {
        match &self {
            Node::Directory(directory) => &directory.name,
            Node::Property(property) => &property.name,
        }
    }
}
