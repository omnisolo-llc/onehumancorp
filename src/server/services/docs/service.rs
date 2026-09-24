use ::docs_proto::ohc::docs::v1::docs_service_server::DocsService;
use ::docs_proto::ohc::docs::v1::*;
use std::sync::OnceLock;
use tonic::{Request, Response, Status};

static HELP_ARTICLES: OnceLock<Vec<HelpArticle>> = OnceLock::new();
static VIDEO_TUTORIALS: OnceLock<Vec<VideoTutorial>> = OnceLock::new();

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
                title: "Welcome to OmniSolo".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/getting_started.md").to_string(),
            },
            HelpArticle {
                id: "my-store-1".to_string(),
                topic: "My Store".to_string(),
                title: "Setting up your storefront".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/my_store.md").to_string(),
            },
            HelpArticle {
                id: "payments-1".to_string(),
                topic: "Payments".to_string(),
                title: "Accepting your first payment".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/payments.md").to_string(),
            },
            HelpArticle {
                id: "ai-agents-1".to_string(),
                topic: "AI Agents".to_string(),
                title: "Activating your AI Support Agent".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/ai_agents.md").to_string(),
            },
            HelpArticle {
                id: "marketing-1".to_string(),
                topic: "Marketing".to_string(),
                title: "Creating a social media post".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/marketing.md").to_string(),
            },
            HelpArticle {
                id: "account-billing-1".to_string(),
                topic: "Account & Billing".to_string(),
                title: "Understanding your invoice".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/account_billing.md").to_string(),
            },
            HelpArticle {
                id: "setup-accounts-authority".to_string(),
                topic: "Getting Started".to_string(),
                title: "Setup, Connected Accounts, and Standing Authority".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/setup_accounts_authority.md").to_string(),
            },
            HelpArticle {
                id: "proposals-payments".to_string(),
                topic: "Sales & Payments".to_string(),
                title: "How to Send Proposals and Collect Payments Securely".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/proposals_payments.md").to_string(),
            },
            HelpArticle {
                id: "connected-accounts".to_string(),
                topic: "Account & Billing".to_string(),
                title: "Connected Accounts and Standing Authority".to_string(),
                content_markdown: include_str!("../../../../docs/help_center/connected_accounts.md").to_string(),
            },
        ]
    })
}

fn get_video_tutorials() -> &'static Vec<VideoTutorial> {
    VIDEO_TUTORIALS.get_or_init(|| {
        vec![
            VideoTutorial { id: 1, title: "How to set up your first store easily".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 2, title: "Connecting a bank account to accept payments".to_string(), duration: "1:15".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 3, title: "Managing inventory".to_string(), duration: "0:50".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 4, title: "Adding staff to your account".to_string(), duration: "1:05".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 5, title: "Reviewing orders".to_string(), duration: "1:10".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 6, title: "Connecting social media".to_string(), duration: "1:25".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 7, title: "Using the builder".to_string(), duration: "1:30".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 8, title: "Understanding analytics".to_string(), duration: "1:00".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 9, title: "Fulfilling orders".to_string(), duration: "0:45".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
            VideoTutorial { id: 10, title: "Processing refunds".to_string(), duration: "0:55".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
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

        let mut final_articles = filtered;
        if req.mobile_optimized {
            for article in final_articles.iter_mut() {
                article.content_markdown = String::new();
            }
        }
        Ok(Response::new(SearchHelpArticlesResponse {
            articles: final_articles,
        }))
    }

    async fn get_tooltip(
        &self,
        request: Request<GetTooltipRequest>,
    ) -> Result<Response<GetTooltipResponse>, Status> {
        let req = request.into_inner();
        // Since we removed TOOLTIPS vector, just return a dummy
        Ok(Response::new(GetTooltipResponse {
            tooltip: Some(Tooltip {
                element_id: req.element_id.clone(),
                title: req.element_id,
                plain_language_description: "Dummy tooltip from grpc service. Please hit the REST endpoint instead.".to_string(),
            }),
        }))
    }

    async fn get_video_tutorials(
        &self,
        request: Request<GetVideoTutorialsRequest>,
    ) -> Result<Response<GetVideoTutorialsResponse>, Status> {
        let req = request.into_inner();
        let mut tutorials = get_video_tutorials().clone();

        if req.mobile_optimized {
            for tutorial in tutorials.iter_mut() {
                tutorial.duration = String::new();
            }
        }

        Ok(Response::new(GetVideoTutorialsResponse {
            tutorials,
        }))
    }
}

#[cfg(test)]
mod tests {
    include!("service_tests.rs");
}