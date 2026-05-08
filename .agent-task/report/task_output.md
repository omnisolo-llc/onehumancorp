# OHC KAIROS Architecture Research Report: End-to-End Business Journey

## Problem Statement
Small business owners (Maya, Carlos, Priya, Leo, Fatima) need a unified, frictionless path from initial acquisition to sustainable revenue generation without needing technical expertise. OHC must provide a platform where anyone can launch and run a real business from their phone in under 10 minutes.

## Phase 1 & 4: Business Journey Mapping & Mobile UX Flows

The following sequence diagrams map the end-to-end journey for each core persona, designed and tested starting at the 375px breakpoint (Mobile-First constraint). All onboarding uses progressive profiling to minimize input.

### 1. Maya (The Home Baker)
**Acquisition to Revenue Sequence**
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Cust as Customer
    participant Stripe as Stripe API

    Maya->>Ad: Clicks "Launch Bakery in 5 mins"
    Maya->>OHC: Downloads App & Opens (375px)
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: Asks "What do you sell?" (Progressive Profiling)
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>OHC: Generates Storefront & Menu
    OHC->>Maya: Storefront Live! (Activation)
    Cust->>OHC: Messages via IG DM "Vegan?"
    OHC->>AI_Ops: Drafts Reply
    AI_Ops-->>Cust: "Yes, we do vegan cakes!"
    Cust->>OHC: Places Order & Pays Deposit
    OHC->>Stripe: Process Payment
    Stripe-->>OHC: Success
    OHC->>Maya: Push Notification "New Order Paid!" (Retention)
    Maya->>OHC: Clicks "Upgrade to Starter" (Revenue)
    OHC->>Stripe: Setup Recurring Billing
    Maya->>OHC: Shares Store Link with Friend (Referral)
```
**Mobile UX Flow (375px):**
*   **Onboarding:** Clean form utilizing native mobile keyboards.
*   **Dashboard:** Skeleton loading (shimmer effect) for metrics. "Agent Activity Feed" displays AI_Ops drafts (e.g., reply to IG DM) for 1-tap approval.

### 2. Carlos (The Handyman)
**Acquisition to Referral Sequence**
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant OHC as OHC Web App
    participant AI_Mark as Marketing Agent
    participant AI_Sales as Sales Agent
    participant Cust as Customer

    Carlos->>WoM: Hears about OHC
    Carlos->>OHC: Visits website on Android
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Carlos: Asks "What services do you offer?"
    Carlos->>AI_Mark: "Plumbing, Painting"
    AI_Mark->>OHC: Generates Service Listings & Booking Calendar
    OHC->>Carlos: Booking Page Live! (Activation)
    Cust->>OHC: Requests Quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: Approves 1-tap
    AI_Sales-->>Cust: Sends Official Quote
    Cust->>OHC: Books Time & Pays Deposit
    OHC->>Carlos: Notification "Job Booked" (Retention)
    Carlos->>OHC: Taps "Earn $50: Refer a Pro" (Referral)
    Carlos->>Cust: Taps "Send 10% Discount to Friend" (Viral Loop)
```
**Mobile UX Flow (375px):**
*   **Calendar Sync:** Intuitive AI assistance maps real-world availability to digital systems, avoiding complex calendar settings.
*   **Task Approval:** AI_Sales quote drafted and presented in the Home Dashboard feed as a clear card for "1-Tap Approval" with an "Edit" option.

### 3. Priya (The Boutique Owner)
**Omnichannel Omnipresence Sequence**
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Adv as Advisory Agent
    participant POS as In-Store POS (Tap-to-pay)

    Priya->>Search: Searches "Easy online store for boutique"
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Priya: Syncs initial inventory
    AI_Mark->>OHC: Generates Storefront with variants
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale via phone
    POS->>OHC: Update Inventory
    OHC->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: "Inventory low. Upgrade tier for automated re-order alerts." (Revenue)
    Priya->>OHC: Selects "Pro Plan" (Revenue)
    OHC->>Priya: Enables Multi-Store Sync
```
**Mobile UX Flow (375px):**
*   **In-Store Context:** Fast action buttons (≥ 44x44px) for POS integration and inventory reduction.
*   **Reporting:** Visual, plain-language insights on the dashboard rather than complex data tables.

### 4. Leo (The Music Tutor)
**Subscription Engine Sequence**
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Student as Student

    Leo->>Social: Adds OHC link to TikTok bio
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile & Subscriptions
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to 4 lessons/mo
    OHC->>AI_Ops: Sync Calendar & Generate Zoom Links
    AI_Ops-->>Student: Sends Schedule
    OHC->>Leo: Notification "New Subscriber!" (Retention)
    Leo->>OHC: Uses Referral code to invite another tutor (Referral)
```
**Mobile UX Flow (375px):**
*   **Link-in-bio Focus:** Streamlined setup for the public portfolio page, omitting deep eCommerce configurations.

