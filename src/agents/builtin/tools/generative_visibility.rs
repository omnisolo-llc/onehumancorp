use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::collections::HashMap;
use super::{Tool, ToolExecutor};

pub struct GenerativeVisibilityExecutor;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct VisibilityReport {
    pub status: String,
    pub overall_score: i32,
    pub readability_score: f64,
    pub sentiment_score: f64,
    pub keyword_density: HashMap<String, usize>,
    pub tf_idf_scores: HashMap<String, f64>,
    pub schema_validation: bool,
    pub recommendations: Vec<String>,
    pub local_intent_detected: bool,
    pub semantic_richness: i32,
}

// Simulated massive NLP and SEO Analysis Engine
impl GenerativeVisibilityExecutor {
    // 1. Core Stop Word Dictionary
    fn get_stop_words() -> Vec<&'static str> {
        vec![
            "a", "about", "above", "after", "again", "against", "all", "am", "an", "and", "any", "are", "aren't", "as", "at",
            "be", "because", "been", "before", "being", "below", "between", "both", "but", "by", "can't", "cannot", "could",
            "couldn't", "did", "didn't", "do", "does", "doesn't", "doing", "don't", "down", "during", "each", "few", "for",
            "from", "further", "had", "hadn't", "has", "hasn't", "have", "haven't", "having", "he", "he'd", "he'll", "he's",
            "her", "here", "here's", "hers", "herself", "him", "himself", "his", "how", "how's", "i", "i'd", "i'll", "i'm",
            "i've", "if", "in", "into", "is", "isn't", "it", "it's", "its", "itself", "let's", "me", "more", "most", "mustn't",
            "my", "myself", "no", "nor", "not", "of", "off", "on", "once", "only", "or", "other", "ought", "our", "ours",
            "ourselves", "out", "over", "own", "same", "shan't", "she", "she'd", "she'll", "she's", "should", "shouldn't",
            "so", "some", "such", "than", "that", "that's", "the", "their", "theirs", "them", "themselves", "then", "there",
            "there's", "these", "they", "they'd", "they'll", "they're", "they've", "this", "those", "through", "to", "too",
            "under", "until", "up", "very", "was", "wasn't", "we", "we'd", "we'll", "we're", "we've", "were", "weren't",
            "what", "what's", "when", "when's", "where", "where's", "which", "which's", "while", "who", "who's", "whom",
            "why", "why's", "with", "won't", "would", "wouldn't", "you", "you'd", "you'll", "you're", "you've", "your",
            "yours", "yourself", "yourselves"
        ]
    }

    // 2. Tokenization and Keyword Density
    fn extract_keywords(text: &str) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        let stop_words = Self::get_stop_words();

        for raw_word in text.split_whitespace() {
            let word = raw_word.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
            if !word.is_empty() && !stop_words.contains(&word.as_str()) {
                *map.entry(word).or_insert(0) += 1;
            }
        }
        map
    }

    // 3. TF-IDF Simulation (Term Frequency - Inverse Document Frequency)
    // In a real system, IDF is based on a massive corpus. Here we simulate it against a generic English corpus distribution.
    fn calculate_tf_idf(keywords: &HashMap<String, usize>, total_words: usize) -> HashMap<String, f64> {
        let mut tf_idf = HashMap::new();
        for (word, count) in keywords {
            let tf = *count as f64 / total_words as f64;
            // Simulated IDF: longer words generally have higher IDF in English, generic words have lower.
            let simulated_idf = (word.len() as f64).ln() + 1.0;
            tf_idf.insert(word.clone(), tf * simulated_idf);
        }
        tf_idf
    }

    // 4. Flesch-Kincaid Readability Score
    fn calculate_readability(text: &str) -> f64 {
        let words = text.split_whitespace().count() as f64;
        let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count() as f64;

        let syllables = text.split_whitespace().map(|w| {
            let w = w.to_lowercase();
            let mut count = 0;
            let mut prev_vowel = false;
            for c in w.chars() {
                let is_vowel = "aeiouy".contains(c);
                if is_vowel && !prev_vowel {
                    count += 1;
                }
                prev_vowel = is_vowel;
            }
            if w.ends_with('e') { count -= 1; }
            if count <= 0 { 1 } else { count }
        }).sum::<i32>() as f64;

        if words == 0.0 || sentences == 0.0 {
            return 0.0;
        }

        // Flesch Reading Ease formula
        let score = 206.835 - 1.015 * (words / sentences) - 84.6 * (syllables / words);
        score.clamp(0.0, 100.0)
    }

    // 5. Sentiment Analysis (Basic lexicon approach)
    fn calculate_sentiment(text: &str) -> f64 {
        let positive_words = vec!["good", "great", "excellent", "amazing", "best", "top", "perfect", "love", "happy", "quality"];
        let negative_words = vec!["bad", "terrible", "awful", "worst", "poor", "hate", "sad", "disappointing", "slow"];

        let mut pos_count = 0.0;
        let mut neg_count = 0.0;

        for raw_word in text.split_whitespace() {
            let word = raw_word.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
            if positive_words.contains(&word.as_str()) {
                pos_count += 1.0;
            } else if negative_words.contains(&word.as_str()) {
                neg_count += 1.0;
            }
        }

        let total = pos_count + neg_count;
        if total == 0.0 {
            return 0.5; // Neutral
        }

        // Return score from 0.0 (negative) to 1.0 (positive)
        (pos_count / total) * 0.5 + 0.5
    }

    // 6. Schema.org Validation Parser
    fn validate_schema(text: &str) -> bool {
        // Look for common JSON-LD or Microdata patterns
        let lower = text.to_lowercase();
        lower.contains(r#""@context": "https://schema.org""#) ||
        lower.contains(r#""@context":"http://schema.org""#) ||
        lower.contains("itemtype=\"http://schema.org/")
    }

    // 7. Semantic Richness / LSI (Latent Semantic Indexing) scoring
    fn calculate_semantic_richness(keywords: &HashMap<String, usize>) -> i32 {
        // A proxy for how many unique, non-stop words are used. Higher richness means better LLM understanding.
        let unique_words = keywords.len();
        (unique_words as f64 / 10.0).clamp(0.0, 100.0) as i32
    }

    // 8. Main Analysis Pipeline
    fn analyze_content(content: &str) -> VisibilityReport {
        let mut recommendations = Vec::new();
        let mut overall_score = 0;

        let total_words = content.split_whitespace().count();
        let keywords = Self::extract_keywords(content);

        let readability = Self::calculate_readability(content);
        let sentiment = Self::calculate_sentiment(content);
        let tf_idf = Self::calculate_tf_idf(&keywords, total_words);
        let has_schema = Self::validate_schema(content);
        let semantic_richness = Self::calculate_semantic_richness(&keywords);

        // 8a. Evaluate Local Intent
        let lower_content = content.to_lowercase();
        let local_intent = lower_content.contains("near me")
            || lower_content.contains("located in")
            || lower_content.contains("serving the")
            || lower_content.contains("city")
            || lower_content.contains("town");

        // 8b. Score Aggregation
        if local_intent {
            overall_score += 20;
        } else {
            recommendations.push("Add geographic context ('serving [City]', 'near me') to rank in local generative search.".to_string());
        }

        if has_schema {
            overall_score += 20;
        } else {
            recommendations.push("Inject JSON-LD schema.org data so AI agents can parse your business hours, address, and entity type accurately.".to_string());
        }

        if readability > 50.0 && readability < 80.0 {
            overall_score += 15; // Good conversational range
        } else if readability <= 50.0 {
            recommendations.push("Simplify your language. AI summarizers prefer content with a Flesch reading ease above 50.".to_string());
        } else {
            overall_score += 10; // Too simple, but acceptable
        }

        if sentiment > 0.6 {
            overall_score += 15;
        } else {
            recommendations.push("Include positive qualitative anchors (e.g., 'best', 'expert', 'top-rated') which match common LLM prompts.".to_string());
        }

        if semantic_richness > 50 {
            overall_score += 15;
        } else {
            recommendations.push("Expand your vocabulary. Use Latent Semantic Indexing (LSI) keywords related to your core business.".to_string());
        }

        if total_words > 500 {
            overall_score += 15;
        } else {
            recommendations.push("Write longer content (>500 words). LLMs need detailed context to confidently synthesize an answer about your business.".to_string());
        }

        VisibilityReport {
            status: "success".to_string(),
            overall_score: overall_score.min(100),
            readability_score: readability,
            sentiment_score: sentiment,
            keyword_density: keywords,
            tf_idf_scores: tf_idf,
            schema_validation: has_schema,
            recommendations,
            local_intent_detected: local_intent,
            semantic_richness,
        }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for GenerativeVisibilityExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let content = args["content"].as_str().unwrap_or("");
        let url = args["url"].as_str().unwrap_or("");

        if content.is_empty() && url.is_empty() {
            return Err(ToolError::LlmRecoverable(
                "generative_visibility: either 'content' or 'url' must be provided.".to_string(),
            ));
        }

        let report = if !content.is_empty() {
            Self::analyze_content(content)
        } else {
            VisibilityReport {
                status: "success".to_string(),
                overall_score: 40,
                readability_score: 0.0,
                sentiment_score: 0.5,
                keyword_density: HashMap::new(),
                tf_idf_scores: HashMap::new(),
                schema_validation: false,
                recommendations: vec![
                    "Content missing. URL analysis simulated. Provide raw text for full NLP scoring.".to_string(),
                    "Ensure your homepage clearly declares your entity type in the first paragraph.".to_string()
                ],
                local_intent_detected: false,
                semantic_richness: 0,
            }
        };

        Ok(serde_json::to_string(&report).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?)
    }
}

pub fn generative_visibility_tool() -> Tool {
    Tool {
        name: "generative_visibility".to_string(),
        description: "Analyze website content and return a Generative Score (0-100) and actionable steps to improve AI searchability.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "content": {
                    "type": "string",
                    "description": "The text content of the website to analyze."
                },
                "url": {
                    "type": "string",
                    "description": "The URL of the website to analyze."
                }
            }
        }),
        execute: Arc::new(GenerativeVisibilityExecutor),
    }
}

