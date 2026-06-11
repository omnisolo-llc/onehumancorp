use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct HelpArticle {
    pub category: String,
    pub title: String,
    pub desc: String,
    pub link: String,
}

#[derive(Serialize, Clone)]
pub struct VideoTutorial {
    pub id: i32,
    pub title: String,
    pub duration: String,
    pub video_url: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize, Clone)]
pub struct WalkthroughStep {
    pub selector: String,
    pub title: String,
    pub text: String,
}

pub async fn get_walkthrough(axum::extract::Path(page): axum::extract::Path<String>) -> Json<Vec<WalkthroughStep>> {
    let steps = match page.as_str() {
        "dashboard" => vec![
            WalkthroughStep { selector: "#dashboard-title".to_string(), title: "Welcome".to_string(), text: "Welcome to your dashboard! This is your control center.".to_string() },
            WalkthroughStep { selector: "#ai-savings-widget".to_string(), title: "AI Savings".to_string(), text: "Here you can see the time and effort your agents have saved you.".to_string() }
        ],
        "pos" => vec![
            WalkthroughStep { selector: "#charge-btn".to_string(), title: "Accept Payment".to_string(), text: "Enter an amount and tap here to charge.".to_string() }
        ],
        "assistant" => vec![
            WalkthroughStep { selector: "#ohc-help-input-area".to_string(), title: "Activate your AI Support Agent".to_string(), text: "Chat here to activate your AI agent.".to_string() }
        ],
        _ => vec![],
    };
    Json(steps)
}

pub async fn get_tooltips() -> Json<std::collections::HashMap<String, String>> {
    let registry = crate::get_tooltips_registry().read().unwrap();
    let mut tooltips = std::collections::HashMap::new();
    for (k, v) in registry.iter() {
        tooltips.insert(k.clone(), v.clone());
    }
    Json(tooltips)
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

pub async fn update_tooltip(axum::extract::Json(payload): axum::extract::Json<TooltipPayload>) -> Json<SuccessResponse> {
    let mut registry = crate::get_tooltips_registry().write().unwrap();
    registry.insert(payload.id, payload.text);
    Json(SuccessResponse { success: true })
}



pub fn get_articles() -> Vec<HelpArticle> {
    vec![
        HelpArticle { category: "Getting Started".to_string(), title: "Getting Started".to_string(), desc: "Learn how to easily set up your store and accept your first payment.".to_string(), link: "/help/getting-started-1".to_string() },
        HelpArticle { category: "My Store".to_string(), title: "My Store".to_string(), desc: "Add products, track what's in stock, and change how your store looks.".to_string(), link: "/help/my-store".to_string() },
        HelpArticle { category: "Payments".to_string(), title: "Getting Paid".to_string(), desc: "Set up how you get paid, view deposits, and handle simple taxes.".to_string(), link: "/help/payments".to_string() },
        HelpArticle { category: "AI Agents".to_string(), title: "Your AI Helpers".to_string(), desc: "Learn how to hire AI helpers and give them tasks to do.".to_string(), link: "/help/ai-agents".to_string() },
        HelpArticle { category: "Marketing".to_string(), title: "Finding Customers".to_string(), desc: "Send emails to customers and grow your business easily.".to_string(), link: "/help/marketing".to_string() },
        HelpArticle { category: "Account & Billing".to_string(), title: "Account & Billing".to_string(), desc: "View your bills, manage your plan, and invite team members.".to_string(), link: "/help/account-billing".to_string() },
        HelpArticle { category: "Advanced".to_string(), title: "API Reference".to_string(), desc: "Use our OpenAPI specs to integrate with OHC.".to_string(), link: "/api-docs".to_string() },
        HelpArticle { category: "Advanced".to_string(), title: "Webhooks".to_string(), desc: "Listen to real-time events.".to_string(), link: "/help/webhooks".to_string() }
    ]
}

pub fn get_videos() -> Vec<VideoTutorial> {
    vec![
        VideoTutorial { id: 1, title: "How to set up your first store easily".to_string(), duration: "1:20".to_string(), video_url: "/videos/1.mp4".to_string() },
        VideoTutorial { id: 2, title: "Accept your first payment".to_string(), duration: "1:15".to_string(), video_url: "/videos/2.mp4".to_string() },
        VideoTutorial { id: 3, title: "Activate your AI Support Agent".to_string(), duration: "0:50".to_string(), video_url: "/videos/3.mp4".to_string() },
        VideoTutorial { id: 4, title: "Adding staff to your account".to_string(), duration: "1:05".to_string(), video_url: "/videos/4.mp4".to_string() },
        VideoTutorial { id: 5, title: "Review an order".to_string(), duration: "1:10".to_string(), video_url: "/videos/5.mp4".to_string() },
        VideoTutorial { id: 6, title: "Send a campaign".to_string(), duration: "1:25".to_string(), video_url: "/videos/6.mp4".to_string() },
        VideoTutorial { id: 7, title: "Connect Stripe".to_string(), duration: "1:30".to_string(), video_url: "/videos/7.mp4".to_string() },
        VideoTutorial { id: 8, title: "Manage inventory".to_string(), duration: "1:00".to_string(), video_url: "/videos/8.mp4".to_string() },
        VideoTutorial { id: 9, title: "How to use the OpenAPI spec".to_string(), duration: "3:45".to_string(), video_url: "/videos/9.mp4".to_string() },
    ]
}

pub async fn list_articles() -> Json<Vec<HelpArticle>> {
    Json(get_articles())
}

pub async fn search_articles(Query(query): Query<SearchQuery>) -> Json<Vec<HelpArticle>> {
    let q = query.q.to_lowercase();
    let articles = get_articles();
    let filtered = articles.into_iter().filter(|a| {
        a.category.to_lowercase().contains(&q) || a.title.to_lowercase().contains(&q) || a.desc.to_lowercase().contains(&q)
    }).collect();
    Json(filtered)
}

pub async fn list_videos() -> Json<Vec<VideoTutorial>> {
    Json(get_videos())
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
        "my-store" => Some(HelpArticleDetail {
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
        "marketing" => Some(HelpArticleDetail {
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
        "account-billing" => Some(HelpArticleDetail {
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
        "payments" => Some(HelpArticleDetail {
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
        "ai-agents" => Some(HelpArticleDetail {
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
    vec![
        ChangelogSection {
            version: "Version 1.1 (Latest)".to_string(),
            screenshot_url: None,
            content_lines: vec![
                "### 🌟 New Features".to_string(),
                "- **Help Center:** Fully searchable help center with video tutorials and articles.".to_string(),
                "- **Contextual Tooltips:** Added plain language tooltips across the app to guide you.".to_string(),
            ]
        },
        ChangelogSection {
            version: "Version 1.0".to_string(),
            screenshot_url: Some("/dashboard_with_charts.png".to_string()),
            content_lines: vec![
                "### 🌟 New Features".to_string(),
                "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.".to_string(),
                "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.".to_string(),
                "- **Help Center Upgrade:** Find answers instantly with our new searchable Help Center.".to_string(),
                "### 🛠️ Improvements".to_string(),
                "- Faster loading times for product images.".to_string(),
                "- Simplified checkout process for your customers.".to_string(),
            ]
        }
    ]
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
            "description": "API Reference for advanced users integrating with OneHumanCorp.",
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
