pub enum Error {
    BuildError(BuildError),
    StorageError(StorageError),
}

impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        Self::BuildError(value)
    }
}

pub enum BuildError {
    EmbedderNotFound,
    StorageNotFound,
}

pub enum StorageError {
    EmbeddingNotExists(String),
}

impl StorageError {
    /// Create an error where an embedding with a given ID does not exist.
    pub fn embedding_not_exists(id: &str) -> Self {
        Self::EmbeddingNotExists(id.to_string())
    }
}

impl From<StorageError> for Error {
    fn from(value: StorageError) -> Self {
        Self::StorageError(value)
    }
}
