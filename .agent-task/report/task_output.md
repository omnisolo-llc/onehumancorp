# [architecture] Business Journey Architecture

## Title
Architectural Mapping of the End-to-End Business Journey for OHC Personas

## Problem Statement
The OHC platform must serve a diverse set of real-world small business owners (Maya, Carlos, Priya, Leo, and Fatima) who share a common goal: launching and growing their business entirely from a mobile device without technical expertise. Currently, the overarching business journeys—from initial acquisition to sustainable revenue generation and referral loops—are fragmented. We need a unified architectural map of the end-to-end user journeys for these personas to ensure that the system naturally supports their progression through Acquisition, Onboarding, Activation, Retention, Revenue generation, and Referral, while identifying critical friction points where non-technical users might abandon the platform.

## Research Report
### Context and Competitive Analysis
Small business owners often churn on platforms like Shopify, Wix, or Squarespace because the acquisition promise ("build a site in minutes") conflicts with the onboarding reality (complex dashboards, technical jargon like CNAME, Liquid templates).
- **Shopify:** High friction onboarding requiring extensive configuration before accepting the first payment. Desktop-primary dashboard.
- **Wix/Squarespace:** Complex drag-and-drop interfaces that are difficult to use on mobile devices.
- **OHC Advantage:** OHC provides an AI-driven, mobile-first, 10-minute zero-to-live experience that defers complex configuration until after the first "Aha!" moment (Activation).

### Journey Stages
1. **Acquisition:** The entry point. Organic search, social media ads (Instagram/TikTok), or word-of-mouth. The call-to-action (CTA) must clearly promise a functional business setup in under 10 minutes without code.
2. **Onboarding:** A highly guided, AI-driven wizard flow. Crucial to minimize initial input; deferring advanced configurations (like custom domains) to a later stage. Minimum inputs to go live: Business Name, What they sell, and a rough description.
3. **Activation:** The "Aha!" moment. A live storefront, the first booking, or the first payment. Must be achieved within Day 1. Success by Week 1: First customer interaction. Success by Month 1: Consistent sales or bookings.
4. **Retention:** Kept engaged through actionable notifications (e.g., new order alerts) and AI-generated weekly health reports from the Business Advisory Agent.
5. **Revenue:** Transitioning from a free tier to a paid plan. Triggered by hitting specific milestones (e.g., reaching product/action limits, needing custom domains). Upgrade CTA presented contextually (e.g., when adding the 11th product on a free plan).
6. **Referral:** Incentivized sharing. Creating a viral loop through referral discounts and shareable success metrics ("Maya made $500 this week on OHC, start your bakery today").

### Identified Friction Points
1. **Cognitive Overload during Onboarding:** Requesting too much setup information upfront causes drop-offs.
2. **Payment Gateway Integration:** Technical jargon during Stripe connection can stall progress.
3. **Inventory/Calendar Sync:** Difficulties mapping real-world availability to digital systems.
4. **Language Barriers:** Interfaces that assume high technical literacy or English fluency (e.g., for Fatima).

## Design Doc
### Key Design Decisions
- **Progressive Disclosure:** The onboarding flow requests the absolute minimum data to generate a viable starting point. Advanced settings are dynamically suggested by the Business Advisory Agent post-activation.
- **AI-First Setup:** The Marketing & Advertising Agent acts as the primary onboarding guide, generating the initial website layout and copy based on a simple prompt.
- **Mobile Parity:** All journey flows are designed and tested starting at the 375px breakpoint. Desktop is additive.
- **Business Owner Lens:** Non-technical terminology (e.g., "Connect your bank" instead of "Configure Stripe Webhooks").

