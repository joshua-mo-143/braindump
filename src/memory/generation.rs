use serde::Serialize;
use serde_json::Map;

#[cfg(feature = "rig")]
#[cfg_attr(docsrs, doc(cfg(feature = "rig")))]
pub use rig::create_rig_memory_extractor;

use crate::{
    id_gen::{IdGenerationStrategy, MemoryIdGenerator},
    memory::{MemoryDraft, MemoryEntry},
    wasm::WasmCompatSend,
};

/// A simple trait to represent generating memories.
pub trait MemoryGeneration {
    fn generate(&self, input: &str) -> impl Future<Output = Vec<MemoryDraft>> + WasmCompatSend;
}

pub struct MemoryGenerator<IdGen, T>
where
    T: MemoryGeneration,
{
    id_generator: IdGen,
    mem_generator: T,
}

impl<T> MemoryGenerator<MemoryIdGenerator, T>
where
    T: MemoryGeneration,
{
    pub fn new(mem_generator: T) -> Self {
        Self {
            id_generator: MemoryIdGenerator::default(),
            mem_generator,
        }
    }
}

impl<IdGen, T> MemoryGenerator<IdGen, T>
where
    IdGen: IdGenerationStrategy,
    T: MemoryGeneration,
{
    pub fn into_split(self) -> (IdGen, T) {
        (self.id_generator, self.mem_generator)
    }

    pub async fn generate_memory<Input>(&mut self, memory: Input) -> Vec<MemoryEntry>
    where
        Input: Serialize,
    {
        let input = serde_json::to_string(&memory).unwrap();

        let drafts = self.mem_generator.generate(&input).await;
        let created_at = chrono::Utc::now().timestamp();

        drafts
            .into_iter()
            .map(|draft| MemoryEntry {
                id: self.id_generator.generate_id(),
                content: draft.content,
                kind: draft.kind,
                importance: draft.importance,
                created_at,
                last_accessed: created_at,
                access_count: 0,
                metadata: Map::new(),
            })
            .collect()
    }
}

#[cfg(feature = "rig")]
#[cfg_attr(docsrs, doc(cfg(feature = "rig")))]
mod rig {
    use crate::memory::{MemoryDraft, generation::MemoryGeneration};
    use rig::completion::CompletionModel;
    use rig::extractor::Extractor;

    impl<T> MemoryGeneration for Extractor<T, Vec<MemoryDraft>>
    where
        T: CompletionModel,
    {
        async fn generate(&self, input: &str) -> Vec<MemoryDraft> {
            self.extract(input).await.unwrap()
        }
    }

    /// Create memory extractor with
    pub fn create_rig_memory_extractor<Client, T>(
        client: &Client,
        model_name: &str,
    ) -> Extractor<Client::CompletionModel, Vec<MemoryDraft>>
    where
        Client: rig::client::CompletionClient,
    {
        client.extractor::<Vec<MemoryDraft>>(model_name)
            .preamble("Please extract memories from the conversation between the user and the assistant using the provided JSON format.
                Skip all prose.")
            .build()
    }
}
