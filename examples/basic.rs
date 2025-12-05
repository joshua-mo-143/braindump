use braindump::{
    fastembed::FastembedTextEmbedder,
    memory::{MemoryEntry, MemoryKind, manager::MemoryManager},
    vector_store::InMemoryDB,
};
use serde_json::Map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default model here should be `bge-small-en-v-1.5`
    // default size is 384 dims
    let model = FastembedTextEmbedder::default();
    let storage = InMemoryDB::new(384);

    let mut memory = MemoryManager::builder()
        .embedder(model)
        .storage(storage)
        .build()?;

    let memory_contents = "User likes rabbits".to_string();

    let memory_entry = MemoryEntry {
        id: "001".to_string(),
        content: memory_contents.clone(),
        kind: MemoryKind::Episodic,
        importance: 0.8,
        created_at: 1764632395,
        last_accessed: 1764632395,
        access_count: 0,
        metadata: Map::new(),
    };

    memory.store(memory_contents, memory_entry).await?;

    let res = memory.retrieve("What animals are the best?", 1).await?;
    println!("Results: {res:?}");

    Ok(())
}
