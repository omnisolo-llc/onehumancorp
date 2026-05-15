
pub mod text_summarizer {
    //! # Text Summarization Utilities
    //!
    //! Provides chunking, token estimation, and extractive summarization
    //! to prevent agent context overflow (Context Compaction mechanic).

    /// Estimates the token count of a string (rough approximation: 4 chars/token).
    pub fn estimate_tokens(text: &str) -> usize {
        text.chars().count() / 4
    }

    /// Chunks a large text into smaller segments based on token limits.
    pub fn chunk_text(text: &str, max_tokens: usize) -> Vec<String> {
        let max_chars = max_tokens * 4;
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for paragraph in text.split("\n\n") {
            if current_chunk.len() + paragraph.len() > max_chars && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }
            current_chunk.push_str(paragraph);
            current_chunk.push_str("\n\n");
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    /// Performs a very naive extractive summarization by picking the first and last sentences.
    pub fn summarize_extractive(text: &str) -> String {
        let sentences: Vec<&str> = text.split(". ").collect();
        if sentences.len() <= 2 {
            return text.to_string();
        }

        let first = sentences.first().unwrap_or(&"");
        let last = sentences.last().unwrap_or(&"");

        format!("{}. [...] {}.", first, last)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_estimate_tokens() {
            let text = "This is a simple test string to count tokens.";
            assert_eq!(estimate_tokens(text), 11);
        }

        #[test]
        fn test_chunk_text() {
            let text = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
            let chunks = chunk_text(text, 5); // 20 chars max
            assert_eq!(chunks.len(), 3);
        }

        #[test]
        fn test_summarize_extractive() {
            let text = "This is the start. The middle part is long and boring. This is the end";
            let sum = summarize_extractive(text);
            assert!(sum.contains("This is the start"));
            assert!(sum.contains("This is the end"));
            assert!(sum.contains("[...]"));
        }
    }
}
