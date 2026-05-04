# [architecture] Business Journey Architecture

## Problem Statement
Small business owners often abandon SaaS products because the journey from "sign up" to "first dollar earned" is filled with technical friction. We need a clearly defined, persona-driven architecture for the complete end-to-end user journey—from acquisition to referral—ensuring a zero-code, zero-jargon path to value in under 10 minutes.

## Research Report
An analysis of common SaaS drop-off points reveals that non-technical users abandon platforms mostly during DNS setup, complex store configurations, or when confronted with empty dashboards post-signup. OHC solves this by treating AI as an invisible teammate that auto-generates the store, processes orders, and proactively advises the owner.

### Key Lifecycle Phases Assessed
- **Acquisition:** The entry point. Must be highly context-aware (e.g., an ad targeting bakers must land on a baker-specific onboarding).
- **Onboarding:** Must ask only 3-4 essential questions and auto-generate the rest.
- **Activation:** The "Aha!" moment. Earning the first dollar or receiving the first booking.
- **Retention:** AI proactive notifications (e.g., "Your vegan cake is trending") replace passive dashboards.
- **Revenue:** Upsells must be tied to success limits (e.g., outgrowing the 100-action AI limit), not arbitrary paywalls.
- **Referral:** Organic sharing driven by branded links and customer satisfaction.

## Design Doc: Persona Sequence Diagrams

Below are the end-to-end journey maps for our 5 core personas, designed for a 375px mobile-first experience.

### 1. Maya — The Home Baker (Custom Orders)
Maya needs a simple storefront, custom deposit-based orders, and automated DM replies.

```mermaid
sequenceDiagram
    actor Maya
    participant OHC Mobile
    participant AI Marketing
    participant AI Success
    participant Customer

    %% Acquisition
    Maya->>OHC Mobile: Clicks Instagram Ad ("Start a Bakery in 10 mins")
    %% Onboarding
    OHC Mobile->>Maya: Ask: Business Name? Colors?
    Maya->>OHC Mobile: "Maya's Cakes", Pink/Gold
    OHC Mobile->>AI Marketing: Generate Storefront & Catalog
    AI Marketing-->>Maya: Store Live (5 mins)
    %% Activation
    Customer->>Maya's IG: DM "Do you do vegan cakes?"
    AI Success->>Customer: "Yes! Here is the link to order with deposit: [Link]"
    Customer->>OHC Mobile: Places custom order & pays deposit
    OHC Mobile-->>Maya: Push: "New custom cake order + $50 deposit paid"
    %% Retention
    AI Marketing->>Maya: Weekly notification: "Post these 3 cake photos to IG"
    %% Revenue
    OHC Mobile->>Maya: "You've reached 100 AI responses. Upgrade to Starter to keep AI replying while you bake."
    Maya->>OHC Mobile: Upgrades to Starter ($9/mo)
    %% Referral
    Maya->>IG: Shares "Order my cakes at maya.ohc.store"
```

### 2. Carlos — The Freelance Handyman (Service Bookings)
Carlos relies on word-of-mouth. He needs service listings, booking with deposits, and quotes.

```mermaid
sequenceDiagram
    actor Carlos
    participant OHC Android
    participant AI Sales
    participant AI Operations
    participant Client

    %% Acquisition
    Carlos->>OHC Android: Searches Google "app for handyman booking"
    %% Onboarding
    OHC Android->>Carlos: Ask: What services? Hourly rate?
    Carlos->>OHC Android: Plumbing, Painting, $50/hr
    OHC Android->>AI Sales: Generate Service Page & Quote Form
    %% Activation
    Client->>OHC Android: Requests quote for "Leaky pipe under sink"
    AI Sales->>Carlos: Drafts Quote: "$150 + Parts. Send?"
    Carlos->>AI Sales: 1-Tap Approve
    AI Sales->>Client: Sends quote with booking calendar link
    Client->>OHC Android: Books Friday 2PM, pays $50 deposit
    %% Retention
    AI Operations->>Carlos: Push: "Reminder: Leaky pipe job in 1 hour."
    %% Revenue
    OHC Android->>Carlos: "You've booked 10 jobs this month! Add a custom domain with Starter tier."
    Carlos->>OHC Android: Upgrades to Starter ($9/mo)
    %% Referral
    AI Operations->>Client: "Happy with Carlos? Leave a review!"
    Client->>OHC Android: Leaves 5-star review, boosting Carlos's public page
```

### 3. Priya — The Boutique Owner (Physical/In-Person)
Priya needs online/offline sync, POS tap-to-pay, and inventory management.

