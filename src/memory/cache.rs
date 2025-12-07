use crate::{memory::MemoryEntry, storage::Storage, vector_store::InMemoryDB};

/// A memory cache.
/// Uses
pub struct MemoryCache {
    pub store: InMemoryDB,
    cache_stats: CacheStats,
    pub max_memory_limit: u32,
}

impl MemoryCache {
    pub fn new(store: InMemoryDB) -> Self {
        Self {
            store,
            cache_stats: CacheStats::new(),
            max_memory_limit: 10,
        }
    }

    pub fn stats(&self) -> &CacheStats {
        &self.cache_stats
    }

    pub fn stats_mut(&mut self) -> &mut CacheStats {
        &mut self.cache_stats
    }

    pub async fn evict_from_cache(&mut self, count: usize) {
        const SAMPLE_SIZE: usize = 100;
        // SAFETY: This shouldn't panic unless it's NaN, which is fine as this is
        // currently not intended to be used in production
        // TODO: Should we make the Cache API sync?
        let store_len = self.store.count().await.unwrap();

        let sample_size = SAMPLE_SIZE.min(store_len);
        let candidates = self.store.random_sample(sample_size);

        // Find worst from sample
        let mut to_evict: Vec<(i64, String)> = candidates
            .into_iter()
            .map(|entry| (eviction_score(entry), entry.id.clone()))
            .collect();

        to_evict.sort_by_key(|(score, _)| *score);

        for (_, id) in to_evict.iter().take(count) {
            // SAFETY: This shouldn't panic unless it's NaN, which is fine as this is
            // currently not intended to be used in production
            // TODO: This should be un-unwrapped once we reach 0.1.0
            self.store.delete(id.to_owned()).await.unwrap();
        }
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

pub struct CacheStats {
    hits: u32,
    misses: u32,
}

impl CacheStats {
    pub fn new() -> Self {
        Self { hits: 0, misses: 0 }
    }

    pub fn add_hit(&mut self) {
        self.hits += 1;
    }

    pub fn add_miss(&mut self) {
        self.misses += 1;
    }
}
