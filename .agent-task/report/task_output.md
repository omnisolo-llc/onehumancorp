# [Business Journey] End-to-End User Journey Architecture

## Problem Statement
For the everyday small business owner—like a home baker, freelance handyman, or food cart operator—launching and managing an online presence is technically overwhelming. Existing platforms like Shopify, Wix, and Squarespace require significant setup time, basic technical knowledge (e.g., DNS setup, responsive design concepts), and constant manual intervention to operate. This friction causes many non-technical founders to abandon their online ambitions or rely entirely on fragmented social media interactions. OHC must provide a zero-configuration, AI-driven, and mobile-first experience that empowers users to go from idea to a live, transactional business in under 10 minutes without touching code or jargon.

## Research Report
### Competitive Analysis
| Platform | Setup Time | Tech Skill Required | Mobile Management | AI Integration | Target Persona |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **OHC** | **< 10 min** | **Zero** | **Native/Mobile-First** | **Deep, invisible background agents** | **Non-technical (Maya, Carlos, Fatima)** |
| Shopify | 30-60 min | Low/Medium | Partial | Chatbot (Sidekick) | SMB / Tech-savvy |
| Wix | 20-40 min | Low/Medium | Partial | Website generation | Semi-technical |
| Squarespace| 30-60 min | Low | Poor | Limited | Creative Professional |

**Key Findings:**
1. **Mobile is the computer:** 85% of our target demographic (Maya, Carlos, Fatima) do not own or prefer not to use a laptop for business management. Shopify and Wix have companion apps, but complex tasks (like setting up variants or modifying themes) require desktop. OHC must be 100% manageable via a 375px screen.
2. **AI as infrastructure, not a chatbot:** Current market solutions treat AI as an optional assistant. OHC treats AI as the fundamental business operations engine (Marketing, Support, Operations) working silently in the background.
3. **Friction at Onboarding:** The highest drop-off rate in competitors occurs when asking for DNS setup, payment gateway keys, or catalog initial upload. OHC must automate this through conversational AI or one-click integrations (e.g., pulling data from an existing Instagram page).

## Design Doc

### The 6 Stages of the OHC Business Journey
1. **Acquisition:** How the user discovers OHC.
2. **Onboarding:** The step-by-step wizard to go live.
3. **Activation:** The "Aha!" moment (first product added, first dollar earned).
4. **Retention:** Daily engagement loops driven by AI insights.
5. **Revenue:** The upgrade path from Free to Starter/Pro.
6. **Referral:** Organic viral loops (e.g., "Powered by OHC" link-in-bio).

---

### Persona Journeys & Architecture Diagrams

#### 1. Maya (The Home Baker, 28)
**Goal:** Sell custom cakes with deposits via Instagram DMs.

```mermaid
sequenceDiagram
    autonumber
    actor Maya
    participant Instagram
    participant OHC_App
    participant AI_Marketing
    participant AI_Ops
    participant AI_Advisory

    %% Acquisition
    Maya->>Instagram: Sees OHC Ad ("Turn DMs into a Bakery in 5 mins")
    %% Onboarding
    Maya->>OHC_App: Downloads & connects IG profile
    OHC_App->>AI_Marketing: Generate storefront from IG photos
    AI_Marketing-->>OHC_App: Storefront Ready
    %% Activation
    Maya->>OHC_App: Adds "Custom Cake" product with $50 deposit rule
    %% Retention
    Customer->>Instagram: DM: "Do you do vegan cakes?"
    AI_Ops->>Instagram: Auto-reply: "Yes! Here is the link to order." (while Maya sleeps)
    Customer->>OHC_App: Pays $50 deposit
    OHC_App-->>Maya: Push Notification: "New Cake Order!"
    %% Revenue
    AI_Advisory->>Maya: Weekly Report: "Vegan cakes are trending. Upgrade to Pro for automated email marketing."
    Maya->>OHC_App: Upgrades to $29/mo Pro Tier
    %% Referral
    Maya->>Instagram: Adds OHC link-in-bio
```

#### 2. Carlos (The Freelance Handyman, 42)
**Goal:** Move from word-of-mouth to professional online booking.

```mermaid
sequenceDiagram
    autonumber
    actor Carlos
    participant OHC_App
    participant AI_Sales
    participant Customer

    %% Acquisition
    Carlos->>OHC_App: Friend recommendation
    %% Onboarding
    Carlos->>OHC_App: Enters Name & "Handyman Services"
    OHC_App->>AI_Sales: Generate service list & standard pricing
    %% Activation
    Carlos->>OHC_App: Reviews and publishes booking page
    %% Retention
    Customer->>OHC_App: Books "Plumbing Fix" & pays $20 booking fee
    OHC_App-->>Carlos: SMS Notification: New Booking at 2 PM Tomorrow
    Customer->>OHC_App: "My sink is leaking rapidly."
    AI_Sales->>Customer: Auto-sends revised quote + prep instructions
    %% Revenue
    Carlos->>OHC_App: Upgrades to remove OHC branding (Starter $9/mo)
```

