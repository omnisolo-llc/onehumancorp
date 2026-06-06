use axum::{extract::Query, Json};


use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct HelpArticle {
    pub title: String,
    pub desc: String,
    pub link: String,
}

#[derive(Serialize, Clone)]
pub struct VideoTutorial {
    pub id: i32,
    pub title: String,
    pub duration: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub fn get_articles() -> Vec<HelpArticle> {
    vec![
        HelpArticle { title: "Getting Started".to_string(), desc: "Learn how to easily set up your store and accept your first payment.".to_string(), link: "/help/getting-started-1".to_string() },
        HelpArticle { title: "My Store".to_string(), desc: "Add products, track what's in stock, and change how your store looks.".to_string(), link: "/help/my-store".to_string() },
        HelpArticle { title: "Getting Paid".to_string(), desc: "Set up how you get paid, view deposits, and handle simple taxes.".to_string(), link: "/help/payments".to_string() },
        HelpArticle { title: "Your AI Helpers".to_string(), desc: "Learn how to hire AI helpers and give them tasks to do.".to_string(), link: "/help/ai-agents".to_string() },
        HelpArticle { title: "Finding Customers".to_string(), desc: "Send emails to customers and grow your business easily.".to_string(), link: "/help/marketing".to_string() },
        HelpArticle { title: "Account & Billing".to_string(), desc: "View your bills, manage your plan, and invite team members.".to_string(), link: "/help/account-billing".to_string() }
    ]
}

pub fn get_videos() -> Vec<VideoTutorial> {
    vec![
        VideoTutorial { id: 1, title: "How to set up your first store easily".to_string(), duration: "1:20".to_string() },
        VideoTutorial { id: 2, title: "Accept your first payment".to_string(), duration: "1:15".to_string() },
        VideoTutorial { id: 3, title: "Activate your AI Support Agent".to_string(), duration: "0:50".to_string() },
        VideoTutorial { id: 4, title: "Adding staff to your account".to_string(), duration: "1:05".to_string() },
        VideoTutorial { id: 5, title: "Review an order".to_string(), duration: "1:10".to_string() },
        VideoTutorial { id: 6, title: "Send a campaign".to_string(), duration: "1:25".to_string() },
        VideoTutorial { id: 7, title: "Connect Stripe".to_string(), duration: "1:30".to_string() },
        VideoTutorial { id: 8, title: "Manage inventory".to_string(), duration: "1:00".to_string() },
    ]
}

pub async fn list_articles() -> Json<Vec<HelpArticle>> {
    Json(get_articles())
}

pub async fn search_articles(Query(query): Query<SearchQuery>) -> Json<Vec<HelpArticle>> {
    let q = query.q.to_lowercase();
    let articles = get_articles();
    let filtered = articles.into_iter().filter(|a| {
        a.title.to_lowercase().contains(&q) || a.desc.to_lowercase().contains(&q)
    }).collect();
    Json(filtered)
}

pub async fn list_videos() -> Json<Vec<VideoTutorial>> {
    Json(get_videos())
}
