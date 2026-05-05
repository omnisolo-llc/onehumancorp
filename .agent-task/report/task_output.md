# [architecture]_business_journey.md

## Title
Business Journey Architecture: End-to-End Flows for Core Personas

## Problem Statement
Non-technical small business owners face significant friction when trying to launch their business online. Current onboarding and management tools (like Shopify, Wix, Calendly) are overly complex and disjointed, requiring manual setup of storefronts, payment gateways, and booking systems. Our users (Maya, Carlos, Priya, Leo, Fatima) need a guided, under-10-minute journey that invisibly utilizes AI agents to set up their specific business types and handle ongoing operations across various channels.

## Research Report
- **Competitive Analysis**: Shopify takes 30-60 minutes for basic setup. Wix and Squarespace demand design effort. GoDaddy lacks comprehensive booking/pos tools. None offer built-in AI agents that actively manage operations like customer replies or schedule coordination.
- **Data**: Our personas primarily use mobile devices (especially smartphones) for business management. Customer interactions happen across diverse channels (Instagram DMs, email, SMS, in-person).
- **Proposed Solution**: Frictionless, mobile-first setup wizards driven by a conversational AI agent ("The Promoter"), combined with background agents ("The Manager", "The Ambassador", etc.) to handle ongoing operations, bookings, inventory, and customer interactions tailored to each persona's unique needs.

## Design Doc

### Key Friction Points Identified (General)
- **Technical Jargon**: Terms like "DNS", "Payment Gateways", and "Webhooks" cause immediate abandonment.
- **Upfront Pricing Models**: Rigid pricing forcing paid tiers before seeing value.
- **Disjointed Tools**: Having to connect separate tools for website, booking, payments, and messaging.

---

### Persona 1: Maya (The Home Baker)
**Profile**: Sells custom cakes via Instagram DMs. Needs deposit-based custom orders and DM auto-replies. Runs everything from iPhone.

#### Architecture Diagram
```mermaid
sequenceDiagram
    actor Maya
    participant OHC_UI as OHC Mobile App
    participant WizAgent as AI: The Promoter
    participant Auth as Auth & Multi-tenant (PostgreSQL RLS)
    participant Stripe as Stripe (Payments)
    participant Instagram as Instagram (Meta Graph API)
    participant Ambassador as AI: The Ambassador
    participant Manager as AI: The Manager
    actor Customer

    Maya->>OHC_UI: Starts App
    OHC_UI->>Auth: Creates Tenant ID
    OHC_UI->>WizAgent: Initiates Setup Wizard
    WizAgent-->>Maya: Asks: "What do you sell?"
    Maya->>WizAgent: "Custom Cakes"
    WizAgent-->>Maya: "Do you take deposits?"
    Maya->>WizAgent: "Yes, 50% upfront"
    WizAgent->>OHC_UI: Generates Storefront
    Maya->>OHC_UI: Connects Instagram & Stripe

    Customer->>Instagram: DM: "Vegan cakes?"
    Instagram->>Ambassador: Webhook
    Ambassador->>Manager: Check Catalog
    Manager-->>Ambassador: "Available"
    Ambassador->>Instagram: Auto-reply with order link
    Customer->>OHC_UI: Fills order
    OHC_UI->>Stripe: Processes deposit
    Stripe->>Manager: Payment Confirmed
    Manager->>Maya: Push: "New Order + Deposit!"
```

#### Mobile UX Flow
1. **Acquisition**: Clicks Instagram ad.
2. **Onboarding**: Enters name -> Selects "Physical Products" -> Uploads cake photos -> AI generates storefront.
3. **Activation**: Connects Stripe and Instagram. Store goes live.
4. **Friction Points**: Connecting Stripe can be complex; offer "Receive money later" deferred setup.

---

### Persona 2: Carlos (The Freelance Handyman)
**Profile**: Needs service listings, booking calendar with deposits, customer inbox, and AI quote generator. Android phone only.

#### Architecture Diagram
```mermaid
sequenceDiagram
    actor Carlos
    participant OHC_UI as OHC Mobile App (Android)
    participant WizAgent as AI: The Promoter
    participant Manager as AI: The Manager
    participant Sales as AI: The Salesperson
    participant Calendar as Google Calendar Sync
    participant Stripe as Stripe (Payments)
    actor Customer

    Carlos->>OHC_UI: Starts App
    OHC_UI->>WizAgent: Initiates Setup Wizard
    WizAgent-->>Carlos: "What services do you provide?"
    Carlos->>WizAgent: "Plumbing, Painting"
    WizAgent->>OHC_UI: Generates Service Listings & Booking Page
    Carlos->>OHC_UI: Connects Google Calendar

    Customer->>OHC_UI: Requests Quote for "Leaky Pipe"
    OHC_UI->>Sales: Trigger Quote Gen
    Sales-->>Customer: Auto-sends estimated quote & booking link
    Customer->>OHC_UI: Books slot & pays deposit
    OHC_UI->>Stripe: Processes deposit
    OHC_UI->>Calendar: Syncs appointment
    Manager->>Carlos: Push: "New Booking for Tuesday!"
```

#### Mobile UX Flow
1. **Acquisition**: Word of mouth; signs up via mobile web.
2. **Onboarding**: Selects "Services" -> Lists basic services -> AI builds booking page.
3. **Activation**: Syncs Google Calendar to avoid double-booking.
4. **Friction Points**: Accurately pricing unknown jobs; offer "Request Quote" instead of fixed prices.

---

