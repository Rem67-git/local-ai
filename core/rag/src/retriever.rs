use crate::document::Document;
use crate::indexer::DocumentIndexer;

pub struct DocumentRetriever {
    indexer: DocumentIndexer,
}

impl DocumentRetriever {
    pub fn new(indexer: DocumentIndexer) -> Self {
        Self { indexer }
    }

    pub fn retrieve_by_keyword(&self, keyword: &str) -> Vec<Document> {
        let doc_ids = self.indexer.search_by_keyword(keyword);
        doc_ids
            .iter()
            .filter_map(|id| self.indexer.get_document(id).ok())
            .collect()
    }

    pub fn retrieve_by_keywords(&self, keywords: &[&str]) -> Vec<Document> {
        let mut results = Vec::new();

        for keyword in keywords {
            results.extend(self.retrieve_by_keyword(keyword));
        }

        // Remove duplicates
        results.sort_by(|a, b| a.id.cmp(&b.id));
        results.dedup_by(|a, b| a.id == b.id);

        results
    }

    pub fn retrieve_top_k(&self, keyword: &str, k: usize) -> Vec<Document> {
        let mut docs = self.retrieve_by_keyword(keyword);
        docs.truncate(k);
        docs
    }

    pub fn get_context(&self, keyword: &str, k: usize) -> String {
        let docs = self.retrieve_top_k(keyword, k);

        docs.iter()
            .enumerate()
            .map(|(i, doc)| {
                format!(
                    "Document {}:\nTitle: {}\nContent (excerpt): {}\n",
                    i + 1,
                    doc.title,
                    &doc.content[..std::cmp::min(200, doc.content.len())]
                )
            })
            .collect::<Vec<_>>()
            .join("\n---\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::DocumentType;

    fn create_test_retriever() -> DocumentRetriever {
        let mut indexer = DocumentIndexer::new();

        indexer
            .add_document(Document::new(
                "Rust Guide".to_string(),
                "Rust programming language fundamentals".to_string(),
                DocumentType::Text,
                "rust.txt".to_string(),
            ))
            .unwrap();

        indexer
            .add_document(Document::new(
                "Python Guide".to_string(),
                "Python programming language basics".to_string(),
                DocumentType::Text,
                "python.txt".to_string(),
            ))
            .unwrap();

        DocumentRetriever::new(indexer)
    }

    #[test]
    fn test_retrieve_by_keyword() {
        let retriever = create_test_retriever();
        let results = retriever.retrieve_by_keyword("rust");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Guide");
    }

    #[test]
    fn test_retrieve_by_keywords() {
        let retriever = create_test_retriever();
        let results = retriever.retrieve_by_keywords(&["rust", "python"]);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_retrieve_top_k() {
        let retriever = create_test_retriever();
        let results = retriever.retrieve_top_k("programming", 1);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_context() {
        let retriever = create_test_retriever();
        let context = retriever.get_context("language", 2);

        assert!(context.contains("Document"));
        assert!(context.contains("Title"));
    }
}
