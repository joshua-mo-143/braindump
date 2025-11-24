use crate::{
    memory::MemoryEntry,
    wasm::{WasmCompatSend, WasmCompatSync},
};

/// Handle storage.
/// This should be implemented for vector stores as well as any databases that have vector database functionality.
pub trait Storage: WasmCompatSend + WasmCompatSync {
    /// Insert a document
    fn insert(
        &mut self,
        embedding: Vec<f32>,
        entry: MemoryEntry,
    ) -> impl Future<Output = ()> + WasmCompatSend;
    /// Search (typically, using semantic search)
    fn search(
        &self,
        embedding: Vec<f32>,
        limit: usize,
    ) -> impl Future<Output = Vec<MemoryEntry>> + WasmCompatSend;
    /// Search the storage by ID and get the embedding as well as the memory entry
    fn search_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<(Vec<f32>, MemoryEntry), crate::Error>> + WasmCompatSend;
    /// Search for all recent inserts
    fn get_recent(&self, limit: usize) -> impl Future<Output = Vec<MemoryEntry>> + WasmCompatSend;

    /// Delete a document (by ID)
    fn delete(&mut self, id: String) -> impl Future<Output = ()> + WasmCompatSend;
    /// Delete multiple documents (by ID)
    fn delete_batch(&mut self, ids: Vec<String>) -> impl Future<Output = ()> + WasmCompatSend;
    /// Get documents with the oldest inserts
    fn get_oldest(&self, limit: usize) -> impl Future<Output = Vec<MemoryEntry>> + WasmCompatSend;

    /// Update a payload by ID
    fn update_payload_by_id(
        &mut self,
        id: String,
        payload: MemoryEntry,
    ) -> impl Future<Output = ()> + WasmCompatSend;
    /// Get the total count of storage
    fn count(&self) -> impl Future<Output = usize> + WasmCompatSend;
}

/// A placeholder struct to show that the storage type has not been set.
/// This will automatically return an error if the user tries to use it to build a `MemoryManager`, and will automatically return an `unimplemented`-related panic should you try to use its `Storage` implementation.
pub struct StorageNotSet;

impl Storage for StorageNotSet {
    async fn count(&self) -> usize {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn delete(&mut self, _: String) {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn delete_batch(&mut self, _: Vec<String>) -> () {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn get_oldest(&self, _: usize) -> Vec<MemoryEntry> {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn get_recent(&self, _: usize) -> Vec<MemoryEntry> {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn insert(&mut self, _: Vec<f32>, _: MemoryEntry) -> () {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn search(&self, _: Vec<f32>, _: usize) -> Vec<MemoryEntry> {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn search_by_id(&self, _: String) -> Result<(Vec<f32>, MemoryEntry), crate::Error> {
        unimplemented!("You need to set a proper storage type!");
    }

    async fn update_payload_by_id(&mut self, _: String, _: MemoryEntry) {
        unimplemented!("You need to set a proper storage type!");
    }
}
