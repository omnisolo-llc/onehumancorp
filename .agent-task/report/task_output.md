# [Business Journey Architecture] End-to-End User Journey for OHC Personas

## Title
Business Journey Architecture: End-to-End User Journey Standardization

## Problem Statement
Small business owners coming to OneHumanCorp (OHC) need to be able to go from idea to a live business in under 10 minutes. A complex onboarding process, unclear value propositions, or lack of ongoing engagement will cause non-technical users to churn. The current platform lacks a formally defined, end-to-end journey standardizing Acquisition, Onboarding, Activation, Retention, Revenue, and Referral across all target personas. Without this, there is high risk of friction points causing abandonment.

## Research Report

### Acquisition
How do our personas discover OHC?
*   **Maya (The Home Baker):** Targeted Instagram Ad ("Start your baking business from your phone in 10 minutes"). CTA: "Launch Your Store Free".
*   **Carlos (The Freelance Handyman):** Organic Google Search for "easy booking app for handyman". CTA: "Get Booked Today".
*   **Priya (The Boutique Owner):** Word of mouth (friend referral) or TikTok business tool roundup. CTA: "Sync Your Store Now".
*   **Leo (The Music Tutor):** TikTok/Instagram Reel showing a seamless link-in-bio booking page. CTA: "Create Your Booking Page".
*   **Fatima (The Food Cart Operator):** Local community organization recommendation or targeted Facebook ad for food operators. CTA: "Take Pre-Orders Now".

### Onboarding
Step-by-step wizard flow to get to live in under 10 minutes.
*   **Minimum Inputs Needed to Go Live:**
    1. Business Name & Category.
    2. Primary Goal (e.g., "Sell products", "Take bookings", "Show portfolio").
    3. One core entity (e.g., 1 product, 1 service, 1 portfolio item).
    4. Connect Payment (Stripe OAuth / basic banking info).
*   **What can be deferred:** Custom domain setup, advanced SEO configuration, full inventory sync, multi-language setup, detailed policy generation (AI drafts these initially).
*   **AI Intervention:** The "Marketing & Advertising" AI department instantly generates a drafted website and social media post during the onboarding loading screen.

### Activation
What does success look like?
*   **Day 1:** Live URL generated, shared on at least one social platform, first AI interaction completed.
*   **Week 1:** First transaction/booking processed, first AI-generated customer follow-up sent.
*   **Month 1:** Regular cadence established (e.g., 5+ orders/bookings), Weekly AI Health Report reviewed.

### Retention
What brings users back daily?
*   **Carlos:** Push notifications for new bookings ("New Booking Request: Plumbing Fix"), AI quote generation approvals.
*   **Priya:** Daily mobile analytics ("Revenue Today: $450"), inventory alerts.
*   **Maya:** Chatting with customers via the Customer Success agent's drafted replies.
*   **Leo:** Weekly schedule summaries and AI-driven inactive student reminders.
*   **Fatima:** Daily printable order list for pre-orders, real-time pickup notifications.

### Revenue
When do users upgrade from Free ($0) to Starter ($9/mo) or Pro ($29/mo)?
*   **Trigger:** Hitting the 10-product limit (Free tier) or wanting a custom domain.
*   **CTA Presentation:** Contextual, non-intrusive banners. E.g., when Maya tries to add her 11th cake design, the Operations agent gently suggests, "Your business is growing! Upgrade to Starter to add unlimited products and a custom domain for just $9/mo."

### Referral
What is the viral loop?
*   **Mechanism:** "Powered by OneHumanCorp" badge on Free tier storefronts and booking pages.
*   **Incentive:** "Invite a fellow business owner and both get 1 month of Pro free."
*   **Example:** Priya shares her new online store with a fellow boutique owner; the seamless checkout experience prompts the friend to click the badge and sign up.

---

## Design Doc

### Key Friction Points to Avoid
1.  **Technical Jargon:** Asking for "DNS Settings" during onboarding. Must use "Connect your domain" and handle DNS invisibly.
2.  **Payment Friction:** Forcing full Stripe KYC before letting them see the generated site. Allow deferring full KYC until the first payout.
3.  **Blank Canvas Syndrome:** Presenting an empty dashboard. AI must pre-populate products, a website, and a welcome post based on the business category.
4.  **Mobile Overwhelm:** Displaying desktop-optimized data tables on a 375px screen. Analytics must be conversational ("You made $200 today").

### AI Agent Integration Points
*   **Onboarding:** Promoter Agent generates the website. Legal Agent drafts the TOS.
*   **Activation:** Salesperson Agent drafts the first social post to share the new link.
*   **Retention:** Advisor Agent pushes the weekly health report notification.

### Mermaid.js Sequence Diagrams

