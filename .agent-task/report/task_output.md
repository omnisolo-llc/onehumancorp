# Issue Brief: End-to-End Business Journey Architecture

## Problem Statement
The current onboarding and lifecycle flows for non-technical users are disjointed. Users like Maya (the baker) or Carlos (the handyman) need to go from idea to a live, functional business in under 10 minutes. Any friction or technical jargon during acquisition, onboarding, activation, retention, or revenue generation leads to drop-off. We need a unified architectural design mapping out the complete end-to-end user journeys for our key personas.

## Research Report
The core personas dictate the requirements of our journey flows:
- **Maya (The Home Baker):** Mobile-only, needs a beautiful visual storefront, custom order deposit flows, and an agent for Instagram DMs.
- **Carlos (The Handyman):** Android-only, relies on word-of-mouth, needs service listings with pricing, booking calendar, and AI quote generation.
- **Priya (The Boutique Owner):** Omni-channel, needs online/in-store sync, product variants, and daily mobile analytics.
- **Leo (The Music Tutor):** Needs calendar integration, Zoom auto-generation, subscription billing, and AI follow-ups for inactive students.
- **Fatima (The Food Cart Operator):** Multi-lingual (Arabic/English), low-end Android device, needs a simple pre-order pickup flow with push notifications and printable order lists.

**Friction Points to Avoid:**
- Complex DNS configuration for custom domains.
- Dense settings panels with technical jargon (e.g., "Webhooks", "SMTP settings").
- Empty state anxiety (starting with a blank screen instead of a generated draft).
- Multi-step account creation before showing value.

**Competitor Benchmarks:**
- Shopify/Wix require 30-60 minutes and significant technical configuration to go live. OHC aims for under 10 minutes with zero technical knowledge required.

## Design Doc

### User Journey Maps

#### 1. Acquisition & Onboarding
- **Trigger:** Organic search, social media ad, or referral link.
- **CTA:** "Launch your business in 10 minutes."
- **Flow:** User enters a 1-paragraph description of their business -> *The Advisor* agent extracts name, type, and drafts a tagline -> *The Promoter* agent selects a template and drafts the first product/service -> A Live Preview is generated -> User clicks "Launch" -> User creates an account.

#### 2. Activation
- **Success Criteria:** First product added, first payment link generated, or first booking received.
- **Action:** User shares their link-in-bio or storefront URL. *The Promoter* agent automatically drafts a social media post announcing the launch.

#### 3. Retention & Revenue
- **Daily Engagement:** *The Advisor* agent sends daily/weekly health reports (e.g., "Tuesday is your busiest day. The vegan cake is trending.").
- **Upsell Trigger:** When the user hits the limit of the Free tier (e.g., 100 AI actions), the system presents a clear, value-based upgrade prompt for the Starter tier.

#### 4. Referral
- **Viral Loop:** Satisfied users can share an affiliate link from their dashboard.

### Architecture Diagrams (Mermaid.js)

#### Persona: Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Marketing as Marketing & Advertising (Promoter)
    participant OHC as OHC Platform (Mobile App)
    participant CS as Customer Success (Ambassador)
    participant Customer

    Maya->>OHC: Submits 1-paragraph bio "I bake custom vegan cakes in Seattle"
    OHC->>Marketing: Trigger: Instant Storefront Generation
    Marketing-->>OHC: Returns generated template, bio, and sample products
    OHC-->>Maya: Displays Live Preview
    Maya->>OHC: Clicks "Launch"
    Marketing->>OHC: Auto-posts announcement to Instagram
    Customer->>OHC: DMs Maya on Instagram "Do you do gluten-free?"
    OHC->>CS: Trigger: Incoming Inquiry
    CS-->>OHC: Drafts reply "Yes, we do! Here is the link to order."
    OHC-->>Maya: Notification: "Approve reply to customer?"
    Maya->>OHC: 1-Tap Approve
    OHC->>Customer: Sends reply
```

#### Persona: Carlos (The Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant OHC as OHC Platform (Android App)
    participant Sales as Sales & Acquisition (Salesperson)
    participant Ops as Operations (Manager)
    participant Client

    Carlos->>OHC: Completes Onboarding (Service Listing & Calendar)
    Client->>Carlos: Calls/Texts asking for a quote on a plumbing fix
    Carlos->>OHC: Enters basic details of the request
    OHC->>Sales: Trigger: Generate Quote
    Sales-->>OHC: Drafts professional quote with deposit link
    OHC-->>Client: Sends Quote (SMS/Email)
    Client->>OHC: Approves and pays deposit
    OHC->>Ops: Trigger: Payment Received
    Ops-->>OHC: Blocks time on Carlos's Calendar
    OHC-->>Carlos: Push Notification: "New Job Booked! Deposit Paid."
```