### Mobile UX Flow (375px First)
1. **Landing (Acquisition):** Single clear input field: "What kind of business do you run?" -> "Start Free" CTA.
2. **Wizard (Onboarding):** Chat-like interface with the Marketing Agent. "Upload a few photos of your work" -> "Generating your storefront..."
3. **Live View (Activation):** Confetti animation. "Your store is live at maya-bakes.ohc.app!" with a prominent "Share on Instagram" button.
4. **Dashboard (Retention):** Simple activity feed. "New message from customer", "Weekly sales report ready".

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):** Handles initial onboarding, store generation, and referral campaign creation.
- **The Salesperson (Sales & Acquisition):** Generates quotes and follows up on leads.
- **The Ambassador (Customer Success):** Drafts replies to customer messages.
- **The Manager (Operations):** Processes orders and updates inventory.
- **The Advisor (Business Advisory):** Contextually prompts revenue upgrades (e.g., "You're getting lots of traffic, time for a custom domain?").

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Ops as The Manager
    participant AI_Adv as The Advisor
    participant Cust as Customer

    Maya->>Ad: Clicks "Launch Bakery in 10 mins" (Acquisition)
    Maya->>OHC: Downloads App & Opens
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: "What do you sell?"
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>OHC: Generates Storefront & Menu
    OHC->>Maya: Storefront Live! (Activation)
    Cust->>OHC: Messages via IG DM "Vegan options?"
    OHC->>AI_Ops: Drafts Reply
    AI_Ops-->>Cust: "Yes, we do vegan cakes!"
    Cust->>OHC: Places Order & Pays Deposit
    OHC->>Maya: Push Notification "New Order Paid!" (Retention)
    AI_Adv->>Maya: "You've hit 10 products! Upgrade to Starter for unlimited listings." (Revenue)
    Maya->>OHC: Upgrades to Starter Plan
    Maya->>OHC: Shares Store Link with Friend (Referral)
    OHC->>Maya: Bakery Credit applied for Referral
```

#### 2. Carlos (The Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant OHC as OHC Web App
    participant AI_Mark as The Promoter
    participant AI_Sales as The Salesperson
    participant Cust as Customer

    Carlos->>WoM: Hears about OHC (Acquisition)
    Carlos->>OHC: Visits website on Android
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Carlos: "What services do you offer?"
    Carlos->>AI_Mark: "Plumbing, Painting"
    AI_Mark->>OHC: Generates Service Listings & Booking Calendar
    OHC->>Carlos: Booking Page Live! (Activation)
    Cust->>OHC: Requests Quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: Approves with 1-tap (Retention)
    AI_Sales-->>Cust: Sends Official Quote
    Cust->>OHC: Books Time & Pays Deposit
    Carlos->>OHC: Taps "Earn $50: Refer a Pro" (Referral)
    Carlos->>Cust: Sends 10% Discount Link to Friend (Viral Loop)
```

#### 3. Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Adv as The Advisor
    participant POS as In-Store POS

    Priya->>Search: Searches "Easy online store" (Acquisition)
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Priya: Syncs initial inventory
    AI_Mark->>OHC: Generates Storefront with variants
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale via phone
    POS->>OHC: Update Inventory (Retention)
    OHC->>Priya: Daily Analytics Report via Push
    AI_Adv->>Priya: "Inventory low. Upgrade for automated re-order alerts." (Revenue)
    Priya->>OHC: Selects "Pro Plan" (Revenue)
    OHC->>Priya: Enables Multi-Store Sync
```

#### 4. Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Ops as The Manager
    participant Student as Student

    Leo->>Social: Adds OHC link to TikTok bio (Acquisition)
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile & Subscriptions
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to 4 lessons/mo
    OHC->>AI_Ops: Sync Calendar & Generate Zoom Links
    AI_Ops-->>Student: Sends Schedule (Retention)
    OHC->>Leo: Notification "New Subscriber!"
    Leo->>OHC: Uses Referral code to invite another tutor (Referral)
```

#### 5. Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant OHC_UI as Simplified Mobile UI
    participant Cust as Customer

    Fatima->>Local: Shows QR Code (Acquisition)
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation (Photos + Prices)
    AI_Mark->>OHC: Generates Bilingual Menu (Arabic/English)
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>OHC: Scans QR, views menu, places pre-order
    OHC->>OHC_UI: Loud Audio Notification + Simple Order Card (Retention)
    Fatima->>OHC_UI: Taps "Preparing"
    OHC_UI->>Cust: Updates Status
    Fatima->>OHC_UI: Prints Daily Summary
```

## Implementation Prompt
**To Implementer Agent:**
Implement the foundational user onboarding flow and dashboard state management that supports the progression from Acquisition to Activation. The system should define the required data models to capture the user's business type and minimal initial configuration. Build the mobile-first (375px) UI wizard that guides a user through the initial setup, ensuring that advanced configurations are deferred (Progressive Disclosure). The final step of the wizard should instantly generate a functional "Storefront/Booking Page" view, satisfying the 'Activation' milestone. Ensure that interactions feel premium (Glassmorphism, correct typography) and are resilient to network issues (optimistic updates). Do not prescribe specific database schemas or backend routing; focus on the unified API contract and the user journey transitions. Include E2E test coverage verifying a successful run-through from login to the generated storefront.

## Priority
P0

## Estimated Scope
Large