#### Persona: Maya (The Home Baker)
```mermaid
sequenceDiagram
    autonumber
    actor Maya as Maya (Phone)
    participant Ad as Instagram Ad
    participant OHC as OHC App / Web
    participant AI as AI Promoter Agent
    participant Stripe as Stripe Connect

    Maya->>Ad: Clicks "Launch Your Store Free"
    Ad-->>OHC: Redirects to Mobile Onboarding
    OHC->>Maya: Asks "What's your business name and what do you sell?"
    Maya-->>OHC: "Maya's Cakes", "Custom Cakes"
    OHC->>AI: Trigger website generation (Category: Bakery)
    AI-->>OHC: Returns generated storefront with placeholder cake images
    OHC->>Maya: Shows preview. Asks for 1 real cake photo & price.
    Maya-->>OHC: Uploads photo, sets deposit price
    OHC->>Stripe: Init lightweight merchant onboarding
    Stripe-->>Maya: Collects basic payment info
    Stripe-->>OHC: Payment ready
    OHC->>Maya: Store is Live! Shares Link to Bio.
    Maya->>OHC: Approves AI-generated Instagram launch post
```

#### Persona: Carlos (The Freelance Handyman)
```mermaid
sequenceDiagram
    autonumber
    actor Carlos as Carlos (Android)
    participant Search as Google Search
    participant OHC as OHC App
    participant AI as AI Salesperson Agent
    participant Client as Client (Web)

    Carlos->>Search: Searches "easy booking app"
    Search-->>OHC: Clicks OHC Organic Link
    Carlos->>OHC: Enters "Carlos Repairs", "Handyman"
    OHC->>Carlos: Connect Google Calendar?
    Carlos-->>OHC: Authorizes Calendar Sync
    OHC->>Carlos: Store live with "General Repair" service at $50/hr
    Carlos->>Carlos: Shares link with client via SMS
    Client->>OHC: Books Carlos for Thursday, describes problem ("Leaky pipe")
    OHC->>AI: Analyzes problem description
    AI-->>OHC: Drafts quote and repair estimate
    OHC->>Carlos: Push Notification: "New Booking. Review Quote."
    Carlos-->>OHC: Approves Quote
    OHC->>Client: Sends official quote and deposit link
```

#### Persona: Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    autonumber
    actor Priya as Priya (iPhone/MacBook)
    participant Referral as Friend's Referral Link
    participant OHC as OHC App
    participant AI as AI Advisor
    participant POS as OHC Tap-to-Pay

    Priya->>Referral: Clicks link
    Referral-->>OHC: Lands on sign up
    Priya->>OHC: Signs up, bulk uploads 15 inventory items
    OHC->>Priya: "You've reached the 10-item limit for Free."
    Priya-->>OHC: Upgrades to Starter ($9/mo)
    OHC->>Priya: Full inventory synced.
    Priya->>POS: Uses Tap-to-Pay in-store for customer
    POS-->>OHC: Records transaction, updates inventory
    OHC->>AI: Triggers daily aggregation
    AI-->>Priya: Push Notification: "Daily Wrap-up: $450 in sales, 2 online, 3 in-store."
```

#### Persona: Leo (The Music Tutor)
```mermaid
sequenceDiagram
    autonumber
    actor Leo as Leo (Phone)
    participant TikTok as TikTok Reel
    participant OHC as OHC App
    participant AI as AI Ambassador Agent
    participant Zoom as Zoom Integration

    Leo->>TikTok: Watches OHC tutorial
    TikTok-->>OHC: Downloads App
    Leo->>OHC: Sets up "Leo Guitar", enables subscriptions
    OHC->>Zoom: Auto-configures meeting link generation
    Leo->>Leo: Adds OHC link to TikTok Bio
    Note over Leo, OHC: 3 weeks later
    AI->>OHC: Detects student 'Alex' hasn't booked in 14 days
    OHC->>Leo: AI Drafts Message: "Hey Alex, ready for next lesson?"
    Leo-->>OHC: Taps 'Send'
```

#### Persona: Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    autonumber
    actor Fatima as Fatima (Low-end Android)
    participant OHC as OHC App (Arabic/English UI)
    participant AI as AI Operations Agent
    participant Customer as Customer (Web)

    Fatima->>OHC: Opens App, Sets UI to Arabic
    Fatima->>OHC: Snaps photo of menu board
    OHC->>AI: OCR & translate menu to structured items
    AI-->>OHC: Returns digitized menu
    Fatima-->>OHC: Confirms items and prices
    Customer->>OHC: Browses English version of menu, places order for pickup
    Customer->>OHC: Pays via Apple Pay
    OHC->>Fatima: LOUD Push Notification: "New Order: 2x Halal Chicken Over Rice"
    Fatima-->>OHC: Taps "Preparing"
    OHC->>Customer: SMS Update: "Food is being prepared!"
    Note over Fatima, OHC: End of day
    Fatima->>OHC: Taps 'Print Daily Summary'
```

## Implementation Prompt
**For the Implementer Agent:**
Implement the Onboarding Wizard CUJ (Critical User Journey) for the Flutter Web and Mobile clients, backed by the Go API.
*   The flow must allow the user to input their business name and category, followed by connecting a payment method (simulate Stripe connect).
*   During the final loading screen, the frontend must mock an AI-generation state (calling the AI Promoter Agent) that returns a completed storefront structure.
*   All screens must be responsive, starting strictly from a 375px mobile layout.
*   Do not use technical jargon; keep all copy conversational and simple.
*   Ensure full E2E test coverage for this onboarding flow using Playwright/Flutter testing tools, simulating the complete path from landing to live storefront. Do not define specific backend database tables; focus on the API contract and client-side state.

## Priority
P0 (Critical)

## Estimated Scope
Large