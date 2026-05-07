# [architecture]_business_journey

## Title
Architectural Mapping of the End-to-End Business Journey for OHC Personas

## Problem Statement
The OneHumanCorp (OHC) platform aims to empower a diverse range of non-technical small business owners—such as Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator—to launch and grow their businesses entirely from a mobile device within 10 minutes. A critical gap exists: the end-to-end user journeys (Acquisition, Onboarding, Activation, Retention, Revenue, Referral) lack a cohesive architectural map, risking fragmentation and user drop-off at critical friction points. We must define these pathways to ensure the system naturally guides users to success without exposing them to technical jargon or complexity.

## Research Report
The business journey is evaluated against our core personas, reflecting real-world pain points and requirements:
- **Maya (Home Baker, 28):** Driven by Instagram, requires easy custom order forms, deposit handling, and an AI to intercept and draft replies for incoming inquiries.
- **Carlos (Handyman, 42):** Reliant on word of mouth, needs a mobile-friendly service listing, automated quoting via an AI salesperson, and a straightforward booking calendar with deposits.
- **Priya (Boutique Owner, 35):** Needs a unified system for in-store tap-to-pay and online sales, syncing inventory variants (size/color), and actionable, plain-language analytics for reordering.
- **Leo (Music Tutor, 22):** Operates online and in-person, requiring subscription management for lesson packages, automated meeting link generation, and a TikTok link-in-bio presence.
- **Fatima (Food Cart Operator, 50):** Faces language barriers and relies on older Android hardware. Needs extreme simplicity: a photo menu with sold-out toggles, pre-order capabilities, and prominent push notifications.

### Journey Stages & Friction Points
1.  **Acquisition:** Discovery via social channels or word-of-mouth. The CTA ("Launch in 5 mins") must match the rapid onboarding reality.
2.  **Onboarding:** The most critical friction point. Requesting complex DNS setup or exhaustive product catalogs upfront causes abandonment. We must capture minimum viable data (Vibe, Name, Basic Service/Product) and let the "Promoter" AI generate the rest.
3.  **Activation:** The "Aha!" moment—a live, shareable URL and the first transaction. This must happen on Day 1.
4.  **Retention:** Friction occurs if users have to hunt for metrics. "The Advisor" AI mitigates this with weekly, plain-language health reports and proactive suggestions.
5.  **Revenue:** Transitioning to paid tiers ($9/mo Starter). Friction: Unexpected hard limits. Solution: Graceful degradation and contextual upgrade prompts.
6.  **Referral:** Viral loops built into the storefront footer ("Built with OHC") and referral discounts for both parties.

## Design Doc

### Key Design Decisions
-   **Progressive Onboarding:** The initial setup wizard asks for the absolute minimum (e.g., "Describe your business in one sentence"). Advanced settings are deferred and introduced contextually later.
-   **AI-First Setup:** The Marketing & Advertising Agent ("The Promoter") uses the initial input to automatically select a visual vibe, generate placeholder content, and structure the Smart Blocks (Hero, Catalog, Booking).
-   **Mobile-First Constraint:** Every flow, especially the complex ones like Payment Gateway integration, is designed strictly for the 375px mobile viewport first. Large touch targets (≥ 44x44px) and clear, jargon-free typography are mandatory.
-   **Offline Resilience:** For users like Fatima on spotty networks, critical actions (e.g., toggling an item "Sold Out") must update optimistically in the local SQLite SIPDB and sync asynchronously via the KAIROS Orchestrator.

### Mobile UX Flow (375px First) - Maya's Onboarding
1.  **Welcome Screen:** "Let's build your business. What do you do?" [Text Input: "I make custom vegan cakes in Brooklyn."]
2.  **AI Generation Shimmer:** "The Promoter is building your storefront..." (2-3 seconds, progressive loading).
3.  **Preview Screen:** A fully rendered, mobile-responsive draft storefront (Glassmorphism styling, Outfit headers).
4.  **One-Tap Activation:** "Looks great, let's go live!" -> Provisions `maya.ohc.app` instantly.
5.  **First Action Prompt:** "Your shop is live! Add your first cake to start taking orders."

