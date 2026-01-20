use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldStorageError {
    #[error("storage for component `{0}` does not exist!")]
    ComponentStorageDoesNotExist(&'static str),
    #[error("component storage type `{0}` does not match")]
    ComponentStorageTypeMismatch(&'static str),
    #[error("failed to insert resource `{0}`")]
    ResourceInsertError(&'static str),
}
