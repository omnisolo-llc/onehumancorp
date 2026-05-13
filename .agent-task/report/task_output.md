# [architecture] End-to-End Business Journey Architecture

## Title
OHC End-to-End Business Journey Architecture

## Problem Statement
Small business owners—bakers, handymen, boutique owners, tutors, and food cart operators—need a frictionless path from zero to a live business in under 10 minutes. The complexity of websites, domain setup, payments, and marketing is overwhelming. The business journey needs to be simple, mobile-first, and natively infused with AI to handle the heavy lifting invisibly.

## Research Report
The current market offerings (Shopify, Wix, Squarespace) have too high a learning curve for micro-businesses. Most users abandon setup due to excessive configuration. Our target personas need targeted experiences:
- **Maya (Baker)**: Needs deposit-based custom orders and an AI agent to handle Instagram DMs.
- **Carlos (Handyman)**: Needs service listings, booking calendar, deposits, and quotes.
- **Priya (Boutique Owner)**: Needs physical/online inventory sync, product variants, and in-person tap-to-pay.
- **Leo (Music Tutor)**: Needs subscription packages, booking calendar, and auto-generated meeting links.
- **Fatima (Food Cart)**: Needs multilingual pre-order menu with simple toggles on a low-end Android device.

## Design Doc

### Key Design Decisions
- **Mobile-First Everything**: The primary device for setup and management is mobile. Complex flows must be chunked or simplified.
- **AI-Led Onboarding**: Instead of forms, use an conversational UI or minimal inputs to generate the initial site.
- **Frictionless Activation**: The goal is "first product added, first payment received" within the first session.

### Mobile UX Flow (375px first)
1. **Acquisition (Wireframe)**: Landing page emphasizing "Live in 10 minutes." with a single "Start Now" button.
2. **Onboarding (Wireframe)**: Full-screen modal with conversational UI: "What's your business name?" -> "What do you sell?" -> AI generation loading screen (Glassmorphism blur, Outfit headings).
3. **Activation (Wireframe)**: Dashboard view. "Add Product" button is prominent. "Connect Bank" card shown below.
4. **Retention (Wireframe)**: Daily push notification leading to an Insights tab.
5. **Revenue (Wireframe)**: Upgrade banner appearing only when limits (e.g., product count) are reached.
6. **Referral (Wireframe)**: A dedicated "Share" tab with a one-tap link generator for social bios, featuring a prominent CTA: "Invite a fellow owner and both get 1 month Pro free."

### Identified Friction Points
- **Friction Point 1**: Choosing a template. *Solution*: AI generates a customized draft instead of showing a template gallery.
- **Friction Point 2**: Connecting payments. *Solution*: Defer full KYC. Allow instant deposit collection to OHC wallet, prompt for KYC on first withdrawal.
- **Friction Point 3**: Adding products. *Solution*: Allow uploading photos; AI extracts details (name, price, description) automatically.

### AI Agent Integration Points
- **Onboarding Generation**: AI builds the initial structure.
- **Operations Manager**: Handles basic order updates and fulfillment notifications.
- **Customer Success Ambassador**: Manages simple customer inquiries (e.g., "Do you offer vegan cakes?").

### Architecture Diagrams

#### Maya (Baker) Full Journey
```mermaid
sequenceDiagram
    participant Maya
    participant App
    participant AI
    participant Instagram

    %% Acquisition & Onboarding
    Maya->>App: "Maya's Vegan Cakes"
    App->>AI: Generate Bakery Template
    AI-->>App: Draft UI
    Maya->>App: Upload Cake Photo
    App->>AI: Extract Cake Details
    AI-->>App: Title: Chocolate Cake, Price: $40

    %% Activation
    Maya->>App: Publish & Connect Insta
    Maya->>App: Connect Bank (Instant Deposit)

    %% Retention
    Instagram->>AI: "Do you make gluten-free?"
    AI-->>Instagram: "Yes! Here's the order link."
    AI->>App: Send Daily Summary to Maya

    %% Revenue
    App->>Maya: Push: "You have 9/10 free products."
    Maya->>App: Upgrade to Starter

    %% Referral
    Maya->>App: Share Referral Link on Insta Story
```

