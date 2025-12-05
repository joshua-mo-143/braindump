use braindump::{
    embed::RigEmbedder,
    memory::{generation::MemoryGenerator, manager::MemoryManager},
    vector_store::InMemoryDB,
};
use rig::client::{EmbeddingsClient, ProviderClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = rig::providers::openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-3-small");

    let storage = InMemoryDB::new(1536);

    let mut memory_mgr = MemoryManager::builder()
        .embedder(RigEmbedder::new(embedding_model))
        .storage(storage)
        .build()?;

    // Here we're using a Vec<str> for brevity
    // however in a *real* application, you may use your message history
    let chat_history = vec![
        "User: Can you help me write a Rust program?",
        "Assistant: Of course! What would you like to write today?",
        "User: Please help me write a simple web server using Axum.",
    ];

    // FIXME: This should be made much easier to use before release!
    let ext = braindump::memory::generation::create_rig_memory_extractor::<
        rig::client::Client<rig::providers::openai::OpenAIResponsesExt>,
        T,
    >(&openai_client, "gpt-5");
    let mut memory_gen = MemoryGenerator::new(ext);

    let memories = memory_gen.generate_memory(chat_history).await;
    println!("Memories generated: {memories:?}");

    for memory in memories {
        memory_mgr.store(memory.content, memory).await?;
    }

    let res = memory_mgr.retrieve("Rust web server", 1).await?;
    println!("Found result: {res:?}");

    Ok(())
}
