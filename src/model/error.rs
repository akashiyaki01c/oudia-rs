use thiserror::Error;

/// モデル構築・操作上のエラーを表す構造体
#[derive(Error, Debug)]
pub enum Error {
    #[error("Node type is different")]
    NodeTypeError,
    #[error("version is invalid")]
    InvalidVersion,
    #[error("{0} is not found")]
    KeyIsNotFound(String),
    #[error("property {0} is invalid value \"{1}\"")]
    InvalidValue(String, String),
    #[error("value of \"{0}\" is empty")]
    EmptyValue(String),
    #[error("value of \"{field}\" is not a number: \"{value}\"")]
    InvalidNumber { field: String, value: String },
    #[error("value of \"{field}\" is not a valid enum value: \"{value}\"")]
    InvalidEnum { field: String, value: String },
    #[error("color is invalid: \"{0}\"")]
    InvalidColor(String),
    #[error("{context} has invalid format: \"{value}\"")]
    InvalidFormat { context: String, value: String },
    #[error("property \"{0}\" must be an array")]
    ExpectedArray(String),
}
