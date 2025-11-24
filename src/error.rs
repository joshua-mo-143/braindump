use std::fmt::{self};

/// Any kind of error.
#[derive(Clone, Debug)]
pub enum Error {
    Build(BuildError),
    Storage(StorageError),
    Custom(String),
    NoOp,
}

impl Error {
    pub fn custom(input: &str) -> Self {
        Self::Custom(input.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Build(err) => write!(f, "{err}"),
            Self::Storage(err) => write!(f, "{err}"),
            Self::Custom(err) => write!(f, "{err}"),
            Self::NoOp => write!(f, "Type has no implementation"),
        }
    }
}

impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        Self::Build(value)
    }
}

impl From<StorageError> for Error {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

#[derive(Clone, Debug)]
pub enum BuildError {
    EmbedderNotFound,
    StorageNotFound,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmbedderNotFound => write!(f, "Embedder not found"),
            Self::StorageNotFound => write!(f, "Storage not found"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum StorageError {
    EmbeddingNotExists(String),
    MismatchedDimensions(usize, usize),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmbeddingNotExists(id) => {
                write!(f, "Embedding with ID {id} doesn't exist in storage")
            }
            Self::MismatchedDimensions(store_dims, embed_dims) => {
                write!(
                    f,
                    "Mismatched dimensions when trying to store an embedding: {store_dims}, {embed_dims}"
                )
            }
        }
    }
}

impl StorageError {
    /// Create an error where an embedding with a given ID does not exist.
    pub fn embedding_not_exists(id: &str) -> Self {
        Self::EmbeddingNotExists(id.to_string())
    }

    /// Create an error where the vector store dimensions and the dimensions of the vector to be added do not match up.
    pub fn mismatched_dimensions(store_dims: usize, embed_dims: usize) -> Self {
        Self::MismatchedDimensions(store_dims, embed_dims)
    }
}
