# Title
Architectural Map: End-to-End Business Journey for OHC Personas

## Problem Statement
The OneHumanCorp (OHC) platform aims to empower a diverse range of non-technical small business owners (e.g., Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, Fatima the Food Cart Operator) to launch and run their businesses entirely from their mobile devices. Currently, the overarching journeys from user acquisition through activation, retention, revenue generation, and referral are fragmented. There is no unified architectural map that guarantees users can achieve a live, functioning business in under 10 minutes without hitting cognitive overload or technical friction. A cohesive architectural vision of the business journey is required to ensure all platform features and AI agents operate synergistically toward owner success.

## Research Report
### Persona Context & Findings
1.  **Maya (Baker, 28)**: Needs seamless Instagram integration, a visual storefront for custom cakes, and AI-driven direct message handling.
2.  **Carlos (Handyman, 42)**: Operates strictly via an Android device. Needs service listings, simple booking/calendar, deposits, and AI-generated quotes.
3.  **Priya (Boutique Owner, 35)**: Requires omnichannel capability (online + in-store POS), inventory sync, and actionable daily analytics.
4.  **Leo (Music Tutor, 22)**: Needs subscription-based packages, auto-generated meeting links, and a strong TikTok link-in-bio presence.
5.  **Fatima (Food Cart Operator, 50)**: Needs a multi-language, ultra-simple interface for pre-orders and pickup management on a low-end Android device.

### Journey Stages
- **Acquisition**: Users typically discover OHC via Instagram/TikTok ads, word of mouth, or search. The CTA must promise "Live business in 5 mins."
- **Onboarding**: Must be a highly guided, AI-driven chat/wizard flow. Minimum inputs required: Business name, category, and primary goal (e.g., sell products vs. book services). Defer complex setup.
- **Activation**: The true "Aha!" moment. Achieved when the first product is added, the storefront is live, or the first payment is received (must occur by Day 1).
- **Retention**: Maintained via push notifications for real-time events (e.g., new order alerts) and weekly plain-language AI health reports.
- **Revenue**: Triggered when a user outgrows the Free tier (e.g., needs a custom domain, exceeds product limits). Upgrades must be presented contextually as business growth milestones, not hard errors.
- **Referral**: Driven by a viral loop (e.g., built-in referral codes, "Built with OHC" links in footers, rewards for successful referrals).

### Competitive Analysis
- **Shopify/Wix**: Overwhelming onboarding with complex terminology (DNS, shipping zones).
- **OHC Advantage**: Mobile-first, zero-configuration setup with background AI handling all technical lifting.

### Identified Friction Points
1. **Cognitive Overload**: Too many upfront form fields cause abandonment.
2. **Technical Jargon**: Terms like "SSL" or "Payment Gateways" deter non-technical users.
3. **Inventory Sync**: Difficulty mapping real-world physical availability to the digital system without intuitive design.
4. **Language Barriers**: Assuming high English proficiency and technical literacy alienates users like Fatima.

## Design Doc
### Key Design Decisions
- **Progressive Profiling**: Request only critical info upfront. Advanced settings are dynamically suggested by the Business Advisory Agent post-activation.
- **AI-First Setup**: The Marketing & Advertising Agent drives the initial layout and copy generation based on simple prompts.
- **Mobile-First UX**: All flows are designed strictly for a 375px viewport. Desktop views are progressive enhancements.
- **Optimistic UI & Async Processing**: Background agents handle tasks (like domain provisioning) asynchronously to keep the UI instantly responsive.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant IG as Instagram Ad
    participant App as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Cust as Customer

    Maya->>IG: Clicks "Launch Bakery in 5 mins"
    Maya->>App: Downloads & Opens App
    App->>AI_Mark: Trigger AI Onboarding
    AI_Mark->>Maya: Asks "What do you sell?"
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>App: Generates Storefront & Menu
    App->>Maya: Storefront Live! (Activation)
    Cust->>App: DM via IG Integration "Are these vegan?"
    App->>AI_Ops: Drafts Reply
    AI_Ops-->>Cust: "Yes, fully vegan!"
    Cust->>App: Places Order & Pays Deposit
    App->>Maya: Push Notification "New Order Paid" (Retention)
    Maya->>App: Hits Free limit, upgrades to Starter (Revenue)
    Maya->>IG: Shares Referral Code (Referral)
