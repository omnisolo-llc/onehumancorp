# Business Journey Architecture

## 1. Overview
The Business Journey Architecture maps the complete lifecycle of a non-technical user (persona) on the OneHumanCorp (OHC) platform. It covers everything from initial discovery and onboarding to activation, retention, revenue scaling, and referral. The architecture ensures that a user can go from zero to a fully operational business in under 10 minutes from a mobile device (375px baseline width), with AI agents invisibly handling the underlying complexity.

## 2. Core Personas & Journeys

The journeys are designed to accommodate the distinct needs of our core personas:
*   **Maya (The Home Baker)**: Physical products, custom orders with deposits, Instagram DM integration.
*   **Carlos (The Freelance Handyman)**: Services, pricing catalogs, booking with deposits, quote generation.
*   **Priya (The Boutique Owner)**: Physical products, POS/online inventory sync, variants, analytics.
*   **Leo (The Music Tutor)**: Subscriptions, booking with calendar/Zoom sync, portfolio presence.
*   **Fatima (The Food Cart Operator)**: Food pre-orders, sold-out toggles, multi-lingual (Arabic/English), low-end Android.

## 3. Journey Phases

### 3.1 Acquisition
*   **Entry Points**: Organic search, targeted Instagram/TikTok ads, or referrals from existing OHC users (e.g., "Powered by OHC" badge on a link-in-bio).
*   **Landing Page**: A clear, jargon-free CTA ("Launch your business in 10 minutes"). Emphasizes the "No Code, No Servers" promise and mobile accessibility.

### 3.2 Onboarding (Zero → Live in 10 Mins)
The onboarding flow is a guided conversational wizard powered by the AI Marketing & Advertising Agent ("The Promoter").
1.  **Business Name & Type**: User inputs name and selects category (e.g., "Food & Beverage", "Services").
2.  **Core Needs Assessment**: User selects what they want to do (e.g., "Take bookings", "Sell online").
3.  **Visual Identity**: AI generates a preliminary storefront design based on the business type, utilizing the OHC Premium Token library (Glassmorphism, Outfit/Inter typography).
4.  **First Catalog Item/Service**: User adds their first product/service (e.g., Maya adds a "Custom Vanilla Cake").

### 3.3 Activation (Day 1 Success)
*   **Success Metrics**: The user has a live storefront, has connected a payment method (Stripe), and is ready to accept orders/bookings.
*   **First Action**: The Operations Agent ("The Manager") simulates a test order or booking to demonstrate the flow. The Customer Success Agent ("The Ambassador") drafts an initial welcome email template for review.

### 3.4 Retention & Daily Use
*   **Mobile Dashboard**: A 375px-optimized dashboard showing daily metrics (e.g., new orders, messages).
*   **AI Advisory**: The Business Advisory Agent ("The Advisor") sends weekly plain-language reports ("You had 5 bookings this week. Tuesdays are your busiest day.").
*   **Push Notifications**: Alerts for new orders, low inventory, or pending AI drafts requiring 1-tap approval (e.g., "Review Instagram post draft").

### 3.5 Revenue (Tier Upgrades)
*   **Trigger**: The user hits limits on the Free tier (e.g., 10 products, 100 AI actions/month) or wants a custom domain.
*   **Upgrade Flow**: A seamless transition to the Starter ($9/mo) or Pro ($29/mo) tier, highlighting the unlocked value (e.g., unlimited products, custom domain, full AI agent access).

### 3.6 Referral
*   **Viral Loop**: Existing users share their storefront link or link-in-bio. The "Powered by OHC" badge serves as passive marketing.
*   **Incentive**: Referral rewards tracked by the Sales & Acquisition Agent.

## 4. Architecture Diagrams

### 4.1 End-to-End User Journey (Maya - Home Baker)

```mermaid
sequenceDiagram
    participant M as Maya (Mobile User)
    participant App as OHC Mobile App
    participant MA as Marketing Agent (Promoter)
    participant Op as Operations Agent (Manager)
    participant CS as Customer Success Agent (Ambassador)
    participant Stripe as Stripe (Payments)

    M->>App: Signs up & selects "Baker"
    App->>MA: Trigger: Generate Storefront
    MA-->>App: Presents Draft Design
    M->>App: Approves Design & Adds "Custom Cake"
    App->>Op: Initialize Inventory
    M->>App: Connects Bank (Stripe)
    App->>Stripe: Setup Account
    Note over M, App: --- Activation Complete (<10 mins) ---

    M->>App: Receives Instagram DM "Vegan cake?"
    App->>CS: Trigger: Draft Reply
    CS-->>App: Presents drafted reply
    M->>App: 1-Tap Approves Draft
    App->>M: Customer places order via Storefront
    App->>Stripe: Process Deposit
    Stripe-->>Op: Payment Confirmed
    Op-->>App: Notify Maya & Update Calendar
    Op->>CS: Trigger: Order Confirmation
    CS-->>M: Sends Confirmation to Customer
```

### 4.2 End-to-End User Journey (Carlos - Freelance Handyman)

