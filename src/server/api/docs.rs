use axum::{extract::Query, Json};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HelpArticle {
    pub category: String,
    pub title: String,
    pub desc: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoTutorial {
    pub id: i32,
    pub title: String,
    pub duration: String,
    pub video_url: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct DocsQuery {
    pub mobile_optimized: Option<bool>,
}

#[derive(Serialize, Clone)]
pub struct WalkthroughStep {
    #[serde(rename = "targetId")]
    pub target_id: String,
    pub title: String,
    pub content: String,
}


pub async fn get_walkthrough(axum::extract::Path(page): axum::extract::Path<String>) -> Json<Vec<WalkthroughStep>> {
    let steps = match page.as_str() {
        "store-setup" => vec![
            WalkthroughStep { target_id: "dashboard-title".to_string(), title: "Set up your store".to_string(), content: "Learn how to easily set up your store and accept your first payment.".to_string() },
            WalkthroughStep { target_id: "bio-input-tooltip".to_string(), title: "Describe your business".to_string(), content: "Tell us what you sell so we can create the perfect storefront for you.".to_string() },
            WalkthroughStep { target_id: "generate-btn-tooltip".to_string(), title: "Generate Store".to_string(), content: "Click here and watch our AI build your store from scratch.".to_string() },
        ],
        "dashboard" => vec![
            WalkthroughStep { target_id: "dashboard-title".to_string(), title: "Welcome".to_string(), content: "Welcome to your dashboard! This is your control center.".to_string() },
            WalkthroughStep { target_id: "wrapped-summary".to_string(), title: "AI Savings".to_string(), content: "Here you can see the time and effort your agents have saved you.".to_string() }
        ],
        "pos" => vec![
            WalkthroughStep { target_id: "pos-keypad".to_string(), title: "Enter Amount".to_string(), content: "Type in the total sale amount using the keypad.".to_string() },
            WalkthroughStep { target_id: "charge-btn".to_string(), title: "Charge Customer".to_string(), content: "Tap here to process the payment. It's that easy!".to_string() }
        ],
        "assistant" => vec![
            WalkthroughStep { target_id: "ai-chat-trigger".to_string(), title: "Open Assistant".to_string(), content: "Click here to open your AI Support Agent.".to_string() },
            WalkthroughStep { target_id: "ohc-help-input-area".to_string(), title: "Ask Anything".to_string(), content: "Type your request here and the agent will handle it while you sleep.".to_string() }
        ],
        _ => vec![],
    };
    Json(steps)
}

pub async fn get_tooltips(
    axum::extract::Extension(db): axum::extract::Extension<std::sync::Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap
) -> Result<Json<std::collections::HashMap<String, String>>, axum::http::StatusCode> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");
    let mut tooltips = std::collections::HashMap::new();
    match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query("SELECT id, text FROM tooltips WHERE tenant_id = $1").bind(tenant_id)
                .fetch_all(&db.pool)
                .await
            {
                Ok(rows) => {
                    for row in rows {
                        use sqlx::Row;
                        let id: String = row.get("id");
                        let text: String = row.get("text");
                        tooltips.insert(id, text);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to fetch tooltips: {}", e);
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            match sqlx::query("SELECT id, text FROM tooltips WHERE tenant_id = ?").bind(tenant_id)
                .fetch_all(pool)
                .await
            {
                Ok(rows) => {
                    for row in rows {
                        use sqlx::Row;
                        let id: String = row.get("id");
                        let text: String = row.get("text");
                        tooltips.insert(id, text);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to fetch tooltips: {}", e);
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }

    if tooltips.is_empty() {
        tooltips.insert("dashboard-walkthrough-btn".to_string(), "Take a tour of the dashboard".to_string());
        tooltips.insert("api-docs-tooltip".to_string(), "Direct API access is only for custom integrations.".to_string());
        tooltips.insert("kairos-nav-link-tooltip".to_string(), "Click here to see what your AI helpers are working on and how they plan.".to_string());
        tooltips.insert("dashboard-tooltip".to_string(), "View your daily sales and overall business health.".to_string());
        tooltips.insert("generate-link-btn".to_string(), "Click here to share access with a team member.".to_string());
        tooltips.insert("ask-ai-tooltip".to_string(), "Open AI Help Chat to get answers instantly.".to_string());
        tooltips.insert("settings-delivery-tooltip".to_string(), "Turn this on to offer local delivery to your customers.".to_string());
        tooltips.insert("help-btn-tooltip".to_string(), "Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.".to_string());
        tooltips.insert("help-search-tooltip".to_string(), "Search for help articles and videos...".to_string());
    }

    Ok(Json(tooltips))
}

#[derive(Deserialize)]
pub struct TooltipPayload {
    pub id: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

pub async fn update_tooltip(
    axum::extract::Extension(db): axum::extract::Extension<std::sync::Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap,
    axum::extract::Json(payload): axum::extract::Json<TooltipPayload>
) -> Result<Json<SuccessResponse>, axum::http::StatusCode> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");
    match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query("INSERT INTO tooltips (id, tenant_id, text) VALUES ($1, $2, $3) ON CONFLICT (tenant_id, id) DO UPDATE SET text = EXCLUDED.text").bind(payload.id).bind(tenant_id).bind(payload.text)
                .execute(&db.pool)
                .await
            {
                Ok(_) => Ok(Json(SuccessResponse { success: true })),
                Err(e) => {
                    tracing::error!("Failed to update tooltip: {}", e);
                    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            match sqlx::query("INSERT INTO tooltips (id, tenant_id, text) VALUES (?, ?, ?) ON CONFLICT (tenant_id, id) DO UPDATE SET text = excluded.text").bind(payload.id).bind(tenant_id).bind(payload.text)
                .execute(pool)
                .await
            {
                Ok(_) => Ok(Json(SuccessResponse { success: true })),
                Err(e) => {
                    tracing::error!("Failed to update tooltip: {}", e);
                    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}



pub fn get_articles() -> Vec<HelpArticle> {
    vec![
        HelpArticle { category: "Getting Started".to_string(), title: "Getting Started with Your Store".to_string(), desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.".to_string(), link: "/help/getting-started-1".to_string() },
        HelpArticle { category: "My Store".to_string(), title: "Adding Products".to_string(), desc: "Add products, track what's in stock, and change how your store looks.".to_string(), link: "/help/add-products".to_string() },
        HelpArticle { category: "Payments".to_string(), title: "Accepting Payments".to_string(), desc: "Learn how to accept credit cards and manage your payouts.".to_string(), link: "/help/accept-payments".to_string() },
        HelpArticle { category: "AI Agents".to_string(), title: "Activate AI Support".to_string(), desc: "Let our AI handle customer inquiries and triage your inbox.".to_string(), link: "/help/ai-support".to_string() },
        HelpArticle { category: "Marketing".to_string(), title: "Grow Your Audience".to_string(), desc: "Use our built-in tools to run promotions and track performance.".to_string(), link: "/help/marketing-tools".to_string() },
        HelpArticle { category: "Account & Billing".to_string(), title: "Manage Billing".to_string(), desc: "Update your subscription and payment methods.".to_string(), link: "/help/billing-settings".to_string() },
        HelpArticle { category: "Advanced".to_string(), title: "API Documentation".to_string(), desc: "Interactive API reference for connecting external services to your workspace.".to_string(), link: "/api-docs".to_string() },
    ]
}

pub fn get_videos() -> Vec<VideoTutorial> {
    vec![
        VideoTutorial { id: 1, title: "How to set up your first store easily".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 2, title: "Connecting a bank account to accept payments".to_string(), duration: "0:45".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 3, title: "Activating your AI Support Agent".to_string(), duration: "1:25".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 4, title: "Adding a new product to your inventory".to_string(), duration: "0:50".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 5, title: "Managing staff and user permissions".to_string(), duration: "1:10".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 6, title: "Creating a marketing campaign".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 7, title: "Using the Analytics Dashboard".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 8, title: "How to handle refunds and returns".to_string(), duration: "1:05".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 9, title: "Customizing your storefront design".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 10, title: "Setting up automated email receipts".to_string(), duration: "0:55".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
    ]
}







macro_rules! with_cache_fallback {
    ($cache:expr, $cache_key:expr, $fetch_fn:expr) => {{
        let cache_key_str = $cache_key.to_string();
        if let Some((cached, is_stale)) = $cache.get_with_swr(&cache_key_str).await {
            if !is_stale {
                // Ignore the early return since we map manually afterwards
                // The macro shouldn't return Json directly since we change the types
            } else {
                let cache_key_bg = cache_key_str.clone();
                tokio::spawn(async move {
                    let items = $fetch_fn();
                    let _ = $cache.set(&cache_key_bg, items, std::time::Duration::from_secs(3600)).await;
                });
            }
            (cached, true)
        } else {
            let items = $fetch_fn();
            let _ = $cache.set(&cache_key_str, items.clone(), std::time::Duration::from_secs(3600)).await;
            (items, false)
        }
    }};
}

static DOCS_ARTICLES_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<Vec<HelpArticle>>> = std::sync::OnceLock::new();
static DOCS_VIDEOS_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<Vec<VideoTutorial>>> = std::sync::OnceLock::new();

pub async fn list_articles(Query(query): Query<DocsQuery>) -> Json<Vec<serde_json::Value>> {
    let cache = DOCS_ARTICLES_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));
    let articles = with_cache_fallback!(cache, "docs:articles:all", || get_articles()).0;
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let json_articles = articles.into_iter().map(|a| {
        if mobile_optimized {
            serde_json::json!({
                "category": a.category,
                "title": a.title,
                "link": a.link
            })
        } else {
            serde_json::json!({
                "category": a.category,
                "title": a.title,
                "desc": a.desc,
                "link": a.link
            })
        }
    }).collect();
    Json(json_articles)
}

pub async fn search_articles(Query(query): Query<SearchQuery>) -> Json<Vec<serde_json::Value>> {
    let q = query.q.to_lowercase();
    let cache_key = format!("docs:articles:search:{}", q);
    let cache = DOCS_ARTICLES_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));

    let articles = with_cache_fallback!(cache, cache_key, || {
        get_articles().into_iter().filter(|a| {
            a.category.to_lowercase().contains(&q) || a.title.to_lowercase().contains(&q) || a.desc.to_lowercase().contains(&q)
        }).collect::<Vec<HelpArticle>>()
    }).0;

    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let json_articles = articles.into_iter().map(|a| {
        if mobile_optimized {
            serde_json::json!({
                "category": a.category,
                "title": a.title,
                "link": a.link
            })
        } else {
            serde_json::json!({
                "category": a.category,
                "title": a.title,
                "desc": a.desc,
                "link": a.link
            })
        }
    }).collect();
    Json(json_articles)
}

pub async fn list_videos(Query(query): Query<DocsQuery>) -> Json<Vec<serde_json::Value>> {
    let cache = DOCS_VIDEOS_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));
    let videos = with_cache_fallback!(cache, "docs:videos:all", || get_videos()).0;

    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let _ = mobile_optimized; // Keep parameter for future use
    let json_videos = videos.into_iter().map(|v| {
        serde_json::json!({
            "id": v.id,
            "title": v.title,
            "duration": v.duration,
            "video_url": v.video_url
        })
    }).collect();
    Json(json_videos)
}



#[derive(Serialize, Clone)]
pub struct HelpArticleDetail {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "contentHtml")]
    pub content_html: String,
}

pub fn get_article(id: &str) -> Option<HelpArticleDetail> {
    match id {
        "getting-started-1" => Some(HelpArticleDetail {
            title: "Getting Started with Your Store".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        Welcome to OneHumanCorp! Setting up your store is quick and easy. Our app helps you get everything ready to sell online.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 1: Tell us about your business</h2>
      <p class="text-gray-700 mb-4">
        Start by telling us what you sell and who your customers are. Keep it simple! Just describe what makes your shop special.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 2: Let AI build your store</h2>
      <p class="text-gray-700 mb-4">
        Once you tell us about your business, click the "Generate" button. Our AI will build your store for you. It will pick a design and write some text to get you started.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 3: Launch to the world</h2>
      <p class="text-gray-700 mb-4">
        When you are happy with how your store looks, click the "Launch" button. This makes your store live on the internet so customers can visit and buy from you!
      </p>
      <div class="mt-8 bg-blue-50 p-4 rounded-lg border border-blue-100">
        <p class="text-blue-800 font-medium">Need more help? Click the chat button to ask our AI assistant any questions you have.</p>
      </div>
            "#.to_string()
        }),
        "add-products" => Some(HelpArticleDetail {
            title: "Managing My Store".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        Your store is where you show off what you sell. You can easily add new items, keep track of what you have in stock, and change how your store looks.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Adding Products</h2>
      <p class="text-gray-700 mb-4">
        To add a new item, go to the products page and click "Add Product". You can upload a picture, type in a name and description, and set the price. Our AI can even help you write a catchy description!
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Tracking Your Stock</h2>
      <p class="text-gray-700 mb-4">
        When you add a product, you can tell the app how many you have to sell. When someone buys it, the number goes down on its own. This helps you know when you need to make or buy more.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Changing How Your Store Looks</h2>
      <p class="text-gray-700 mb-4">
        You can pick different colors, fonts, and layouts to make your store match your brand. Just go to the Storefront Builder to try out different styles.
      </p>
            "#.to_string()
        }),
        "marketing-tools" => Some(HelpArticleDetail {
            title: "Finding Customers".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        To grow your business, you need people to know about it. We have tools to help you find and talk to customers.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Sending Emails</h2>
      <p class="text-gray-700 mb-4">
        You can send emails to people who have bought from you before or signed up on your store. You can use this to tell them about new products or special sales. Our AI can even help you write the emails!
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Running Promos and Sales</h2>
      <p class="text-gray-700 mb-4">
        Everyone loves a good deal. You can easily set up a weekend sale or a holiday promotion. You can choose to give a percentage off or a set amount of money off.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Sharing Your Store</h2>
      <p class="text-gray-700 mb-4">
        Don't forget to share your store link on social media or with your friends and family. You can find your store's link on your Dashboard.
      </p>
            "#.to_string()
        }),
        "billing-settings" => Some(HelpArticleDetail {
            title: "Account & Billing".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        Manage your monthly plan, view your past bills, and invite people to help run your business.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Managing Your Plan</h2>
      <p class="text-gray-700 mb-4">
        You can check what plan you are on by going to the Billing page. If your business is growing and you need more features, you can upgrade your plan at any time.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Viewing Your Bills</h2>
      <p class="text-gray-700 mb-4">
        You can see a history of all the payments you have made to OneHumanCorp. This makes it easy to keep track of your expenses for your own records.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Inviting Team Members</h2>
      <p class="text-gray-700 mb-4">
        If you have business partners or staff who need to access your store settings, you can invite them to your team. Just enter their email address and they will get an invite to join.
      </p>
            "#.to_string()
        }),
        "accept-payments" => Some(HelpArticleDetail {
            title: "Getting Paid".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        Getting paid is the most exciting part! We make it secure and easy for your customers to pay you.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Connecting Your Bank Account</h2>
      <p class="text-gray-700 mb-4">
        To start taking money, you need to connect a bank account. We use Stripe, a safe and trusted system. Just click the "Connect Stripe" button in your setup to securely link your bank.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Viewing Your Deposits</h2>
      <p class="text-gray-700 mb-4">
        When a customer buys something, the money goes into your connected bank account. You can check the Dashboard to see your recent sales and see when the money will arrive in your bank.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Taxes and Fees</h2>
      <p class="text-gray-700 mb-4">
        We help handle simple taxes for you at checkout. A small fee is taken out of each sale to cover the cost of securely moving the money from the customer's card to your bank.
      </p>
            "#.to_string()
        }),
        "ai-support" => Some(HelpArticleDetail {
            title: "Your AI Helpers".to_string(),
            content_html: r#"
      <p class="text-gray-700 mb-4 leading-relaxed text-lg">
        Running a business takes a lot of work. That's why we give you AI helpers—smart computer programs that can do tasks for you, like a real team!
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Hiring AI Helpers</h2>
      <p class="text-gray-700 mb-4">
        Go to the AI Departments page to see all the helpers you can hire. Some helpers are good at marketing, some are good at writing, and others are good at keeping track of numbers.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Giving Them Tasks</h2>
      <p class="text-gray-700 mb-4">
        Once you hire a helper, you can tell them what to do. You just type what you need in plain English. For example, "Write an email to my customers about a summer sale." The helper will do the work and show it to you.
      </p>
      <h2 class="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Approving Their Work</h2>
      <p class="text-gray-700 mb-4">
        Helpers are smart, but you are the boss. Before they send an email or change your store, they will ask for your permission. You can check your Inbox to review and approve their tasks.
      </p>
            "#.to_string()
        }),
        _ => None,
    }
}

pub async fn get_article_handler(axum::extract::Path(article_id): axum::extract::Path<String>) -> Result<Json<HelpArticleDetail>, axum::http::StatusCode> {
    if let Some(article) = get_article(&article_id) {
        Ok(Json(article))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

#[derive(Serialize, Clone)]
pub struct ChangelogSection {
    pub version: String,
    #[serde(rename = "contentLines")]
    pub content_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_url: Option<String>,
}

pub fn get_changelog_data() -> Vec<ChangelogSection> {
    let mut sections = Vec::new();
    let content = std::include_str!("../../../CHANGELOG.md");

    let mut current_version = String::new();
    let mut current_lines = Vec::new();

    let mut current_screenshot: Option<String> = None;

    for line in content.lines() {
        if line.starts_with("## ") {
            if !current_version.is_empty() {
                sections.push(ChangelogSection {
                    version: current_version.clone(),
                    screenshot_url: current_screenshot.clone(),
                    content_lines: current_lines.clone(),
                });
            }
            current_version = line.trim_start_matches("## ").trim().to_string();
            current_lines = Vec::new();
            current_screenshot = None;
        } else if !current_version.is_empty() && !line.trim().is_empty() {
            // Check for markdown image format: ![alt text](url)
            if line.starts_with("![") {
                if let Some(start_idx) = line.find("](") {
                    if let Some(end_idx) = line.find(")") {
                        if current_screenshot.is_none() {
                            let url = &line[start_idx + 2..end_idx];
                            current_screenshot = Some(url.to_string());
                        }
                        continue; // Skip adding image line to content_lines
                    }
                }
            }
            current_lines.push(line.to_string());
        }
    }

    if !current_version.is_empty() {
        sections.push(ChangelogSection {
            version: current_version,
            screenshot_url: current_screenshot,
            content_lines: current_lines,
        });
    }

    sections
}

pub async fn get_changelog() -> Json<Vec<ChangelogSection>> {
    Json(get_changelog_data())
}

pub async fn get_api_docs_spec() -> Json<serde_json::Value> {
    let spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "OHC Advanced API Reference",
            "version": "1.0.0",
            "description": "OHC Advanced API Reference integrating with OneHumanCorp.",
        },
        "servers": [
            {
                "url": "http://localhost:8080",
                "description": "Backend Server"
            }
        ],
        "paths": {
            "/api/help": {
                "get": {
                    "summary": "Get Help Articles",
                    "description": "Retrieves a list of available help articles for the Help Center.",
                    "tags": ["Documentation"],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "title": { "type": "string" },
                                                "desc": { "type": "string" },
                                                "link": { "type": "string" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/tooltips": {
                "get": {
                    "summary": "Get Tooltips Registry",
                    "description": "Retrieves the key-value dictionary of all UI tooltips.",
                    "tags": ["Documentation"],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "additionalProperties": { "type": "string" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/help/search": {
                "get": {
                    "summary": "Search Help Articles",
                    "description": "Searches for help articles by query.",
                    "tags": ["Documentation"],
                    "parameters": [
                        {
                            "name": "q",
                            "in": "query",
                            "description": "Search query",
                            "required": true,
                            "schema": {
                                "type": "string"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "title": { "type": "string" },
                                                "desc": { "type": "string" },
                                                "link": { "type": "string" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/videos": {
                "get": {
                    "summary": "Get Video Tutorials",
                    "description": "Retrieves a list of available video tutorials.",
                    "tags": ["Documentation"],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "id": { "type": "integer" },
                                                "title": { "type": "string" },
                                                "duration": { "type": "string" },
                                                "video_url": { "type": "string" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/changelog": {
                "get": {
                    "summary": "Get Release Notes and Changelog",
                    "description": "Retrieves the release notes and changelog.",
                    "tags": ["Documentation"],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "version": { "type": "string" },
                                            "features": {
                                                "type": "array",
                                                "items": { "type": "string" }
                                            },
                                            "fixes": {
                                                "type": "array",
                                                "items": { "type": "string" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/catalog/product": {
                "post": {
                    "summary": "Create a Product",
                    "description": "Creates a new product or service in the catalog.",
                    "tags": ["Catalog"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": { "type": "string", "example": "Vegan Birthday Cake" },
                                        "price": { "type": "string", "example": "45.00" },
                                        "duration": { "type": "integer", "example": 60 },
                                        "description": { "type": "string", "example": "Delicious plant-based cake." },
                                        "item_type": { "type": "string", "example": "physical" },
                                        "is_subscription": { "type": "boolean", "example": false },
                                        "subscription_interval": { "type": "string", "example": "month" },
                                        "subscription_discount": { "type": "integer", "example": 10 }
                                    },
                                    "required": ["name", "price", "description", "item_type"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "message": { "type": "string" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/builder/generate": {
                "post": {
                    "summary": "Generate a Storefront",
                    "description": "Generates a new storefront draft using AI.",
                    "tags": ["Builder"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "description": { "type": "string", "example": "A boutique pet bakery" }
                                    },
                                    "required": ["description"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "domain": { "type": "string", "nullable": true },
                                            "theme": { "type": "string" },
                                            "pages": { "type": "array", "items": { "type": "object" } },
                                            "sample_products": { "type": "array", "items": { "type": "object" } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/builder/publish_draft": {
                "post": {
                    "summary": "Publish Storefront Draft",
                    "description": "Publishes a storefront draft.",
                    "tags": ["Builder"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "domain": { "type": "string", "nullable": true },
                                        "draft": { "type": "object" }
                                    },
                                    "required": ["draft"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "id": { "type": "string" },
                                            "domain": { "type": "string", "nullable": true }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/catalog/generate-offering": {
                "post": {
                    "summary": "Generate an Offering",
                    "description": "Generates a product offering using AI based on a prompt.",
                    "tags": ["Catalog"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "prompt": { "type": "string", "example": "A weekly coffee bean subscription" }
                                    },
                                    "required": ["prompt"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "title": { "type": "string" },
                                            "description": { "type": "string" },
                                            "price": { "type": "string" },
                                            "item_type": { "type": "string" },
                                            "is_subscription": { "type": "boolean" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/orgs/register": {
                "post": {
                    "summary": "Register an Organization",
                    "description": "Registers a new tenant organization in the multi-tenant OHC environment.",
                    "tags": ["Tenants"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": { "type": "string", "example": "acme" },
                                        "name": { "type": "string", "example": "Acme Corp" },
                                        "domain": { "type": "string", "example": "acme.com" }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "tenant_id": { "type": "string" },
                                            "message": { "type": "string" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/agents/task": {
                "post": {
                    "summary": "Dispatch a task",
                    "description": "Dispatches a new task to the AI Swarm Orchestrator.",
                    "tags": ["Agents"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "task_description": { "type": "string", "example": "Build a landing page for a dog groomer" },
                                        "priority": { "type": "string", "example": "high" }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "202": {
                            "description": "Accepted",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "task_id": { "type": "string" },
                                            "status": { "type": "string" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/videos": {
                "get": {
                    "summary": "Get video tutorials",
                    "description": "Retrieves a list of video tutorial metadata for the Help Center.",
                    "tags": ["Documentation"],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "id": { "type": "integer" },
                                                "title": { "type": "string" },
                                                "duration": { "type": "string" },
                                                "video_url": { "type": "string" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/agents/status": {
                "get": {
                    "summary": "Get workforce status",
                    "description": "Retrieves the current status of the agent swarm workforce.",
                    "tags": ["Agents"],
                    "parameters": [
                        {
                            "name": "tenant_id",
                            "in": "query",
                            "description": "Optional. Filter by organization.",
                            "required": false,
                            "schema": {
                                "type": "string"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "active_agents": { "type": "integer" },
                                            "queued_tasks": { "type": "integer" },
                                            "system_health": { "type": "string" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    Json(spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Json as AxumJson;


    #[tokio::test]
    async fn test_list_articles() {
        let res = list_articles(axum::extract::Query(DocsQuery { mobile_optimized: None })).await;
        assert!(!res.0.is_empty());
    }

    #[tokio::test]
    async fn test_search_articles_found() {
        let res = search_articles(axum::extract::Query(SearchQuery { q: "getting".to_string(), mobile_optimized: None })).await;
        assert!(!res.0.is_empty());
    }

    #[tokio::test]
    async fn test_search_articles_not_found() {
        let res = search_articles(axum::extract::Query(SearchQuery { q: "unlikelysearchterm123".to_string(), mobile_optimized: None })).await;
        assert!(res.0.is_empty());
    }

    #[tokio::test]
    async fn test_list_videos() {
        let res = list_videos(axum::extract::Query(DocsQuery { mobile_optimized: None })).await;
        assert!(!res.0.is_empty());
    }

    #[test]
    fn test_get_changelog_data_with_screenshots() {
        // We will indirectly test it since get_changelog_data pulls directly from CHANGELOG.md via include_str!
        // but for now let's just make sure it parses properly.
        let data = get_changelog_data();
        assert!(!data.is_empty());
    }

    #[tokio::test]
    #[ignore] // Ignoring since it requires a real database
    async fn test_tooltips_api() {
        let db_pool = crate::db::create_sqlite_pool_for_test().await;
        let pg_pool = crate::db::create_dummy_pg_pool().await;
        sqlx::query("CREATE TABLE IF NOT EXISTS tooltips (id TEXT, tenant_id TEXT, text TEXT, PRIMARY KEY (tenant_id, id))").execute(&db_pool).await.unwrap();
        let db = std::sync::Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(db_pool) });

        // Prepare the payload to update a tooltip
        let payload = TooltipPayload {
            id: "test-tooltip-id".to_string(),
            text: "This is a test tooltip".to_string(),
        };

        // Update the tooltip
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("x-tenant-id", axum::http::HeaderValue::from_static("test-tenant"));
        let res = update_tooltip(axum::extract::Extension(db.clone()), headers.clone(), AxumJson(payload)).await.unwrap();
        assert!(res.0.success);

        // Fetch tooltips and verify the update
        let tooltips_res = get_tooltips(axum::extract::Extension(db.clone()), headers).await.unwrap();
        let tooltips = tooltips_res.0;

        assert_eq!(
            tooltips.get("test-tooltip-id").map(|s| s.as_str()),
            Some("This is a test tooltip")
        );
    }
}
