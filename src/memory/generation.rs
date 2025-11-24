use crate::{memory::MemoryDraft, wasm::WasmCompatSend};

/// A simple trait to represent generating memories.
pub trait MemoryGeneration {
    fn generate(&self, input: &str) -> impl Future<Output = Vec<MemoryDraft>> + WasmCompatSend;
}
