# Title
Business Journey Architecture

# Problem Statement
Small business owners—from bakers to handymen—often lack the technical skills to launch, run, and grow their businesses online. Current platforms are fragmented and require manual setup, leading to high drop-off rates and abandoned tools. OHC aims to solve this by providing a unified, AI-driven platform that guides users from zero to a live business in under 10 minutes. This architecture defines the end-to-end user journey for all personas (Maya, Carlos, Priya, Leo, Fatima) to ensure the system naturally supports their progression through Acquisition, Onboarding, Activation, Retention, Revenue, and Referral without requiring a manual.

# Research Report

### Persona-Specific Pain Point Summaries
- **Maya (baker, 28)**: Struggles with manually tracking custom orders via Instagram DMs and chasing deposits. Needs an automated storefront and AI that can answer simple queries ("do you do vegan cakes?") while she sleeps.
- **Carlos (handyman, 42)**: Relies on word of mouth and phone calls; lacks a centralized booking system and quote generation tool. Needs a simple way to list services with prices and collect deposits securely from an Android phone.
- **Priya (boutique owner, 35)**: Struggles to sync in-store sales with online inventory. Needs omnichannel support, product variants (size/color), tap-to-pay POS capabilities, and unified analytics on her mobile device.
- **Leo (music tutor, 22)**: Juggles teaching online and in-person without an integrated booking and meeting system. Needs subscription management, automated calendar sync, and a link-in-bio portfolio.
- **Fatima (food cart, 50)**: Has limited English and low technical literacy; uses a low-end Android phone. Needs extreme simplicity, a bilingual photo menu, and loud audio notifications for pre-orders.

### Key advantages and risks
**Advantages**:
- Frictionless onboarding via conversational AI.
- Centralized data model eliminates syncing errors across multiple tools.
- Unified customer experience and mobile-first design.

**Risks**:
- AI misinterpreting user intent during onboarding or customer interaction.
- Over-simplification limiting advanced users.
- Connectivity issues for users like Fatima on low-end devices without robust offline support.

### Rough Pricing & Competitor Comparison
| Platform | Key Features | Pricing | OHC Advantage |
|---|---|---|---|
| Shopify | E-commerce, Inventory, POS | $39/mo + fees | Requires setup time; OHC is zero-setup. |
| Wix | Website Builder, Bookings | $16/mo - $59/mo | Often needs desktop; OHC is 100% mobile native. |
| Calendly | Scheduling, Routing | $10/mo - $16/mo | Fragmented; OHC has native unified billing & CRM. |
| OHC | AI-driven, Unified platform | Free to $79/mo | Conversational setup, zero manual configuration. |

### Whether it works in both Cloud and Standalone modes
The underlying journey orchestration layer leverages a hybrid mesh design. User state (onboarding progress, activation status) and AI agent interactions seamlessly operate in Cloud mode via distributed databases/Redis, and in Standalone mode using local IPC and SQLite, ensuring identical behavior and robust offline capabilities for mobile users.

# Design Doc

### Key Design Decisions
- **AI-Driven Progressive Onboarding**: Instead of traditional forms, users converse with an AI agent to build their business profile step-by-step.
- **Mobile-First UX**: The entire flow is optimized for 375px viewports, emphasizing tap targets, clear typography (Outfit & Inter), and offline-first data caching.
- **Unified Event Pipeline**: User progression (from Acquisition to Referral) emits standard events triggering relevant AI departments (e.g., Marketing, Sales).

### Business Journey Diagrams

#### Maya (Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Cust as Customer
    participant Stripe as Stripe API

    Maya->>Ad: Clicks "Launch Bakery in 5 mins"
    Maya->>OHC: Downloads App & Opens
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: Asks "What do you sell?"
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>OHC: Generates Storefront & Menu
    OHC->>Maya: Storefront Live! (Activation)
    Cust->>OHC: Messages via IG DM "Vegan?"
    OHC->>AI_Ops: Drafts Reply
    AI_Ops-->>Cust: "Yes, we do vegan cakes!"
    Cust->>OHC: Places Order & Pays Deposit
    OHC->>Stripe: Process Payment
    Stripe-->>OHC: Success
    OHC->>Maya: Push Notification "New Order Paid!" (Retention)
    Maya->>OHC: Clicks "Upgrade to Starter" (Revenue)
    OHC->>Stripe: Setup Recurring Billing
    Maya->>OHC: Shares Store Link with Friend (Referral)
    OHC->>Maya: Credit "Bakery Credit" for Referral
```

#### Carlos (Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant OHC as OHC Web App
    participant AI_Mark as Marketing Agent
    participant AI_Sales as Sales Agent
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
    Carlos->>OHC: Taps "Earn $50: Refer a Pro" (Referral)
    Carlos->>Cust: Taps "Send 10% Discount to Friend" (Referral Loop)
```

#### Priya (Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Adv as Advisory Agent
    participant POS as In-Store POS

    Priya->>Search: Searches "Easy online store for boutique"
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Priya: Syncs initial inventory
    AI_Mark->>OHC: Generates Storefront with variants
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale via phone
    POS->>OHC: Update Inventory
    OHC->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: "Inventory low. Upgrade tier for automated re-order alerts."
    Priya->>OHC: Selects "Pro Plan" (Revenue)
    OHC->>Priya: Enables Multi-Store Sync
```

#### Leo (Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Student as Student

    Leo->>Social: Adds OHC link to TikTok bio
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile & Subscriptions
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to 4 lessons/mo
    OHC->>AI_Ops: Sync Calendar & Generate Zoom Links
    AI_Ops-->>Student: Sends Schedule
    OHC->>Leo: Notification "New Subscriber!" (Retention)
    Leo->>OHC: Uses Referral code to invite another tutor (Referral)
```

#### Fatima (Food Cart)
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage
    participant OHC as OHC App (Arabic/English)
    participant AI_Mark as Marketing Agent
    participant OHC_UI as Simplified Mobile UI
    participant Cust as Customer

    Fatima->>Local: Shows QR Code
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation (Photos + Prices)
    AI_Mark->>OHC: Generates Bilingual Menu
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>OHC: Scans QR, views menu, places pre-order
    OHC->>OHC_UI: Loud Audio Notification + Simple Order Card
    Fatima->>OHC_UI: Taps "Preparing"
    OHC_UI->>Cust: Updates Status
    Fatima->>OHC_UI: Prints Daily Summary (Retention)
```

# Implementation Prompt
Implement the foundational user onboarding flow and dashboard state management that supports the progression from Acquisition to Activation. Build the mobile-first (375px) conversational UI wizard that captures the user's business context. Ensure all interactions feel premium (using OHC Design Standards: Glassmorphism, correct typography) and handle optimistic updates for low-connectivity environments. The final wizard step must trigger the instantaneous generation of a functional Storefront or Booking Page. Ensure that the core data pipelines emitting stage-transition events (Onboarding -> Activated) are properly instrumented.

# Priority
P0

# Estimated Scope
Large