```

#### 2. Carlos (The Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant App as OHC Android App
    participant AI_Sales as Sales Agent
    participant Cust as Customer

    Carlos->>WoM: Hears about OHC
    Carlos->>App: Installs Android App
    App->>Carlos: Minimal setup (Services & Rates)
    App->>Carlos: Booking Page Live! (Activation)
    Cust->>App: Requests quote for "Leaky Pipe"
    App->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: 1-Tap Approves
    AI_Sales-->>Cust: Sends Official Quote
    Cust->>App: Books Time & Pays Deposit
    App->>Carlos: Notification "Job Booked" (Retention)
    Carlos->>Cust: Taps "Send 10% Discount to Friend" (Referral)
```

#### 3. Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant App as OHC App
    participant AI_Adv as Advisory Agent
    participant POS as In-Store POS

    Priya->>Search: Searches "Easy online store"
    Priya->>App: Signs up
    App->>Priya: Guided inventory sync
    App->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale via phone
    POS->>App: Update Inventory
    App->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: "Inventory low. Upgrade for automated re-orders." (Revenue)
    Priya->>App: Selects Pro Plan (Revenue)
```

#### 4. Leo (The Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok
    participant App as OHC App
    participant AI_Ops as Operations Agent
    participant Student as Student

    Leo->>Social: Adds OHC link to bio
    Leo->>App: Configures lesson packages
    App->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks link
    Student->>App: Subscribes to 4 lessons/mo
    App->>AI_Ops: Sync Calendar & Generate Links
    AI_Ops-->>Student: Sends Schedule
    App->>Leo: Notification "New Subscriber!" (Retention)
    Leo->>App: Shares platform with another tutor (Referral)
```

#### 5. Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Signage
    participant App as OHC App (Bilingual)
    participant AI_Mark as Marketing Agent
    participant Cust as Customer

    Fatima->>Local: Displays QR Code
    Fatima->>App: Opens App (Arabic UI)
    App->>AI_Mark: Fast photo-based menu creation
    AI_Mark->>App: Generates Menu
    App->>Fatima: Menu Live! (Activation)
    Cust->>App: Scans QR, places pre-order
    App->>Fatima: Loud Audio Ping & Visual Alert (Retention)
    Fatima->>App: Prints Daily Order Summary
```

### UI Wireframes & Screen Flow (375px)
1. **Welcome Screen**: Single CTA ("Start My Business").
2. **AI Chat Wizard**: Chat interface. "Hi, I'm your Marketing Agent. What's the name of your business?"
3. **Generation Screen**: Glassmorphism shimmer loading state ("Building your storefront...").
4. **Dashboard**: Bottom navigation. Large 44x44px touch targets. Key metrics (Orders, Messages) prominently displayed.

### Mobile UX Flow
- Native keyboard mapping (numeric for pricing, email for contacts).
- Optimistic updates for all actions (e.g., toggling an item to "Sold Out" immediately updates the local UI while background syncs happen).

### AI Agent Integration Points
- **Marketing & Advertising**: Drives onboarding layout and auto-creates content blocks.
- **Operations & Sales**: Drafts customer replies and quotes, pushing them to the dashboard for 1-tap approval.
- **Business Advisory**: Monitors usage and contextually suggests revenue tier upgrades.

## Implementation Prompt
Implement the foundational unified user onboarding flow and dashboard state management supporting the transition from Acquisition to Activation.
1. Build the mobile-first (375px) AI-driven onboarding wizard UI.
2. Implement progressive profiling so users provide minimal data upfront, deferring complex settings.
3. Ensure the final step generates a live storefront/booking page.
4. Integrate the initial AI Agent "1-Tap Approval" interface for the Operations agent on the dashboard.
Acceptance Criteria: A non-technical user can complete the signup flow and view their generated storefront within 10 minutes. The UI must use premium design tokens and optimistic updates.

## Priority
P0

## Estimated Scope
Large
