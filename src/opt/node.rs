use crate::opt::{directory::Directory, property::Property};

// OuPropertiesText における PropertyとDirectotyを表す列挙体
pub enum Node {
    Property(Property),
    Directory(Directory),
}
