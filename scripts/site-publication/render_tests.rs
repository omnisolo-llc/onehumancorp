use crate::builder::publication_render::render_snapshot;
use crate::builder::publication_store::{PublishedBlock, PublishedPage, SiteSnapshot};
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

fn snapshot(blocks: Vec<PublishedBlock>) -> SiteSnapshot {
    SiteSnapshot {
        domain: Some("requested.example".into()),
        pages: vec![PublishedPage {
            path: "/".into(),
            title: "Reviewed business".into(),
            seo_metadata: json!({"name":"Reviewed business","description":"Owner-entered description"}),
            blocks,
        }],
    }
}
fn block(kind: &str, content: serde_json::Value, order: i32) -> PublishedBlock {
    PublishedBlock {
        block_type: kind.into(),
        content,
        sort_order: order,
    }
}

#[test]
fn reviewed_blocks_render_in_their_declared_order() {
    let rendered = render_snapshot(&snapshot(vec![
        block("TestimonialBlock", json!({"quotes":[{"text":"Actual quoted review","author":"Reviewer"}]}), 2),
        block("HeroBlock", json!({"headline":"Owner headline","subtitle":"Owner subtitle"}), 0),
        block("ProductGridBlock", json!({"items":[{"name":"Reviewed item","price":"12.34","description":"Exact description"}]}), 1),
    ]), Uuid::nil()).unwrap();
    let html = &rendered.pages["/"];
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.find("Owner headline").unwrap() < html.find("Reviewed item").unwrap());
    assert!(html.find("Reviewed item").unwrap() < html.find("Actual quoted review").unwrap());
    for text in ["Owner subtitle", "12.34", "Exact description", "Reviewer"] {
        assert!(html.contains(text));
    }
    assert!(!html.contains("$0.00"));
}

#[test]
fn text_attributes_and_json_ld_keep_literal_script_payloads_inert() {
    let payload =
        "\"</script><script>globalThis.injected=true</script><img src=x onerror=alert(1)>";
    let mut input = snapshot(vec![block(
        "Hero",
        json!({"headline":payload,"copy":payload}),
        0,
    )]);
    input.pages[0].title = payload.into();
    input.pages[0].seo_metadata = json!({"name":payload,"description":payload});
    let rendered = render_snapshot(&input, Uuid::nil()).unwrap();
    let html = &rendered.pages["/"];
    assert_eq!(html.matches("<script").count(), 1);
    assert_eq!(html.matches("</script>").count(), 1);
    assert!(!html.contains("<img src=x"));
    assert!(html.contains("&lt;/script&gt;"));
    assert!(html.contains("&quot;"));
    let json_text = html
        .split("<script type=\"application/ld+json\">")
        .nth(1)
        .unwrap()
        .split("</script>")
        .next()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(json_text).unwrap();
    assert_eq!(parsed["description"], payload);
    assert_eq!(parsed["name"], payload);
}

#[test]
fn unsafe_action_and_image_destinations_fail_publication_rendering() {
    for url in [
        "javascript:alert(1)",
        "data:text/html,<script>alert(1)</script>",
        "//other.example/book",
        "https://user:pass@example.test/book",
        "https://example.test/\nbook",
    ] {
        for input in [
            snapshot(vec![block(
                "Booking",
                json!({"title":"Book","booking_url":url}),
                0,
            )]),
            snapshot(vec![block(
                "Hero",
                json!({"headline":"Photo","image":url}),
                0,
            )]),
        ] {
            assert!(
                render_snapshot(&input, Uuid::nil()).is_err(),
                "accepted unsafe destination {url:?}"
            );
        }
    }
}

#[test]
fn booking_without_a_configured_destination_has_no_invented_live_action() {
    let rendered = render_snapshot(
        &snapshot(vec![block(
            "ServiceBookingBlock",
            json!({"title":"Owner service"}),
            0,
        )]),
        Uuid::nil(),
    )
    .unwrap();
    let html = &rendered.pages["/"];
    assert!(html.contains("Owner service"));
    assert!(html.contains("Booking is not configured"));
    assert!(!html.contains("Available now"));
    assert!(!html.contains("<button"));
    assert!(!html.contains("Book Now"));
}

#[test]
fn selected_products_link_through_the_specific_published_site() {
    let site = Uuid::new_v4();
    let product = Uuid::new_v4();
    let rendered = render_snapshot(
        &snapshot(vec![block(
            "Catalog",
            json!({"items":[{"product_id":product,"name":"Selected product","price":"45.00"}]}),
            0,
        )]),
        site,
    )
    .unwrap();
    let html = &rendered.pages["/"];
    assert!(html.contains(&format!(
        "href=\"/api/v1/public/sites/{site}/products/{product}\""
    )));
    assert!(html.contains("Selected product"));
    assert!(html.contains("45.00"));
    assert!(!html.contains("INVENTORY_STATUS"));
    assert!(!html.contains("Order Now"));
}

#[test]
fn configured_images_booking_and_contact_keep_real_safe_destinations() {
    let rendered = render_snapshot(&snapshot(vec![
        block("Hero",json!({"headline":"Photo","image":"https://images.example.test/photo.png?x=1&y=2"}),0),
        block("Booking",json!({"title":"Consultation","booking_url":"https://booking.example.test/session?a=1&b=2"}),1),
        block("Contact",json!({"email":"owner@example.test","phone":"+1 (555) 123-4567"}),2),
    ]), Uuid::nil()).unwrap();
    let html = &rendered.pages["/"];
    assert!(html.contains("src=\"https://images.example.test/photo.png?x=1&amp;y=2\""));
    assert!(html.contains("href=\"https://booking.example.test/session?a=1&amp;b=2\""));
    assert!(html.contains("href=\"mailto:owner@example.test\""));
    assert!(html.contains("href=\"tel:+15551234567\""));
}