```mermaid
sequenceDiagram
    participant C as Carlos (Android User)
    participant App as OHC Mobile App
    participant MA as Marketing Agent (Promoter)
    participant SA as Sales Agent (Salesperson)
    participant Op as Operations Agent (Manager)
    participant Stripe as Stripe (Payments)

    C->>App: Signs up & selects "Services/Repairs"
    App->>MA: Trigger: Generate Service Catalog
    MA-->>App: Presents Draft Listings
    C->>App: Approves & Sets Prices/Availability
    C->>App: Connects Bank (Stripe)
    Note over C, App: --- Activation Complete (<10 mins) ---

    C->>App: Customer submits inquiry "Leaky pipe"
    App->>SA: Trigger: Generate Quote
    SA-->>App: Presents Draft Quote & Booking Link
    C->>App: 1-Tap Approves Quote
    SA-->>App: Sends Quote to Customer
    App->>Stripe: Customer Pays Deposit
    Stripe-->>Op: Payment Confirmed
    Op-->>App: Notify Carlos & Add to Calendar
```

### 4.3 End-to-End User Journey (Priya - Boutique Owner)

```mermaid
sequenceDiagram
    participant P as Priya (Mobile/Desktop User)
    participant App as OHC App
    participant Op as Operations Agent (Manager)
    participant MA as Marketing Agent (Promoter)
    participant Stripe as Stripe Terminal/Online

    P->>App: Signs up & selects "Retail Boutique"
    App->>MA: Trigger: Generate Storefront
    P->>App: Imports Inventory & Variants (Size/Color)
    App->>Op: Sync Initial Inventory
    P->>App: Connects Bank (Stripe)
    Note over P, App: --- Activation Complete (<10 mins) ---

    P->>Stripe: In-store tap-to-pay (Customer buys Red Dress)
    Stripe-->>Op: Payment Confirmed
    Op->>App: Update Inventory (-1 Red Dress)
    App->>MA: Trigger: Low Stock Alert
    MA-->>P: Notification "Red Dress low in stock"
    App->>P: Online Customer buys Blue Dress
    App->>Stripe: Process Online Payment
    Stripe-->>Op: Payment Confirmed
    Op-->>App: Update Inventory & Notify Priya
```

### 4.4 End-to-End User Journey (Leo - Music Tutor)

```mermaid
sequenceDiagram
    participant L as Leo (Mobile User)
    participant App as OHC App
    participant MA as Marketing Agent (Promoter)
    participant Op as Operations Agent (Manager)
    participant CS as Customer Success Agent (Ambassador)
    participant Stripe as Stripe (Subscriptions)

    L->>App: Signs up & selects "Tutoring"
    App->>MA: Trigger: Generate Profile/Link-in-bio
    L->>App: Sets Monthly Lesson Packages & Calendar Sync
    L->>App: Connects Bank (Stripe)
    Note over L, App: --- Activation Complete (<10 mins) ---

    App->>L: Student subscribes to "4 Lessons/Month"
    App->>Stripe: Create Subscription
    Stripe-->>Op: Subscription Active
    Op->>CS: Trigger: Welcome Email & Zoom Links
    CS-->>App: Sends Details to Student
    Op-->>L: Update Google Calendar with slots
    App->>CS: Trigger (2 weeks later): Check inactive student
    CS-->>L: Drafts "Ready for next lesson?" follow-up
    L->>App: 1-Tap Approves Follow-up
```

### 4.5 End-to-End User Journey (Fatima - Food Cart Operator)

```mermaid
sequenceDiagram
    participant F as Fatima (Low-end Android User)
    participant App as OHC App (Arabic/English)
    participant MA as Marketing Agent (Promoter)
    participant Op as Operations Agent (Manager)
    participant Stripe as Stripe (Payments)

    F->>App: Signs up & selects "Food Cart"
    App->>MA: Trigger: Generate Photo Menu
    F->>App: Approves Menu & Translates to Arabic
    F->>App: Connects Bank (Stripe)
    Note over F, App: --- Activation Complete (<10 mins) ---

    App->>F: Customer places Pre-Order (Falafel)
    App->>Stripe: Process Payment
    Stripe-->>Op: Payment Confirmed
    Op-->>F: Urgent Phone Notification "New Order"
    F->>App: Marks Falafel as "Sold Out"
    App->>Op: Update Menu Instantly
    App->>F: Customer arrives for Pickup
    F->>App: Marks Order as Completed
```

## 5. Friction Points & Mitigation

*   **Payment Setup**: Stripe onboarding can be complex.
    *   *Mitigation*: Use Stripe Connect with simplified onboarding. Allow users to start taking orders (as "pending payment") before full verification if permitted by risk profile.
*   **Content Generation Blank Page Syndrome**: Users struggle to write descriptions or policies.
    *   *Mitigation*: AI completely generates these based on a few keywords. User only reviews/edits.
*   **Mobile UI Density**: Managing a complex store on a 375px screen is hard.
    *   *Mitigation*: Strict adherence to mobile-first UI patterns, hidden complexity, and conversational interfaces for settings.
