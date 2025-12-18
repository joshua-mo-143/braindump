use serde::{Deserialize, Serialize};

pub mod cache;
pub mod generation;
pub mod manager;

/// A memory entry (ie, a summarized version of a conversation).
///
/// It is generally advised that the contents of an agent memory be generated from an LLM as the contents are often very non-deterministic.
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    /// Memory ID
    pub id: String,
    /// The content of the memory (eg, a fact or a summarization of a previous conversation).
    pub content: String,
    pub kind: MemoryKind,
    /// How important the memory is (using a decimal number between 0.0 and 1.0).
    pub importance: f32,
    /// Whenever the memory was created (as a Unix timestamp).
    pub created_at: i64,
    /// Whenever the memory was last accessed (as a Unix timestamp).
    pub last_accessed: i64,
    /// However many times the memory has been accessed (as a Unix timestamp).
    pub access_count: u32,
    /// The context in which this memory has been created
    pub source_context: String,
    pub confidence: Confidence,
    /// Any additional metadata
    pub metadata: Vec<MetadataEntry>,
}

/// The type of memory.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub enum MemoryKind {
    /// Working memory (ie, stuff that's in the current context window)
    Working,
    /// Past conversations and events
    Episodic,
    /// Facts and ground truths
    Semantic,
}

/// A memory entry draft.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MemoryDraft {
    /// The content of the memory (eg, a fact or a summarization of a previous conversation).
    pub content: String,
    pub kind: MemoryKind,
    /// The context in which this memory has been created
    pub source_context: String,
    /// How important the memory is (using a decimal number between 0.0 and 1.0).
    pub importance: f32,
    pub confidence: Confidence,
    /// Any additional metadata
    pub metadata: Vec<MetadataEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MetadataEntry {
    key: String,
    value: String,
}

/// A confidence score (provided by an LLM). Can either be low, medium or high.
/// Represents the LLM's confidence about a fact or conversation history observation.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub enum Confidence {
    Low,
    Medium,
    High,
}