```mermaid
sequenceDiagram
    actor Priya
    participant OHC App
    participant AI Finance
    participant AI Marketing
    participant Shopper

    %% Acquisition
    Priya->>OHC App: Hears about OHC from a fellow shop owner
    %% Onboarding
    OHC App->>Priya: Ask: Connect existing Stripe/Bank?
    Priya->>OHC App: Connects Bank
    OHC App->>AI Finance: Configures Tap-to-Pay on iPhone
    %% Activation
    Shopper->>Priya: Buys red dress in-store
    Priya->>OHC App: Taps Shopper's card on phone
    OHC App->>AI Finance: Process payment, deduct "Red Dress M" inventory
    %% Retention
    AI Marketing->>Priya: "Red dress is almost sold out. Send email to waitlist?"
    Priya->>AI Marketing: 1-Tap Approve
    %% Revenue
    OHC App->>Priya: "Inventory size exceeded Free tier limits. Upgrade to Pro."
    Priya->>OHC App: Upgrades to Pro ($29/mo)
    %% Referral
    Shopper->>OHC App: Receives digital receipt with "Shop our online store" link
```

### 4. Leo — The Music Tutor (Digital/Subscriptions)
Leo needs calendar sync, auto-Zoom links, and subscription management.

```mermaid
sequenceDiagram
    actor Leo
    participant OHC App
    participant AI Ops
    participant AI Success
    participant Student

    %% Acquisition
    Leo->>OHC App: Sees TikTok ad for "Link in bio for tutors"
    %% Onboarding
    OHC App->>Leo: Ask: Connect Google Calendar? Zoom?
    Leo->>OHC App: Authorizes OAuth
    %% Activation
    Student->>Leo's TikTok: Clicks Link-in-Bio
    Student->>OHC App: Books 4-lesson monthly package
    OHC App->>AI Ops: Generate 4 Zoom links, sync to Calendar
    AI Ops-->>Student: Emails schedule & Zoom links
    %% Retention
    AI Success->>Leo: "Student hasn't booked next month yet. Send auto-follow-up?"
    Leo->>AI Success: 1-Tap Approve
    %% Revenue
    OHC App->>Leo: "You have 5 active subscribers. Upgrade to Business tier for 0% transaction fees."
    Leo->>OHC App: Upgrades to Business ($79/mo)
    %% Referral
    Leo->>TikTok: Adds OHC link to all future videos
```

### 5. Fatima — The Food Cart Operator (Pre-orders)
Fatima needs photo menus, pre-order payments, and loud phone notifications.

```mermaid
sequenceDiagram
    actor Fatima
    participant OHC Android
    participant AI Advisor
    participant AI Ops
    participant HungryCustomer

    %% Acquisition
    Fatima->>OHC Android: Needs an alternative to UberEats 30% fees
    %% Onboarding
    OHC Android->>Fatima: Upload menu photo? Language?
    Fatima->>OHC Android: Snaps picture of printed menu, selects Arabic/English
    OHC Android->>AI Ops: OCR menu, create digital items with prices
    %% Activation
    HungryCustomer->>OHC Android: Scans QR code on cart, orders 2 Falafel wraps
    OHC Android->>Fatima: LOUD notification "New Order Paid - 2 Falafel"
    Fatima->>OHC Android: Taps "Ready for pickup"
    %% Retention
    AI Advisor->>Fatima: Weekly Report: "Tuesday was slow. Offer 10% discount this Tuesday?"
    %% Revenue
    OHC Android->>Fatima: "Daily order limit reached. Upgrade to Pro for unlimited."
    Fatima->>OHC Android: Upgrades to Pro ($29/mo)
    %% Referral
    HungryCustomer->>OHC Android: Shares QR code link with coworkers
```

## Key Invariants & Architectural Constraints
1. **AI Invisible Teammate:** All persona journeys rely on asynchronous AI events (e.g., auto-drafting replies, OCRing menus). These require durable event queuing (PostgreSQL SKIP LOCKED) to guarantee delivery.
2. **Push Notifications:** The retention loops for Maya, Fatima, and Carlos depend entirely on reliable mobile push notifications instead of requiring them to actively open the app to check dashboards.
3. **Usage-Based Upsells:** Upsell CTAs must be contextual (e.g., reaching inventory limits or AI action quotas), driven by telemetry rather than time-gating.

## Implementation Prompt
**For Implementer Agents:** Use these sequence diagrams to guide the implementation of the onboarding wizard and KAIROS event triggers. Do not ask the user technical setup questions. If a value can be inferred or generated by an LLM (e.g., store description, initial inventory from a photo), use the AI department to fulfill it.

- **Acceptance Criteria**: E2E flows for all 5 personas must be fully navigable without manual configuration of underlying services (e.g., Stripe keys should be OAuth'd, Zoom links auto-generated).
- **Priority**: P0
- **Estimated Scope**: Large
