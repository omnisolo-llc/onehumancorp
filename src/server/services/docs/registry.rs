
use std::collections::HashMap;

pub struct HelpRegistry {
    pub articles: HashMap<String, crate::services::docs::content::HelpArticle>,
    pub tooltips: HashMap<String, crate::services::docs::content::Tooltip>,
    pub guides: HashMap<String, crate::services::docs::content::Guide>,
}

impl HelpRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            articles: HashMap::new(),
            tooltips: HashMap::new(),
            guides: HashMap::new(),
        };
        crate::services::docs::content::populate_registry(&mut registry);
        registry
    }
}
