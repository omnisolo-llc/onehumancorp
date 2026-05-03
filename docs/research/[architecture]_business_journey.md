# Title: Complete End-to-End User Journey Architecture for OHC

## Problem Statement
Small business owners (our personas: Maya, Carlos, Priya, Leo, Fatima) have vastly different business models, yet they all need a simple, intuitive, and frictionless path from discovery to becoming active, revenue-generating users on OneHumanCorp (OHC). Currently, we lack a unified architectural map that details the exact lifecycle stages—Acquisition, Onboarding, Activation, Retention, Revenue, and Referral—for each of these distinct personas. Without this, we risk creating fragmented user experiences or building features that do not align with how non-technical users actually discover and use software to run their businesses.

## Research Report
The success of platforms like Shopify and Wix lies in their ability to abstract complex workflows into simple steps. However, OHC's target demographic is even less technical ("zero technical knowledge needed") and heavily mobile-first.

### Persona Analysis
1.  **Maya (Home Baker, 28):** Driven by social media. Needs seamless Instagram integration.
2.  **Carlos (Freelance Handyman, 42):** Driven by local word-of-mouth. Needs simple booking and invoicing.
3.  **Priya (Boutique Owner, 35):** Needs an omnichannel solution syncing physical store inventory with an online presence.
4.  **Leo (Music Tutor, 22):** Digital-native, needs scheduling, subscription management, and social media tools.
5.  **Fatima (Food Cart, 50):** Requires extreme simplicity, multi-language support, and clear notification flows for orders.

### Competitive Gap
Competitors often provide generic onboarding (e.g., "What is your industry?") but fail to tailor the *entire* journey to the specific business model. OHC must proactively configure the AI agents and platform features based on the initial onboarding signal.

## Design Doc

### Key Design Decisions
1.  **Mobile-First Journey:** The entire journey, from signup to configuring complex features like subscriptions or booking systems, must be executable on a 375px mobile screen.
2.  **Progressive Profiling:** We collect only essential information during onboarding (Name, Business Type, Contact). The rest is collected gradually through "Next Best Action" prompts driven by the Business Advisory AI.
3.  **AI-Driven Setup:** Instead of making users build a site or configure products manually, the "Marketing & Advertising" agent generates a starting point based on their business type and a brief description.

### Lifecycle Stages Map

#### 1. Acquisition
*   **Maya:** Sees a TikTok ad showing another baker easily managing orders. Clicks link to download app.
*   **Carlos:** Hears about OHC from another contractor. Searches Google, lands on SEO-optimized landing page highlighting "Get paid faster."
*   **Priya:** Reads a blog post about syncing physical and online stores. Clicks through to OHC site.
*   **Leo:** Sees an Instagram reel about a "link-in-bio on steroids."
*   **Fatima:** A younger relative downloads the app for her after hearing it supports Arabic.

#### 2. Onboarding (Zero to Live in < 10 mins)
*   **Universal Flow:** Enter Phone Number -> OTP -> "What do you do?" (Select Category) -> "What's your business name?" -> AI Agent generates initial storefront and operational setup.
*   **Maya:** AI generates a visual gallery template for cakes. Prompted to connect Stripe to accept custom order deposits.
*   **Carlos:** AI generates a service listing template. Prompted to set available hours for bookings.
*   **Priya:** AI generates a retail catalog template. Prompted to add first product variant.
*   **Leo:** AI generates a portfolio/booking template. Prompted to connect Google Calendar.
*   **Fatima:** AI generates a menu template (Arabic/English). Prompted to enable push notifications for orders.

#### 3. Activation (The "Aha!" Moment)
*   **Maya:** Receives her first deposit for a custom cake order via an Instagram DM handled by the AI agent.
*   **Carlos:** A client books a plumbing fix and pays the deposit online.
*   **Priya:** Syncs her first in-store sale with her online inventory via the OHC POS.
*   **Leo:** A student books a recurring weekly lesson package.
*   **Fatima:** Her phone pings with her first pickup pre-order.

#### 4. Retention (Daily Habit)
*   **Maya:** Checks the app daily to manage cake delivery dates and respond to complex DMs escalated by the AI.
*   **Carlos:** Uses the app to send quotes after inspecting jobs and checks his daily schedule.
*   **Priya:** Reviews daily analytics (sales trends, inventory alerts).
*   **Leo:** Manages student communications and checks his upcoming Zoom links.
*   **Fatima:** Keeps the app open during service hours to manage incoming orders and toggle sold-out items.