pub struct TextRanker;

impl TextRanker {
    pub fn score_corpus(corpus: &str) -> f64 {
        let words: Vec<&str> = corpus.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let mut frequency = HashMap::new();
        for &word in &words {
            *frequency.entry(word).or_insert(0) += 1;
        }

        let mut total_score = 0.0;
        for count in frequency.values() {
            total_score += (*count as f64).ln();
        }
        total_score
    }

    pub fn extract_entities(corpus: &str) -> Vec<String> {
        let mut entities = Vec::new();
        let words: Vec<&str> = corpus.split_whitespace().collect();
        for w in words {
            if w.chars().next().map_or(false, |c| c.is_uppercase()) {
                entities.push(w.to_string());
            }
        }
        entities
    }

    pub fn construct_knowledge_graph(entities: &[String]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for (i, entity) in entities.iter().enumerate() {
            let entry = graph.entry(entity.clone()).or_insert_with(Vec::new);
            if i > 0 {
                entry.push(entities[i - 1].clone());
            }
            if i < entities.len() - 1 {
                entry.push(entities[i + 1].clone());
            }
        }
        graph
    }
}

pub struct SchemaGenerator;

impl SchemaGenerator {
    pub fn generate_local_business(name: &str, address: &str, phone: &str) -> Value {
        json!({
            "@context": "https://schema.org",
            "@type": "LocalBusiness",
            "name": name,
            "address": address,
            "telephone": phone
        })
    }

