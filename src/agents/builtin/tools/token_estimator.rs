pub fn estimate_tokens(text: &str) -> usize {
    // A simple estimation: roughly 4 chars per token for typical English text/code.
    // Real implementation would use tiktoken or similar, but this is sufficient for
    // "proper token accounting" in our JIT retrieval context without external deps.
    text.len() / 4
}
