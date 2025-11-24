use chrono::Utc;
use serde_json::{Map, Value};

use crate::memory::manager::MemoryConfig;

pub mod generation;
pub mod manager;

/// A memory entry.
///
/// It is generally advised that the contents of an agent memory be generated from an LLM as the contents are often very non-deterministic.
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    /// Memory ID
    pub id: String,
    /// The content of the memory (eg, a fact or a summarization of a previous conversation).
    pub content: String,
    /// The kind of memory that this memory represents (eg working memory, episodic memory or semantic memory)
    pub kind: MemoryKind,
    /// How important the memory is.
    pub importance: f32,
    /// Whenever the memory was created (as a Unix timestamp).
    pub created_at: i64,
    /// Whenever the memory was last accessed (as a Unix timestamp).
    pub last_accessed: i64,
    /// However many times the memory has been accessed (as a Unix timestamp).
    pub access_count: u32,
    /// Any additional metadata (using `serde_json::Map`)
    pub metadata: Map<String, Value>,
}

impl MemoryEntry {
    /// Whether or not a memory entry should be cached.
    /// Currently, it uses a simple time-based loop.
    /// This will be improved in future since this is quite basic and we could probably do better.
    pub fn should_cache(&self, cfg: &MemoryConfig) -> bool {
        let current_time = Utc::now().timestamp();
        let age = current_time - self.created_at;
        age >= cfg.cache_window
    }
}

/// The type of memory.
#[derive(Clone, Debug)]
pub enum MemoryKind {
    /// Working memory (ie, stuff that's in the current context window)
    Working,
    /// Past conversations and events
    Episodic,
    /// Facts and ground truths
    Semantic,
}

/// A memory entry draft.
#[derive(Clone, Debug)]
pub struct MemoryDraft {
    /// The content of the memory (eg, a fact or a summarization of a previous conversation).
    pub content: String,
    /// Any additional metadata (using `serde_json::Map`)
    pub metadata: Map<String, Value>,
}