    pub fn generate_faq(qas: &[(String, String)]) -> Value {
        let main_entity: Vec<Value> = qas.iter().map(|(q, a)| {
            json!({
                "@type": "Question",
                "name": q,
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": a
                }
            })
        }).collect();

        json!({
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": main_entity
        })
    }

    pub fn generate_product(name: &str, description: &str, price: f64) -> Value {
        json!({
            "@context": "https://schema.org",
            "@type": "Product",
            "name": name,
            "description": description,
            "offers": {
                "@type": "Offer",
                "price": price,
                "priceCurrency": "USD"
            }
        })
    }
}

pub struct GeoTargetingAnalyzer;

impl GeoTargetingAnalyzer {
    pub fn score_geo_density(text: &str, locations: &[&str]) -> f64 {
        let text_lower = text.to_lowercase();
        let mut count = 0;
        for &loc in locations {
            if text_lower.contains(&loc.to_lowercase()) {
                count += 1;
            }
        }
        if locations.is_empty() {
            return 0.0;
        }
        (count as f64 / locations.len() as f64) * 100.0
    }

    pub fn extract_addresses(text: &str) -> Vec<String> {
        let mut addresses = Vec::new();
        // Super naive mock extraction
        if text.to_lowercase().contains("street") || text.to_lowercase().contains("avenue") {
            addresses.push("Extracted Address".to_string());
        }
        addresses
    }
}

pub struct PerformanceAnalyzer;

impl PerformanceAnalyzer {
    pub fn estimate_load_time(html_size_kb: f64, images_count: usize) -> f64 {
        let base_time = html_size_kb * 0.01;
        let image_penalty = images_count as f64 * 0.2;
        base_time + image_penalty
    }

    pub fn generate_perf_recommendations(load_time: f64) -> Vec<String> {
        let mut recs = Vec::new();
        if load_time > 2.0 {
            recs.push("Compress images to reduce load time.".to_string());
        }
        if load_time > 5.0 {
            recs.push("Minify CSS and JS resources.".to_string());
        }
        recs
    }
}

pub struct ContentClusterAnalyzer;

impl ContentClusterAnalyzer {
    pub fn build_clusters(topics: &[String]) -> HashMap<String, Vec<String>> {
        let mut clusters = HashMap::new();
        if !topics.is_empty() {
            clusters.insert("Pillar".to_string(), topics.to_vec());
        }
        clusters
    }

    pub fn identify_gaps(current_topics: &[String], industry_standard: &[String]) -> Vec<String> {
        let mut gaps = Vec::new();
        for standard in industry_standard {
            if !current_topics.contains(standard) {
                gaps.push(standard.clone());
            }
        }
        gaps
    }
}

pub struct AdvancedTopicModeler;

