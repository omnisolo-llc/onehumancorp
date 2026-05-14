
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
}

pub struct DocEngine {
    articles: HashMap<String, Article>,
}

impl DocEngine {
    pub fn new() -> Self {
        Self {
            articles: HashMap::new(),
        }
    }

    pub fn add_article(&mut self, article: Article) {
        self.articles.insert(article.id.clone(), article);
    }

    pub fn get_article(&self, id: &str) -> Option<Article> {
        self.articles.get(id).cloned()
    }

    pub fn search(&self, query: &str) -> Vec<Article> {
        let q = query.to_lowercase();
        let mut results: Vec<Article> = self.articles
            .values()
            .filter(|a| {
                a.title.to_lowercase().contains(&q) ||
                a.content.to_lowercase().contains(&q) ||
                a.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect();

        // Simple relevance sorting
        results.sort_by(|a, b| {
            let a_score = if a.title.to_lowercase().contains(&q) { 2 } else { 1 };
            let b_score = if b.title.to_lowercase().contains(&q) { 2 } else { 1 };
            b_score.cmp(&a_score)
        });

        results
    }
}
