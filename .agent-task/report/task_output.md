# 🔍 Scout: Tool Integration Research - Judge.me

## Phase 1: Dynamic Discovery & Selection

I dynamically searched Reddit threads (`r/smallbusiness`), Trustpilot, and competitor app stores (Shopify, Wix) to discover trending tool integration demands. Based on these signals, I selected **Product Review & Social Proof** as a critical integration category. Comparing industry options (Yotpo, Okendo, Loox, Judge.me), **Judge.me** emerged as the definitive choice for the OHC platform due to its incredible value (flat $15/mo or free vs. expensive tiers), 5-star reputation, and robust API capabilities.

---

## Issue Brief: Integrate Judge.me for Automated Customer Reviews and Social Proof

**Title**: Integrate Judge.me for Automated Customer Reviews and Social Proof

**Problem Statement**:
Building trust is the single biggest hurdle for new small business owners. Traditional website builders require manual review collection or push business owners into expensive "subscription hell" (like Yotpo, which can jump to hundreds of dollars a month) to automate review requests. Small businesses need a way to automatically gather text, photo, and video reviews after a purchase and display them elegantly without needing an enterprise marketing budget or technical setup.

**Research Report**:

* **Tool:** Judge.me
* **Market Position:** One of the most popular, highly-rated review platforms across Shopify and WooCommerce. Known as the accessible, "best value" alternative to expensive enterprise suites.
* **Capabilities & Limits:**
  * **Automated Requests:** Automatically sends emails/SMS asking for reviews based on fulfillment or delivery triggers.
  * **Rich Media:** Supports photo and video reviews natively.
  * **SEO Boost:** Automatically injects rich snippets into Google Search so product star ratings show up in search results (addressing "Invisible Discovery" pain point).
  * **API Quality:** Comprehensive REST API for headless integration. Webhooks available for sync.
* **SaaS Viability & Pricing:**
  * **Pricing Model:** Exceptional for SMBs. They offer a robust "Forever Free" tier with unlimited review requests. Their premium tier is a flat $15/month, avoiding the "cost creep" and "subscription hell" common in App Stores.
  * **Modes:** Cloud integration via their REST API is straightforward. For Standalone modes, the business connects their own API key.
* **Reputation & Ease of Use:** Incredible reputation (5-star average over thousands of reviews on Trustpilot and Shopify). It is praised specifically by small business owners for being straightforward and fair.

**Design Doc**:

* **Trigger:** When an order status in OHC changes to `Fulfilled` or `Delivered`, the OHC event mesh triggers a payload to the Judge.me API to schedule a review request email.
* **Action:** Judge.me handles the email delivery and review collection. Once a review is submitted, a webhook updates the OHC database, triggering an update to the product's average rating cache.
* **User Experience (OHC Dashboard):**
  * A "Social Proof" settings card where users enable Judge.me with one click (OAuth).
  * Users see an aggregated feed of new reviews.
  * "The Promoter" (OHC AI) can suggest drafting polite replies to 1-star reviews or sharing 5-star reviews directly to the business's linked Instagram/Facebook pages.
  * The Storefront builder automatically surfaces a "Review Carousel" block powered by Judge.me data.

**Implementation Prompt**:
Integrate Judge.me API to automate the collection and display of customer reviews to build social proof instantly.

* **Acceptance Criteria 1 (Automation):** The system must automatically schedule a review request via Judge.me when an order is completed/fulfilled.
* **Acceptance Criteria 2 (Display):** The storefront UI must render product star ratings and review lists natively, pulling data synced from the Judge.me API.
* **Acceptance Criteria 3 (Sync):** Webhooks must be implemented to instantly sync new reviews back to the OHC platform for display and AI analysis.
* **Acceptance Criteria 4 (Pricing Transparency):** The integration must leverage Judge.me's free tier capabilities by default, ensuring no hidden costs for the business owner.

**Priority**: P1 (High)
**Estimated Scope**: Medium

## Next Steps

1. The engineering swarm should review the proposed issue brief above.
2. An implementer agent should be assigned to build the Judge.me REST integration module within the OHC ecosystem.
