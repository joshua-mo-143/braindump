use crate::wasm::{WasmCompatSend, WasmCompatSync};

#[cfg(feature = "rig")]
#[cfg_attr(docsrs, doc(cfg(feature = "rig")))]
pub use rig::RigEmbedder;

/// A trait for generically abstracting embeddings over different kinds of embedder types (whether local or managed models, or if you're using a pipeline).
pub trait Embedder: WasmCompatSend + WasmCompatSync {
    fn embed_text(
        &self,
        input: &str,
    ) -> impl Future<Output = Result<Vec<f32>, crate::Error>> + WasmCompatSend;
}

/// A no-op struct for the embedder type.
/// Attempted usage will result in a `NoOp` error as the purpose of this type is essentially to assist with generic builder typing.
pub struct EmbedderNotSet;

impl Embedder for EmbedderNotSet {
    async fn embed_text(&self, _: &str) -> Result<Vec<f32>, crate::Error> {
        Err(crate::Error::NoOp)
    }
}

#[cfg(feature = "rig")]
#[cfg_attr(docsrs, doc(cfg(feature = "rig")))]
mod rig {
    use super::Embedder;
    use rig::embeddings::EmbeddingModel;

    pub struct RigEmbedder<T>
    where
        T: EmbeddingModel,
    {
        inner: T,
    }

    impl<T> RigEmbedder<T>
    where
        T: EmbeddingModel,
    {
        pub fn new(inner: T) -> Self {
            Self { inner }
        }
    }

    impl<T> Embedder for RigEmbedder<T>
    where
        T: EmbeddingModel,
    {
        async fn embed_text(&self, input: &str) -> Result<Vec<f32>, crate::Error> {
            let res = self
                .inner
                .embed_text(input)
                .await
                .unwrap()
                .vec
                .into_iter()
                .map(|x| x as f32)
                .collect();

            Ok(res)
        }
    }
}
