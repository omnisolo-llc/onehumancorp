// Business Share Embed
pub struct BusinessShareEmbed {
    pub business_name: String,
    pub tagline: String,
    pub logo_url: String,
    pub shareable_link: String,
}

impl BusinessShareEmbed {
    pub fn generate_og_card(&self) -> String {
        format!("OpenGraph card for {}", self.business_name)
    }
}