#### 5. Revenue (Monetization Trigger)
*   **Trigger:** Users upgrade from Free to Starter ($9/mo) when they hit limits or need premium features.
*   **Maya:** Upgrades to get a custom domain (`mayascakes.com`) to look more professional.
*   **Carlos:** Upgrades to access advanced quote generation and follow-up sequences.
*   **Priya:** Upgrades to Pro to support unlimited products and in-person tap-to-pay.
*   **Leo:** Upgrades to enable subscription billing for his students.
*   **Fatima:** Upgrades to support higher order volumes and custom SMS notifications.

#### 6. Referral
*   **Mechanism:** Built-in incentivized referral program managed by the "Sales & Acquisition" agent.
*   **Scenario:** Priya loves the inventory sync and shares a referral link with another boutique owner. Both get a month of the Pro tier free.

### Friction Points & Abandonment Risks
1.  **Stripe/Payment Connection:** Often requires SSN/EIN or complex verification. If this interrupts the "10 minutes to live" flow, users will bounce. *Mitigation:* Allow "cash on delivery/pickup" or standard bank transfer initially, deferring full Stripe setup until the first actual online payment is required.
2.  **Initial Catalog/Service Entry:** Manually typing out 20 services or products on a mobile keyboard is tedious. *Mitigation:* Use AI to scrape an existing Instagram page or let the user take a single photo of a paper menu to auto-generate the catalog.
3.  **Domain Setup:** Non-technical users struggle with DNS records. *Mitigation:* We handle DNS entirely on our end. Users only ever see "Pick your domain name" within OHC.
4.  **AI Trust:** Users might fear the AI will send a wrong or inappropriate message on their behalf. *Mitigation:* AI always drafts messages for review ("Approval Mode") until the user explicitly toggles "Auto-Send" after gaining confidence.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) Full Journey
```mermaid
sequenceDiagram
    participant Maya
    participant App as OHC App
    participant AI_Marketing as Marketing Agent
    participant AI_Success as Customer Success Agent
    participant AI_Sales as Sales Agent

    Note over Maya, AI_Sales: Acquisition & Onboarding
    Maya->>App: Downloads app from TikTok link
    App->>Maya: OTP Login
    Maya->>App: Selects "Baked Goods", Enters Name
    App->>AI_Marketing: Generate Storefront
    AI_Marketing-->>App: Visual cake gallery template ready
    App->>Maya: "Connect Stripe for custom order deposits"
    Maya->>App: Connects Stripe

    Note over Maya, AI_Sales: Activation & Retention
    Customer->>AI_Success: Instagram DM: "Need vegan cake for Saturday"
    AI_Success->>Maya: Drafts reply with quote & deposit link
    Maya->>App: Approves Draft
    App->>Customer: Sends Reply
    Customer->>App: Pays Deposit
    App->>Maya: Ping! "New Cake Order" (Activation)

    Note over Maya, AI_Sales: Revenue & Referral
    App->>Maya: "Want your own domain?"
    Maya->>App: Upgrades to Starter ($9/mo)
    Maya->>AI_Sales: Sends referral link to friend
```

#### 2. Carlos (The Handyman) Full Journey
```mermaid
sequenceDiagram
    participant Carlos
    participant App as OHC App
    participant AI_Ops as Ops Agent
    participant AI_Finance as Finance Agent

    Note over Carlos, AI_Finance: Acquisition & Onboarding
    Carlos->>App: Finds via Google Search "Invoicing App"
    App->>Carlos: OTP Login
    Carlos->>App: Selects "Home Services"
    App->>AI_Ops: Generate Service Listings & Calendar
    AI_Ops-->>App: Plumbing/Repair templates ready
    App->>Carlos: "Set your working hours"
    Carlos->>App: Sets M-F 8am-5pm

    Note over Carlos, AI_Finance: Activation & Retention
    Client->>App: Books "Plumbing Assessment" & Pays Deposit
    App->>Carlos: Ping! "New Booking for Tuesday" (Activation)
    Carlos->>App: Performs assessment, requests full quote
    App->>AI_Finance: Generate Quote based on assessment notes
    AI_Finance-->>Client: Sends detailed quote for approval
    Client->>App: Approves & Pays Balance

    Note over Carlos, AI_Finance: Revenue
    App->>Carlos: "You've sent 10 quotes this month. Upgrade for unlimited."
    Carlos->>App: Upgrades to Pro ($29/mo)
```

