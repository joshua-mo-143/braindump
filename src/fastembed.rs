//! A module for handling integrations with `fastembed-rs`.
//! Ensure that you have the `fastembed` feature enabled.
//! NOTE: This module is not WASM-friendly. Attempting to compile this module to `wasm` architecture will return an error.

use fastembed::TextEmbedding;
use std::sync::Arc;
use std::sync::Mutex;

/// A text embedder using `fastembed-rs`, made compliant to work with the `Embedder` trait.
/// Under the hood, `std::sync::Arc` and `std::sync::Mutex` are used due to `fastembed::TextEmbedding` requiring `&mut self` to embed.
pub struct FastembedTextEmbedder(Arc<Mutex<TextEmbedding>>);

impl Default for FastembedTextEmbedder {
    fn default() -> Self {
        let model = TextEmbedding::try_new(Default::default()).unwrap();

        Self(Arc::new(Mutex::new(model)))
    }
}

impl FastembedTextEmbedder {
    /// Creates a new instance of `FastembedTextEmbedder`.
    pub fn new(embedder: TextEmbedding) -> Self {
        Self(Arc::new(Mutex::new(embedder)))
    }
}

impl From<TextEmbedding> for FastembedTextEmbedder {
    fn from(embedder: TextEmbedding) -> Self {
        Self::new(embedder)
    }
}

impl crate::embed::Embedder for FastembedTextEmbedder {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>, crate::Error> {
        let embedding = self.0.lock().unwrap().embed(vec![text], None).unwrap();

        Ok(embedding.first().cloned().unwrap())
    }
}
