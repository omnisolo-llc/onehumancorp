# Zero-Touch AI Onboarding & Proactive Agent Coordination Design

## Background
Small business owners find the "blank slate" of a new workspace overwhelming. The transition from onboarding to a useful dashboard is too passive. The Onboarding Agent emits events upon completion, but there's no proactive "Day Zero" task suite that populates the feed immediately.

## Proposed Design
When a tenant completes onboarding, we want to simulate proactive work by their new "AI team" to build trust and show immediate value.

The design modifies `src/server/services/onboarding/preparation.rs` to generate these action cards directly when onboarding is launched, ensuring they are reliably inserted.

We will add the following events to the `Preparation` notifications, which are emitted as `TeammateMeshEvent` during `launch_preparation` or handled directly:
1. **The Scout**: Emit an event or directly create an `agent_feed_item` simulating SEO research.
2. **The Promoter**: Emit an event or directly create an `agent_feed_item` simulating drafted social media launch posts.
3. **The Manager**: Emit an event or directly create an `agent_feed_item` simulating operational setup (e.g., delivery zone and hours).

However, looking at the current architecture, `src/server/services/onboarding/preparation.rs` already inserts an `agent_feed_item` during `launch` (the "review storefront" welcome card):
```rust
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state) VALUES($1,$2,'system',$3,$4,'PENDING_APPROVAL')")
        .bind(format!("onboarding-welcome-{}",p.preparation_id)).bind(tenant).bind(json!({"description":"Your local business setup is saved. Review storefront and provider readiness before publishing.","feature_type":"onboarding_welcome","company_name":p.reviewed_request["company_name"]})).bind(json!({"action_type":"review_storefront"})).execute(&mut *tx).await?;
```

We will extend this section in `launch` to insert three additional high-value "Action Cards" into `agent_feed_items`:
1. **The Scout (SEO Meta Tags)**:
   - `event_source`: `system`
   - `context_payload`: `{"description": "The Scout has researched your market and drafted SEO meta tags for your storefront.", "feature_type": "day_zero_seo"}`
   - `proposed_action`: `{"action_type": "review_seo_tags", "draft": {"title": "... SEO Title ...", "description": "... SEO Description ..."}}`
2. **The Promoter (Social Media Launch Campaign)**:
   - `event_source`: `system`
   - `context_payload`: `{"description": "The Promoter has drafted a 3-post 'We are Open!' social media campaign.", "feature_type": "day_zero_social"}`
   - `proposed_action`: `{"action_type": "review_social_campaign", "drafts": ["Post 1...", "Post 2...", "Post 3..."]}`
3. **The Manager (Operational Setup)**:
   - `event_source`: `system`
   - `context_payload`: `{"description": "The Manager has drafted suggested operating hours and delivery/service zones.", "feature_type": "day_zero_ops"}`
   - `proposed_action`: `{"action_type": "review_operations", "draft": {"hours": "...", "zone": "..."}}`

This approach guarantees that when the user is redirected to the dashboard after `launch`, the action cards are immediately visible in the Unified Agent Feed, waiting for approval.

This avoids complex asynchronous race conditions where background workers might not finish generating the cards before the user's first page load.