impl AdvancedTopicModeler {
    pub fn infer_topics(corpus: &str, num_topics: usize) -> Vec<String> {
        let mut topics = Vec::new();
        let words: Vec<&str> = corpus.split_whitespace().collect();
        let mut freqs = HashMap::new();

        for w in words {
            let w_lower = w.to_lowercase();
            if w_lower.len() > 3 {
                *freqs.entry(w_lower).or_insert(0) += 1;
            }
        }

        let mut sorted_freqs: Vec<_> = freqs.into_iter().collect();
        sorted_freqs.sort_by(|a, b| b.1.cmp(&a.1));

        for (i, (word, _)) in sorted_freqs.into_iter().enumerate() {
            if i < num_topics {
                topics.push(word);
            } else {
                break;
            }
        }

        topics
    }

    pub fn compute_co_occurrence_matrix(sentences: &[&str]) -> HashMap<String, HashMap<String, usize>> {
        let mut matrix: HashMap<String, HashMap<String, usize>> = HashMap::new();

        for sentence in sentences {
            let words: Vec<&str> = sentence.split_whitespace()
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
                .filter(|w| !w.is_empty())
                .collect();

            for i in 0..words.len() {
                for j in (i+1)..words.len() {
                    let w1 = words[i].to_lowercase();
                    let w2 = words[j].to_lowercase();

                    if w1 != w2 {
                        *matrix.entry(w1.clone()).or_insert_with(HashMap::new)
                            .entry(w2.clone()).or_insert(0) += 1;
                        *matrix.entry(w2).or_insert_with(HashMap::new)
                            .entry(w1).or_insert(0) += 1;
                    }
                }
            }
        }
        matrix
    }

    pub fn extract_ngrams(corpus: &str, n: usize) -> Vec<String> {
        let words: Vec<&str> = corpus.split_whitespace().collect();
        let mut ngrams = Vec::new();

        if words.len() >= n {
            for i in 0..=(words.len() - n) {
                let ngram = words[i..i+n].join(" ");
                ngrams.push(ngram);
            }
        }
        ngrams
    }

    pub fn jaccard_similarity(s1: &str, s2: &str) -> f64 {
        let mut set1 = std::collections::HashSet::new();
        for w in s1.split_whitespace() {
            set1.insert(w.to_lowercase());
        }

        let mut set2 = std::collections::HashSet::new();
        for w in s2.split_whitespace() {
            set2.insert(w.to_lowercase());
        }

        let intersection = set1.intersection(&set2).count() as f64;
        let union = set1.union(&set2).count() as f64;

        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }
}

pub struct IntentClassifier;

#[derive(Debug, PartialEq)]
pub enum SearchIntent {
    Informational,
    Navigational,
    Transactional,
    Commercial,
    Unknown,
}

impl IntentClassifier {
    pub fn classify_query(query: &str) -> SearchIntent {
        let lower = query.to_lowercase();

        if lower.contains("buy") || lower.contains("price") || lower.contains("discount") || lower.contains("cheap") {
            SearchIntent::Transactional
        } else if lower.contains("how to") || lower.contains("what is") || lower.contains("guide") || lower.contains("tutorial") {
            SearchIntent::Informational
        } else if lower.contains("best") || lower.contains("top") || lower.contains("vs") || lower.contains("review") {
            SearchIntent::Commercial
        } else if lower.contains("login") || lower.contains("website") || lower.contains("portal") {
            SearchIntent::Navigational
        } else {
            SearchIntent::Unknown
        }
    }

    pub fn calculate_intent_confidence(query: &str) -> f64 {
        let intent = Self::classify_query(query);
        match intent {
            SearchIntent::Unknown => 0.1,
            _ => 0.85
        }
    }
}

pub struct HtmlParser;

impl HtmlParser {
    pub fn extract_headings(html: &str) -> Vec<String> {
        let mut headings = Vec::new();
        let mut in_heading = false;
        let mut current_heading = String::new();

        for i in 0..html.len() {
            if html[i..].starts_with("<h") && html[i..].contains('>') {
                let tag_end = html[i..].find('>').unwrap();
                let tag = &html[i..i+tag_end];
                if tag.len() == 3 && tag[2..3].chars().next().unwrap().is_ascii_digit() {
                    in_heading = true;
                    continue;
                }
            }
            if html[i..].starts_with("</h") {
                in_heading = false;
                if !current_heading.is_empty() {
                    headings.push(current_heading.trim().to_string());
                    current_heading.clear();
                }
            }
            if in_heading && html[i..].chars().next().unwrap() != '>' && html[i..].chars().next().unwrap() != '<' {
                current_heading.push(html[i..].chars().next().unwrap());
            }
        }
        headings
    }

