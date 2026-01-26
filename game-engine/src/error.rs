use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldStorageError {
    #[error("storage for component `{0}` does not exist!")]
    ComponentStorageDoesNotExist(&'static str),
    #[error("Component `{component}` not found for entity {entity}")]
    ComponentNotFoundForEntity {
        component: &'static str,
        entity: u64,
    },
    #[error("component type mismatch. Expected: '{expected}'")]
    ComponentTypeMismatch { expected: &'static str },
    #[error("failed to insert resource `{0}`")]
    ResourceInsertError(&'static str),
}

#[derive(Error, Debug)]
pub enum WorldResourceError {
    #[error("Resource `{0}` does not exist!")]
    ResourceDoesNotExist(&'static str),
    #[error("Resource `{0}` type mismatch")]
    ResourceTypeMismatch(&'static str),
}
