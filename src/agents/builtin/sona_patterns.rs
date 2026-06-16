/// Ruflo Unique Harness Innovations: SONA neural patterns (Self-learning trajectory patterns)
/// Implements a simple pattern matching system to record and retrieve successful trajectories.

use std::path::Path;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrajectoryPattern {
    pub id: String,
    pub initial_context: String,
    pub successful_tools: Vec<String>,
    pub outcome_score: f32, // 0.0 to 1.0 representing success
}

pub struct PatternMatcher {
    patterns: Vec<TrajectoryPattern>,
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Loads patterns from a JSON file.
    pub async fn load_from_disk<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| format!("Failed to read patterns file: {}", e))?;
        let patterns: Vec<TrajectoryPattern> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse patterns file: {}", e))?;
        Ok(Self { patterns })
    }

    /// Saves patterns to a JSON file.
    pub async fn save_to_disk<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.patterns)
            .map_err(|e| format!("Failed to serialize patterns: {}", e))?;
        tokio::fs::write(path, content)
            .await
            .map_err(|e| format!("Failed to write patterns file: {}", e))?;
        Ok(())
    }

    /// Records a successful trajectory pattern.
    pub fn record_pattern(&mut self, pattern: TrajectoryPattern) {
        if pattern.outcome_score > 0.5 {
            // Prevent exact duplicates
            if !self.patterns.iter().any(|p| p.id == pattern.id) {
                self.patterns.push(pattern);
            }
        }
    }

    /// Computes Jaccard similarity between two sets of keywords/words
    fn compute_jaccard_similarity(set1: &HashSet<&str>, text2: &str) -> f32 {
        let set2: HashSet<&str> = text2.split_whitespace().collect();

        if set1.is_empty() || set2.is_empty() {
            return 0.0;
        }

        let intersection: HashSet<_> = set1.intersection(&set2).collect();
        let union: HashSet<_> = set1.union(&set2).collect();

        intersection.len() as f32 / union.len() as f32
    }

    /// Finds the best matching pattern for the current context.
    /// Returns the pattern.
    pub fn find_best_match(&self, current_context: &str) -> Option<TrajectoryPattern> {
        if self.patterns.is_empty() {
            return None;
        }

        let context_set: HashSet<&str> = current_context.split_whitespace().collect();

        let mut best_pattern: Option<&TrajectoryPattern> = None;
        let mut best_score = 0.0;

        for pattern in &self.patterns {
            // Only consider patterns with a positive outcome score
            if pattern.outcome_score <= 0.0 {
                continue;
            }

            let sim = Self::compute_jaccard_similarity(&context_set, &pattern.initial_context);
            // Weight the similarity by the outcome score
            let weighted_score = sim * pattern.outcome_score;

            if weighted_score > best_score {
                best_score = weighted_score;
                best_pattern = Some(pattern);
            }
        }

        // Only return if we have a reasonably confident match
        if best_score > 0.1 {
            best_pattern.cloned()
        } else {
            None
        }
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

        // Deduplication test
        matcher.record_pattern(success_pattern);
        assert_eq!(matcher.get_patterns().len(), 1);
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

    #[tokio::test]
    async fn test_load_save_disk() {
        let mut matcher = PatternMatcher::new();
        matcher.record_pattern(TrajectoryPattern {
            id: "test1".to_string(),
            initial_context: "fix null pointer exception in java".to_string(),
            successful_tools: vec!["grep".to_string()],
            outcome_score: 0.9,
        });

        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("patterns.json");

        let save_res = matcher.save_to_disk(&file_path).await;
        assert!(save_res.is_ok());

        let load_res = PatternMatcher::load_from_disk(&file_path).await;
        assert!(load_res.is_ok());

        let loaded_matcher = load_res.unwrap();
        assert_eq!(loaded_matcher.get_patterns().len(), 1);
        assert_eq!(loaded_matcher.get_patterns()[0].id, "test1");
        assert_eq!(loaded_matcher.get_patterns()[0].initial_context, "fix null pointer exception in java");
    }

    #[test]
    fn test_jaccard_similarity() {
        let set1: HashSet<&str> = "a b c".split_whitespace().collect();
        let sim1 = PatternMatcher::compute_jaccard_similarity(&set1, "a b c");
        assert_eq!(sim1, 1.0);

        let sim2 = PatternMatcher::compute_jaccard_similarity(&set1, "d e f");
        assert_eq!(sim2, 0.0);

        let sim3 = PatternMatcher::compute_jaccard_similarity(&set1, "b c d");
        // union: a b c d (4), intersection: b c (2). 2/4 = 0.5
        assert_eq!(sim3, 0.5);
    }
}