### Architecture Diagrams

#### End-to-End Flow for Maya (The Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant App as OHC Mobile App (375px)
    participant Auth as Auth & Tier Service
    participant Orch as KAIROS Orchestrator
    participant Mktg as Marketing Agent (The Promoter)
    participant Ops as Operations Agent
    participant DB as Shared DB (Tenant Scoped)

    %% Acquisition & Onboarding
    Maya->>App: Submits "Vegan Cake Shop"
    App->>Auth: Register Tenant & Issue JWT
    Auth-->>App: JWT (Free Tier)
    App->>Orch: Trigger Onboarding Setup
    Orch->>Mktg: Generate Storefront Draft
    Mktg->>DB: Save Smart Blocks (Draft)
    Mktg-->>App: Return Live Preview

    %% Activation
    Maya->>App: "Go Live"
    App->>Orch: Publish Storefront
    Orch->>DB: Update Status to LIVE
    Orch-->>App: Provisioned URL (maya.ohc.app)

    %% Retention (First Order)
    note right of App: Customer places order
    Orch->>Ops: Process Order & Deposit
    Ops->>DB: Record Transaction
    Ops-->>App: Push Notification: "New Cake Order!"

    %% Revenue (Hitting limits)
    note right of App: Month 3: Maya needs custom domain
    Maya->>App: Request Custom Domain
    App->>Auth: Check Tier
    Auth-->>App: Exceeds Free Tier -> Prompt Upgrade
    Maya->>App: Upgrade to Starter ($9/mo)
```

## Implementation Prompt
**To Implementer Agent:**
Implement the progressive onboarding flow for the Mobile UI (Slint framework) strictly adhering to the 375px width constraint. Develop the "Welcome Screen" capturing minimal business data, the AI Generation shimmer state, and the "Preview Screen" displaying the generated storefront draft. Wire these screens to the KAIROS Orchestrator to trigger "The Promoter" agent for automatic Smart Block generation. Ensure all text uses plain language (e.g., "Go Live" instead of "Publish to CDN") and all touch targets are ≥ 44x44px. Do not prescribe specific database schemas or API endpoints; focus on delivering the described User Journey and UX flow.

## Priority
P0

## Estimated Scope
Medium

#### End-to-End Flow for Carlos (The Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant App as OHC Mobile App (375px)
    participant Auth as Auth & Tier Service
    participant Orch as KAIROS Orchestrator
    participant Mktg as Marketing Agent (The Promoter)
    participant Sales as Sales Agent (The Salesperson)
    participant DB as Shared DB (Tenant Scoped)
    participant Cust as Customer

    %% Acquisition & Onboarding
    Carlos->>App: Submits "Handyman Services in Austin"
    App->>Auth: Register Tenant & Issue JWT
    App->>Orch: Trigger Onboarding Setup
    Orch->>Mktg: Generate Storefront Draft (Service Vibe)
    Mktg->>DB: Save Smart Blocks (Booking/Service List)
    Mktg-->>App: Return Live Preview

    %% Activation
    Carlos->>App: "Go Live"
    App->>Orch: Publish Storefront
    Orch-->>App: Provisioned URL (carlos.ohc.app)

    %% Lead Generation & Quoting
    Cust->>App: Submits Booking Request for "Leaky Faucet"
    App->>Orch: Trigger Lead Event
    Orch->>Sales: Auto-Draft Quote ($150 estimated)
    Sales->>App: Notification: "Draft Quote Ready"
    Carlos->>App: Approve Quote
    Sales->>Cust: Email/SMS Quote to Customer

    %% Retention
    Cust->>App: Accepts & Pays Deposit
    Orch->>DB: Record Booking & Deposit
    Orch-->>App: Push Notification: "New Job Booked!"
```

