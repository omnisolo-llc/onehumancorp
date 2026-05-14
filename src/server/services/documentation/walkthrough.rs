use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughStep {
    pub step_index: i32,
    pub target_selector: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughDefinition {
    pub walkthrough_id: String,
    pub title: String,
    pub steps: Vec<WalkthroughStep>,
}

pub struct WalkthroughManager;

impl WalkthroughManager {
    pub fn get_setup_store_walkthrough() -> WalkthroughDefinition {
        WalkthroughDefinition {
            walkthrough_id: "setup_store".to_string(),
            title: "Set up your store in 3 minutes".to_string(),
            steps: vec![
                WalkthroughStep {
                    step_index: 0,
                    target_selector: "#nav-store-settings".to_string(),
                    title: "Welcome to your Store!".to_string(),
                    content: "Let's get your business online. First, click here to open your store settings.".to_string(),
                },
                WalkthroughStep {
                    step_index: 1,
                    target_selector: "#input-store-name".to_string(),
                    title: "Name your business".to_string(),
                    content: "Type the name your customers know you by.".to_string(),
                }
            ],
        }
    }
}