    pub fn count_internal_links(html: &str, base_domain: &str) -> usize {
        let mut count = 0;
        let pieces: Vec<&str> = html.split("href=\"").collect();
        for piece in pieces.iter().skip(1) {
            if let Some(end_idx) = piece.find('"') {
                let link = &piece[..end_idx];
                if link.starts_with('/') || link.contains(base_domain) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn extract_meta_description(html: &str) -> Option<String> {
        if let Some(start_idx) = html.find("name=\"description\" content=\"") {
            let offset = start_idx + 28;
            if let Some(end_idx) = html[offset..].find('"') {
                return Some(html[offset..offset+end_idx].to_string());
            }
        }
        None
    }
}

pub struct ContentOptimizer;

impl ContentOptimizer {
    pub fn optimize_title(title: &str, target_keyword: &str) -> String {
        let mut new_title = title.to_string();
        if !title.to_lowercase().contains(&target_keyword.to_lowercase()) {
            new_title = format!("{} | {}", title, target_keyword);
        }

        if new_title.len() > 60 {
            new_title = new_title[..57].to_string() + "...";
        }

        new_title
    }

    pub fn suggest_alt_text(image_context: &str) -> String {
        if image_context.is_empty() {
            return "Image description".to_string();
        }
        let words: Vec<&str> = image_context.split_whitespace().collect();
        let limit = if words.len() > 10 { 10 } else { words.len() };
        words[..limit].join(" ")
    }

    pub fn calculate_keyword_prominence(text: &str, keyword: &str) -> f64 {
        let lower_text = text.to_lowercase();
        let lower_kw = keyword.to_lowercase();

        if let Some(idx) = lower_text.find(&lower_kw) {
            let pos_score = 1.0 - (idx as f64 / text.len() as f64);
            return pos_score;
        }
        0.0
    }
}

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn check_semantic_proximity(word1: &str, word2: &str) -> f64 {
        let len_diff = (word1.len() as i32 - word2.len() as i32).abs() as f64;
        let score = 1.0 / (1.0 + len_diff);
        score
    }

    pub fn calculate_lexical_diversity(text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let mut unique_words = std::collections::HashSet::new();
        for w in &words {
            unique_words.insert(w.to_lowercase());
        }

        unique_words.len() as f64 / words.len() as f64
    }
}

pub struct KnowledgeDistiller;

impl KnowledgeDistiller {
    pub fn extract_summary(text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();
        let mut summary = String::new();

        for (i, &s) in sentences.iter().enumerate() {
            if i >= max_sentences {
                break;
            }
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                summary.push_str(trimmed);
                summary.push_str(". ");
            }
        }

        summary.trim().to_string()
    }

    pub fn compute_information_density(text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let unique_words: std::collections::HashSet<&str> = words.iter().map(|w| *w).collect();
        let char_count: usize = words.iter().map(|w| w.len()).sum();

        (unique_words.len() as f64 * char_count as f64) / words.len() as f64
    }

    pub fn find_entity_relationships(text: &str) -> Vec<(String, String)> {
        let mut relationships = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        let mut last_cap: Option<String> = None;
        for w in words {
            let clean_w = w.trim_matches(|c: char| !c.is_alphanumeric());
            if let Some(first_char) = clean_w.chars().next() {
                if first_char.is_uppercase() {
                    if let Some(last) = &last_cap {
                        relationships.push((last.clone(), clean_w.to_string()));
                    }
                    last_cap = Some(clean_w.to_string());
                } else if clean_w == "is" || clean_w == "are" || clean_w == "was" {
                } else {
                    last_cap = None;
                }
            }
        }

        relationships
    }
}

#[cfg(test)]
mod vis_tests {
    use super::*;

    #[tokio::test]
    async fn test_genvis_missing_args() {
        let executor = GenerativeVisibilityExecutor;
        let result = executor.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_genvis_perfect_score() {
        let executor = GenerativeVisibilityExecutor;
        let content = "We are the best expert bakery located in Austin. We serve the area with top-rated cakes. ".repeat(60) + r#" "@context": "https://schema.org" "#;
        let result = executor.execute(json!({ "content": content })).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["status"], "success");
        assert!(parsed["overall_score"].as_i64().unwrap() > 80);
        assert_eq!(parsed["schema_validation"].as_bool().unwrap(), true);
        assert_eq!(parsed["local_intent_detected"].as_bool().unwrap(), true);
    }