#### End-to-End Flow for Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant App as OHC Mobile App / POS (375px)
    participant Auth as Auth & Tier Service
    participant Orch as KAIROS Orchestrator
    participant Mktg as Marketing Agent (The Promoter)
    participant Adv as Advisory Agent (The Advisor)
    participant DB as Shared DB (Tenant Scoped)
    participant Cust as In-Store Customer

    %% Acquisition & Onboarding
    Priya->>App: Submits "Women's Clothing Boutique"
    App->>Auth: Register Tenant & Issue JWT
    App->>Orch: Trigger Onboarding Setup
    Orch->>Mktg: Generate Storefront Draft (Retail Vibe)
    Mktg->>DB: Save Smart Blocks (Catalog Grid)
    Mktg-->>App: Return Live Preview

    %% Activation
    Priya->>App: "Go Live"
    App->>Orch: Publish Storefront
    Orch-->>App: Provisioned URL (priya.ohc.app)

    %% Omnichannel Sync
    Priya->>App: Adds new dress variant (Red/Medium)
    App->>DB: Update Inventory

    %% In-Store Sale
    Cust->>App: Tap-to-Pay for Dress
    App->>Orch: Process POS Transaction
    Orch->>DB: Deduct Inventory
    Orch-->>App: "Payment Successful"

    %% Retention & Advisory
    note right of Orch: Weekly Schedule Trigger
    Orch->>Adv: Generate Weekly Report
    Adv->>DB: Fetch Sales Data
    Adv-->>App: Push Notification: "Red dresses selling fast. Reorder?"
```

#### End-to-End Flow for Leo (The Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant App as OHC Mobile App (375px)
    participant Auth as Auth & Tier Service
    participant Orch as KAIROS Orchestrator
    participant Mktg as Marketing Agent (The Promoter)
    participant Ops as Operations Agent (The Manager)
    participant DB as Shared DB (Tenant Scoped)
    participant Stud as Student

    %% Acquisition & Onboarding
    Leo->>App: Submits "Online Guitar Lessons"
    App->>Auth: Register Tenant & Issue JWT
    App->>Orch: Trigger Onboarding Setup
    Orch->>Mktg: Generate Storefront Draft (Creator Vibe)
    Mktg->>DB: Save Smart Blocks (Subscriptions/Calendar)
    Mktg-->>App: Return Live Preview

    %% Activation
    Leo->>App: "Go Live"
    App->>Orch: Publish Storefront (Link-in-Bio optimized)
    Orch-->>App: Provisioned URL (leo.ohc.app)

    %% Subscription Booking
    Stud->>App: Buys "4 Lessons/Month" Package
    App->>Orch: Process Subscription Payment
    Orch->>DB: Record Subscription

    %% Lesson Scheduling
    Stud->>App: Books 1st Lesson Slot
    Orch->>Ops: Generate Meeting Link
    Ops->>DB: Store Zoom/Meet Link
    Ops->>Stud: Email Meeting Details
    Ops-->>App: Push Notification: "New Lesson Booked with Link"
```

#### End-to-End Flow for Fatima (The Food Cart)
```mermaid
sequenceDiagram
    actor Fatima
    participant App as OHC Mobile App (Low-End Android)
    participant Auth as Auth & Tier Service
    participant Orch as KAIROS Orchestrator
    participant Mktg as Marketing Agent (The Promoter)
    participant Ops as Operations Agent (The Manager)
    participant DB as Shared DB (SQLite Local Sync)
    participant Cust as Customer

    %% Acquisition & Onboarding
    Fatima->>App: Submits "Halal Cart Menu"
    App->>Auth: Register Tenant & Issue JWT
    App->>Orch: Trigger Onboarding Setup
    Orch->>Mktg: Generate Storefront Draft (Menu Vibe, Arabic+English)
    Mktg->>DB: Save Smart Blocks (Photo Menu/Pre-Order)
    Mktg-->>App: Return Live Preview

    %% Activation
    Fatima->>App: "Go Live"
    App->>Orch: Publish Storefront
    Orch-->>App: Provisioned URL (fatima.ohc.app)

    %% Pre-Order & Offline Resilience
    Cust->>App: Orders "Chicken over Rice" for Pickup
    App->>Orch: Process Order
    Orch->>DB: Record Order
    Orch-->>App: LOUD Push Notification / Ring: "New Order!"

    %% Quick Action
    Fatima->>App: One-Tap "Sold Out" on White Sauce
    App->>DB: Update Local SQLite (Optimistic)
    App->>Orch: Background Sync to Shared DB
```