#### Persona: Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant OHC as OHC Platform (Mobile & Web App)
    participant Ops as Operations (Manager)
    participant Finance as Finance (Accountant)
    participant Advisory as Business Advisory
    participant Customer

    Priya->>OHC: Adds new clothing line to online store
    OHC->>Ops: Trigger: Inventory Sync
    Ops-->>OHC: Syncs online inventory with in-store POS
    Customer->>OHC: Purchases dress online
    OHC->>Finance: Processes payment & updates revenue dashboard
    OHC->>Ops: Decrements inventory for dress (Size M)
    Advisory->>Priya: Weekly Report: "Dress is trending. Reorder soon."
```

#### Persona: Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant OHC as OHC Platform (Mobile App)
    participant Ops as Operations (Manager)
    participant Sales as Sales & Acquisition
    participant Student

    Leo->>OHC: Sets up subscription packages & calendar
    Student->>OHC: Books a monthly package & pays
    OHC->>Ops: Trigger: Lesson Booked
    Ops-->>Student: Sends Zoom link & calendar invite
    Ops-->>Leo: Adds lesson to calendar
    Student->>OHC: Doesn't book for 3 weeks
    OHC->>Sales: Trigger: Inactive Student
    Sales-->>Leo: Notification: "Drafted follow-up for Student. Approve?"
    Leo->>OHC: 1-Tap Approve
    OHC->>Student: Sends re-engagement email
```

#### Persona: Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant OHC as OHC Platform (Low-End Android)
    participant Ops as Operations (Manager)
    participant Customer

    Fatima->>OHC: Marks "Chicken Over Rice" as available in Arabic UI
    Customer->>OHC: Pre-orders and pays online
    OHC->>Ops: Trigger: Order Received
    Ops-->>Fatima: Push notification (loud ping) & adds to printable list
    Fatima->>OHC: Marks order "Ready for Pickup"
    OHC->>Customer: SMS: "Your order is ready!"
    Customer->>Fatima: Picks up food
    actor Maya
    participant Marketing as Marketing & Advertising (Promoter)
    participant OHC as OHC Platform (Mobile App)
    participant CS as Customer Success (Ambassador)
    participant Customer

    Maya->>OHC: Submits 1-paragraph bio "I bake custom vegan cakes in Seattle"
    OHC->>Marketing: Trigger: Instant Storefront Generation
    Marketing-->>OHC: Returns generated template, bio, and sample products
    OHC-->>Maya: Displays Live Preview
    Maya->>OHC: Clicks "Launch"
    Marketing->>OHC: Auto-posts announcement to Instagram
    Customer->>OHC: DMs Maya on Instagram "Do you do gluten-free?"
    OHC->>CS: Trigger: Incoming Inquiry
    CS-->>OHC: Drafts reply "Yes, we do! Here is the link to order."
    OHC-->>Maya: Notification: "Approve reply to customer?"
    Maya->>OHC: 1-Tap Approve
    OHC->>Customer: Sends reply
```

#### Persona: Carlos (The Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant OHC as OHC Platform (Android App)
    participant Sales as Sales & Acquisition (Salesperson)
    participant Ops as Operations (Manager)
    participant Client

    Carlos->>OHC: Completes Onboarding (Service Listing & Calendar)
    Client->>Carlos: Calls/Texts asking for a quote on a plumbing fix
    Carlos->>OHC: Enters basic details of the request
    OHC->>Sales: Trigger: Generate Quote
    Sales-->>OHC: Drafts professional quote with deposit link
    OHC-->>Client: Sends Quote (SMS/Email)
    Client->>OHC: Approves and pays deposit
    OHC->>Ops: Trigger: Payment Received
    Ops-->>OHC: Blocks time on Carlos's Calendar
    OHC-->>Carlos: Push Notification: "New Job Booked! Deposit Paid."
```

### Mobile UX Flows
- **Zero-State to Live:** Single text box input for business description -> Loading spinner with AI agent activity text ("Designing your storefront...", "Writing product descriptions...") -> Live preview screen -> "Publish" button.
- **1-Tap Approvals:** Push notification for an agent-drafted action -> Tapping opens a bottom sheet with the drafted content and an "Approve" / "Reject" / "Edit" button.

### Key Architectural Decisions
- **AI as the Onboarding Engine:** Instead of forms, onboarding is driven by a single unstructured text input processed by agents to pre-fill the entire tenant configuration.
- **Progressive Profiling:** We only ask for essential information to launch. Additional details (tax info, specific shipping zones) are gathered progressively as the business grows.

## Implementation Prompt
Implement the End-to-End Business Journey orchestration logic. This involves replacing the current multi-step onboarding wizard with a single-prompt "Instant Build" flow that leverages the existing Agent infrastructure to extrapolate tenant configuration, generate a storefront, and draft initial content. Furthermore, integrate the retention loop by configuring `The Advisor` agent to schedule weekly plain-language health reports via the internal task queue. Ensure the entire flow is testable via a mobile-first UI layout (375px width minimum) and handles failures gracefully with optimistic UI updates.

## Priority
P1

## Estimated Scope
Large
