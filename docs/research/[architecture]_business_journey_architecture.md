# Architecture Brief: Business Journey Architecture

## Title
OHC Business Journey Architecture: From Zero to Live in 10 Minutes

## Problem Statement
Small business owners—like Maya the baker, Carlos the handyman, and Fatima the food cart operator—need a digital platform that feels as simple and responsive as a native calculator app. They have no time to learn complex concepts like CNAME records, payment gateways, or web hosting. The goal is to design an architecture that supports a frictionless end-to-end journey from their first interaction (Acquisition) to daily usage (Retention) and expansion (Revenue/Referral). The current onboarding and usage flows on traditional platforms (like Shopify and Wix) are too complex and fragmented, causing non-technical users to abandon the process.

## Research Report
- **Competitive Analysis:**
  - *Shopify/Wix/Squarespace:* Often require desktop interaction for complete setup, use technical jargon (e.g., DNS, SSL, integrations), and lack a unified mobile-first onboarding.
  - *OHC Advantage:* A genuinely mobile-first (375px baseline) experience where AI agents handle the complexity invisibly.
- **Friction Points:**
  - Initial configuration requests too much data upfront.
  - Linking bank accounts or configuring payment gateways stalls the "Aha!" moment.
  - Mapping real-world inventory or availability to digital systems without AI assistance is tedious.
- **Key Metric Target:** Time-to-activation (live storefront/booking page) must be under 10 minutes.

## Design Doc

### User Journey Sequence Diagrams (Mermaid.js)

#### Maya (Baker) - Physical Products & Social Selling
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Ops as The Manager
    participant Cust as Customer
    participant Stripe as Payment Gateway

    Maya->>Ad: Clicks "Launch Bakery in 5 mins"
    Maya->>OHC: Installs & Opens App
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: Prompt: "What do you sell?"
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>OHC: Generates Storefront & Photo Catalog Draft
    OHC->>Maya: Storefront Live! (Activation)
    Cust->>OHC: DMs via Instagram "Do you do vegan?"
    OHC->>AI_Ops: Drafts Reply
    AI_Ops-->>Maya: Notification: "Approve DM Reply"
    Maya->>AI_Ops: 1-Tap Approve
    AI_Ops-->>Cust: "Yes, we do vegan cakes!"
    Cust->>OHC: Places Custom Order & Pays Deposit
    OHC->>Stripe: Process Payment
    Stripe-->>OHC: Success
    OHC->>Maya: Push Notification "New Order Paid!" (Retention)
    Maya->>OHC: Clicks "Upgrade to Starter" (Revenue)
```

#### Carlos (Handyman) - Services & Bookings
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant OHC as OHC Web App
    participant AI_Mark as The Promoter
    participant AI_Sales as The Salesperson
    participant Cust as Customer

    Carlos->>WoM: Hears about OHC
    Carlos->>OHC: Visits website on Android
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Carlos: Asks "What services do you offer?"
    Carlos->>AI_Mark: "Plumbing, Painting"
    AI_Mark->>OHC: Generates Service Listings & Booking Calendar
    OHC->>Carlos: Booking Page Live! (Activation)
    Cust->>OHC: Requests Quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: Approves 1-tap
    AI_Sales-->>Cust: Sends Official Quote
    Cust->>OHC: Books Time & Pays Deposit
    OHC->>Carlos: Notification "Job Booked" (Retention)
```

#### Priya (Boutique) - Omnichannel Retail
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Adv as The Advisor
    participant POS as In-Store POS

    Priya->>Search: Searches "Easy online store for boutique"
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Priya: "Take a photo of a receipt or shelf to sync inventory"
    Priya->>AI_Mark: Uploads Photo
    AI_Mark->>OHC: Extracts Variants (Size/Color) & Generates Storefront
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store tap-to-pay via phone
    POS->>OHC: Update Unified Inventory
    OHC->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: "Inventory low on Red Dresses. Upgrade for auto re-order alerts."
    Priya->>OHC: Selects "Pro Plan" (Revenue)
```

#### Leo (Music Tutor) - Digital Subscriptions
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as The Promoter
    participant AI_Ops as The Manager
    participant Student as Student

    Leo->>Social: Adds OHC link to TikTok bio
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile & Subscriptions
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to 4 lessons/mo
    OHC->>AI_Ops: Sync Calendar & Generate Video Links
    AI_Ops-->>Student: Sends Schedule
    OHC->>Leo: Notification "New Subscriber!" (Retention)
```

#### Fatima (Food Cart) - Offline-First Pre-orders
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage (QR)
    participant OHC as OHC App (Low-end Android)
    participant AI_Mark as The Promoter
    participant Cust as Customer

    Fatima->>Local: Shows QR Code on Cart
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation (Photos + Sold-Out Toggles)
    AI_Mark->>OHC: Generates Bilingual Menu (Arabic/English)
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>Local: Scans QR
    Cust->>OHC: Places pre-order & Pays
    OHC->>Fatima: Loud Audio Notification + Simple Order Card (Retention)
    Fatima->>OHC: Taps "Preparing" -> "Ready"
```

### Mobile UX Flow (375px)
- **Onboarding Wizard:**
  - Progressive profiling.
  - Step 1: Business name and type.
  - Step 2: Auto-generate catalog/services via AI prompt or image upload.
  - Step 3: 1-Tap "Go Live".
- **Glassmorphism Shimmer:** Used during the 30-60 second wait while AI generates the storefront.
- **Agent Activity Feed:** Post-activation, the home screen centers on actionable items (e.g., "Approve Quote", "New Order").

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):** Drives the onboarding wizard, extrapolating minimal user input into a fully fleshed-out digital presence (Vibe Coding).
- **The Salesperson (Sales & Acquisition):** Drafts quotes and analyzes customer requests (e.g., for Carlos).
- **The Manager (Operations):** Handles downstream fulfillment, such as booking calendar synchronization and video link generation (e.g., for Leo).
- **The Advisor (Business Advisory):** Contextually suggests tier upgrades based on usage patterns (e.g., inventory tracking for Priya).

### Key Design Decisions
- **Mobile-First Exclusively:** The entire journey, including advanced settings, is achievable on a 375px screen without horizontal scrolling.
- **Progressive Disclosure:** Do not ask for custom domains or complex shipping rules during onboarding. Introduce these later via "The Advisor".
- **Optimistic UI:** Interactions (like approving a quote) must feel instantaneous locally, even if the backend is syncing.

## Implementation Prompt
**To Implementer Agent:**
Implement the foundational user onboarding flow and unified dashboard state management that supports the progression from Acquisition to Activation for all business types.
- Build the mobile-first (375px) UI wizard that guides a user through the initial setup, ensuring that advanced configurations are deferred.
- The final step of the wizard should instantly generate a functional "Storefront/Booking Page" view, satisfying the 'Activation' milestone.
- Ensure that interactions feel premium (Glassmorphism, correct typography) and use plain, jargon-free language (The "Grandmother Test").
- Integrate the AI agent handoffs (e.g., passing user intent to "The Promoter" for generation).
- Include Playwright E2E test coverage verifying a successful run-through from login to the generated storefront for at least two personas (e.g., product and service).

## Priority
P0

## Estimated Scope
Large
