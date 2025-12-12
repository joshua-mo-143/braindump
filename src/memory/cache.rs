use crate::{memory::MemoryEntry, storage::Storage, vector_store::InMemoryDB};

/// A memory cache.
/// Uses [`crate::vector_store::InMemoryDB`] internally.
pub struct MemoryCache {
    pub store: InMemoryDB,
    cache_stats: CacheStats,
    max_memory_limit: u32,
}

impl MemoryCache {
    /// Creates a new instance of [`MemoryCache`].
    /// NOTE: The max memory limit by using this method is set to 500. If you'd like to change it, please use the builder.
    pub fn new(store: InMemoryDB) -> Self {
        Self {
            store,
            cache_stats: CacheStats::new(),
            max_memory_limit: 500,
        }
    }

    /// Creates an empty builder instance for this struct
    pub fn builder() -> MemoryCacheBuilder {
        MemoryCacheBuilder::default()
    }

    /// Get the cache stats.
    pub fn stats(&self) -> &CacheStats {
        &self.cache_stats
    }

    /// The max memory limit before automatic eviction of items to make way for new cached memories.
    pub fn memory_limit(&self) -> u32 {
        self.max_memory_limit
    }

    pub fn stats_mut(&mut self) -> &mut CacheStats {
        &mut self.cache_stats
    }

    pub async fn evict_from_cache(&mut self, count: usize) -> Result<(), crate::Error> {
        const SAMPLE_SIZE: usize = 100;
        let store_len = self.store.count().await?;

        let sample_size = SAMPLE_SIZE.min(store_len);
        let candidates = self.store.random_sample(sample_size);

        // Find worst from sample
        let mut to_evict: Vec<(i64, String)> = candidates
            .into_iter()
            .map(|entry| (eviction_score(entry), entry.id.clone()))
            .collect();

        to_evict.sort_by_key(|(score, _)| *score);

        for (_, id) in to_evict.iter().take(count) {
            self.store.delete(id.to_owned()).await?;
        }

        Ok(())
    }
}

/// Generates an eviction score - the lower, the better.
/// This is used when the maximum cache size is reached and room needs to be made for new memories.
fn eviction_score(entry: &MemoryEntry) -> i64 {
    let recency = chrono::Utc::now().timestamp() - entry.last_accessed;
    let frequency = entry.access_count as i64;
    let importance = (entry.importance * 100.0) as i64;

    // Lower = more evictable
    frequency * 1000 + importance * 100 - recency
}

#[derive(Default)]
pub struct MemoryCacheBuilder {
    pub store: Option<InMemoryDB>,
    max_memory_limit: Option<u32>,
}

impl MemoryCacheBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures the cache builder to use a pre-existing in-memory database.
    pub fn store(mut self, store: InMemoryDB) -> Self {
        self.store = Some(store);
        self
    }

    /// Configures manual max memory limit.
    pub fn max_memory_limit(mut self, limit: u32) -> Self {
        self.max_memory_limit = Some(limit);
        self
    }

    // FIXME: Fix error type
    /// Build the [`MemoryCache`]. Returns an error if no store was provided.
    pub fn build(self) -> Result<MemoryCache, Box<dyn std::error::Error>> {
        let Some(store) = self.store else {
            return Err("Expected `store` to be present. You need to add an InMemoryDB to your memory cache builder.".into());
        };

        let max_memory_limit = self.max_memory_limit.unwrap_or_default();

        let res = MemoryCache {
            store,
            max_memory_limit,
            cache_stats: CacheStats::new(),
        };

        Ok(res)
    }
}

#[derive(Default)]
pub struct CacheStats {
    hits: u32,
    misses: u32,
}

impl CacheStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_hit(&mut self) {
        self.hits += 1;
    }

    pub fn add_miss(&mut self) {
        self.misses += 1;
    }

    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}
