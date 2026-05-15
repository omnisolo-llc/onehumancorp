use crate::services::docs::registry::HelpRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DocService {
    pub registry: Arc<RwLock<HelpRegistry>>,
}

impl DocService {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HelpRegistry::new())),
        }
    }

    pub async fn get_article(&self, id: &str) -> Option<crate::services::docs::content::HelpArticle> {
        let registry = self.registry.read().await;
        registry.articles.get(id).map(|a| crate::services::docs::content::HelpArticle {
            id: a.id.clone(),
            title: a.title.clone(),
            content: a.content.clone(),
            category: a.category.clone(),
            tags: a.tags.clone(),
        })
    }

    pub async fn get_tooltip(&self, id: &str) -> Option<crate::services::docs::content::Tooltip> {
        let registry = self.registry.read().await;
        registry.tooltips.get(id).map(|t| crate::services::docs::content::Tooltip {
            id: t.id.clone(),
            text: t.text.clone(),
            target_element: t.target_element.clone(),
        })
    }

    pub async fn search_articles(&self, query: &str) -> Vec<crate::services::docs::content::HelpArticle> {
        let registry = self.registry.read().await;
        let query_lower = query.to_lowercase();
        registry.articles.values()
            .filter(|a| a.title.to_lowercase().contains(&query_lower) || a.content.to_lowercase().contains(&query_lower))
            .map(|a| crate::services::docs::content::HelpArticle {
                id: a.id.clone(),
                title: a.title.clone(),
                content: a.content.clone(),
                category: a.category.clone(),
                tags: a.tags.clone(),
            })
            .collect()
    }
}