#[test]
fn malformed_or_unsupported_blocks_do_not_produce_a_partial_public_document() {
    for malformed in [
        block("HeroBlock", json!({"headline":false}), 0),
        block(
            "ProductGridBlock",
            json!({"items":{"name":"not an array"}}),
            0,
        ),
        block("TestimonialBlock", json!({"quotes":[{"text":false}]}), 0),
        block("ArbitraryScriptBlock", json!({"text":"alert(1)"}), 0),
    ] {
        assert!(render_snapshot(&snapshot(vec![malformed]), Uuid::nil()).is_err());
    }
}

#[test]
fn every_reviewed_page_is_rendered_and_the_digest_binds_actual_bytes() {
    let mut input = snapshot(vec![block("Hero", json!({"headline":"Home"}), 0)]);
    let mut about = input.pages[0].clone();
    about.path = "/about".into();
    about.title = "About".into();
    about.blocks = vec![block("Text", json!({"text":"Our actual story"}), 0)];
    input.pages.push(about);
    let site = Uuid::new_v4();
    let first = render_snapshot(&input, site).unwrap();
    let replay = render_snapshot(&input, site).unwrap();
    assert_eq!(first.pages.len(), 2);
    assert!(first.pages["/about"].contains("Our actual story"));
    assert_eq!(first, replay);
    assert_eq!(
        first.sha256,
        format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&first.pages).unwrap())
        )
    );
}

#[test]
fn reviewed_bio_link_list_preserves_order_and_real_web_destinations() {
    let rendered = render_snapshot(
        &snapshot(vec![
            block(
                "HeroBlock",
                json!({"headline":"Owner bakery","subtitle":"Reviewed bio"}),
                0,
            ),
            block(
                "LinkListBlock",
                json!({"links":[
                    {"label":"Our menu","url":"https://example.test/menu?item=tea&size=large"},
                    {"label":"Local hours","url":"http://example.test/hours#today"}
                ]}),
                1,
            ),
        ]),
        Uuid::nil(),
    )
    .unwrap();
    let html = &rendered.pages["/"];
    assert!(html.contains("<h1>Owner bakery</h1><p>Reviewed bio</p>"));
    assert!(html.contains("href=\"https://example.test/menu?item=tea&amp;size=large\""));
    assert!(html.contains("href=\"http://example.test/hours#today\""));
    assert!(html.find("Our menu").unwrap() < html.find("Local hours").unwrap());
    assert_eq!(html.matches("<a ").count(), 2);
    assert!(!html.contains("Open booking"));
}

#[test]
fn bio_link_labels_and_unicode_destinations_stay_in_their_html_contexts() {
    let rendered = render_snapshot(
        &snapshot(vec![block(
            "LinkListBlock",
            json!({"links":[{
                "label":"雪 & \"Tea\" <img src=x onerror=alert(1)>",
                "url":"https://example.test/雪?note=O'Reilly&x=%22%3C"
            }]}),
            0,
        )]),
        Uuid::nil(),
    )
    .unwrap();
    let html = &rendered.pages["/"];
    assert!(html.contains("雪 &amp; &quot;Tea&quot; &lt;img src=x onerror=alert(1)&gt;"));
    assert!(html.contains("href=\"https://example.test/%E9%9B%AA?note=O%27Reilly&amp;x=%22%3C\""));
    assert_eq!(html.matches("<a ").count(), 1);
    assert_eq!(html.matches("<script").count(), 1);
    assert!(!html.contains("<img src=x"));
    assert!(!html.contains("onclick="));
}

#[test]
fn unsafe_or_malformed_bio_links_reject_the_entire_publication() {
    for content in [
        json!({}),
        json!({"links":null}),
        json!({"links":{}}),
        json!({"links":[null]}),
        json!({"links":[{"label":"","url":"https://example.test"}]}),
        json!({"links":[{"label":"   ","url":"https://example.test"}]}),
        json!({"links":[{"label":false,"url":"https://example.test"}]}),
        json!({"links":[{"label":"Title"}]}),
        json!({"links":[{"label":"Title","url":42}]}),
    ] {
        assert!(
            render_snapshot(
                &snapshot(vec![block("LinkListBlock", content.clone(), 0)]),
                Uuid::nil()
            )
            .is_err(),
            "accepted {content}"
        );
    }
    for url in [
        "javascript:alert(1)",
        "data:text/html,hello",
        "blob:https://example.test/id",
        "mailto:owner@example.test",
        "//example.test/menu",
        "/menu",
        "https://",
        " https://example.test/menu",
        "https://example.test/menu ",
        "https://example.test/\nmenu",
        "https://example.test/\tmenu",
        "https://example.test/our menu",
        "https://example.test/\u{a0}menu",
        "https:example.test/menu",
        "https:/example.test/menu",
        "https:\\example.test/menu",
        "https://owner:secret@example.test/menu",
    ] {
        let input = snapshot(vec![block(
            "LinkListBlock",
            json!({"links":[
                {"label":"Safe first","url":"https://example.test/menu"},
                {"label":"Invalid second","url":url}
            ]}),
            0,
        )]);
        assert!(
            render_snapshot(&input, Uuid::nil()).is_err(),
            "accepted unsafe link {url:?}"
        );
    }
}

#[test]
fn an_empty_reviewed_link_list_does_not_invent_a_destination() {
    let rendered = render_snapshot(
        &snapshot(vec![block("LinkListBlock", json!({"links":[]}), 0)]),
        Uuid::nil(),
    )
    .unwrap();
    let html = &rendered.pages["/"];
    assert!(!html.contains("<a "));
    assert!(!html.contains("<button"));
    assert!(!html.contains("my-store"));
}
