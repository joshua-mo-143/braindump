use braindump::{
    fastembed::FastembedTextEmbedder,
    memory::{Confidence, MemoryEntry, MemoryKind, manager::MemoryManager},
    vector_store::InMemoryDB,
};

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
        confidence: Confidence::High,
        metadata: Vec::new(),
        source_context: "Generated for the purposes of testing".to_string(),
    };

    memory.store(memory_contents, memory_entry).await?;

    let res = memory.retrieve("What animals are the best?", 1).await?;
    println!("Results: {res:?}");

    Ok(())
}
