# Zero-Touch AI Onboarding & Proactive Agent Coordination Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Modify the onboarding `launch` step to proactively generate "Day Zero" action cards in the agent feed for The Scout, The Promoter, and The Manager.

**Architecture:** We will insert three additional `agent_feed_items` during the execution of `launch` in `src/server/services/onboarding/preparation.rs`. These items simulate the proactive work completed by agents (SEO tags, Social Media drafts, and Operational Setup) to instantly populate the new owner's dashboard.

**Tech Stack:** Rust, SQLx (Postgres)

**Spec:** `docs/superpowers/specs/2026-09-20-day-zero-onboarding-design.md`

## Global Constraints

- Follow established error handling and db transaction patterns.
- Ensure the new DB inserts use `&mut *tx`.

## Review Focus

- The action cards are created transactionally along with the welcome card and the state updates.
- The `company_name` and `business_type` are injected dynamically into the payloads if applicable.

---

### Task 1: Insert Day Zero action cards during launch

**Files:**
- Modify: `src/server/services/onboarding/preparation.rs`

**Interfaces:**
- Consumes: The `launch` function signature and transaction handling in `src/server/services/onboarding/preparation.rs`.
- Produces: Proactive agent feed items for the newly launched tenant.

- [ ] **Step 1: Write the failing test**

We don't need to write a new test, but we can verify this by checking if the project compiles and tests pass after modification.
Instead, we will directly edit `src/server/services/onboarding/preparation.rs` to insert the three proactive action cards.

- [ ] **Step 2: Implement the feature**

In `src/server/services/onboarding/preparation.rs`, inside the `launch` function, right after the insertion of the `onboarding-welcome` feed item (around line 631), add three more `sqlx::query` executions for The Scout, The Promoter, and The Manager:

```rust
    // 1. The Scout (SEO Meta Tags)
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state) VALUES($1,$2,'system',$3,$4,'PENDING_APPROVAL')")
        .bind(format!("onboarding-scout-{}", p.preparation_id))
        .bind(tenant)
        .bind(json!({
            "description": "The Scout has researched your market and drafted SEO meta tags for your storefront.",
            "feature_type": "day_zero_seo",
            "company_name": p.reviewed_request["company_name"],
            "business_type": p.reviewed_request["business_type"],
            "agent_role": "The Scout"
        }))
        .bind(json!({
            "action_type": "review_seo_tags",
            "draft": {
                "title": format!("{} - {}", p.reviewed_request["company_name"].as_str().unwrap_or("Store"), p.reviewed_request["business_type"].as_str().unwrap_or("Services")),
                "description": "Welcome to our store. We provide the best services in town."
            }
        }))
        .execute(&mut *tx).await?;

    // 2. The Promoter (Social Media Launch Campaign)
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state) VALUES($1,$2,'system',$3,$4,'PENDING_APPROVAL')")
        .bind(format!("onboarding-promoter-{}", p.preparation_id))
        .bind(tenant)
        .bind(json!({
            "description": "The Promoter has drafted a 3-post 'We are Open!' social media campaign.",
            "feature_type": "day_zero_social",
            "company_name": p.reviewed_request["company_name"],
            "agent_role": "The Promoter"
        }))
        .bind(json!({
            "action_type": "review_social_campaign",
            "drafts": [
                format!("We are thrilled to announce the grand opening of {}! Come visit us.", p.reviewed_request["company_name"].as_str().unwrap_or("our new store")),
                "Check out our new products and services. We can't wait to serve you!",
                "It's official! We are now open for business. See you soon!"
            ]
        }))
        .execute(&mut *tx).await?;

    // 3. The Manager (Operational Setup)
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state) VALUES($1,$2,'system',$3,$4,'PENDING_APPROVAL')")
        .bind(format!("onboarding-manager-{}", p.preparation_id))
        .bind(tenant)
        .bind(json!({
            "description": "The Manager has drafted suggested operating hours and delivery/service zones.",
            "feature_type": "day_zero_ops",
            "company_name": p.reviewed_request["company_name"],
            "agent_role": "The Manager"
        }))
        .bind(json!({
            "action_type": "review_operations",
            "draft": {
                "hours": "Mon-Fri: 9am - 5pm",
                "zone": "Standard Delivery Area"
            }
        }))
        .execute(&mut *tx).await?;
```

- [ ] **Step 3: Verify tests pass**

Run `make test` and `make lint` to verify that everything compiles and all tests pass.

- [ ] **Step 4: Commit**

Commit the changes.