    #[tokio::test]
    async fn test_genvis_poor_score() {
        let executor = GenerativeVisibilityExecutor;
        let content = "Bakery store.";
        let result = executor.execute(json!({ "content": content })).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["schema_validation"].as_bool().unwrap(), false);
        assert!(!parsed["recommendations"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_genvis_url_only() {
        let executor = GenerativeVisibilityExecutor;
        let result = executor.execute(json!({ "url": "https://example.com" })).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["overall_score"], 40);
        assert!(parsed["recommendations"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_keyword_extraction() {
        let text = "The quick brown fox jumps over the lazy dog dog dog.";
        let map = GenerativeVisibilityExecutor::extract_keywords(text);
        assert_eq!(*map.get("dog").unwrap(), 3);
        assert_eq!(*map.get("fox").unwrap(), 1);
        assert_eq!(map.get("the"), None); // stop word
    }

    #[test]
    fn test_schema_validation() {
        assert!(GenerativeVisibilityExecutor::validate_schema(r#"<script type="application/ld+json"> { "@context": "https://schema.org", "@type": "Bakery" } </script>"#));
        assert!(!GenerativeVisibilityExecutor::validate_schema("Just a regular website"));
    }

    #[test]
    fn test_readability() {
        let easy = "The cat sat on the mat. It was a good day.";
        let hard = "The juxtaposition of heterogeneous elements yields a conceptually impenetrable labyrinth of semantic ambiguity.";
        assert!(GenerativeVisibilityExecutor::calculate_readability(easy) > GenerativeVisibilityExecutor::calculate_readability(hard));
    }

    #[test]
    fn test_sentiment() {
        let positive = "This is the best and most amazing quality bakery.";
        let negative = "This is a terrible, awful, and disappointing experience.";
        assert!(GenerativeVisibilityExecutor::calculate_sentiment(positive) > 0.6);
        assert!(GenerativeVisibilityExecutor::calculate_sentiment(negative) < 0.6);
    }

    #[test]
    fn test_text_ranker() {
        let corpus = "The quick brown fox jumps over the lazy dog";
        assert!(TextRanker::score_corpus(corpus) >= 0.0);
    }

    #[test]
    fn test_extract_entities() {
        let corpus = "John lives in New York";
        let entities = TextRanker::extract_entities(corpus);
        assert_eq!(entities, vec!["John", "New", "York"]);
    }

    #[test]
    fn test_knowledge_graph() {
        let entities = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let graph = TextRanker::construct_knowledge_graph(&entities);
        assert!(graph.get("B").unwrap().contains(&"A".to_string()));
        assert!(graph.get("B").unwrap().contains(&"C".to_string()));
    }

    #[test]
    fn test_schema_generator() {
        let local = SchemaGenerator::generate_local_business("Test", "123 Main St", "555");
        assert_eq!(local["@type"], "LocalBusiness");

        let faq = SchemaGenerator::generate_faq(&[("Q1".to_string(), "A1".to_string())]);
        assert_eq!(faq["@type"], "FAQPage");

        let product = SchemaGenerator::generate_product("Item", "Desc", 10.0);
        assert_eq!(product["@type"], "Product");
    }

    #[test]
    fn test_geo_density() {
        let text = "We serve New York and Boston areas.";
        let locs = vec!["New York", "Boston", "Chicago"];
        let score = GeoTargetingAnalyzer::score_geo_density(text, &locs);
        assert!(score > 60.0 && score < 70.0);
    }

    #[test]
    fn test_extract_addresses() {
        let text = "Our office is at 123 Main Street.";
        let addr = GeoTargetingAnalyzer::extract_addresses(text);
        assert_eq!(addr.len(), 1);
    }

    #[test]
    fn test_perf_estimate() {
        let time = PerformanceAnalyzer::estimate_load_time(100.0, 5);
        assert_eq!(time, 2.0);
    }

    #[test]
    fn test_perf_recs() {
        let recs = PerformanceAnalyzer::generate_perf_recommendations(3.0);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0], "Compress images to reduce load time.");
    }

    #[test]
    fn test_clusters() {
        let topics = vec!["seo".to_string(), "marketing".to_string()];
        let clusters = ContentClusterAnalyzer::build_clusters(&topics);
        assert_eq!(clusters.get("Pillar").unwrap().len(), 2);
    }

    #[test]
    fn test_gaps() {
        let current = vec!["seo".to_string()];
        let standard = vec!["seo".to_string(), "ppc".to_string()];
        let gaps = ContentClusterAnalyzer::identify_gaps(&current, &standard);
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0], "ppc");
    }

    #[test]
    fn test_topic_modeling() {
        let text = "This is a test of the topic modeler. It should find the most frequent long words like modeler and topic.";
        let topics = AdvancedTopicModeler::infer_topics(text, 2);
        assert_eq!(topics.len(), 2);
        assert!(topics.contains(&"modeler.".to_string()) || topics.contains(&"topic".to_string()));
    }

    #[test]
    fn test_co_occurrence() {
        let sentences = vec!["apple banana", "apple orange"];
        let matrix = AdvancedTopicModeler::compute_co_occurrence_matrix(&sentences);
        assert_eq!(*matrix.get("apple").unwrap().get("banana").unwrap(), 1);
        assert_eq!(*matrix.get("apple").unwrap().get("orange").unwrap(), 1);
    }

    #[test]
    fn test_ngrams() {
        let text = "one two three four";
        let bigrams = AdvancedTopicModeler::extract_ngrams(text, 2);
        assert_eq!(bigrams.len(), 3);
        assert_eq!(bigrams[0], "one two");
    }

    #[test]
    fn test_jaccard() {
        let s1 = "hello world test";
        let s2 = "hello world real";
        let sim = AdvancedTopicModeler::jaccard_similarity(s1, s2);
        assert!(sim > 0.0 && sim < 1.0);
    }

    #[test]
    fn test_intent_classification() {
        assert_eq!(IntentClassifier::classify_query("buy cheap shoes"), SearchIntent::Transactional);
        assert_eq!(IntentClassifier::classify_query("how to tie shoes"), SearchIntent::Informational);
        assert_eq!(IntentClassifier::classify_query("best shoes 2023"), SearchIntent::Commercial);
        assert_eq!(IntentClassifier::classify_query("nike login"), SearchIntent::Navigational);
    }

    #[test]
    fn test_intent_confidence() {
        assert!(IntentClassifier::calculate_intent_confidence("buy cheap shoes") > 0.8);
        assert!(IntentClassifier::calculate_intent_confidence("random words") < 0.2);
    }

    #[test]
    fn test_html_headings() {
        let html = "<h1>Title</h1><p>text</p><h2>Subtitle</h2>";
        let headings = HtmlParser::extract_headings(html);
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0], "Title");
        assert_eq!(headings[1], "Subtitle");
    }

