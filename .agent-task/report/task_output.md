# Business Journey Architecture

## Problem Statement
The OneHumanCorp (OHC) platform aims to empower non-technical users to launch and manage their businesses in under 10 minutes. However, we lack a cohesive, documented end-to-end user journey that maps out how different real-world personas interact with the platform from discovery to ongoing retention and growth. Without this, architectural decisions may not align with the actual needs of our users.

## Research Report
The research focuses on defining the complete business lifecycle for key personas defined in the OHC platform:
1.  **Maya (The Home Baker):** Needs a mobile-first, simple storefront, custom orders with deposits, and Instagram DM management.
2.  **Carlos (The Freelance Handyman):** Needs a service listing, a booking calendar with deposits, and automated quote generation via Android.
3.  **Priya (The Boutique Owner):** Needs online/in-store inventory sync, product variants, and cross-platform (mobile/desktop) access.
4.  **Leo (The Music Tutor):** Needs lesson booking, subscription pricing, Zoom links, and a link-in-bio page.
5.  **Fatima (The Food Cart Operator):** Needs a multi-language UI, pre-order functionality, and low-data mobile performance.

The end-to-end journey encompasses the following phases:
*   **Acquisition:** How the user discovers OHC.
*   **Onboarding:** The initial setup wizard to go live.
*   **Activation:** The first critical action (e.g., first product, first sale).
*   **Retention:** Ongoing engagement via AI agents and reports.
*   **Revenue:** Upgrading from free to paid tiers based on limits or premium features.
*   **Referral:** Spreading the word to other potential users.

## Design Doc

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) - Product / Social Focus
```mermaid
sequenceDiagram
    participant M as Maya (Mobile)
    participant O as OHC Platform
    participant A as AI Agents (Marketing/Ops/CS)
    participant C as Customer (Instagram)

    Note over M, O: Acquisition & Onboarding
    M->>O: Discovers OHC via Instagram Ad. Clicks "Start Free"
    O-->>M: Prompts for basic info (Name, Business Type)
    M->>O: Enters "Maya's Custom Cakes"
    O-->>A: Trigger: Setup Storefront
    A-->>M: Generates draft storefront & catalog with placeholder cake images
    M->>O: Customizes images and prices, enables Stripe
    M->>O: Clicks "Publish" (Live in < 10 mins)

    Note over M, C: Activation
    M->>O: Connects Instagram Account
    C->>M: Sends Instagram DM: "Do you do vegan cakes?"
    A->>C: Auto-replies: "Yes! Here is our vegan menu [Link]" (Customer Success Agent)
    C->>O: Places custom order, pays deposit
    O-->>M: Push Notification: "New Order + Deposit Received!"

    Note over M, A: Retention & Revenue
    A-->>M: Weekly Advisor Report: "Vegan cakes are trending. Consider adding more options."
    M->>O: Reaches 100th order, prompts to upgrade to Starter Tier
    M->>O: Upgrades to custom domain (mayascakes.com)
```

#### 2. Carlos (The Freelance Handyman) - Service / Booking Focus
```mermaid
sequenceDiagram
    participant C as Carlos (Android)
    participant O as OHC Platform
    participant A as AI Agents (Ops/Sales)
    participant Cust as Customer (Web)

    Note over C, O: Acquisition & Onboarding
    C->>O: Hears from friend, downloads Android App
    O-->>C: Setup Wizard (Services)
    C->>O: Adds "Plumbing Fixes", "Painting" with hourly rates
    O-->>A: Trigger: Generate Booking Page
    A-->>C: Provides live link to Booking Calendar

    Note over C, Cust: Activation
    Cust->>O: Visits Carlos's page, requests "Leaky pipe repair"
    O-->>A: Trigger: Generate Quote
    A-->>Cust: Auto-sends Quote with estimated hours and deposit link
    Cust->>O: Approves quote, pays deposit, selects time slot
    O-->>C: SMS/Push: "New job booked: Thursday 2 PM"

    Note over C, A: Retention & Referral
    A-->>Cust: Post-job: "Rate Carlos's work!" (Collects 5-star review)
    A-->>C: Weekly Report: "You have 3 open quotes. Send a follow-up?"
```

