# Issue Brief: Unified Architecture for AI Agent Departments & Business Journey

## Title
Unified Architectural Map: AI Agent Departments & End-to-End Business Journey

## Problem Statement
OneHumanCorp (OHC) is designed to empower non-technical small business owners (like Maya the Baker, Carlos the Handyman, and Fatima the Food Cart Operator) to launch and run their businesses in under 10 minutes. However, currently, the overarching business journeys—from Acquisition through Activation to Revenue and Referral—are fragmented, and the AI capabilities are not cohesively integrated into these flows. We need a unified architectural design that maps the end-to-end user journeys for these diverse personas and clearly defines how the 7 AI Agent Departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory) seamlessly and autonomously operate within these journeys to reduce operational fatigue and drive success.

## Research Report
### Context and Personas
The architecture is evaluated against core real-world personas, ensuring maximum simplicity and value:
1.  **Maya (Home Baker):** Needs mobile-first storefront, IG integration, deposit payments, and AI handling DMs.
2.  **Carlos (Handyman):** Requires service listings, booking with deposits, unified inbox, and AI quote generation.
3.  **Priya (Boutique Owner):** Wants omnichannel support, POS integration, inventory sync, and daily analytics.
4.  **Leo (Music Tutor):** Needs subscription packages, schedule syncing, auto-meeting links, and strong public profile.
5.  **Fatima (Food Cart Operator):** Prioritizes extreme simplicity, pre-orders, multi-language UI, and fast low-data mobile performance.

### Journey Stages & Friction Points
-   **Acquisition to Onboarding:** Initial setup must take <10 mins. **Friction:** Cognitive overload from requesting too much info upfront.
-   **Activation:** The "Aha!" moment (live storefront/first booking) within Day 1.
-   **Retention & Revenue:** Kept engaged via actionable notifications. **Friction:** "Financial Fog" and "Operational Fatigue" (e.g., answering repetitive DMs).

### Competitor Comparison
-   **Shopify/Wix:** High setup complexity. They rely on "chatbot" assistance rather than autonomous, event-driven background agents.
-   **Squarespace/GoDaddy:** Focus primarily on initial site generation but lack deep operational automation (e.g., automated inventory sync or unified DMs with drafted replies).

### AI Agent Departments
To solve these friction points, AI operates proactively as a "Teammate", not reactively as a "Tool".
-   **Operations ("The Manager"):** Order processing, inventory, bookings.
-   **Marketing ("The Promoter"):** Website design, SEO, social posts.
-   **Sales ("The Salesperson"):** Quotes, lead follow-up.
-   **Customer Success ("The Ambassador"):** Drafted DM replies, review requests.
-   **Finance ("The Accountant"):** Payments, subscription billing.
-   **Legal ("The Protector"):** Terms, contracts, compliance.
-   **Advisory ("The Advisor"):** Human-language daily briefings.

## Design Doc

### Key Architectural Decisions
1.  **Event-Driven Autonomous AI:** Agents are triggered by KAIROS mesh events (e.g., new order, new DM) rather than explicit user prompts.
2.  **1-Tap Approval Workflow:** High-risk actions (e.g., sending emails, publishing posts) are drafted by agents and pushed to an "Action Required" feed on the mobile dashboard for 1-tap approval.
3.  **Progressive Profiling & Setup:** The onboarding wizard requests minimal data; the Marketing Agent generates the initial storefront. Advanced configs are deferred and suggested later by the Advisory Agent.
4.  **Unified Memory (AutoDream):** All agents access `pgvector` embeddings (`autodream_memories`) for long-term context, scoped strictly by `tenant_id`.
5.  **Usage & Throttling:** AI usage is budgeted per tenant tier (e.g., Free: 100 actions, Pro: unlimited). This is enforced by a `RedisRateLimiter` integrated directly into the orchestrator before an agent task is dequeued.

### Mobile UX Flow (375px First)
1.  **Onboarding Screen:** A single, chat-like interface where the Marketing Agent asks, "What do you sell?" User types a response. No complex forms.
2.  **Loading Overlay:** "Generating your business..." with progressive glassmorphism loading indicators.
3.  **Dashboard Home:** A prioritized "Action Required" feed at the top showing agent drafts (e.g., "Review reply to Maya's DM"). Below that, a simple daily summary card.
4.  **Action Detail:** Tapping an action shows the full draft context and a large, full-width "Approve" button.

### Architecture Diagrams (Mermaid.js)

#### Overall AI Department Coordination
```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh (Hub)
    participant Op as Operations Agent
    participant CS as Customer Success Agent
    participant DB as OHC-SIP DB (Memory)
    participant UI as Mobile Dashboard

    O->>Hub: Event: New DM (e.g., "Vegan cakes?")
    Hub->>CS: Trigger: Draft Reply
    CS->>DB: Fetch Memory (Business Context)
    DB-->>CS: Context: "Sells vegan cakes"
    CS->>DB: Save Draft Action
    CS->>UI: Push Notification: "Action Required"
    UI->>O: User 1-Tap Approves
    O->>Hub: Execute Drafted Reply

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class O,Hub,Op,CS,DB,UI premium;
```

#### 1. Maya (The Home Baker) Journey
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
    Maya->>OHC: Downloads App & Opens
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: Asks "What do you sell?"
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
    OHC->>AI_Mark: Suggests "Upgrade for Custom Domain" (Revenue)
```

#### 2. Carlos (The Handyman) Journey
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
```

#### 3. Priya (The Boutique Owner) Journey
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
```

#### 4. Leo (The Music Tutor) Journey
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

#### 5. Fatima (The Food Cart Operator) Journey
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

## Implementation Prompt
**To Implementer Agent:**
Implement the unified KAIROS orchestrator flow supporting the event-driven triggers for the 7 AI Agent Departments and the mobile-first onboarding journey. Create the necessary `Action Required` feed data structures in the database to support the Draft-for-Review (1-tap approval) workflow. Build the mobile UI (optimized for 375px) that displays these pending actions clearly without technical jargon. Ensure that all background agent operations utilize the `Teammate Mesh` for coordination and respect `tenant_id` isolation. Provide robust E2E test coverage demonstrating a user moving from initial onboarding to approving an agent-drafted action.

## Priority
P0

## Estimated Scope
Large
