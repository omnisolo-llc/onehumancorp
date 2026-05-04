# Architecture: End-to-End Business Journey for OHC Personas

## Problem Statement
The OHC platform must serve a diverse set of real-world small business owners (e.g., Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator). These users share a common goal: launching and growing their business entirely from a mobile device without technical expertise. The overarching business journeys—from initial acquisition to sustainable revenue generation and referral loops—must be explicitly mapped out architecturally. This ensures the system naturally supports their progression through Acquisition, Onboarding, Activation, Retention, Revenue generation, and Referral, while identifying critical friction points where non-technical users might abandon the platform.

## Research Report

### Competitive Analysis
- **Shopify / Wix / Squarespace**: Focus primarily on semi-technical users and take 30-60 minutes to set up. Mobile management is partial or an afterthought.
- **OHC Advantage**: Zero technical knowledge required. We treat AI as infrastructure (not a bolted-on chatbot) to handle complex setup steps. The management platform is genuinely mobile-first (375px native focus).

### Journey Stages
- **Acquisition**: Initial discovery via organic search, social media ads (Instagram/TikTok), or word-of-mouth. The CTA promises a functional business setup in under 10 minutes.
- **Onboarding**: A highly guided, AI-driven wizard flow minimizing upfront data entry.
- **Activation**: The "Aha!" moment (e.g., live storefront, first booking, first payment).
- **Retention**: Engagement through actionable notifications and AI-generated weekly health reports.
- **Revenue**: Transitioning from free to paid plans based on reached limits or need for premium features (custom domains).
- **Referral**: Incentivized sharing creating a viral loop.

### Identified Friction Points
1. **Cognitive Overload**: Requesting too much setup information upfront causes drop-offs.
2. **Payment Gateway**: Technical jargon during Stripe integration stalls progress.
3. **Availability Sync**: Difficulties mapping real-world schedule availability to digital systems without intuitive AI assistance.

## Design Doc

### Key Design Decisions
- **Progressive Profiling**: The onboarding flow requests the absolute minimum required data. Advanced settings are dynamically suggested by the Business Advisory Agent post-activation.
- **AI-First Setup**: The Marketing & Advertising Agent generates the initial website layout and copy based on minimal inputs.
- **Mobile-First Constraint**: All journey flows must be designed and tested starting at the 375px breakpoint, utilizing native mobile keyboards.
- **Asynchronous Processing**: Non-critical setup tasks are handled via the KAIROS Orchestrator.

### Architecture Diagrams (Mermaid.js)

#### Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Cust as Customer

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
```

#### Carlos (The Handyman) Journey
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
    AI_Mark->>OHC: Generates Service Listings & Booking
    OHC->>Carlos: Booking Page Live! (Activation)
    Cust->>OHC: Requests Quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
```

#### Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Adv as Advisory Agent
    participant POS as In-Store POS

    Priya->>Search: Searches "Easy online store"
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Syncs initial inventory
    AI_Mark->>OHC: Generates Storefront with variants
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale
    POS->>OHC: Update Inventory
    OHC->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: Suggests upgrade for automated alerts (Revenue)
```

#### Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Student as Student

    Leo->>Social: Adds OHC link
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to lessons
    OHC->>AI_Ops: Sync Calendar & Generate Links
```

#### Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant Cust as Customer

    Fatima->>Local: Shows QR Code
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation
    AI_Mark->>OHC: Generates Bilingual Menu
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>OHC: Scans QR, places pre-order
    OHC->>Fatima: Loud Audio Notification
```

## Implementation Prompt
**To Implementer Agent:**
Implement the foundational user onboarding flow and dashboard state management that supports the progression from Acquisition to Activation across all platforms (Flutter/Go Router). Define the required entity types and relations (business, product, customer) using Go/Bazel struct definitions. Build the mobile-first (375px) UI wizard that guides a user through initial setup with minimal input. Provide 100% test coverage including an end-to-end flow from login to generated storefront.

**Priority**: P0
**Estimated Scope**: Large
