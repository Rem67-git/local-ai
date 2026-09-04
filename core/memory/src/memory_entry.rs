use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    ContextHistory,
    ToolObservation,
    DecisionRecord,
    ErrorLog,
    LearningPattern,
    MissionCheckpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub memory_type: MemoryType,
    pub content: String,
    pub metadata: serde_json::Map<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: usize,
    pub relevance_score: f32,
}

impl MemoryEntry {
    pub fn new(
        memory_type: MemoryType,
        content: String,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            memory_type,
            content,
            metadata,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            relevance_score: 0.5,
        }
    }

    pub fn record_access(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }

    pub fn set_relevance(&mut self, score: f32) {
        self.relevance_score = score.clamp(0.0, 1.0);
    }

    pub fn is_stale(&self, days_old: i64) -> bool {
        let age = Utc::now() - self.last_accessed;
        age.num_days() > days_old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_entry_creation() {
        let entry = MemoryEntry::new(
            MemoryType::ContextHistory,
            "test content".to_string(),
            serde_json::Map::new(),
        );

        assert_eq!(entry.memory_type, MemoryType::ContextHistory);
        assert_eq!(entry.content, "test content");
        assert_eq!(entry.access_count, 0);
    }

    #[test]
    fn test_record_access() {
        let mut entry = MemoryEntry::new(
            MemoryType::DecisionRecord,
            "decision".to_string(),
            serde_json::Map::new(),
        );

        let initial_count = entry.access_count;
        entry.record_access();

        assert_eq!(entry.access_count, initial_count + 1);
    }

    #[test]
    fn test_set_relevance() {
        let mut entry = MemoryEntry::new(
            MemoryType::ToolObservation,
            "obs".to_string(),
            serde_json::Map::new(),
        );

        entry.set_relevance(0.8);
        assert_eq!(entry.relevance_score, 0.8);

        // Test clamping
        entry.set_relevance(1.5);
        assert_eq!(entry.relevance_score, 1.0);

        entry.set_relevance(-0.5);
        assert_eq!(entry.relevance_score, 0.0);
    }
}
