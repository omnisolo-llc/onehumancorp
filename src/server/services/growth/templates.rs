use serde_json::json;

pub fn get_growth_templates() -> serde_json::Value {
    json!([
        {
            "id": "new_arrival",
            "name": "New Product Arrival",
            "category": "social",
            "prompt": "Create a buzz-worthy social media post for a new product arrival. Mention its unique value proposition and use emojis."
        },
        {
            "id": "flash_sale",
            "name": "Flash Sale",
            "category": "email",
            "prompt": "Draft a high-urgency email for a 24-hour flash sale. Include a clear call to action and a sense of scarcity."
        },
        {
            "id": "customer_thank_you",
            "name": "Customer Gratitude",
            "category": "email",
            "prompt": "Write a heartfelt thank you email for a first-time customer. Offer a small discount on their next purchase to encourage retention."
        },
        {
            "id": "milestone_celebration",
            "name": "Business Anniversary",
            "category": "social",
            "prompt": "Draft a celebratory social media post for a business milestone (like 1 year in business or 1000 orders). Thank the community for their support."
        },
        {
            "id": "educational_nurture",
            "name": "Value-Add Tips",
            "category": "email",
            "prompt": "Create an educational email that provides 3 useful tips related to our industry. Position us as an authority and helpful partner."
        }
    ])
}
