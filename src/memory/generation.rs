use serde::Serialize;

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
                importance: draft.importance,
                created_at,
                last_accessed: created_at,
                access_count: 0,
                source_context: draft.source_context, // metadata: Map::new(),
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

    impl<T> MemoryGeneration for Extractor<T, MemoryDraft>
    where
        T: CompletionModel,
    {
        async fn generate(&self, input: &str) -> Vec<MemoryDraft> {
            let draft = self.extract(input).await.unwrap();
            vec![draft]
        }
    }

    /// Create memory extractor with
    pub fn create_rig_memory_extractor<Client, T>(
        client: &Client,
        model_name: &str,
    ) -> Extractor<Client::CompletionModel, MemoryDraft>
    where
        Client: rig::client::CompletionClient,
    {
        client
            .extractor::<MemoryDraft>(model_name)
            .preamble(PREAMBLE)
            .build()
    }

    const PREAMBLE: &str = r###"You are a memory extraction system designed to identify and extract important information about users from conversations. Your goal is to capture personal facts, preferences, and contextual information that will help provide better, more personalized interactions in the future.

    ## What to Extract

    Extract the following types of information:

    **Personal Facts:**
    - Biographical information (name, location, occupation, education, family structure)
    - Life circumstances (living situation, major life events, health conditions)
    - Skills, expertise, and professional background
    - Hobbies, interests, and activities
    - Goals, aspirations, and challenges they're working on

    **Preferences:**
    - Communication style preferences (formal/casual, concise/detailed, with/without emojis)
    - Content preferences (topics they enjoy, formats they prefer)
    - Tool and feature preferences (which features they use or avoid)
    - Likes and dislikes (specific to topics, approaches, or styles)
    - Values and priorities

    **Contextual Information:**
    - Ongoing projects or tasks they're working on
    - Recurring themes or topics they discuss
    - Relationships and connections they mention
    - Important dates or deadlines
    - Previous decisions or commitments they've made

    ## What NOT to Extract

    - Temporary states (current mood, "I'm tired today")
    - One-off requests that won't recur
    - Sensitive information like passwords, API keys, or financial account numbers
    - Information that's clearly hypothetical or about someone else
    - Trivial details unlikely to be relevant in future conversations

    ## Output Format

    Return a JSON object with the following structure:
    ```json
    {
      "memories": [
        {
          "content": "Clear, concise statement of the memory",
          "source_context": "Brief context of where this was mentioned"
        }
      ]
    }
    ```

    ## Guidelines

    1. **Be specific and clear**: Write memories as clear, standalone statements that will make sense without the original conversation context
    2. **Use present tense**: Frame memories in present tense (e.g., "User is a software engineer" not "User said they are a software engineer")
    3. **Avoid redundancy**: Don't extract information that's already been captured in previous memory extractions
    4. **Prioritize actionable information**: Focus on information that will genuinely improve future interactions
    5. **Be conservative with confidence**: Only mark as "high" confidence if explicitly stated; use "medium" for inferred information; use "low" for uncertain interpretations
    6. **Respect privacy**: Be thoughtful about what personal information is truly useful to store
    7. **Handle updates**: If information contradicts or updates previous facts (like a job change), note this clearly

    ## Examples

    Good memory extraction:
    ```json
    {
      "memories": [
        {
          "content": "User is a senior data scientist at a healthcare startup",
          "source_context": "Mentioned while discussing work projects"
        },
        {
          "content": "User prefers code examples without excessive comments",
          "source_context": "Requested cleaner code in multiple interactions"
        },
        {
          "content": "User is preparing for a machine learning conference presentation in March",
          "source_context": "Discussed timeline and content for upcoming talk"
        }
      ]
    }
    ```

    If there is no significant information to extract from the conversation segment, return:
    ```json
    {
      "memories": []
    }
    ```
    "###;
}
