use crate::proto::help::{HelpArticle, Tooltip, Walkthrough, WalkthroughStep, VideoTutorial, ReleaseNote};
use sqlx::{Pool, Postgres, Error};

pub struct HelpRepository {
    pool: Pool<Postgres>,
}

impl HelpRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_articles(&self, query: &str) -> Result<Vec<HelpArticle>, Error> {
        let mut articles = vec![
            HelpArticle {
                id: "1".to_string(),
                title: "Set up your store in 5 minutes".to_string(),
                content: "Learn how to add your business name, logo, and hours.".to_string(),
                category: "Getting Started".to_string(),
                created_at: "2023-10-01".to_string(),
                updated_at: "2023-10-01".to_string(),
            },
            HelpArticle {
                id: "2".to_string(),
                title: "Accept your first payment".to_string(),
                content: "Connect your bank account and start getting paid today.".to_string(),
                category: "Payments".to_string(),
                created_at: "2023-10-02".to_string(),
                updated_at: "2023-10-02".to_string(),
            },
            HelpArticle {
                id: "3".to_string(),
                title: "What is an AI Support Agent?".to_string(),
                content: "Let our AI handle basic customer questions for you.".to_string(),
                category: "AI Agents".to_string(),
                created_at: "2023-10-03".to_string(),
                updated_at: "2023-10-03".to_string(),
            },
        ];

        let q = query.to_lowercase();
        if !q.is_empty() {
            articles.retain(|a| a.title.to_lowercase().contains(&q));
        }

        Ok(articles)
    }

    pub async fn get_tooltips(&self, screen: &str) -> Result<Vec<Tooltip>, Error> {
        Ok(vec![
            Tooltip {
                id: "api-docs-link".to_string(),
                element_id: "api-docs-link".to_string(),
                text: "For developers only: API documentation".to_string(),
                screen: screen.to_string(),
            }
        ])
    }

    pub async fn get_walkthroughs(&self) -> Result<Vec<Walkthrough>, Error> {
        Ok(vec![
            Walkthrough {
                id: "w1".to_string(),
                title: "Onboarding".to_string(),
                steps: vec![
                    WalkthroughStep {
                        id: "s1".to_string(),
                        element_id: "store-name".to_string(),
                        text: "Enter your store name here.".to_string(),
                        order: 1,
                    }
                ],
            }
        ])
    }

    pub async fn get_video_tutorials(&self) -> Result<Vec<VideoTutorial>, Error> {
        Ok(vec![
            VideoTutorial {
                id: "v1".to_string(),
                title: "How to add a product".to_string(),
                url: "https://example.com/v1.mp4".to_string(),
                duration: "1:15".to_string(),
                thumbnail_url: "https://via.placeholder.com/300x169?text=Product+Tutorial".to_string(),
            },
            VideoTutorial {
                id: "v2".to_string(),
                title: "Connecting your bank".to_string(),
                url: "https://example.com/v2.mp4".to_string(),
                duration: "0:45".to_string(),
                thumbnail_url: "https://via.placeholder.com/300x169?text=Bank+Tutorial".to_string(),
            },
            VideoTutorial {
                id: "v3".to_string(),
                title: "Customizing your storefront".to_string(),
                url: "https://example.com/v3.mp4".to_string(),
                duration: "1:20".to_string(),
                thumbnail_url: "https://via.placeholder.com/300x169?text=Store+Tutorial".to_string(),
            }
        ])
    }

    pub async fn get_release_notes(&self) -> Result<Vec<ReleaseNote>, Error> {
        Ok(vec![
            ReleaseNote {
                version: "1.2.0".to_string(),
                title: "Faster checkouts".to_string(),
                description: "We made the checkout process 20% faster so your customers can complete purchases with fewer clicks.".to_string(),
                date: "October 24, 2023".to_string(),
                screenshots: vec![],
            },
            ReleaseNote {
                version: "1.1.5".to_string(),
                title: "New AI Agent features".to_string(),
                description: "Your AI agent can now automatically send receipts after a successful purchase.".to_string(),
                date: "October 10, 2023".to_string(),
                screenshots: vec![],
            }
        ])
    }

    pub async fn ask_chat(&self, query: &str) -> Result<(String, Vec<String>), Error> {
        let answer = format!("I can help with \"{}\". Have you checked our Getting Started guide?", query);
        Ok((answer, vec!["1".to_string()]))
    }
}
