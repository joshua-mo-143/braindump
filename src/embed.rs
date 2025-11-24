use crate::wasm::{WasmCompatSend, WasmCompatSync};

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
