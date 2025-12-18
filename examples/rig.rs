use braindump::{
    embed::RigEmbedder,
    memory::{generation::MemoryGenerator, manager::MemoryManager},
    vector_store::InMemoryDB,
};
use rig::{
    client::{EmbeddingsClient, ProviderClient},
    parallel_internal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = rig::providers::openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-3-small");

    let storage = InMemoryDB::new(1536);

    println!("Setup initialised");

    let mut memory_mgr = MemoryManager::builder()
        .embedder(RigEmbedder::new(embedding_model))
        .storage(storage)
        .build()?;

    println!("Memory manager initialised");

    // Here we're using a Vec<str> for brevity
    // however in a *real* application, you may use your message history
    let chat_history = vec![
        "User: Can you help me write a Rust program?",
        "Assistant: Of course! What would you like to write today?",
        "User: Please help me write a simple web server using Axum.",
    ];

    let ext =
        braindump::memory::generation::create_rig_memory_extractor(&openai_client, "gpt-5-mini");
    let mut memory_gen = MemoryGenerator::new(ext);

    println!("Generating memories...");
    let memories = memory_gen.generate_memory(chat_history).await;
    println!("Memories generated: {memories:?}");

    for memory in memories {
        memory_mgr.store(memory.content.clone(), memory).await?;
    }

    let res = memory_mgr.retrieve("Rust web server", 1).await?;
    println!("Found result: {res:?}");

    Ok(())
}