### Persona 3: Priya (The Boutique Owner)
**Profile**: Needs storefront synced with in-store inventory, variants, in-person POS, email marketing, daily analytics.

#### Architecture Diagram
```mermaid
sequenceDiagram
    actor Priya
    participant OHC_UI as OHC Mobile App & Desktop
    participant WizAgent as AI: The Promoter
    participant Inventory as Inventory System (PostgreSQL)
    participant StripeTerm as Stripe Terminal
    participant Marketing as AI: The Promoter (Email)
    participant Advisor as AI: The Advisor
    actor InStoreCustomer
    actor OnlineCustomer

    Priya->>OHC_UI: Starts App (Desktop/Mobile)
    OHC_UI->>WizAgent: Setup Wizard
    WizAgent->>Inventory: Creates basic catalog (size/color variants)

    InStoreCustomer->>Priya: Buys shirt in-store
    Priya->>StripeTerm: Tap-to-pay
    StripeTerm->>Inventory: Deducts stock

    OnlineCustomer->>OHC_UI: Views storefront
    OHC_UI->>Inventory: Fetches live stock

    Advisor->>Priya: Weekly push: "Top seller: Blue Dress"
    Marketing->>OHC_UI: Suggests email campaign: "New Arrivals"
```

#### Mobile UX Flow
1. **Acquisition**: Upgrading from disjointed systems (Shopify + manual POS).
2. **Onboarding**: Uploads CSV of inventory or adds variants manually -> AI sets up omnichannel store.
3. **Activation**: Orders Stripe Reader for in-person sales.
4. **Friction Points**: Managing complex variants (Size/Color matrix) on mobile; needs highly optimized mobile grid UI.

---

### Persona 4: Leo (The Music Tutor)
**Profile**: Needs lesson booking, automated Zoom links, subscription pricing, TikTok link-in-bio, automated follow-ups.

#### Architecture Diagram
```mermaid
sequenceDiagram
    actor Leo
    participant OHC_UI as OHC Mobile App
    participant WizAgent as AI: The Promoter
    participant Manager as AI: The Manager
    participant Sales as AI: The Salesperson
    participant Zoom as Jitsi/Zoom Integration
    participant Calendar as Google Calendar
    actor Student

    Leo->>OHC_UI: Starts App
    OHC_UI->>WizAgent: Setup Wizard
    WizAgent->>OHC_UI: Generates Link-in-Bio & Subscriptions
    Leo->>OHC_UI: Connects Zoom & Calendar

    Student->>OHC_UI: Books monthly subscription (from TikTok bio)
    OHC_UI->>Manager: Creates recurring billing
    OHC_UI->>Calendar: Syncs weekly slots
    Manager->>Zoom: Generates unique meeting link
    Manager-->>Student: Emails Zoom link

    Sales->>Student: (If inactive 2 weeks) Auto-sends re-engagement email
```

#### Mobile UX Flow
1. **Acquisition**: Sees TikTok ad for easy booking pages.
2. **Onboarding**: Selects "Subscriptions/Services" -> Sets up monthly packages -> Generates link-in-bio.
3. **Activation**: Pastes OHC link into TikTok bio.
4. **Friction Points**: Understanding subscription vs one-off pricing; ensure wizard clearly separates these.

---

### Persona 5: Fatima (The Food Cart Operator)
**Profile**: Needs photo menu, sold-out toggles, pre-order/pickup flow, simple daily order list printable from app, Arabic+English UI. Low-end Android, slow data.

#### Architecture Diagram
```mermaid
sequenceDiagram
    actor Fatima
    participant OHC_UI as OHC Mobile App (Lite/Android)
    participant WizAgent as AI: The Promoter
    participant Manager as AI: The Manager
    participant Twilio as Twilio (SMS)
    actor Customer

    Fatima->>OHC_UI: Starts App (Arabic UI selected)
    OHC_UI->>WizAgent: Setup Wizard
    WizAgent->>OHC_UI: Generates dual-language visual menu

    Customer->>OHC_UI: Views menu (QR code scan at cart)
    Customer->>OHC_UI: Pre-orders Falafel Wrap & Pays
    OHC_UI->>Manager: Logs order
    Manager->>Twilio: Sends SMS notification to Fatima (due to slow data)
    Manager->>OHC_UI: Updates "Orders to Prepare" list

    Fatima->>OHC_UI: Marks item "Sold Out" (optimistic UI update)
    OHC_UI->>Customer: Menu instantly updates
```

#### Mobile UX Flow
1. **Acquisition**: Local flyer or community referral.
2. **Onboarding**: Selects "Food & Beverage" -> Uploads menu photos -> AI sets up pre-order system.
3. **Activation**: Prints QR code for cart window.
4. **Friction Points**: Slow network connections causing app timeouts; requires robust offline-first/optimistic UI architecture for order management.

---

## Implementation Prompt
Implement the underlying database entities, API endpoints, and event routing to support the unified setup wizard and onboarding logic. Create generic webhook handlers for connected third-party apps (Instagram, Stripe, Calendars). Ensure multi-tenant isolation via Row-Level Security across all generated data (catalogs, bookings, orders).

- **User Story**: As a business owner across various verticals, I want a unified, conversational setup process that configures the right modules (booking, physical, POS) automatically.
- **Acceptance Criteria**:
  - The backend supports dynamic tenant configuration based on business type.
  - The database schema robustly handles different product types (services vs variants vs subscriptions).
  - RLS policies ensure secure tenant data separation.
- **Priority**: P0
- **Estimated Scope**: Large
