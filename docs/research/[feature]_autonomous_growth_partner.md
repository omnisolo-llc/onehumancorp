# Issue Brief: Autonomous Growth Partner

## Title
Autonomous Growth Partner: Invisible Business Growth Engine for Solopreneurs

## Problem Statement
Small business owners (like Carlos the handyman or Priya the boutique owner) struggle to consistently execute growth strategies such as SEO optimization, targeted marketing, and lead follow-up. Existing platforms like Wix and Shopify offer marketing "tools" (e.g., email builders, SEO wizards), but these still require the user to invest significant time to learn and operate them. This results in "operational fatigue" where the business stops growing because the owner is too busy working *in* the business, rather than *on* it.

## Research Report
- **Durable Benchmark:** Durable provides tools to build a site quickly and has some basic AI generation for blogs/emails. However, it still largely relies on the user to initiate and execute these campaigns.
- **Shopify Benchmark:** Shopify provides powerful marketing automation tools, but configuring them requires technical knowledge and manual setup.
- **OHC Current State:** We currently have reactive tools. The user must initiate actions.
- **Target Opportunity:** We need to leapfrog from "Ask AI" to "AI acts for you." An Autonomous Growth Partner that operates independently in the background, proactively suggesting and executing growth activities.

## Design Doc
### High-Level Architecture
- **The Analyst Module:** Monitors sales data, customer inquiries, and market trends to identify growth opportunities.
- **The Strategist Module:** Formulates actionable plans (e.g., "Send a promotional email for slow-moving inventory," "Update Google Business Profile with new photos").
- **The Executor Module:** Drafts the content (emails, social posts, SEO updates) and prepares them for execution.
- **The 1-Tap Approval Interface:** Presents the formulated plan and drafted content to the user via a mobile-first activity feed for a simple 1-tap "Approve & Execute" or "Dismiss."
- **Entity Types:** `GrowthOpportunity`, `ActionPlan`, `DraftContent`, `ExecutionLog`.

### Mobile UX Flow (375px first)
1.  **Notification:** User receives a push notification: "Your Autonomous Growth Partner found an opportunity."
2.  **Activity Feed Card:** A card appears in the main feed outlining the opportunity (e.g., "Carlos, you haven't posted about your new plumbing service. I've drafted an Instagram post and a promotional email to past clients.").
3.  **Review Screen:** User taps the card to review the drafted content and target audience.
4.  **Action:** User taps "Approve" (executes the plan) or "Edit" (modifies the content before execution).

## Implementation Prompt
Implement the "Autonomous Growth Partner" engine. This feature should autonomously analyze user data (e.g., recent inventory additions, lack of recent social activity) and proactively generate marketing campaigns or SEO improvements. The output must be presented to the user in a mobile-optimized activity feed requiring only a single tap to approve and deploy. The system should utilize the existing KAIROS orchestrator for background processing and the Teammate Mesh for cross-department coordination. Ensure the UI adheres to the "Grandmother Test" for extreme simplicity. Do not prescribe specific database schemas or API endpoints.

## Priority
P0

## Estimated Scope
Large
