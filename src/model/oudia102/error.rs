use thiserror::Error;

/// モデル構築・操作上のエラーを表す構造体
#[derive(Error, Debug)]
pub enum Error {
    #[error("Node type is different")]
    NodeTypeError,
    #[error("version is invalid")]
    InvalidVersion,

    #[error("Rosen is not found")]
    RosenIsNotFound,
    #[error("DispProp is not found")]
    DispPropIsNotFound,
    #[error("DiagramDgrYZahyouKyoriDefault")]
    DiagramDgrYZahyouKyoriDefaultError,

    #[error("TODO")]
    TodoError,
}
