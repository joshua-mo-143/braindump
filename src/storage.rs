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
    ) -> impl Future<Output = Result<(), crate::Error>> + WasmCompatSend;
    /// Search (typically, using semantic search)
    fn search(
        &self,
        embedding: Vec<f32>,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<MemoryEntry>, crate::Error>> + WasmCompatSend;
    /// Search the storage by ID and get the embedding as well as the memory entry
    fn search_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<(Vec<f32>, MemoryEntry), crate::Error>> + WasmCompatSend;
    /// Search for all recent inserts
    fn get_recent(
        &self,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<MemoryEntry>, crate::Error>> + WasmCompatSend;

    /// Delete a document (by ID)
    fn delete(
        &mut self,
        id: String,
    ) -> impl Future<Output = Result<(), crate::Error>> + WasmCompatSend;
    /// Delete multiple documents (by ID)
    fn delete_batch(
        &mut self,
        ids: Vec<String>,
    ) -> impl Future<Output = Result<(), crate::Error>> + WasmCompatSend;
    /// Get documents with the oldest inserts
    fn get_oldest(
        &self,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<MemoryEntry>, crate::Error>> + WasmCompatSend;

    /// Update a payload by ID
    fn update_payload_by_id(
        &mut self,
        id: String,
        payload: MemoryEntry,
    ) -> impl Future<Output = Result<(), crate::Error>> + WasmCompatSend;
    /// Get the total count of storage
    fn count(&self) -> impl Future<Output = Result<usize, crate::Error>> + WasmCompatSend;
}

/// A placeholder struct to show that the storage type has not been set.
/// Attempted usage will result in a `NoOp` error as the purpose of this type is essentially to assist with generic builder typing.
pub struct StorageNotSet;

impl Storage for StorageNotSet {
    async fn count(&self) -> Result<usize, crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn delete(&mut self, _: String) -> Result<(), crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn delete_batch(&mut self, _: Vec<String>) -> Result<(), crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn get_oldest(&self, _: usize) -> Result<Vec<MemoryEntry>, crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn get_recent(&self, _: usize) -> Result<Vec<MemoryEntry>, crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn insert(&mut self, _: Vec<f32>, _: MemoryEntry) -> Result<(), crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn search(&self, _: Vec<f32>, _: usize) -> Result<Vec<MemoryEntry>, crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn search_by_id(&self, _: String) -> Result<(Vec<f32>, MemoryEntry), crate::Error> {
        Err(crate::Error::NoOp)
    }

    async fn update_payload_by_id(
        &mut self,
        _: String,
        _: MemoryEntry,
    ) -> Result<(), crate::Error> {
        Err(crate::Error::NoOp)
    }
}
