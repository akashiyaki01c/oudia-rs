use itertools::Itertools;

use crate::opt::node::Node;

/// プロパティの集合を表す
#[derive(Debug, PartialEq, Clone)]
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
        !name.contains(".") && !name.contains("=") && !name.contains("\n")
    }

    /// ディレクトリが配列であるか
    pub fn is_array(&self) -> bool {
        self.values.iter().map(|v| v.get_name()).all_equal()
    }

	/// ディレクトリが構造体であるか
	pub fn is_struct(&self) -> bool {
		self.values.iter().map(|v| v.get_name()).all_unique()
	}

    pub fn find<'a>(&'a self, name: &str) -> Option<&'a Node> {
        self.values.iter().find(|v| v.get_name() == name)
    }

    pub fn find_all<'a>(&'a self, name: &str) -> Vec<&'a Node> {
        self.values.iter().filter(|v| v.get_name() == name).collect()
    }
}
