# OHC Business Journey Architecture

## 1. Overview
This document outlines the complete end-to-end user journey for each key persona on the OneHumanCorp (OHC) platform. It maps out the stages of Acquisition, Onboarding, Activation, Retention, Revenue, and Referral, detailing how non-technical small business owners interact with the system from discovery to scaling their business.

## 2. Core Personas
- **Maya (The Home Baker, 28)**: Needs a mobile-first catalog with deposit-based custom ordering and an AI agent to handle DMs.
- **Carlos (The Freelance Handyman, 42)**: Needs service listings, booking calendar with deposits, and automated quote generation.
- **Priya (The Boutique Owner, 35)**: Needs online-offline inventory sync, product variants, and in-person POS payments.
- **Leo (The Music Tutor, 22)**: Needs lesson bookings, auto-generated Zoom links, subscription packages, and a TikTok link-in-bio.
- **Fatima (The Food Cart Operator, 50)**: Needs a simple mobile UI for pre-orders/pickups, photo menus with sold-out toggles, and multi-language support.

## 3. Journey Phases

### 3.1 Acquisition
**How do they discover OHC?**
- **Maya**: Sees a targeted Instagram ad emphasizing "Turn your DMs into a real business in 10 minutes".
- **Carlos**: A friend (maybe another tradesperson) shares a referral link via WhatsApp.
- **Priya**: Searches Google for "easiest way to sell clothes online and in store".
- **Leo**: Clicks a "Built with OHC" link on another creator's TikTok bio.
- **Fatima**: A local community group organizer recommends OHC for its simplicity and Arabic language support.

**Landing Page CTA**: "Launch Your Business Now - Free" (with no credit card required).

### 3.2 Onboarding
**Step-by-step Wizard (Mobile-First 375px)**
1. **Name & Idea**: "What's the name of your business?" / "What do you sell?" (e.g., "Cakes", "Handyman Services").
2. **AI Magic**: OHC’s "Promoter" AI instantly generates a beautiful, premium (Glassmorphism, Outfit font) draft storefront/profile based on the input.
3. **Core Need**: Select primary feature (e.g., Maya selects "Take custom orders", Leo selects "Book appointments").
4. **Account Creation**: Sign up with Google/Apple or Email.
5. **Live in 10 Min**: The basic site goes live on an OHC subdomain (e.g., `maya-bakes.ohc.site`).

*Minimum Inputs*: Business Name, Category. Everything else is deferred.

### 3.3 Activation
**What defines success (Day 1 / Week 1)?**
- **Day 1**: The user adds their first product/service, connects their payment method (Stripe Onboarding), and views their live site.
- **Week 1**: Receiving the first real order or booking and processing it through the Operations Agent ("The Manager").

### 3.4 Retention
**What keeps them coming back?**
- **Push Notifications**: Real-time alerts for new orders, messages, or bookings.
- **AI Activity Feed**: The dashboard shows what the AI agents have done (e.g., "The Ambassador drafted 3 replies to your DMs").
- **Weekly Health Reports**: The "Business Advisory" agent sends plain-language summaries (e.g., "Tuesday was your busiest day!").

### 3.5 Revenue (Upgrade Triggers)
**Free → Starter ($9/mo)**
- **Trigger**: Maya wants a custom domain (`mayabakes.com`).
- **Trigger**: Carlos hits the 100 AI actions/month limit and needs more automated quotes.
- **Presentation**: In-context upgrade prompt when trying to access a premium feature or nearing a limit. "Upgrade to Starter to unlock custom domains and more AI power."

### 3.6 Referral
**The Viral Loop**
- "Built with OHC" badge on free tier sites.
- **In-App Prompt**: After a successful week (e.g., 5 orders processed), the "Salesperson" agent suggests: "Give a friend $20 off their first month, and you get a free month of Pro!"

## 4. Persona Sequence Diagrams

### 4.1 Maya's Journey (The Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as IG Ad
    participant OHC as OHC App
    participant AI as Promoter Agent
    participant Customer

    Maya->>Ad: Clicks "Launch in 10 mins"
    Ad->>OHC: Lands on Sign Up
    Maya->>OHC: Enters "Maya's Cakes"
    OHC->>AI: Generate Storefront Draft
    AI-->>OHC: Draft Storefront (Glassmorphism UI)
    OHC-->>Maya: "Your store is ready. Connect Stripe?"
    Maya->>OHC: Connects Stripe (Deferred)
    Maya->>OHC: Uploads 3 Cake Photos
    OHC-->>Maya: Store is Live!
    Customer->>OHC: Views Store & DMs for Vegan option
    OHC->>AI: Drafts reply ("Yes, we do vegan!")
    AI-->>Maya: Notification: Approve Draft?
    Maya->>OHC: Approves Reply (1-tap)