### 5. Fatima (The Food Cart Operator)
**High-Speed Ordering Sequence**
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage
    participant OHC as OHC App (Arabic/English)
    participant AI_Mark as Marketing Agent
    participant OHC_UI as Simplified Mobile UI
    participant Cust as Customer

    Fatima->>Local: Shows QR Code
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation (Photos + Prices)
    AI_Mark->>OHC: Generates Bilingual Menu
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>OHC: Scans QR, views menu, places pre-order
    OHC->>OHC_UI: Loud Audio Notification + Simple Order Card
    Fatima->>OHC_UI: Taps "Preparing"
    OHC_UI->>Cust: Updates Status
    Fatima->>OHC_UI: Prints Daily Summary (Retention)
```
**Mobile UX Flow (375px):**
*   **Accessibility:** Support for low-end Androids, multi-language UI, lightweight payloads, and offline-first drafting for areas with poor connectivity. Optimistic UI is critical here.

---

## Phase 2: Data Model Architecture

The data model must support high-concurrency agent operations, multi-tenant isolation, and fast mobile-first access patterns.

### Key Invariants & Design Features
1.  **Mandatory Tenant Scoping:** Every table must include a `tenant_id` (organization_id) column.
2.  **RLS-First Security:** All PostgreSQL queries must be executed within the `app.current_tenant` context.
3.  **Agent Isolation:** Agents only process and view tasks for their assigned tenant.
4.  **Semantic Memory:** `pgvector` empowers "AutoDream" append-only memories to provide AI agents with business context.

### Entity-Relationship Diagram
```mermaid
erDiagram
    TENANT ||--o{ USER : "has"
    TENANT ||--o{ PRODUCT : "sells"
    TENANT ||--o{ CUSTOMER : "serves"
    TENANT ||--o{ AGENT : "employs"
    TENANT ||--o{ ORDER : "receives"
    TENANT ||--o{ BOOKING : "manages"

    PRODUCT ||--o{ PRODUCT_VARIANT : "has"
    PRODUCT ||--o{ INVENTORY_LOG : "tracks"

    ORDER ||--|{ ORDER_ITEM : "contains"
    ORDER ||--|| PAYMENT : "processed_by"

    AGENT ||--o{ TASK : "claims"
    AGENT ||--o{ MEMORY : "accesses"

    TASK ||--o{ STATE_TRANSITION : "tracks"

    MEMORY {
        uuid id
        uuid tenant_id
        uuid agent_id
        vector embedding "1536 dims"
        text content
        jsonb metadata
    }

    TASK {
        uuid id
        string status "PENDING, EXECUTING, COMPLETED"
        string priority
        jsonb payload
        uuid assigned_agent_id
    }
```

### Migration Strategy
*   **Zero-Downtime Rollout:** Implement `tenant_id` across all tables. Apply RLS policies progressively in "audit" mode before strictly enforcing.
*   **Agent Data Backfill:** Initialize `MEMORY` entities for existing businesses by summarizing their historical order and product logs using backend jobs to populate `pgvector` embeddings.

---

## Phase 3: AI Integration & Department Architecture

AI departments run invisibly, interacting with the system via KAIROS.

### The 7 Departments
1.  **Operations ("The Manager")**
2.  **Marketing & Advertising ("The Promoter")**
3.  **Sales & Acquisition ("The Salesperson")**
4.  **Customer Success ("The Ambassador")**
5.  **Finance & Payments ("The Accountant")**
6.  **Legal & Compliance ("The Protector")**
7.  **Business Advisory ("The Advisor")**

### Integration Points & Handoffs
*   **Triggers:** Cron jobs, event-driven (Teammate Mesh), or on-demand user commands.
*   **1-Tap Handoff Example:** Operations marks order `SHIPPED` -> Customer Success automatically drafts thank-you email for review.
*   **Approval Levels:**
    *   *Auto-Execute:* Low-risk, reversible actions (e.g., tagging inventory).
    *   *Draft-for-Review:* High-risk, external actions requiring 1-Tap approval on the Mobile UI (e.g., publishing social posts, quotes).

### AI Resource Limits
Agents are throttled based on SaaS Tiers (e.g., Free = 100 actions/mo; Pro = Unlimited). When a user approaches limits, "The Advisor" will present plain-language, graceful upgrade suggestions via the mobile dashboard.

---