#### 3. Priya (The Boutique Owner, 35)
**Goal:** Sync in-store inventory with an online storefront.

```mermaid
sequenceDiagram
    autonumber
    actor Priya
    participant OHC_App
    participant Stripe_Terminal
    participant AI_Finance

    %% Acquisition
    Priya->>OHC_App: Searches "Easy POS and website synced"
    %% Onboarding
    Priya->>OHC_App: Scans clothing tags to auto-create catalog
    %% Activation
    Priya->>Stripe_Terminal: First in-person Tap-to-Pay transaction
    OHC_App->>OHC_App: Deducts inventory count
    %% Retention
    AI_Finance->>Priya: "Red dress size M sold out. Email waitlist when restocked?"
    %% Revenue
    Priya->>OHC_App: Upgrades to Business tier for unlimited products & custom domain
```

#### 4. Leo (The Music Tutor, 22)
**Goal:** Automate lesson bookings, Zoom links, and student follow-ups.

```mermaid
sequenceDiagram
    autonumber
    actor Leo
    participant TikTok
    participant OHC_App
    participant AI_CustomerSuccess

    %% Acquisition
    Leo->>OHC_App: Signs up to create link-in-bio
    %% Onboarding
    Leo->>OHC_App: Connects Google Calendar & sets $40/hr rate
    %% Activation
    Leo->>TikTok: Posts guitar cover with OHC link
    Student->>OHC_App: Books 4-lesson subscription
    OHC_App-->>Student: Auto-generates & sends Zoom link
    %% Retention
    AI_CustomerSuccess->>Student: "It's been 2 weeks since your last lesson. Ready to book?"
    %% Referral
    Student->>Friend: Shares Leo's OHC booking link
```

#### 5. Fatima (The Food Cart Operator, 50)
**Goal:** Take pre-orders easily in multiple languages.

```mermaid
sequenceDiagram
    autonumber
    actor Fatima
    participant OHC_App
    participant AI_Ops

    %% Acquisition
    Fatima->>OHC_App: Local community center referral
    %% Onboarding
    Fatima->>OHC_App: Takes photos of menu items
    AI_Ops->>OHC_App: Extracts dishes, prices, and translates to English/Arabic
    %% Activation
    Customer->>OHC_App: Scans QR code on cart, orders Halal Platter
    %% Retention
    OHC_App-->>Fatima: High-volume audio alert on Android phone
    Fatima->>OHC_App: Taps "Sold Out" on chicken
    OHC_App->>Customer: Menu instantly updates to hide chicken
    %% Revenue
    Fatima->>OHC_App: Uses Free tier indefinitely (OHC takes small transaction fee)
```

### Identified Friction Points & Mitigations
- **Friction:** Initial catalog creation is tedious.
  - **Mitigation:** AI agent generates catalog from a single Instagram link or by scanning physical menus/tags using the camera.
- **Friction:** Setting up a custom domain is intimidating.
  - **Mitigation:** OHC provisions `[business].ohc.app` instantly. If they upgrade, OHC handles DNS programmatically behind the scenes.
- **Friction:** Getting the first customer.
  - **Mitigation:** The AI Promoter department automatically generates an optimized social media post with a booking/purchase link immediately upon going live.

## Implementation Prompt
**Task for Implementer:**
Implement the user onboarding API and initial screen flows to support the "10-minute Idea to Live Business" promise.
- Create the backend endpoints for business creation, integrating the AI Marketing agent to auto-generate the initial store configuration from user text input or social media link.
- Develop the Flutter UI (mobile-first 375px) for the 3-step onboarding wizard: (1) Business Name/Type, (2) Auto-generation loading screen with tips, (3) The "You are Live" dashboard with the shareable link.
- Ensure the UI utilizes the OHC Premium Token library (Glassmorphism, Outfit/Inter typography).
- Verify end-to-end functionality via an E2E Playwright/Flutter integration test.

**Acceptance Criteria:**
- User can create an account and business in under 3 screens.
- AI correctly interprets the business type and populates the initial database entities (Products, Services).
- The dashboard is perfectly usable on a 375px width simulator.

**Priority:** P0
**Estimated Scope:** Large
