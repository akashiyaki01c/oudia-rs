use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// 文字列がデコードできない
    #[error("文字列がデコードできません。")]
    UndecodedText,
    /// ディレクトリが途中で閉じている
    #[error("ディレクトリが途中で閉じています。")]
    ContainerAborted,
    /// ディレクトリが閉じていない
    #[error("ディレクトリが閉じていません。")]
    ContainerIsNotClosed,
    #[error("ファイルが空です。")]
    EmptyFile,
    #[error("未定義の構造")]
    UnknownStruct,
}
