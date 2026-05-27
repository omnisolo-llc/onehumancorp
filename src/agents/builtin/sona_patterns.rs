

/// Ruflo Unique Harness Innovations: SONA neural patterns (Self-learning trajectory patterns)
/// Implements a simple pattern matching system to record and retrieve successful trajectories.

#[derive(Debug, Clone, PartialEq)]
pub struct TrajectoryPattern {
    pub id: String,
    pub initial_context: String,
    pub successful_tools: Vec<String>,
    pub outcome_score: f32, // 0.0 to 1.0 representing success
}

#[derive(Debug)]
pub struct PatternMatcher {
    patterns: Vec<TrajectoryPattern>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Records a successful trajectory pattern.
    pub fn record_pattern(&mut self, pattern: TrajectoryPattern) {
        if pattern.outcome_score > 0.5 {
            self.patterns.push(pattern);
        }
    }

    /// Finds the best matching pattern based on simple context similarity (word overlap).
    pub fn find_best_match(&self, current_context: &str) -> Option<&TrajectoryPattern> {
        if self.patterns.is_empty() {
            return None;
        }

        let context_words: Vec<&str> = current_context.split_whitespace().collect();
        let mut best_match: Option<&TrajectoryPattern> = None;
        let mut best_score = 0.0;

        for pattern in &self.patterns {
            let pattern_words: Vec<&str> = pattern.initial_context.split_whitespace().collect();
            let mut match_count = 0;

            for word in &context_words {
                if pattern_words.contains(word) {
                    match_count += 1;
                }
            }

            // Simple Jaccard-like similarity
            let similarity = match_count as f32 / (context_words.len() + pattern_words.len() - match_count) as f32;

            // Combine with outcome score
            let total_score = similarity * pattern.outcome_score;

            if total_score > best_score && similarity > 0.1 { // Threshold
                best_score = total_score;
                best_match = Some(pattern);
            }
        }

        best_match
    }

    pub fn get_patterns(&self) -> &[TrajectoryPattern] {
        &self.patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_pattern() {
        let mut matcher = PatternMatcher::new();

        let success_pattern = TrajectoryPattern {
            id: "1".to_string(),
            initial_context: "read file analyze error".to_string(),
            successful_tools: vec!["read_file".to_string(), "grep".to_string()],
            outcome_score: 0.9,
        };
        matcher.record_pattern(success_pattern.clone());
        assert_eq!(matcher.get_patterns().len(), 1);

        let fail_pattern = TrajectoryPattern {
            id: "2".to_string(),
            initial_context: "delete database".to_string(),
            successful_tools: vec!["bash".to_string()],
            outcome_score: 0.2, // Below threshold
        };
        matcher.record_pattern(fail_pattern);
        assert_eq!(matcher.get_patterns().len(), 1); // Should not record failures
    }

    #[test]
    fn test_find_best_match() {
        let mut matcher = PatternMatcher::new();

        matcher.record_pattern(TrajectoryPattern {
            id: "1".to_string(),
            initial_context: "fix null pointer exception in java".to_string(),
            successful_tools: vec!["grep".to_string(), "edit_file".to_string()],
            outcome_score: 0.9,
        });

        matcher.record_pattern(TrajectoryPattern {
            id: "2".to_string(),
            initial_context: "setup new react project".to_string(),
            successful_tools: vec!["bash".to_string()],
            outcome_score: 1.0,
        });

        let context1 = "how to fix null pointer in java code";
        let match1 = matcher.find_best_match(context1);
        assert!(match1.is_some());
        assert_eq!(match1.unwrap().id, "1");

        let context2 = "create react project setup";
        let match2 = matcher.find_best_match(context2);
        assert!(match2.is_some());
        assert_eq!(match2.unwrap().id, "2");

        let context3 = "totally unrelated task about kubernetes deployment";
        let match3 = matcher.find_best_match(context3);
        // Should have low similarity and return None, or maybe a very low match we filter out
        assert!(match3.is_none() || match3.unwrap().outcome_score < 1.0); // based on threshold

        // Let's test the threshold explicitly. "kubernetes deployment" has 0 overlap.
        assert!(matcher.find_best_match("kubernetes deployment").is_none());
    }
}
