use ::docs::ohc::docs::v1::docs_service_server::DocsService;
use ::docs::ohc::docs::v1::*;
use std::sync::OnceLock;
use tonic::{Request, Response, Status};

static HELP_ARTICLES: OnceLock<Vec<HelpArticle>> = OnceLock::new();
static TOOLTIPS: OnceLock<Vec<Tooltip>> = OnceLock::new();

pub struct MyDocsService;

impl MyDocsService {
    pub fn new() -> Self {
        Self
    }
}

fn get_articles() -> &'static Vec<HelpArticle> {
    HELP_ARTICLES.get_or_init(|| {
        vec![
            HelpArticle {
                id: "getting-started-1".to_string(),
                topic: "Getting Started".to_string(),
                title: "Welcome to One Human Corp".to_string(),
                content_markdown: "Welcome to One Human Corp! This is a simple app that helps you manage your small business. You can set up your store, accept payments, and hire AI helpers.".to_string(),
            },
            HelpArticle {
                id: "my-store-1".to_string(),
                topic: "My Store".to_string(),
                title: "Setting up your storefront".to_string(),
                content_markdown: "To set up your storefront, go to the 'My Store' tab and add your products. It's easy! Just upload a photo, write a simple description, and set a price.".to_string(),
            },
            HelpArticle {
                id: "payments-1".to_string(),
                topic: "Payments".to_string(),
                title: "Accepting your first payment".to_string(),
                content_markdown: "When a customer buys something, the money goes straight to your account. We handle all the technical details so you can focus on your business.".to_string(),
            },
            HelpArticle {
                id: "ai-agents-1".to_string(),
                topic: "AI Agents".to_string(),
                title: "Activating your AI Support Agent".to_string(),
                content_markdown: "Need a hand? Your AI Support Agent can answer customer emails and chats for you while you sleep. Just turn it on in the 'AI Agents' tab.".to_string(),
            },
            HelpArticle {
                id: "marketing-1".to_string(),
                topic: "Marketing".to_string(),
                title: "Creating a social media post".to_string(),
                content_markdown: "Let our AI write your social media posts! Just tell it what you want to sell, and it will give you a catchy post to share with your customers.".to_string(),
            },
            HelpArticle {
                id: "account-billing-1".to_string(),
                topic: "Account & Billing".to_string(),
                title: "Understanding your invoice".to_string(),
                content_markdown: "Your monthly invoice shows exactly what you paid for. We keep things simple with no hidden fees.".to_string(),
            },
        ]
    })
}

fn get_tooltips() -> &'static Vec<Tooltip> {
    TOOLTIPS.get_or_init(|| {
        vec![
            Tooltip {
                element_id: "nav-store".to_string(),
                title: "Your Storefront".to_string(),
                plain_language_description: "This is where you manage what you sell. Add or edit products here.".to_string(),
            },
            Tooltip {
                element_id: "nav-agents".to_string(),
                title: "AI Helpers".to_string(),
                plain_language_description: "These are your digital employees. They can talk to customers and do tasks for you.".to_string(),
            },
            Tooltip {
                element_id: "btn-new-product".to_string(),
                title: "Add Product".to_string(),
                plain_language_description: "Click here to add something new to sell. You can add a photo and a price.".to_string(),
            },
            Tooltip {
                element_id: "setting-multitenant".to_string(),
                title: "Cloud Mode".to_string(),
                plain_language_description: "Runs your app on our fast servers. This is best for most businesses.".to_string(),
            },
            Tooltip {
                element_id: "setting-standalone".to_string(),
                title: "Standalone Mode".to_string(),
                plain_language_description: "Runs entirely on your computer. Great if you don't have internet.".to_string(),
            },
        ]
    })
}

#[tonic::async_trait]
impl DocsService for MyDocsService {
    async fn get_help_article(
        &self,
        request: Request<GetHelpArticleRequest>,
    ) -> Result<Response<GetHelpArticleResponse>, Status> {
        let req = request.into_inner();
        let articles = get_articles();

        if let Some(article) = articles.iter().find(|a| a.id == req.id) {
            Ok(Response::new(GetHelpArticleResponse {
                article: Some(article.clone()),
            }))
        } else {
            Err(Status::not_found("Help article not found"))
        }
    }

    async fn search_help_articles(
        &self,
        request: Request<SearchHelpArticlesRequest>,
    ) -> Result<Response<SearchHelpArticlesResponse>, Status> {
        let req = request.into_inner();
        let articles = get_articles();

        let query_lower = req.query.to_lowercase();

        let filtered: Vec<HelpArticle> = articles
            .iter()
            .filter(|a| {
                let matches_topic = if req.topic_filter.is_empty() {
                    true
                } else {
                    a.topic.to_lowercase() == req.topic_filter.to_lowercase()
                };

                let matches_query = if query_lower.is_empty() {
                    true
                } else {
                    a.title.to_lowercase().contains(&query_lower)
                        || a.content_markdown.to_lowercase().contains(&query_lower)
                };

                matches_topic && matches_query
            })
            .cloned()
            .collect();

        Ok(Response::new(SearchHelpArticlesResponse {
            articles: filtered,
        }))
    }

    async fn get_tooltip(
        &self,
        request: Request<GetTooltipRequest>,
    ) -> Result<Response<GetTooltipResponse>, Status> {
        let req = request.into_inner();
        let tooltips = get_tooltips();

        if let Some(tooltip) = tooltips.iter().find(|t| t.element_id == req.element_id) {
            Ok(Response::new(GetTooltipResponse {
                tooltip: Some(tooltip.clone()),
            }))
        } else {
            Err(Status::not_found("Tooltip not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    include!("service_tests.rs");
}