```

### 4.2 Carlos's Journey (The Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant Ref as WhatsApp Referral
    participant OHC as OHC App
    participant AI as Salesperson Agent
    participant Client

    Carlos->>Ref: Clicks Referral Link
    Ref->>OHC: Lands on Sign Up
    Carlos->>OHC: Enters "Carlos Repairs"
    OHC->>AI: Generate Service Listing
    AI-->>OHC: Draft Listing (Plumbing, Painting)
    OHC-->>Carlos: "Set your hourly rate"
    Carlos->>OHC: Sets rate & availability
    OHC-->>Carlos: Calendar is Live!
    Client->>OHC: Books "Leaky Faucet" slot
    OHC->>AI: Generate Quote based on description
    AI-->>Carlos: Notification: Approve Quote?
    Carlos->>OHC: Approves Quote
    OHC-->>Client: Sends Quote for Deposit
```

### 4.3 Priya's Journey (The Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC Web/App
    participant POS as Tap-to-Pay (Stripe Terminal)
    participant AI as Advisor Agent

    Priya->>Search: Clicks OHC Result
    Search->>OHC: Signs Up via Desktop
    Priya->>OHC: Imports/Adds Clothing Inventory (Variants)
    OHC-->>Priya: Online Store & POS Ready
    Priya->>POS: Uses Phone to tap-to-pay in store
    POS-->>OHC: Records Sale & Syncs Inventory
    OHC->>AI: Analyze Sales (End of Week)
    AI-->>Priya: Weekly Report: "Red dresses sold out, restock needed."
```

### 4.4 Leo's Journey (The Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant TikTok
    participant OHC as OHC App
    participant Zoom as Zoom Integration
    participant AI as Ambassador Agent
    participant Student

    Leo->>OHC: Signs up & Sets up Lesson Packages
    OHC-->>Leo: Link-in-bio URL generated
    Leo->>TikTok: Adds URL to profile
    Student->>TikTok: Clicks link
    Student->>OHC: Books 4-lesson package & Pays
    OHC->>Zoom: Generate Meeting Link
    Zoom-->>OHC: Link created
    OHC-->>Student: Sends Confirmation + Link
    loop After 2 weeks of inactivity
        OHC->>AI: Check Student Status
        AI-->>Leo: Draft Follow-up Email ("Ready for another lesson?")
    end
```

### 4.5 Fatima's Journey (The Food Cart)
```mermaid
sequenceDiagram
    actor Fatima
    participant Comm as Community Organizer
    participant OHC as OHC App (Arabic UI)
    participant Customer

    Fatima->>Comm: Hears about OHC
    Fatima->>OHC: Signs up on low-end Android (Arabic Mode)
    Fatima->>OHC: Adds Menu Items (Photos, Prices)
    OHC-->>Fatima: Simple Storefront Ready
    Customer->>OHC: Pre-orders Falafel Wrap & Pays
    OHC-->>Fatima: LOUD Push Notification & Big UI Alert
    Fatima->>OHC: Marks "Preparing"
    OHC-->>Customer: Notification: "Order is being prepared"
    Fatima->>OHC: Toggles Falafel "Sold Out" when ingredients run out
```

## 5. Identified Friction Points

1.  **Stripe Onboarding**: Standard KYC procedures can be intimidating. We must ensure the UI clearly explains *why* information is needed and allow deferring steps until the first payout.
2.  **Product/Inventory Entry**: Typing descriptions is tedious. The "Promoter" AI must auto-generate descriptions from simple titles and photos.
3.  **Trusting the AI**: Non-technical users may fear the AI will say the wrong thing to a customer. The "Draft-for-Review" workflow (1-tap approval) is critical for building trust before enabling full autonomy.
4.  **Language Barriers**: The UI, especially error messages and setup instructions, must be robustly translated (starting with Arabic and Spanish) to serve users like Fatima.
5.  **Offline/Poor Connectivity**: Fatima operating her cart might lose signal. The app must queue updates (like marking an item sold out) and sync when the connection is restored, showing optimistic UI updates in the meantime.