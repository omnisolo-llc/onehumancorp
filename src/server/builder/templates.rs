use serde_json::Value;

pub fn render_page(title: &str, blocks_html: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap" rel="stylesheet">
    <style>
        body {{ font-family: 'Inter', sans-serif; }}
        .font-outfit {{ font-family: 'Outfit', sans-serif; }}
        .glass {{
            background: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(10px);
            -webkit-backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.3);
        }}
    </style>
</head>
<body class="bg-gray-50 text-gray-900 overflow-x-hidden">
    <div class="max-w-[375px] mx-auto min-h-screen bg-white shadow-xl relative">
        {}
    </div>
</body>
</html>"#,
        title, blocks_html
    )
}

pub fn render_block(block_type: &str, content: &Value) -> String {
    match block_type {
        "HeroBlock" => render_hero(content),
        "ProductGridBlock" => render_catalog(content),
        "ServiceBookingBlock" => render_booking(content),
        "TestimonialBlock" => render_testimonials(content),
        "ContactFormBlock" => render_contact(content),
        "ReferralBlock" => render_referral(content),
        _ => "".to_string(),
    }
}

fn render_hero(content: &Value) -> String {
    let headline = content["headline"].as_str().unwrap_or("Welcome");
    let subtitle = content["subtitle"].as_str().unwrap_or("");
    let image = content["image"].as_str().unwrap_or("https://images.unsplash.com/photo-1497366216548-37526070297c");

    format!(
        r#"<div class="relative h-[400px] flex items-center justify-center text-center text-white overflow-hidden">
            <img src="{}" class="absolute inset-0 w-full h-full object-cover" alt="Hero">
            <div class="absolute inset-0 bg-black/40"></div>
            <div class="relative z-10 p-6 glass m-4 rounded-2xl">
                <h1 class="text-3xl font-black font-outfit mb-2">{}</h1>
                <p class="text-sm opacity-90">{}</p>
            </div>
        </div>"#,
        image, headline, subtitle
    )
}

fn render_catalog(content: &Value) -> String {
    let items = content["items"].as_array();
    let mut html = r#"<div class="p-6 bg-white"><h2 class="text-xl font-bold font-outfit mb-6">Our Collection</h2><div class="space-y-4">"#.to_string();

    if let Some(items) = items {
        for item in items {
            let name = item["name"].as_str().unwrap_or("Product");
            let price = item["price"].as_str().unwrap_or("");
            let desc = item["description"].as_str().unwrap_or("");
            html.push_str(&format!(
                r#"<div class="flex flex-col p-4 bg-gray-50 rounded-xl border border-gray-100 shadow-sm">
                    <div class="flex justify-between items-start mb-1">
                        <h3 class="font-bold text-gray-900">{}</h3>
                        <span class="text-blue-600 font-black">{}</span>
                    </div>
                    <p class="text-xs text-gray-500 leading-relaxed">{}</p>
                </div>"#,
                name, price, desc
            ));
        }
    }

    html.push_str("</div></div>");
    html
}

fn render_booking(content: &Value) -> String {
    let title = content["title"].as_str().unwrap_or("Book a Service");
    let availability = content["availability"].as_str().unwrap_or("Available Now");

    format!(
        r#"<div class="p-6 bg-gray-50">
            <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 text-center">
                <h2 class="text-lg font-bold font-outfit mb-2">{}</h2>
                <p class="text-sm text-gray-500 mb-6">{}</p>
                <button class="w-full bg-blue-600 text-white font-bold py-4 rounded-xl shadow-lg active:scale-[0.98] transition-transform">
                    Schedule Now
                </button>
            </div>
        </div>"#,
        title, availability
    )
}

fn render_testimonials(content: &Value) -> String {
    let quotes = content["quotes"].as_array();
    let mut html = r#"<div class="p-6 bg-white"><h2 class="text-xl font-bold font-outfit mb-6">What People Say</h2><div class="space-y-4">"#.to_string();

    if let Some(quotes) = quotes {
        for quote in quotes {
            let text = quote["text"].as_str().unwrap_or("");
            let author = quote["author"].as_str().unwrap_or("Anonymous");
            html.push_str(&format!(
                r#"<div class="p-4 bg-blue-50/50 rounded-xl border border-blue-100 italic">
                    <p class="text-sm text-blue-900 mb-2">"{}"</p>
                    <p class="text-xs font-bold text-blue-600 not-italic">— {}</p>
                </div>"#,
                text, author
            ));
        }
    }

    html.push_str("</div></div>");
    html
}

fn render_contact(content: &Value) -> String {
    let title = content["title"].as_str().unwrap_or("Get in Touch");
    format!(
        r#"<div class="p-6 bg-gray-900 text-white">
            <h2 class="text-xl font-bold font-outfit mb-6 text-center">{}</h2>
            <div class="space-y-4">
                <input type="text" placeholder="Your Name" class="w-full bg-white/10 border border-white/20 p-4 rounded-xl text-sm outline-none focus:ring-2 focus:ring-blue-500">
                <input type="email" placeholder="Your Email" class="w-full bg-white/10 border border-white/20 p-4 rounded-xl text-sm outline-none focus:ring-2 focus:ring-blue-500">
                <textarea placeholder="Your Message" rows="4" class="w-full bg-white/10 border border-white/20 p-4 rounded-xl text-sm outline-none focus:ring-2 focus:ring-blue-500"></textarea>
                <button class="w-full bg-white text-gray-900 font-bold py-4 rounded-xl">Send Message</button>
            </div>
        </div>"#,
        title
    )
}

fn render_referral(_content: &Value) -> String {
    format!(
        r#"<div class="p-8 bg-gradient-to-br from-indigo-600 to-purple-700 text-white text-center">
            <h2 class="text-2xl font-black font-outfit mb-2">Refer & Earn</h2>
            <p class="text-sm opacity-80 mb-6">Share this store with friends and get 20% off your next purchase!</p>
            <div class="flex gap-2">
                <button class="flex-1 bg-white text-indigo-600 font-bold py-3 rounded-xl text-sm">WhatsApp</button>
                <button class="flex-1 bg-black/20 text-white font-bold py-3 rounded-xl text-sm border border-white/20">Copy Link</button>
            </div>
        </div>
        <div class="py-6 bg-gray-50 flex items-center justify-center gap-2">
            <span class="text-xs text-gray-400 font-medium">Powered by</span>
            <span class="text-xs font-black font-outfit text-gray-900 tracking-tighter">OHC</span>
        </div>"#
    )
}
