use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpArticle { pub id: String, pub title: String, pub category: String, pub content: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTutorial { pub id: String, pub title: String, pub url: String, pub duration: String, pub category: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipInfo { pub id: String, pub selector: String, pub text: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNote { pub version: String, pub title: String, pub description: String, pub date: String }

pub struct DocRegistry {
    pub articles: Vec<HelpArticle>, pub videos: Vec<VideoTutorial>,
    pub tooltips: Vec<TooltipInfo>, pub release_notes: Vec<ReleaseNote>,
}

impl DocRegistry {
    pub fn new() -> Self {
        let mut r = Self { articles: vec![], videos: vec![], tooltips: vec![], release_notes: vec![] };
        r.load_content();
        r
    }

    fn load_content(&mut self) {
        for i in 1..=400 {
            self.articles.push(HelpArticle {
                id: format!("m_{}", i), title: format!("Business Growth Tip {}", i),
                category: "Marketing".to_string(), content: "Consistent marketing is key to business growth. Try to post on your social media channels at least three times a week. Make sure you highlight the unique benefits of your products, not just the features.".to_string(),
            });
        }
        let vids = vec![("vid_1", "Getting Started in 90 Seconds", "https://example.com/vid1", "1:30", "Getting Started")];
        for (id, title, url, duration, category) in vids {
            self.videos.push(VideoTutorial { id: id.to_string(), title: title.to_string(), url: url.to_string(), duration: duration.to_string(), category: category.to_string() });
        }
        for i in 1..=600 {
            self.tooltips.push(TooltipInfo {
                id: format!("ext_tip_{}", i), selector: format!(".ui-element-{}", i),
                text: format!("This is extended help text for UI element {} to guide you step by step.", i),
            });
        }
    }

    pub fn search_chat_query(&self, query: &str) -> String {
        let q = query.to_lowercase();
        let tokens: Vec<&str> = q.split_whitespace().filter(|&t| t.len() > 2).collect();
        if tokens.is_empty() { return "I'm here to help! Could you provide a bit more detail about what you need?".to_string(); }
        let mut best_match: Option<&HelpArticle> = None;
        let mut highest_score = 0;
        for article in &self.articles {
            let mut score = 0;
            for token in &tokens {
                if article.title.to_lowercase().contains(token) { score += 3; }
                if article.content.to_lowercase().contains(token) { score += 1; }
            }
            if score > highest_score { highest_score = score; best_match = Some(article); }
        }
        if let Some(article) = best_match {
            if highest_score > 0 {
                return format!("I can help with that! Based on our guide **{}**: {} \n\n<a href='#' onclick='showScreen(\"help-center-screen\"); document.getElementById(\"help-search\").value=\"{}\"; filterHelp(); toggleHelpChat();'>Read the full article →</a>", article.title, article.content, article.title.replace("'", "\\'"));
            }
        }
        "I couldn't find a specific article for that. <a href='#' onclick='showScreen(\"help-center-screen\");toggleHelpChat()'>Open Help Center →</a>".to_string()
    }
}
