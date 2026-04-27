# Issue Brief: Multi-Tenant SaaS Tier Architecture

## Problem Statement
Small business owners joining OneHumanCorp (OHC) need a transparent, fair, and scalable pricing model. They should clearly understand what they get for free, why they should upgrade, and what happens when they hit tier limits. Currently, there is a lack of a formalized architectural design for managing and enforcing tier limits across different business resources (products, AI agents, storage, custom domains). We need to architect the Multi-Tenant SaaS Tier system to ensure seamless upgrades, clear user communication, and robust backend enforcement without prescribing rigid implementation details.

## Research Report
### Competitive Landscape
- **Shopify:** Starts at $39/mo for basic features. No free tier. Very complex for simple businesses.
- **Wix/Squarespace:** Free tiers exist but are heavily ad-supported and lack custom domains. Upgrades are often confusing with many overlapping plans.
- **GoDaddy:** Cheap entry points but rapid price hikes on renewal.
- **Opportunity:** OHC can differentiate by offering a genuinely useful Free tier (to capture non-technical users early) and a highly predictable upgrade path. The focus must be on limiting volume (products, AI actions, storage) rather than entirely gating essential features, allowing users to experience the full platform value before paying.

### Key Requirements
1.  **Tier Definitions:**
    *   **Free:** $0/mo. 10 Products, 1 AI Department, 100 AI actions/mo, 500MB Storage, OHC Subdomain.
    *   **Starter:** $9/mo. 100 Products, 3 AI Departments, 1,000 AI actions/mo, 5GB Storage, Custom Domain.
    *   **Pro:** $29/mo. Unlimited Products, 10 AI Departments, Unlimited AI actions, 50GB Storage, Custom Domain + SSL.
    *   **Business:** $79/mo. Unlimited everything, 500GB Storage, Multi-domain.
2.  **Enforcement Strategy:** Enforce limits at the orchestration/API layer, ensuring a graceful user experience when limits are reached, rather than abrupt technical failures.
3.  **Upgrade Flow:** 375px mobile-first UI for seamless in-app upgrades using native platform payments or Stripe.

## Design Doc

### High-Level Architecture
-   **Tenant Context:** Every request (API, background job) carries a `tenant_id` and is augmented with the current `Tier` status.
-   **Limit Enforcement:** The Orchestrator or Gateway checks limits before executing resource-intensive tasks (e.g., triggering an AI action, uploading a file).
-   **Graceful Degradation:** When an AI action limit is hit, the action is paused/queued, and a push notification/UI alert prompts the owner to upgrade.
-   **Billing Sync:** Tier status is kept in sync with the payment provider (e.g., Stripe webhooks) to handle upgrades, downgrades, and payment failures.

### Architecture Diagram
```mermaid
sequenceDiagram
    participant App as Mobile App (375px)
    participant O as KAIROS Orchestrator
    participant Billing as Billing Service (Stripe)
    participant Agent as AI Agent (e.g. The Promoter)

    App->>O: Request Resource (e.g., Generate Marketing Copy)
    O->>O: Check Tier Limits (Actions/mo)
    alt Limit Exceeded
        O-->>App: Return Limit Reached Status
        App->>App: Display Upgrade Prompt UI
    else Within Limit
        O->>Agent: Execute Task
        Agent-->>O: Task Complete
        O->>Billing: Increment Meter (AI Action Count)
        O-->>App: Return Result
    end

    App->>Billing: Initiate Upgrade (Stripe Session)
    Billing-->>App: Upgrade Success
    Billing->>O: Update Tenant Tier Status

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class App,O,Billing,Agent premium;
```

### Mobile UX Flow (375px First)
1.  **Usage Dashboard:** A simple settings screen showing current usage vs. limits (e.g., "AI Actions: 85/100").
2.  **In-Context Upgrade Prompts:** When adding the 11th product on the Free tier, a glassmorphic modal appears: "You're growing! Upgrade to Starter to add up to 100 products."
3.  **Upgrade Screen:** A clean, 3-column swipeable card layout comparing tiers. Highlights the most logical next step.
4.  **Checkout:** Native OS payment prompt or clean Stripe Elements checkout, optimized for mobile keyboards.

### AI Agent Integration Points
-   The "Business Advisory" Agent monitors usage and proactively suggests upgrades when appropriate (e.g., "You're getting lots of traffic. Upgrading to Starter gives you a custom domain to build trust.").
-   AI Agents themselves are subject to the monthly action limits based on the tier.

## Implementation Prompt
Design and implement the SaaS tier enforcement logic and the corresponding user-facing upgrade flows. The system must track usage against tier-specific limits (products, AI actions, storage) and enforce these limits gracefully.
1.  Implement a `TierService` or equivalent middleware to intercept requests and validate them against the tenant's current tier constraints.
2.  Build the mobile-first UI for displaying usage metrics and facilitating upgrades (using OHC premium design tokens: glassmorphism, Outfit/Inter typography).
3.  Ensure the upgrade flow integrates with the existing payment provider (Stripe) and handles asynchronous state updates via webhooks.
Focus on the user experience when a limit is reached—it must be a positive nudge to grow, not an error message. Do not hardcode SQL DDL; use the existing ORM/DB migration patterns.

## Priority
P1

## Estimated Scope
Medium
