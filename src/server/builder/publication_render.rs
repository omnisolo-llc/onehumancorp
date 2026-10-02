//! Deterministic rendering of an explicitly reviewed publication snapshot.
use super::publication_store::{PublicationError, PublishedBlock, SiteSnapshot, prepare_snapshot};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderedSite {
    pub pages: BTreeMap<String, String>,
    pub sha256: String,
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn text<'a>(value: &'a Value, key: &str, required: bool) -> Result<&'a str, PublicationError> {
    match value.get(key) {
        Some(Value::String(text)) if !required || !text.trim().is_empty() => Ok(text),
        None | Some(Value::Null) if !required => Ok(""),
        _ => Err(PublicationError::Invalid(
            "Reviewed block text is missing or malformed",
        )),
    }
}

fn web_url(value: &str) -> Result<String, PublicationError> {
    if value.trim() != value || value.contains('\\') || value.chars().any(char::is_control) {
        return Err(PublicationError::Invalid("Invalid configured destination"));
    }
    let url = url::Url::parse(value)
        .map_err(|_| PublicationError::Invalid("Invalid configured destination"))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(PublicationError::Invalid(
            "Configured destinations must use HTTP or HTTPS without credentials",
        ));
    }
    Ok(url.into())
}

fn image(html: &mut String, value: &Value, alternative: &str) -> Result<(), PublicationError> {
    let source = text(value, "image", false)?;
    if !source.is_empty() {
        html.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\" loading=\"lazy\">",
            escape_html(&web_url(source)?),
            escape_html(alternative)
        ));
    }
    Ok(())
}

fn block_html(block: &PublishedBlock, site_id: Uuid) -> Result<String, PublicationError> {
    let mut html = String::from("<section class=\"block\">");
    let value = &block.content;
    match block.block_type.as_str() {
        "HeroBlock" | "Hero" => {
            let headline = text(value, "headline", true)?;
            image(&mut html, value, headline)?;
            let subtitle = if value.get("subtitle").is_some() {
                text(value, "subtitle", false)?
            } else {
                text(value, "copy", false)?
            };
            html.push_str(&format!(
                "<h1>{}</h1><p>{}</p>",
                escape_html(headline),
                escape_html(subtitle)
            ));
        }
        "ProductGridBlock" | "Catalog" => {
            let items =
                value
                    .get("items")
                    .and_then(Value::as_array)
                    .ok_or(PublicationError::Invalid(
                        "Reviewed catalog items are missing",
                    ))?;
            html.push_str("<div class=\"catalog\">");
            for item in items {
                let name = text(item, "name", true)?;
                let description = text(item, "description", false)?;
                let price = text(item, "price", false)?;
                html.push_str("<article class=\"product\">");
                image(&mut html, item, name)?;
                if let Some(id) = item.get("product_id") {
                    let id = id.as_str().and_then(|id| Uuid::parse_str(id).ok()).ok_or(
                        PublicationError::Invalid("Invalid selected product identity"),
                    )?;
                    html.push_str(&format!(
                        "<h2><a href=\"/api/v1/public/sites/{site_id}/products/{id}\">{}</a></h2>",
                        escape_html(name)
                    ));
                } else {
                    html.push_str(&format!("<h2>{}</h2>", escape_html(name)));
                }
                html.push_str(&format!(
                    "<p>{}</p><p class=\"price\">{}</p></article>",
                    escape_html(description),
                    escape_html(price)
                ));
            }
            html.push_str("</div>");
        }
        "ServiceBookingBlock" | "Booking" | "BookingCalendarBlock" => {
            let title = text(value, "title", true)?;
            html.push_str(&format!(
                "<h2>{}</h2><p>{}</p>",
                escape_html(title),
                escape_html(text(value, "availability", false)?)
            ));
            let destination = text(value, "booking_url", false)?;
            if destination.is_empty() {
                html.push_str("<p>Booking is not configured</p>");
            } else {
                html.push_str(&format!(
                    "<a class=\"action\" href=\"{}\" rel=\"noopener noreferrer\">Open booking</a>",
                    escape_html(&web_url(destination)?)
                ));
            }
        }
        "TestimonialBlock" | "Testimonials" => {
            let quotes =
                value
                    .get("quotes")
                    .and_then(Value::as_array)
                    .ok_or(PublicationError::Invalid(
                        "Reviewed testimonials are missing",
                    ))?;
            for quote in quotes {
                html.push_str(&format!(
                    "<figure><blockquote>{}</blockquote><figcaption>{}</figcaption></figure>",
                    escape_html(text(quote, "text", true)?),
                    escape_html(text(quote, "author", false)?)
                ));
            }
        }
        "Contact" | "ContactFormBlock" => {
            let email = text(value, "email", false)?;
            let phone = text(value, "phone", false)?;
            if !email.is_empty() {
                if email
                    .chars()
                    .any(|c| !c.is_ascii_alphanumeric() && !"@._+-".contains(c))
                    || email.matches('@').count() != 1
                    || email.starts_with('@')
                    || email.ends_with('@')
                {
                    return Err(PublicationError::Invalid(
                        "Invalid configured contact email",
                    ));
                }
                html.push_str(&format!(
                    "<p><a href=\"mailto:{}\">{}</a></p>",
                    escape_html(email),
                    escape_html(email)
                ));
            }
            if !phone.is_empty() {
                if phone
                    .chars()
                    .any(|c| !c.is_ascii_digit() && !"+ ()-.".contains(c))
                {
                    return Err(PublicationError::Invalid(
                        "Invalid configured contact phone",
                    ));
                }
                let destination: String = phone
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '+')
                    .collect();
                if !destination.chars().any(|c| c.is_ascii_digit())
                    || destination[1..].contains('+')
                {
                    return Err(PublicationError::Invalid(
                        "Invalid configured contact phone",
                    ));
                }
                html.push_str(&format!(
                    "<p><a href=\"tel:{}\">{}</a></p>",
                    escape_html(&destination),
                    escape_html(phone)
                ));
            }
            if email.is_empty() && phone.is_empty() {
                html.push_str("<p>Contact details are not configured</p>");
            }
        }
        "Text" | "TextBlock" => html.push_str(&format!(
            "<p>{}</p>",
            escape_html(text(value, "text", true)?)
        )),
        _ => {
            return Err(PublicationError::Invalid(
                "This reviewed block type is not supported for publication",
            ));
        }
    }
    html.push_str("</section>");
    Ok(html)
}