#### 3. Priya (The Boutique Owner) - Omni-channel / Inventory Focus
```mermaid
sequenceDiagram
    participant P as Priya (Mobile & Desktop)
    participant O as OHC Platform
    participant A as AI Agents (Ops/Finance)
    participant Cust as Customer (In-store/Online)

    Note over P, O: Acquisition & Onboarding
    P->>O: Needs online sync. Signs up on MacBook
    O-->>P: Imports current inventory CSV
    O-->>A: Trigger: Organize Variants (Size/Color)
    A-->>P: Sets up omni-channel storefront

    Note over P, Cust: Activation
    Cust->>P: Buys red dress in-store
    P->>O: Uses Tap-to-Pay on iPhone (Stripe Terminal)
    O-->>A: Trigger: Update Inventory
    A->>O: Reduces stock of red dress across all channels

    Note over P, A: Retention & Revenue
    A-->>P: Daily Mobile Analytics: "Revenue up 15%. Red dress is low on stock."
    P->>O: Expands catalog, hits product limit, upgrades to Pro Tier.
```

#### 4. Leo (The Music Tutor) - Subscription / Digital Focus
```mermaid
sequenceDiagram
    participant L as Leo (Web/Mobile)
    participant O as OHC Platform
    participant A as AI Agents (Sales/Ops)
    participant S as Student

    Note over L, O: Acquisition & Onboarding
    L->>O: Needs link-in-bio for TikTok. Signs up.
    O-->>A: Trigger: Create Profile
    A-->>L: Generates profile page with Lesson Packages (Subscriptions)
    L->>O: Links Google Calendar

    Note over L, S: Activation
    S->>O: Clicks TikTok link, buys "Monthly Guitar Pro" package
    O-->>A: Trigger: Schedule & Provision
    A-->>S: Sends calendar invite + Zoom link
    A-->>L: Notifies of new subscriber

    Note over L, A: Retention
    A-->>S: "You haven't booked a lesson in 2 weeks. Schedule now?"
    A-->>L: Monthly Report: "Subscription retention is 90%."
```

#### 5. Fatima (The Food Cart Operator) - Local / Fast Paced Focus
```mermaid
sequenceDiagram
    participant F as Fatima (Low-end Android, Arabic UI)
    participant O as OHC Platform
    participant A as AI Agents (Ops)
    participant C as Hungry Customer

    Note over F, O: Acquisition & Onboarding
    F->>O: Wants to stop taking phone orders. Signs up in Arabic.
    O-->>F: Prompts to take photos of food.
    F->>O: Uploads photos.
    O-->>A: Trigger: Build Menu
    A-->>F: Generates menu page with prices.

    Note over F, C: Activation
    C->>O: Scans QR code at cart, places pre-order for Falafel
    O-->>F: Loud Audio Notification on Android: "New Order! Falafel"
    F->>O: Taps "Preparing"
    A-->>C: SMS: "Your order is being prepared."
    F->>O: Taps "Ready for Pickup"
    A-->>C: SMS: "Order ready!"

    Note over F, O: Retention
    F->>O: Falafel runs out. Taps "Sold Out" toggle.
    O->>A: Instantly updates live menu.
    F->>O: Prints daily summary from app.
```

### UI Flow & Mobile UX (375px First)
*   **Onboarding Wizard:** 3-5 sequential screens. One question per screen (e.g., "What's the name of your business?", "What do you sell?"). Large tap targets (>= 44x44px). Native numeric keypads for pricing.
*   **Dashboard (The "Command Center"):** Glassmorphism cards displaying AI Agent notifications ("Your Promoter Agent drafted a new Instagram post", "Your Accountant Agent prepared your weekly summary").
*   **Friction Points to Avoid:**
    *   No complex DNS or SSL setup for custom domains; handled entirely by OHC in the background.
    *   No manual CSS/HTML layout editing on mobile; reliance on intelligent defaults and AI content block generation.
    *   Ensure critical flows (like Fatima's order received notification) work seamlessly even on 3G connections.

### AI Agent Integration Points
*   **Operations:** Automatically updating inventory, managing calendar syncs.
*   **Customer Success:** Auto-replying to DMs, sending Zoom links/receipts.
*   **Sales:** Generating quotes from customer requests.
*   **Marketing:** Generating the initial storefront and social media posts.
*   **Advisory:** Pushing weekly/daily plain-language insights to the user via push notifications.

## Implementation Prompt
"Implement the foundational 'Onboarding Wizard' UI flow and corresponding backend endpoints. The wizard must guide a new, non-technical user through the creation of their Tenant. It must capture the Business Name, Business Category (e.g., Services, Retail, Food), and automatically trigger the appropriate AI Agent (e.g., Marketing Agent) to generate a foundational, publish-ready storefront template. The UI must strictly adhere to the 375px mobile-first design system, utilizing native mobile keyboards for input, and feature OHC Premium Glassmorphism tokens. Acceptance Criteria: A user can complete the wizard in under 3 minutes, resulting in a provisioned Tenant, an active 'Draft' storefront, and a 'Welcome' notification from their Advisory Agent in the dashboard."

## Priority
P1

## Estimated Scope
Medium