#### 3. Priya (The Boutique Owner) Full Journey
```mermaid
sequenceDiagram
    participant Priya
    participant App as OHC App
    participant POS as OHC POS (In-Store)
    participant AI_Advise as Advisory Agent

    Note over Priya, AI_Advise: Acquisition & Onboarding
    Priya->>App: Direct traffic (Blog referral)
    App->>Priya: OTP Login
    Priya->>App: Selects "Retail/Boutique"
    App->>App: Setup Omni-Channel Inventory
    Priya->>App: Adds first product (Size/Color variants)

    Note over Priya, AI_Advise: Activation & Retention
    Customer->>POS: Buys item in-store via Tap-to-Pay
    POS->>App: Syncs inventory (Activation)
    App->>Priya: "Inventory for Red Dress is low"

    Note over Priya, AI_Advise: Revenue & Referral
    AI_Advise->>Priya: "Weekly Report: Red dresses are trending. Order more!"
    Priya->>App: Upgrades to Pro to support multi-location inventory
    Priya->>App: Shares referral link on Boutique Owners Facebook Group
```

#### 4. Leo (The Music Tutor) Full Journey
```mermaid
sequenceDiagram
    participant Leo
    participant App as OHC App
    participant AI_Success as Customer Success Agent

    Note over Leo, AI_Success: Acquisition & Onboarding
    Leo->>App: Clicks Instagram Reel
    App->>Leo: OTP Login
    Leo->>App: Selects "Tutoring"
    App->>App: Generate Booking Page + Portfolio
    App->>Leo: "Connect Google Calendar"
    Leo->>App: Syncs Calendar

    Note over Leo, AI_Success: Activation & Retention
    Student->>App: Books recurring weekly lesson
    App->>Leo: Ping! "New Recurring Student" (Activation)
    App->>Student: Auto-sends Zoom link for first session

    Note over Leo, AI_Success: Revenue
    AI_Success->>Leo: "Student X hasn't booked in 3 weeks. Send follow-up?"
    Leo->>App: Upgrades to Business tier for unlimited subscription billing
```

#### 5. Fatima (The Food Cart) Full Journey
```mermaid
sequenceDiagram
    participant Fatima
    participant App as OHC App
    participant AI_Marketing as Marketing Agent

    Note over Fatima, AI_Marketing: Acquisition & Onboarding
    Fatima->>App: App installed by relative
    App->>Fatima: OTP Login (Arabic UI selected)
    Fatima->>App: Selects "Food & Beverage"
    App->>AI_Marketing: Generate Menu Template
    AI_Marketing-->>App: Menu ready for items
    Fatima->>App: Takes photo of physical menu
    AI_Marketing->>App: Auto-populates digital menu

    Note over Fatima, AI_Marketing: Activation & Retention
    Customer->>App: Scans QR code, orders 2 Falafels for pickup
    App->>Fatima: Loud Audio Ping! "New Order" (Activation)
    Fatima->>App: Marks order as "Ready"
    App->>Customer: SMS "Your order is ready!"

    Note over Fatima, AI_Marketing: Revenue
    App->>Fatima: Reaches monthly order limit
    Fatima->>App: Upgrades to Starter for higher volume
```

## Implementation Prompt
**Task:** Build the core user onboarding flow and database schema to support the Business Journey Architecture.
**CUJ:** A new user downloads the app, creates an account using phone OTP, selects their business type, and lands on an AI-generated dashboard tailored to their business model.
**Acceptance Criteria:**
1.  Implement a robust authentication flow (e.g., using existing auth mechanisms) tailored for quick mobile entry.
2.  Create database tables/structures to store user profiles, business metadata, and onboarding state.
3.  Implement an API endpoint that takes the selected business type and orchestrates a call to the AI agent service to generate the initial tenant configuration (storefront layout, enabled features).
4.  Ensure all UI elements match the required design tokens (mobile-first, 375px base, touch targets >= 44px).
5.  Include E2E tests validating the full onboarding flow for at least two distinct persona types (e.g., a service business and a product business).

## Priority
P0

## Estimated Scope
Large
