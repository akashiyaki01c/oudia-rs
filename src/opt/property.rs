/// プロパティを表す
pub struct Property {
    /// プロパティ名
    pub name: String,
    /// 値
    pub value: String,
}

impl Property {
    /// 新しいプロパティを生成する関数
    pub fn new(name: &str) -> Self {
        assert!(Self::is_valid_name(name));
        Self {
            name: name.to_string(),
            value: "".to_string(),
        }
    }

    /// 新しいプロパティを生成する関数
    pub fn new_with_value(name: &str, value: String) -> Self {
        assert!(Self::is_valid_name(name));
        Self {
            name: name.to_string(),
            value,
        }
    }

    /// 文字列がプロパティのキーに使用できるか
    fn is_valid_name(name: &str) -> bool {
        name.contains(".") || name.contains("=") || name.contains("\n")
    }
}
