use crate::opt::node::Node;

/// プロパティの集合を表す
pub struct Directory {
    /// ディレクトリの名前
    pub name: String,
    /// プロパティの集合
    pub values: Vec<Node>,
}

impl Directory {
    /// 新しいプロパティを生成する関数
    pub fn new(name: &str) -> Self {
        assert!(Self::is_valid_name(name));
        Self {
            name: name.to_string(),
            values: vec![],
        }
    }

	/// 新しいプロパティを生成する関数
    pub fn new_with_value(name: &str, values: Vec<Node>) -> Self {
        assert!(Self::is_valid_name(name));
        Self {
            name: name.to_string(),
            values,
        }
    }

    /// 文字列がプロパティのキーに使用できるか
    fn is_valid_name(name: &str) -> bool {
        name.contains(".") || name.contains("=") || name.contains("\n")
    }
}
