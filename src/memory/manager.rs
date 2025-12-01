use chrono::Utc;

use crate::{
    embed::{Embedder, EmbedderNotSet},
    error::BuildError,
    memory::MemoryEntry,
    storage::{Storage, StorageNotSet},
    vector_store::InMemoryDB,
};

pub struct MemoryManager<E, S>
where
    E: Embedder,
    S: Storage,
{
    storage: S,
    embedder: E,
    cfg: MemoryConfig,
    hot_cache: Option<InMemoryDB>,
}

impl MemoryManager<EmbedderNotSet, StorageNotSet> {
    /// Creates a new instance of `MemoryManagerBuilder`.
    pub fn builder() -> MemoryManagerBuilder<EmbedderNotSet, StorageNotSet> {
        MemoryManagerBuilder::new()
    }
}

impl<E, S> MemoryManager<E, S>
where
    E: Embedder,
    S: Storage,
{
    /// Store a single memory.
    pub async fn store<AsRefStr>(
        &mut self,
        memory: AsRefStr,
        entry: MemoryEntry,
    ) -> Result<(), crate::Error>
    where
        AsRefStr: AsRef<str>,
    {
        let embedding = self.embedder.embed_text(memory.as_ref()).await?;
        self.storage
            .insert(embedding.clone(), entry.clone())
            .await?;

        if let Some(cache) = &mut self.hot_cache
            && entry.should_cache(&self.cfg)
        {
            cache.insert(embedding, entry).await?;
        }

        Ok(())
    }

    /// Retrieve memories, given a query and a limit for number of returned memories.
    pub async fn retrieve<AsRefStr>(
        &mut self,
        query: AsRefStr,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>, crate::Error>
    where
        AsRefStr: AsRef<str>,
    {
        let embedding = self.embedder.embed_text(query.as_ref()).await?;

        let mut results = if let Some(cache) = &mut self.hot_cache {
            cache.search(embedding.clone(), limit).await?
        } else {
            Vec::new()
        };

        if results.len() < limit {
            let deep_results = self
                .storage
                .search(embedding, limit - results.len())
                .await?;
            results.extend(deep_results);
        }

        Ok(results)
    }

    /// Updates a memory and checks if it needs to be hot cached.
    pub async fn update_memory_access(
        &mut self,
        mut memory: MemoryEntry,
    ) -> Result<(), crate::Error> {
        memory.last_accessed = Utc::now().timestamp();
        memory.access_count += 1;

        if memory.should_cache(&self.cfg)
            && let Some(cache) = &mut self.hot_cache
        {
            cache
                .update_payload_by_id(memory.id.clone(), memory)
                .await?;
        }

        Ok(())
    }
}

/// A builder for `MemoryManager`.
#[derive(Default)]
pub struct MemoryManagerBuilder<E, S> {
    storage: Option<S>,
    embedder: Option<E>,
    cfg: Option<MemoryConfig>,
    hot_cache: Option<InMemoryDB>,
}

impl MemoryManagerBuilder<EmbedderNotSet, StorageNotSet> {
    pub fn new() -> Self {
        MemoryManagerBuilder {
            storage: None,
            embedder: None,
            cfg: None,
            hot_cache: None,
        }
    }
}

impl<E, S> MemoryManagerBuilder<E, S>
where
    E: Embedder,
    S: Storage,
{
    pub fn storage<S2>(self, storage: S2) -> MemoryManagerBuilder<E, S2>
    where
        S2: Storage,
    {
        MemoryManagerBuilder {
            storage: Some(storage),
            embedder: self.embedder,
            cfg: self.cfg,
            hot_cache: self.hot_cache,
        }
    }

    pub fn embedder<E2>(self, embedder: E2) -> MemoryManagerBuilder<E2, S>
    where
        E2: Embedder,
    {
        MemoryManagerBuilder {
            storage: self.storage,
            embedder: Some(embedder),
            cfg: self.cfg,
            hot_cache: self.hot_cache,
        }
    }

    pub fn config(mut self, cfg: MemoryConfig) -> Self {
        self.cfg = Some(cfg);

        self
    }

    pub fn hot_cache(mut self, cache: InMemoryDB) -> Self {
        self.hot_cache = Some(cache);
        self
    }

    pub fn build(self) -> Result<MemoryManager<E, S>, crate::Error> {
        let Some(storage) = self.storage else {
            return Err(BuildError::StorageNotFound)?;
        };

        let Some(embedder) = self.embedder else {
            return Err(BuildError::EmbedderNotFound)?;
        };

        let cfg = if let Some(cfg) = self.cfg {
            cfg
        } else {
            MemoryConfig::new()
        };

        let mgr = MemoryManager {
            storage,
            embedder,
            cfg,
            hot_cache: self.hot_cache,
        };

        Ok(mgr)
    }
}

/// Memory manager configuration
pub struct MemoryConfig {
    /// The maximum number of total memories (don't store any more after max has been reached)
    pub max_total_memories: Option<usize>,
    /// Delete memories after N days
    pub max_age_days: Option<i64>,
    /// The minimum score required to keep a given memory
    pub min_retention_score: Option<f32>,
    /// How many to evict during eviction
    pub eviction_batch_size: usize,
    /// How many seconds a memory should stay in the cache window (default is 1 hour)
    pub cache_window: i64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryConfig {
    pub fn new() -> Self {
        Self {
            max_total_memories: None,
            max_age_days: None,
            min_retention_score: None,
            eviction_batch_size: 1,
            cache_window: 3600,
        }
    }
}
