use crate::document::Document;
use crate::error::{RAGError, RAGResult};
use std::collections::HashMap;

pub struct DocumentIndexer {
    documents: HashMap<String, Document>,
    word_index: HashMap<String, Vec<String>>, // word -> [doc_ids]
}

impl DocumentIndexer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            word_index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc: Document) -> RAGResult<String> {
        let doc_id = doc.id.clone();

        // Index words from document content
        self.index_words(&doc_id, &doc.content);

        self.documents.insert(doc_id.clone(), doc);
        Ok(doc_id)
    }

    fn index_words(&mut self, doc_id: &str, content: &str) {
        let lowercase = content.to_lowercase();
        let words: Vec<&str> = lowercase
            .split_whitespace()
            .filter(|w| w.len() > 2) // Skip short words
            .collect();

        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            self.word_index
                .entry(clean_word.to_string())
                .or_insert_with(Vec::new)
                .push(doc_id.to_string());
        }
    }

    pub fn get_document(&self, doc_id: &str) -> RAGResult<Document> {
        self.documents
            .get(doc_id)
            .cloned()
            .ok_or_else(|| RAGError::DocumentNotFound(doc_id.to_string()))
    }

    pub fn search_by_keyword(&self, keyword: &str) -> Vec<String> {
        let clean_keyword = keyword.to_lowercase();
        self.word_index
            .get(&clean_keyword)
            .map(|ids| ids.clone())
            .unwrap_or_default()
    }

    pub fn list_documents(&self) -> Vec<Document> {
        self.documents.values().cloned().collect()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn remove_document(&mut self, doc_id: &str) -> RAGResult<Document> {
        let doc = self
            .documents
            .remove(doc_id)
            .ok_or_else(|| RAGError::DocumentNotFound(doc_id.to_string()))?;

        // Clean up word index
        for words in self.word_index.values_mut() {
            words.retain(|id| id != doc_id);
        }

        Ok(doc)
    }
}

impl Default for DocumentIndexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::DocumentType;

    #[test]
    fn test_add_document() {
        let mut indexer = DocumentIndexer::new();
        let doc = Document::new(
            "Test".to_string(),
            "hello world".to_string(),
            DocumentType::Text,
            "test.txt".to_string(),
        );

        let result = indexer.add_document(doc);
        assert!(result.is_ok());
        assert_eq!(indexer.document_count(), 1);
    }

    #[test]
    fn test_search_by_keyword() {
        let mut indexer = DocumentIndexer::new();
        let doc = Document::new(
            "Test".to_string(),
            "rust programming language".to_string(),
            DocumentType::Text,
            "test.txt".to_string(),
        );
        let doc_id = doc.id.clone();

        indexer.add_document(doc).unwrap();

        let results = indexer.search_by_keyword("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], doc_id);
    }

    #[test]
    fn test_get_document() {
        let mut indexer = DocumentIndexer::new();
        let doc = Document::new(
            "Test".to_string(),
            "content".to_string(),
            DocumentType::Markdown,
            "test.md".to_string(),
        );
        let doc_id = doc.id.clone();

        indexer.add_document(doc).unwrap();
        let retrieved = indexer.get_document(&doc_id).unwrap();

        assert_eq!(retrieved.title, "Test");
    }

    #[test]
    fn test_remove_document() {
        let mut indexer = DocumentIndexer::new();
        let doc = Document::new(
            "Test".to_string(),
            "content".to_string(),
            DocumentType::Text,
            "test.txt".to_string(),
        );
        let doc_id = doc.id.clone();

        indexer.add_document(doc).unwrap();
        assert_eq!(indexer.document_count(), 1);

        indexer.remove_document(&doc_id).unwrap();
        assert_eq!(indexer.document_count(), 0);
    }
}
