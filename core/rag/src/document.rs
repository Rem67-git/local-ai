use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentType {
    Text,
    Markdown,
    JSON,
    Code,
    PDF,
    HTML,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub doc_type: DocumentType,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
    pub chunks: Vec<String>,
}

impl Document {
    pub fn new(
        title: String,
        content: String,
        doc_type: DocumentType,
        source: String,
    ) -> Self {
        let now = Utc::now();
        let chunks = Self::chunk_content(&content);

        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            doc_type,
            source,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Map::new(),
            chunks,
        }
    }

    fn chunk_content(content: &str) -> Vec<String> {
        const CHUNK_SIZE: usize = 500;
        const OVERLAP: usize = 100;

        let mut chunks = Vec::new();
        let bytes = content.as_bytes();

        let mut start = 0;
        while start < bytes.len() {
            let end = std::cmp::min(start + CHUNK_SIZE, bytes.len());
            if let Ok(chunk) = std::str::from_utf8(&bytes[start..end]) {
                chunks.push(chunk.to_string());
            }
            start += CHUNK_SIZE - OVERLAP;
        }

        chunks
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn get_chunk(&self, index: usize) -> Option<&str> {
        self.chunks.get(index).map(|s| s.as_str())
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "Test".to_string(),
            "content".to_string(),
            DocumentType::Text,
            "test.txt".to_string(),
        );

        assert_eq!(doc.title, "Test");
        assert_eq!(doc.doc_type, DocumentType::Text);
        assert!(!doc.id.is_empty());
    }

    #[test]
    fn test_document_chunking() {
        let long_content = "word ".repeat(200);
        let doc = Document::new(
            "Long".to_string(),
            long_content,
            DocumentType::Text,
            "long.txt".to_string(),
        );

        assert!(doc.chunk_count() > 1);
    }

    #[test]
    fn test_add_metadata() {
        let mut doc = Document::new(
            "Test".to_string(),
            "content".to_string(),
            DocumentType::Markdown,
            "test.md".to_string(),
        );

        doc.add_metadata("author".to_string(), "Claude".into());
        assert!(doc.metadata.contains_key("author"));
    }

    #[test]
    fn test_get_chunk() {
        let doc = Document::new(
            "Test".to_string(),
            "chunk1 chunk2".to_string(),
            DocumentType::Text,
            "test.txt".to_string(),
        );

        assert!(doc.get_chunk(0).is_some());
    }
}
