//! A Rust implementation of an in-memory vector store.

use std::collections::HashMap;

use crate::{memory::MemoryEntry, storage::Storage};

/// An in-memory vector store database. Used to store embeddings.
/// This data structure primarily stores vectors as one long piece of contiguous memory, using separate hashmaps for entries, indexes as well as a separate vector for getting positions of soft-deleted payloads.
pub struct InMemoryDB {
    /// The dimensions of the contained embeddings.
    dim: usize,
    /// The embedding data. Length is calculated by the dimension number plus the number of keys in `id_to_idx` + `free_list`.
    data: Vec<f32>,
    /// A hashmap of currently existing string keys that map to a payload.
    payloads: HashMap<String, MemoryEntry>,
    /// A hashmap of currently existing string keys that map to a position in `data`. The value represents the starting position of the vec.
    id_to_idx: HashMap<String, usize>,
    /// A list of "deleted" keys. We keep these in memory because deleting the vec data in question and shifting everything along may become an extremely computationally intensive process when dealing with even just tens of thousands or hundreds of thousands of embeddings.
    free_list: Vec<usize>,
}

impl InMemoryDB {
    pub fn new(dim: usize) -> Self {
        let data = Vec::new();
        let id_to_idx = HashMap::new();
        let payloads = HashMap::new();
        let free_list = Vec::new();

        Self {
            dim,
            data,
            payloads,
            id_to_idx,
            free_list,
        }
    }

    fn matches_dim_size<R>(&self, embedding: R) -> bool
    where
        R: AsRef<[f32]>,
    {
        let array = embedding.as_ref();

        array.len() == self.dim
    }
}

impl Storage for InMemoryDB {
    async fn insert(&mut self, embedding: Vec<f32>, entry: crate::memory::MemoryEntry) {
        if !self.matches_dim_size(&embedding) {
            unimplemented!("Handle mismatching dimension sizes");
        }

        let mut embedding = embedding;

        if let Some(offset) = self.free_list.pop() {
            // SAFETY: We already checked the dimensions of the embedding and the size of already-existing embeddings
            self.data[offset..offset + self.dim].copy_from_slice(&embedding);
        } else {
            self.data.append(&mut embedding);
        }

        self.payloads.insert(entry.id.clone(), entry);
    }

    async fn search(&self, embedding: Vec<f32>, limit: usize) -> Vec<MemoryEntry> {
        let mut out = Vec::new();
        let idx_map = &self.id_to_idx;
        for (id, &idx) in idx_map {
            let offset = idx * self.dim;
            let arr = self.data[offset..offset + self.dim].to_vec();

            let score = cosine_similarity(&embedding, &arr);

            out.push((id, score));
        }

        // SAFETY: This should never fail because there's no reason that there would *not* be an ordering (ie, -0 vs 0 or NaN vs NaN)
        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        out.truncate(limit);

        out.into_iter()
            .map(|(id, _)| {
                // SAFETY: It is pretty much guaranteed that the payload will exist since the only way to access the payload list is through internal methods
                self.payloads.get(id).cloned().unwrap()
            })
            .collect()
    }

    async fn search_by_id(&self, id: String) -> Result<(Vec<f32>, MemoryEntry), crate::Error> {
        let Some((_, pos_offset)) = self.id_to_idx.iter().find(|x| x.0 == &id) else {
            unimplemented!("Handle not existing ID")
        };

        let pos_offset = pos_offset.to_owned();

        let arr = self.data[pos_offset..pos_offset + self.dim].to_vec();

        let Some(payload) = self.payloads.get(&id).cloned() else {
            unimplemented!("Handle not existing ID")
        };

        Ok((arr, payload))
    }

    async fn get_oldest(&self, limit: usize) -> Vec<MemoryEntry> {
        let mut entries: Vec<_> = self.payloads.iter().map(|x| x.1.to_owned()).collect();

        entries.sort_by_key(|e| e.created_at);
        entries.truncate(limit);

        entries
    }

    async fn get_recent(&self, limit: usize) -> Vec<MemoryEntry> {
        let mut entries: Vec<_> = self.payloads.iter().map(|x| x.1.to_owned()).collect();

        entries.sort_by_key(|e| std::cmp::Reverse(e.created_at));
        entries.truncate(limit);

        entries
    }

    async fn delete(&mut self, id: String) {
        let Some(arr_pos) = self.id_to_idx.remove(&id) else {
            unimplemented!("Handle key not existing")
        };

        self.payloads.remove(&id);

        self.free_list.push(arr_pos);
    }

    async fn delete_batch(&mut self, ids: Vec<String>) {
        for id in ids {
            self.delete(id).await;
        }
    }

    async fn count(&self) -> usize {
        self.id_to_idx.len()
    }

    async fn update_payload_by_id(&mut self, id: String, payload: MemoryEntry) {
        self.payloads.entry(id).insert_entry(payload);
    }
}

/// Computes the cosine similarity between two embeddings and returns a result between 0.0 and 1.0.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let cos = dot / (norm_a.sqrt() * norm_b.sqrt());
    (cos + 1.0) / 2.0
}
