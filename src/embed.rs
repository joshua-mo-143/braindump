use crate::wasm::{WasmCompatSend, WasmCompatSync};

/// A trait for generically abstracting embeddings over
pub trait Embedder: WasmCompatSend + WasmCompatSync {
    fn embed_text(&self, input: &str) -> impl Future<Output = Vec<f32>> + WasmCompatSend;
}

/// A no-op struct for the embedder type. Used as part of the builder (to assist with generics wrangling). Attempting to use this as an embedder will return `unimplemented!()`-type errors.
pub struct EmbedderNotSet;

impl Embedder for EmbedderNotSet {
    async fn embed_text(&self, _: &str) -> Vec<f32> {
        unimplemented!("Use a proper embedder type!")
    }
}