pub fn render_snapshot(
    snapshot: &SiteSnapshot,
    site_id: Uuid,
) -> Result<RenderedSite, PublicationError> {
    prepare_snapshot(snapshot)?;
    let mut pages = BTreeMap::new();
    for page in &snapshot.pages {
        let mut html = format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title>",
            escape_html(&page.title)
        );
        for (key, name) in [("name", "title"), ("description", "description")] {
            let content = text(&page.seo_metadata, key, false)?;
            if !content.is_empty() {
                html.push_str(&format!(
                    "<meta name=\"{name}\" content=\"{}\">",
                    escape_html(content)
                ));
            }
        }
        let mut metadata = page.seo_metadata.clone();
        if metadata.get("@context").is_none() {
            metadata["@context"] = Value::String("https://schema.org".into());
        }
        let metadata = serde_json::to_string(&metadata)
            .map_err(|_| PublicationError::Invalid("Invalid reviewed metadata"))?
            .replace('<', "\\u003c")
            .replace('>', "\\u003e")
            .replace('&', "\\u0026")
            .replace('\u{2028}', "\\u2028")
            .replace('\u{2029}', "\\u2029");
        html.push_str(&format!(
            "<script type=\"application/ld+json\">{metadata}</script>"
        ));
        html.push_str("<style>body{margin:0;font:16px/1.6 system-ui,sans-serif;color:#202124;background:#f6f7fb}main{max-width:960px;margin:auto;background:white;min-height:100vh}.block{padding:32px;border-bottom:1px solid #e4e7ed}h1,h2,p{overflow-wrap:anywhere}img{max-width:100%;height:auto;border-radius:12px}.catalog{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,240px),1fr));gap:24px}.product{padding:20px;background:#f6f7fb;border-radius:12px}a{color:#0758a6}.action{display:inline-block;padding:10px 18px;border:1px solid currentColor;border-radius:8px}.price{font-weight:700}footer{padding:24px;text-align:center;font-size:13px}</style></head><body><main>");
        let mut blocks: Vec<_> = page.blocks.iter().collect();
        blocks.sort_by_key(|block| block.sort_order);
        for block in blocks {
            html.push_str(&block_html(block, site_id)?);
        }
        html.push_str("<footer>Powered by OneHumanCorp</footer></main></body></html>");
        pages.insert(page.path.clone(), html);
    }
    let bytes = serde_json::to_vec(&pages)
        .map_err(|_| PublicationError::Invalid("Could not encode rendered pages"))?;
    Ok(RenderedSite {
        pages,
        sha256: format!("{:x}", Sha256::digest(bytes)),
    })
}