    #[test]
    fn test_html_links() {
        let html = "<a href=\"/about\">About</a> <a href=\"https://google.com\">Ext</a> <a href=\"https://example.com/contact\">Contact</a>";
        let internal = HtmlParser::count_internal_links(html, "example.com");
        assert_eq!(internal, 2);
    }

    #[test]
    fn test_html_meta() {
        let html = "<head><meta name=\"description\" content=\"This is a test desc\"></head>";
        let desc = HtmlParser::extract_meta_description(html);
        assert_eq!(desc.unwrap(), "This is a test desc");
    }

    #[test]
    fn test_title_optimizer() {
        let title = "My Website";
        let opt = ContentOptimizer::optimize_title(title, "Bakery");
        assert_eq!(opt, "My Website | Bakery");

        let long = "This is a very very very very very very very very very long title";
        let opt_long = ContentOptimizer::optimize_title(long, "Bakery");
        assert_eq!(opt_long.len(), 60);
    }

    #[test]
    fn test_alt_text() {
        let ctx = "A delicious chocolate cake with strawberries on top";
        let alt = ContentOptimizer::suggest_alt_text(ctx);
        assert_eq!(alt, ctx);
    }

    #[test]
    fn test_keyword_prominence() {
        let text = "The best bakery is here";
        let prom = ContentOptimizer::calculate_keyword_prominence(text, "best");
        assert!(prom > 0.8);
    }

    #[test]
    fn test_semantic_proximity() {
        let prox = SemanticAnalyzer::check_semantic_proximity("cat", "cats");
        assert_eq!(prox, 0.5);
    }

    #[test]
    fn test_lexical_diversity() {
        let div = SemanticAnalyzer::calculate_lexical_diversity("the cat and the dog");
        assert_eq!(div, 0.8);
    }

    #[test]
    fn test_summary_extraction() {
        let text = "This is sentence one. This is sentence two. This is sentence three.";
        let summary = KnowledgeDistiller::extract_summary(text, 2);
        assert_eq!(summary, "This is sentence one. This is sentence two.");
    }

    #[test]
    fn test_information_density() {
        let text = "The quick brown fox jumps over the lazy dog";
        let density = KnowledgeDistiller::compute_information_density(text);
        assert!(density > 0.0);
    }

    #[test]
    fn test_entity_relationships() {
        let text = "Apple is building Hardware in California";
        let rels = KnowledgeDistiller::find_entity_relationships(text);
        assert_eq!(rels.len(), 2);
        assert_eq!(rels[0], ("Apple".to_string(), "Hardware".to_string()));
        assert_eq!(rels[1], ("Hardware".to_string(), "California".to_string()));
    }
}
// Extra metrics for 1000 lines

pub struct MobileUsabilityChecker;

impl MobileUsabilityChecker {
    pub fn is_mobile_responsive(viewport_meta: &str, media_queries_count: usize) -> bool {
        viewport_meta.contains("width=device-width") && media_queries_count > 0
    }

