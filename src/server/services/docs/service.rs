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
                content_markdown: "Welcome to OmniSolo! This is a simple app that helps you manage your small business. You can set up your store, accept payments, and hire AI helpers.".to_string(),
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
            HelpArticle {
                id: "proposals-payments".to_string(),
                topic: "Sales & Payments".to_string(),
                title: "How to Send Proposals and Collect Payments Securely".to_string(),
                content_markdown: "# How to Send Proposals and Collect Payments Securely\n\n**Problem Statement:**\nAs a small business owner, it's critical to know that the proposals you send to your clients contain the exact scope and pricing you agreed upon, and that payment links are secure and verifiable.\n\n**Current Capabilities:**\n- **No More Placeholder Proposals:** OmniSolo ensures that the quote generated directly reflects the customer's inquiry and your explicit business rules. We validate owner-supplied line items and deposits with checked math. If pricing information is missing, the system will mark it as `NEEDS_PRICING` for your review. Optional, unselected items are strictly excluded from committed totals.\n- **Secure Checkout Links:** When you create an invoice, OmniSolo generates a real, secure session with your connected payment provider (like Stripe). It will not generate fictitious checkout URLs.\n- **Drafts vs. Sent Reminders:** The system clearly distinguishes between drafting a reminder and actually delivering it. We only persist actual, source-grounded drafts.\n- **Your Authority:** These actions operate under your standing authority. The system enforces your hard spend reservations.\n\n**How to Use This Feature:**\n1. Open a Lead or Inquiry in the OmniSolo app.\n2. Click **Generate Proposal**. Review the line items carefully. If any items are marked `NEEDS_PRICING`, fill in the correct amounts.\n3. Approve the proposal to finalize the scope and price.\n4. When ready, click **Create Invoice**. OmniSolo will securely connect to your payment provider to generate a verifiable checkout link.\n5. The resulting email/SMS draft will be placed in your outbox for final review before sending.\n\n**Cost & Expectations:**\nThese actions use your regular OHC subscription. There are no hidden markup fees on your customer's invoice. Note that standard payment processor fees still apply.\n\n**Exceptions and Recovery:**\nIf your payment provider disconnects or fails to create a link, the invoice status will remain \"Draft/Pending Provider\". You can simply try generating the link again later.".to_string(),
            },
            HelpArticle {
                id: "capability-economics-migration-1".to_string(),
                topic: "Operating Capabilities".to_string(),
                title: "OmniSolo Operating Capabilities and Native Migration".to_string(),
                content_markdown: "# OmniSolo Operating Capabilities and Native Migration

Welcome to OmniSolo. This guide outlines how you, as a solo independent service owner, can accomplish verified business work with our current product, and what features are currently supported versus planned for future native migrations.

## Current Capability Evidence

Here is how you can use OmniSolo to run your operating business today:

### Business Setup
- **Current Support:** Comprehensive onboarding state flows exist to help you set up your profile and configure your basic business rules.
- **Planned Improvements:** We are continuing to enhance the verified owner-ready operating business templates for various service niches.

### Customer Proposals
- **Current Support:** Tenant and customer records, deposit tracking, and proposal approvals are fully supported.
- **Planned Improvements:** Currently, intake writes fixed scope independent of inquiries. Future updates will allow for more dynamic, inquiry-driven proposal generation.

### Calendars & Tools
- **Current Support:** Authenticated Google Calendar and Google Workspace requests are functional for basic scheduling.
- **Planned Improvements:** Full, secure credential lifecycles and robust connection management are undergoing separate proofing and will be expanded natively.

### Payments
- **Current Support:** Core invoicing logic is present and available for use.
- **Planned Improvements:** Complete Stripe and MercadoPago workflows with real provider checkout sessions are currently being reconciled for seamless end-to-end payment collection.

### Agent Coordination
- **Current Support:** Department and event coordination logic exists within the system.
- **Planned Improvements:** We are transitioning simulated feed items to real agent execution, providing you with persistent approval workflows and concrete result evidence of your agent\'s actions.

### Provider Runtime
- **Current Support:** Scoped proxy and secret redaction are implemented for security.
- **Planned Improvements:** We are finalizing reliable accounting and clear payer modes before full usage billing is enabled across the platform.".to_string(),
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