use crate::memory_entry::MemoryEntry;

pub struct RecallEngine;

impl RecallEngine {
    pub fn rank_by_relevance(entries: &[MemoryEntry]) -> Vec<MemoryEntry> {
        let mut sorted = entries.to_vec();
        sorted.sort_by(|a, b| {
            // Higher relevance scores first
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                // Then by access count
                .then_with(|| b.access_count.cmp(&a.access_count))
                // Then by recency
                .then_with(|| b.last_accessed.cmp(&a.last_accessed))
        });
        sorted
    }

    pub fn filter_by_similarity(entries: &[MemoryEntry], query: &str) -> Vec<MemoryEntry> {
        entries
            .iter()
            .filter(|entry| {
                // Simple substring matching for now
                entry
                    .content
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                    || entry
                        .metadata
                        .values()
                        .any(|v| {
                            if let Some(s) = v.as_str() {
                                s.to_lowercase().contains(&query.to_lowercase())
                            } else {
                                false
                            }
                        })
            })
            .cloned()
            .collect()
    }

    pub fn find_similar(entries: &[MemoryEntry], query: &str, top_k: usize) -> Vec<MemoryEntry> {
        let matches = Self::filter_by_similarity(entries, query);
        let mut ranked = Self::rank_by_relevance(&matches);
        ranked.truncate(top_k);
        ranked
    }

    pub fn calculate_similarity(text1: &str, text2: &str) -> f32 {
        // Simple Jaccard similarity for words
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }

        let intersection = words1.intersection(&words2).count() as f32;
        let union = words1.union(&words2).count() as f32;

        intersection / union
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_entry::MemoryType;

    #[test]
    fn test_rank_by_relevance() {
        let mut entry1 = MemoryEntry::new(
            MemoryType::ContextHistory,
            "old".to_string(),
            serde_json::Map::new(),
        );
        entry1.relevance_score = 0.3;

        let mut entry2 = MemoryEntry::new(
            MemoryType::ContextHistory,
            "new".to_string(),
            serde_json::Map::new(),
        );
        entry2.relevance_score = 0.9;

        let entries = vec![entry1, entry2];
        let ranked = RecallEngine::rank_by_relevance(&entries);

        assert!(ranked[0].relevance_score > ranked[1].relevance_score);
    }

    #[test]
    fn test_filter_by_similarity() {
        let entry1 = MemoryEntry::new(
            MemoryType::ContextHistory,
            "rust programming".to_string(),
            serde_json::Map::new(),
        );

        let entry2 = MemoryEntry::new(
            MemoryType::ContextHistory,
            "python scripting".to_string(),
            serde_json::Map::new(),
        );

        let entries = vec![entry1, entry2];
        let matches = RecallEngine::filter_by_similarity(&entries, "rust");

        assert_eq!(matches.len(), 1);
        assert!(matches[0].content.contains("rust"));
    }

    #[test]
    fn test_find_similar() {
        let entries = vec![
            MemoryEntry::new(
                MemoryType::ContextHistory,
                "error handling in rust".to_string(),
                serde_json::Map::new(),
            ),
            MemoryEntry::new(
                MemoryType::ContextHistory,
                "python error handling".to_string(),
                serde_json::Map::new(),
            ),
            MemoryEntry::new(
                MemoryType::ContextHistory,
                "javascript loops".to_string(),
                serde_json::Map::new(),
            ),
        ];

        let similar = RecallEngine::find_similar(&entries, "error", 2);
        assert_eq!(similar.len(), 2);
    }

    #[test]
    fn test_calculate_similarity() {
        let sim1 = RecallEngine::calculate_similarity("hello world", "hello world");
        assert_eq!(sim1, 1.0);

        let sim2 = RecallEngine::calculate_similarity("hello world", "hello");
        assert!(sim2 > 0.0 && sim2 < 1.0);

        let sim3 = RecallEngine::calculate_similarity("hello", "goodbye");
        assert_eq!(sim3, 0.0);
    }
}