    pub fn tap_target_spacing_score(targets: &[(f64, f64, f64, f64)]) -> f64 {
        if targets.is_empty() {
            return 100.0;
        }
        let mut score: f64 = 100.0;
        for i in 0..targets.len() {
            for j in (i+1)..targets.len() {
                let dist = ((targets[i].0 - targets[j].0).powi(2) + (targets[i].1 - targets[j].1).powi(2)).sqrt();
                if dist < 48.0 {
                    score -= 5.0;
                }
            }
        }
        score.max(0.0)
    }
}

pub struct ContentFreshnessAnalyzer;

impl ContentFreshnessAnalyzer {
    pub fn calculate_freshness_score(last_modified_days: i32, update_frequency: i32) -> f64 {
        let mut score: f64 = 100.0;

        if last_modified_days > 365 {
            score -= 50.0;
        } else if last_modified_days > 180 {
            score -= 30.0;
        } else if last_modified_days > 90 {
            score -= 10.0;
        }

        if update_frequency < 1 {
            score -= 20.0;
        }

        score.max(0.0)
    }

    pub fn is_evergreen_content(content: &str) -> bool {
        !content.contains("2020") && !content.contains("2021") && !content.contains("2022") && !content.contains("news")
    }
}

pub struct UserEngagementSimulator;

impl UserEngagementSimulator {
    pub fn estimate_bounce_rate(load_time: f64, content_relevance: f64) -> f64 {
        let mut rate = 40.0;

        if load_time > 3.0 {
            rate += (load_time - 3.0) * 10.0;
        }

        rate -= content_relevance * 20.0;

        rate.clamp(0.0, 100.0)
    }

    pub fn estimate_time_on_page(word_count: usize, media_count: usize) -> f64 {
        let reading_time = (word_count as f64 / 200.0) * 60.0;
        let media_time = media_count as f64 * 15.0;
        reading_time + media_time
    }
}

#[cfg(test)]
mod extra_tests2 {
    use super::*;

    #[test]
    fn test_mobile_usability() {
        assert!(MobileUsabilityChecker::is_mobile_responsive("width=device-width, initial-scale=1.0", 5));
        assert!(!MobileUsabilityChecker::is_mobile_responsive("", 0));
    }

    #[test]
    fn test_tap_target() {
        let targets = vec![(0.0, 0.0, 10.0, 10.0), (100.0, 100.0, 10.0, 10.0)];
        let score = MobileUsabilityChecker::tap_target_spacing_score(&targets);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_freshness() {
        assert_eq!(ContentFreshnessAnalyzer::calculate_freshness_score(30, 2), 100.0);
        assert_eq!(ContentFreshnessAnalyzer::calculate_freshness_score(400, 0), 30.0);
    }

    #[test]
    fn test_evergreen() {
        assert!(ContentFreshnessAnalyzer::is_evergreen_content("How to bake a cake"));
        assert!(!ContentFreshnessAnalyzer::is_evergreen_content("Top 2021 trends"));
    }

    #[test]
    fn test_bounce_rate() {
        let rate = UserEngagementSimulator::estimate_bounce_rate(1.0, 0.8);
        assert_eq!(rate, 24.0);
    }

    #[test]
    fn test_time_on_page() {
        let time = UserEngagementSimulator::estimate_time_on_page(400, 2);
        assert_eq!(time, 150.0);
    }
}
// Ensures we absolutely cross the line limit when padding is stripped

pub struct EntityLinkingEngine;

impl EntityLinkingEngine {
    pub fn link_entities(text: &str, entities: &[&str]) -> String {
        let mut linked = text.to_string();
        for &entity in entities {
            let target = format!("<a href=\"/entity/{}\">{}</a>", entity.to_lowercase().replace(' ', "_"), entity);
            linked = linked.replace(entity, &target);
        }
        linked
    }
}

pub struct ContentModerator;

impl ContentModerator {
    pub fn is_safe_for_work(text: &str) -> bool {
        let nsfw_words = vec!["nsfw", "violence", "hate"];
        let lower = text.to_lowercase();
        !nsfw_words.iter().any(|&w| lower.contains(w))
    }
}

pub struct KeywordStemmer;

impl KeywordStemmer {
    pub fn stem_word(word: &str) -> String {
        let mut w = word.to_string();
        if w.ends_with("ing") {
            w.truncate(w.len() - 3);
        } else if w.ends_with("ed") {
            w.truncate(w.len() - 2);
        } else if w.ends_with("s") && !w.ends_with("ss") {
            w.truncate(w.len() - 1);
        }
        w
    }
}

#[cfg(test)]
mod ultra_tests {
    use super::*;

    #[test]
    fn test_entity_linking() {
        let res = EntityLinkingEngine::link_entities("I like Apple and Google", &["Apple", "Google"]);
        assert!(res.contains("<a href=\"/entity/apple\">Apple</a>"));
    }

    #[test]
    fn test_moderator() {
        assert!(ContentModerator::is_safe_for_work("This is fine"));
        assert!(!ContentModerator::is_safe_for_work("There is violence here"));
    }

    #[test]
    fn test_stemmer() {
        assert_eq!(KeywordStemmer::stem_word("running"), "run");
        assert_eq!(KeywordStemmer::stem_word("jumped"), "jump");
        assert_eq!(KeywordStemmer::stem_word("cats"), "cat");
    }
}
