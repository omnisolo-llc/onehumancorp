use std::time::Instant;

pub async fn bench_token_compression() {
    tracing::info!("Benchmarking Token Compression (Stop-word removal)...");

    let stop_words: std::collections::HashSet<&str> = [
        "a", "an", "the", "is", "are", "and", "or", "but", "in", "on", "at", "to",
        "for", "with", "by", "about", "as", "of",
    ]
    .iter()
    .cloned()
    .collect();

    let corpus_size = 100_000;
    let mut original_prompts = Vec::with_capacity(corpus_size);

    let base_text = "This is a test prompt for an AI agent to process and analyze data about the system and report on the status of various components. \
        The quick brown fox jumps over the lazy dog. \
        Artificial Intelligence is rapidly evolving and transforming various industries, including healthcare, finance, and transportation. \
        A comprehensive understanding of machine learning algorithms, such as neural networks and decision trees, is essential for developing intelligent systems. \
        The performance of these models heavily relies on the quality and quantity of the training data. \
        Moreover, ethical considerations, such as fairness, transparency, and accountability, must be taken into account to ensure responsible AI deployment. \
        Continuous research and development are necessary to address the challenges and limitations associated with current AI technologies. \
        In the future, we can expect to see more advanced AI systems capable of complex reasoning, problem-solving, and creative tasks. \
        The integration of AI with other emerging technologies, such as the Internet of Things and blockchain, will further amplify its impact on society. \
        As we navigate this transformative era, it is crucial to foster collaboration among researchers, policymakers, and industry leaders to shape a future where AI benefits all of humanity. \
        The potential applications of AI are vast and diverse, ranging from personalized medicine and autonomous vehicles to smart cities and predictive maintenance. \
        By harnessing the power of AI, we can unlock new opportunities for innovation, efficiency, and sustainability. \
        However, it is equally important to address the potential risks and challenges associated with AI, such as job displacement and privacy concerns. \
        Through proactive measures and responsible practices, we can mitigate these risks and ensure a safe and beneficial AI ecosystem. \
        The journey towards artificial general intelligence remains a topic of ongoing debate and exploration within the scientific community. \
        While significant progress has been made, there are still many unanswered questions and hurdles to overcome. \
        The pursuit of AGI requires a deep understanding of human cognition and the development of novel algorithms capable of general-purpose learning and reasoning. \
        As we continue to push the boundaries of AI research, we must remain mindful of the ethical implications and societal impact of our work. \
        By fostering a culture of responsible innovation and inclusive dialogue, we can harness the transformative potential of AI while safeguarding human values and well-being. \
        The future of AI is bright, and it holds the promise of a better, more connected, and more intelligent world. \
        Let us embrace this exciting journey and work together to build a future where AI serves as a powerful tool for positive change. \
        The possibilities are endless, and the potential for impact is immense. \
        By unlocking the secrets of intelligence, we can unlock the potential of humanity.";

    for _ in 0..corpus_size {
        original_prompts.push(base_text.to_string());
    }

    let start_compression = Instant::now();

    let mut total_original_len = 0;
    let mut total_compressed_len = 0;

    for prompt in original_prompts {
        total_original_len += prompt.len();

        let compressed = prompt
            .split_whitespace()
            .filter(|word| {
                let clean_word = word.to_lowercase();
                !stop_words.contains(clean_word.as_str())
            })
            .collect::<Vec<&str>>()
            .join(" ");

        total_compressed_len += compressed.len();
    }

    let compression_duration = start_compression.elapsed();

    println!("Token Compression processed {} prompts in {:?}", corpus_size, compression_duration);
    println!("Original total length: {}", total_original_len);
    println!("Compressed total length: {}", total_compressed_len);

    let compression_ratio = 100.0 - ((total_compressed_len as f64 / total_original_len as f64) * 100.0);
    println!("Achieved {:.2}% token reduction.", compression_ratio);

    assert!(total_compressed_len < total_original_len, "Compression should reduce total length");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_token_compression() {
        bench_token_compression().await;
    }
}