#### Carlos (Handyman) Full Journey
```mermaid
sequenceDiagram
    participant Carlos
    participant App
    participant AI
    participant Customer

    %% Acquisition & Onboarding
    Carlos->>App: "Carlos Repairs"
    App->>AI: Generate Service Template
    AI-->>App: Draft UI with Calendar
    Carlos->>App: Add "Plumbing Fix" ($50 deposit)

    %% Activation
    App->>AI: Setup booking slots
    AI-->>App: Calendar connected
    Carlos->>App: Publish & Add Wallet

    %% Retention
    Customer->>App: Book "Plumbing Fix"
    AI->>Carlos: Send SMS Notification
    AI->>Customer: Send Confirmation & Receipt

    %% Revenue
    App->>Carlos: Push: "Add AI quoting for $9/mo"
    Carlos->>App: Upgrade to Starter

    %% Referral
    Carlos->>Customer: "Refer a friend via my app link!"
```

#### Priya (Boutique Owner) Full Journey
```mermaid
sequenceDiagram
    participant Priya
    participant App
    participant AI
    participant InStoreCustomer

    %% Acquisition & Onboarding
    Priya->>App: "Priya's Threads"
    App->>AI: Generate Retail Template
    AI-->>App: Draft UI with Variants
    Priya->>App: Add "Summer Dress" (S, M, L)

    %% Activation
    App->>AI: Organize Inventory Matrix
    AI-->>App: Matrix Created
    Priya->>App: Publish & Enable Tap-to-Pay

    %% Retention
    InStoreCustomer->>App: Tap to Pay on Priya's Phone
    AI->>Priya: Update Inventory count

    %% Revenue
    App->>Priya: Push: "Unlock Custom Domain"
    Priya->>App: Upgrade to Pro

    %% Referral
    Priya->>App: "Share my store builder link with boutique friends"
```

#### Leo (Music Tutor) Full Journey
```mermaid
sequenceDiagram
    participant Leo
    participant App
    participant AI
    participant Student

    %% Acquisition & Onboarding
    Leo->>App: "Leo's Guitar Lessons"
    App->>AI: Generate Tutoring Template
    AI-->>App: Draft UI with Subscriptions

    %% Activation
    Leo->>App: Add "Monthly 4-Pack"
    App->>AI: Setup Recurring Billing
    AI-->>App: Stripe Subscription Created
    Leo->>App: Publish & Sync Zoom

    %% Retention
    Student->>App: Subscribes to 4-Pack
    AI->>Student: Auto-send Zoom Link
    AI->>Leo: Send Weekly Class Roster

    %% Revenue
    App->>Leo: Push: "Exceeded 100 students"
    Leo->>App: Upgrade to Business Tier

    %% Referral
    Leo->>App: Post Referral Link in TikTok Bio
```

#### Fatima (Food Cart) Full Journey
```mermaid
sequenceDiagram
    participant Fatima
    participant App
    participant AI
    participant HungryCustomer

    %% Acquisition & Onboarding
    Fatima->>App: "Fatima's Halal" (Arabic selected)
    App->>AI: Generate Food Menu Template (RTL)
    AI-->>App: Draft UI
    Fatima->>App: Add "Chicken Shawarma"

    %% Activation
    App->>AI: Translate to English for Menu
    AI-->>App: Dual-Language Listing
    Fatima->>App: Publish & Enable Pre-orders

    %% Retention
    HungryCustomer->>App: Pre-order & Pay Online
    AI->>Fatima: Ring Loud Notification on Android
    Fatima->>App: Mark "Ready for Pickup"

    %% Revenue
    App->>Fatima: Push: "AI marketing helps you get more orders"
    Fatima->>App: Upgrade to Starter

    %% Referral
    Fatima->>HungryCustomer: Hand out QR code for referral
```

## Implementation Prompt
Implement the End-to-End Business Journey flow. Focus on the mobile-first onboarding experience where a user can input minimal details (business name, type) and an AI agent generates a draft site. The flow should cover acquisition, onboarding, activation, retention, revenue, and referral. Ensure the UX is intuitive, passes the 'grandmother test', and uses the OHC premium design tokens (Glassmorphism, Outfit + Inter typography).

## Priority
P0

## Estimated Scope
Large
