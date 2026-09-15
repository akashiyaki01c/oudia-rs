use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Node type is different")]
    NodeTypeError,

    #[error("Rosen is not found")]
    RosenIsNotFound,
    #[error("DispProp is not found")]
    DispPropIsNotFound,
    #[error("DiagramDgrYZahyouKyoriDefault")]
    DiagramDgrYZahyouKyoriDefaultError,

    #[error("TODO")]
    TodoError,
}
