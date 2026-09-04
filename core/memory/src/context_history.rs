use crate::memory_entry::{MemoryEntry, MemoryType};
use crate::error::{MemoryError, MemoryResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub description: String,
    pub success: bool,
    pub details: serde_json::Map<String, serde_json::Value>,
}

pub struct ContextHistory {
    entries: HashMap<String, MemoryEntry>,
    history: Vec<HistoryEntry>,
    max_entries: usize,
}

impl ContextHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            history: Vec::new(),
            max_entries,
        }
    }

    pub fn add_entry(&mut self, entry: MemoryEntry) -> MemoryResult<String> {
        if self.entries.len() >= self.max_entries {
            return Err(MemoryError::StorageError(
                "Memory storage full".to_string(),
            ));
        }

        let id = entry.id.clone();
        self.entries.insert(id.clone(), entry);

        Ok(id)
    }

    pub fn add_history(
        &mut self,
        event_type: String,
        description: String,
        success: bool,
        details: serde_json::Map<String, serde_json::Value>,
    ) {
        let entry = HistoryEntry {
            timestamp: Utc::now(),
            event_type,
            description,
            success,
            details,
        };

        self.history.push(entry);

        // Keep only recent history
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    pub fn get_entry(&mut self, id: &str) -> MemoryResult<MemoryEntry> {
        let mut entry = self
            .entries
            .get(id)
            .ok_or_else(|| MemoryError::NotFound(id.to_string()))?
            .clone();

        entry.record_access();
        self.entries.insert(id.to_string(), entry.clone());

        Ok(entry)
    }

    pub fn list_entries(&self) -> Vec<MemoryEntry> {
        self.entries.values().cloned().collect()
    }

    pub fn list_by_type(&self, memory_type: MemoryType) -> Vec<MemoryEntry> {
        self.entries
            .values()
            .filter(|e| e.memory_type == memory_type)
            .cloned()
            .collect()
    }

    pub fn prune_stale(&mut self, days_old: i64) -> usize {
        let initial_count = self.entries.len();

        self.entries.retain(|_, entry| !entry.is_stale(days_old));

        initial_count - self.entries.len()
    }

    pub fn get_recent_history(&self, count: usize) -> Vec<HistoryEntry> {
        self.history
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entry() {
        let mut history = ContextHistory::new(10);
        let entry = MemoryEntry::new(
            MemoryType::ContextHistory,
            "test".to_string(),
            serde_json::Map::new(),
        );

        let id = history.add_entry(entry).unwrap();
        assert!(!id.is_empty());
        assert_eq!(history.count(), 1);
    }

    #[test]
    fn test_get_entry() {
        let mut history = ContextHistory::new(10);
        let entry = MemoryEntry::new(
            MemoryType::DecisionRecord,
            "decision".to_string(),
            serde_json::Map::new(),
        );
        let id = entry.id.clone();

        history.add_entry(entry).unwrap();
        let retrieved = history.get_entry(&id).unwrap();

        assert_eq!(retrieved.content, "decision");
    }

    #[test]
    fn test_list_by_type() {
        let mut history = ContextHistory::new(10);

        history
            .add_entry(MemoryEntry::new(
                MemoryType::ContextHistory,
                "ctx1".to_string(),
                serde_json::Map::new(),
            ))
            .unwrap();

        history
            .add_entry(MemoryEntry::new(
                MemoryType::DecisionRecord,
                "dec1".to_string(),
                serde_json::Map::new(),
            ))
            .unwrap();

        let decisions = history.list_by_type(MemoryType::DecisionRecord);
        assert_eq!(decisions.len(), 1);
    }

    #[test]
    fn test_add_history() {
        let mut history = ContextHistory::new(10);

        history.add_history(
            "action".to_string(),
            "test action".to_string(),
            true,
            serde_json::Map::new(),
        );

        let recent = history.get_recent_history(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].event_type, "action");
    }

    #[test]
    fn test_storage_limit() {
        let mut history = ContextHistory::new(2);

        history
            .add_entry(MemoryEntry::new(
                MemoryType::ContextHistory,
                "1".to_string(),
                serde_json::Map::new(),
            ))
            .unwrap();

        history
            .add_entry(MemoryEntry::new(
                MemoryType::ContextHistory,
                "2".to_string(),
                serde_json::Map::new(),
            ))
            .unwrap();

        let result = history.add_entry(MemoryEntry::new(
            MemoryType::ContextHistory,
            "3".to_string(),
            serde_json::Map::new(),
        ));

        assert!(result.is_err());
    }
}
