# [architecture] Business Journey Architecture: End-to-End Persona Workflows

## Title
Business Journey Architecture: End-to-End Persona Workflows

## Problem Statement
The user journey for non-technical small business owners (like Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator) across existing platforms (Shopify, Wix) is fragmented and heavily reliant on manual configuration. They face a high barrier to entry ("Setup Complexity") and significant operational fatigue once live. OHC needs a definitive, radically simple, end-to-end journey architecture that maps exactly how real-world personas interact with the platform from discovery to daily operations, with AI agents invisibly handling the complexity.

## Research Report
Based on the market feature gap analysis and top SMB pain points:
- **Shopify & Wix**: Onboarding takes 20-60 minutes. Users are bombarded with technical jargon (DNS, Liquid templates) and isolated tools requiring manual wiring.
- **Durable**: Offers fast storefront generation (<1m) but lacks operational depth for complex flows (e.g., custom orders with deposits, physical POS sync).
- **OHC Target**: Reduce "Time to Live" to under 10 minutes (or <1m for instant generation) with zero technical knowledge.
- **Journey Friction Points Identified**:
  1. *Acquisition & Onboarding*: Overwhelm from too many setup questions.
  2. *Activation*: Stalled progress after the first product is added but before the first sale.
  3. *Retention*: The "never-ending inbox" and manual marketing tasks cause churn.
  4. *Revenue*: Lack of clear visibility into profit margins prevents business growth.

## Design Doc

### Key Design Decisions & Rationale
1. **Conversational Instant Onboarding**: Instead of multi-step wizards, onboarding uses a single conversational input ("Tell us about your business"). This minimizes friction and leverages the Advisor Agent to fill 80% of metadata.
2. **Mobile-First (375px) Operations**: All daily management (approvals, insights) happens via a continuous activity feed on the mobile dashboard. This matches the reality of solopreneurs who manage their business from their phones.
3. **Event-Driven AI Teammates**: Agents are not reactive tools; they proactively monitor the event mesh (e.g., new order, new DM) and draft responses or tasks for 1-tap approval.

### AI Agent Integration Points
- **The Advisor**: Extrapolates business metadata during onboarding and generates weekly human-language health briefings.
- **The Promoter**: Automatically designs the storefront and schedules 7-day social media calendars upon new product creation.
- **The Ambassador**: Listens to inbound DMs/messages, drafts context-aware replies, and queues them for approval.
- **The Manager (Operations)**: Tracks inventory velocity, flags low stock, and manages order state transitions.
- **The Accountant**: Monitors transactions and provides plain-language financial summaries.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) - Complete Lifecycle Journey
```mermaid
sequenceDiagram
    actor Maya as Maya (Phone)
    participant OHC as OHC Mobile App
    participant Promoter as Promoter Agent
    participant Op as Operations Agent
    participant CS as Customer Success Agent

    %% Acquisition & Onboarding
    Maya->>OHC: Installs App (IG Ad)
    Maya->>OHC: Types "I bake custom vegan cakes in Seattle"
    OHC->>Promoter: Generate Live Site & Social Assets
    Promoter-->>Maya: "Your site is live! Added 1 sample cake."

    %% Activation
    Maya->>OHC: Shares Site Link on IG Bio

    %% Retention (Daily Operations)
    loop Daily Tasks
        IG->>CS: DM: "Do you do gluten free?"
        CS-->>Maya: Drafts Reply: "Yes! Here is the link..."
        Maya->>CS: 1-Tap Approve
    end

    %% Revenue
    Op->>Op: Customer Orders Cake + Pays Deposit
    Op-->>Maya: "New Order! $50 Deposit Received."

    %% Referral
    CS->>CS: Order Completed
    CS-->>Customer: "Love your cake? Refer a friend for 10% off!"
```

#### 2. Carlos (The Handyman) - Complete Lifecycle Journey
```mermaid
sequenceDiagram
    actor Carlos as Carlos (Android)
    participant OHC as OHC Mobile App
    participant Sales as Sales Agent
    participant Op as Operations Agent

    %% Acquisition & Onboarding
    Carlos->>OHC: Word-of-mouth referral
    Carlos->>OHC: Types "I do general home repairs in Austin"
    OHC->>Sales: Generate Service List & Booking Form
    Sales-->>Carlos: "Booking page ready. Add your prices."

    %% Activation
    Carlos->>OHC: Adds Pricing Matrix & Connects Google Calendar

    %% Retention (Daily Operations)
    loop Quote Flow
        Customer->>OHC: Requests "Fix Leaky Pipe"
        Sales->>Sales: Drafts Quote ($150)
        Sales-->>Carlos: "Quote Draft Ready"
        Carlos->>Sales: 1-Tap Approve
    end

    %% Revenue
    Op->>Op: Customer Pays $50 Deposit & Picks Time
    Op-->>Carlos: "Tuesday 2PM Booked!"

    %% Referral
    Sales->>Sales: Job Finished
    Sales-->>Customer: "Please leave Carlos a review!"
```

#### 3. Priya (The Boutique Owner) - Complete Lifecycle Journey
```mermaid
sequenceDiagram
    actor Priya as Priya (iPhone/MacBook)
    participant OHC as OHC Hybrid App
    participant Promoter as Promoter Agent
    participant Op as Operations Agent
    participant Fin as Finance Agent

    %% Acquisition & Onboarding
    Priya->>OHC: Seeks online expansion from physical store
    Priya->>OHC: Connects Stripe Terminal & uploads CSV inventory
    OHC->>Promoter: Generate Online Store matching physical layout

    %% Activation
    Priya->>OHC: Publishes Store

    %% Retention (Daily Operations)
    loop Inventory Management
        Priya->>OHC: Taps Phone to charge in-store customer
        Op->>Op: Syncs inventory (Online Stock -1)
        Op-->>Priya: "Blue Dress Low Stock Alert"
    end

    %% Revenue
    Fin->>Fin: Calculates Daily Omni-channel Profit
    Fin-->>Priya: "Daily Report: $500 Online, $800 In-Store"

    %% Referral
    Promoter->>Promoter: New Stock Arrives
    Promoter-->>Priya: Drafts Email Newsletter to top customers
    Priya->>Promoter: 1-Tap Approve
```

#### 4. Leo (The Music Tutor) - Complete Lifecycle Journey
```mermaid
sequenceDiagram
    actor Leo as Leo (Phone)
    participant OHC as OHC Mobile App
    participant Op as Operations Agent
    participant CS as Customer Success Agent

    %% Acquisition & Onboarding
    Leo->>OHC: Clicks TikTok Link-in-bio Ad
    Leo->>OHC: "I teach guitar lessons over Zoom"
    OHC->>Op: Create Subscription Packages & Calendar Sync

    %% Activation
    Leo->>OHC: Links Zoom Account

    %% Retention (Daily Operations)
    loop Follow-up
        CS->>CS: Student hasn't booked in 2 weeks
        CS-->>Leo: "Drafted Check-in Email to Sarah"
        Leo->>CS: 1-Tap Approve
    end

    %% Revenue
    Op->>Op: Monthly Subscription Bills Automatically
    Op-->>Leo: "Monthly Package Renewed for 5 Students"

    %% Referral
    CS->>CS: Student completes 10th lesson
    CS-->>Student: "Share Leo's TikTok with a friend!"
```

#### 5. Fatima (The Food Cart Operator) - Complete Lifecycle Journey
```mermaid
sequenceDiagram
    actor Fatima as Fatima (Low-end Android)
    participant OHC as OHC App (Arabic/English)
    participant Op as Operations Agent

    %% Acquisition & Onboarding
    Fatima->>OHC: Needs digital menu for pre-orders
    Fatima->>OHC: Uploads photos of 5 dishes
    OHC->>Op: Generate Multi-lingual Menu with Sold-Out Toggles

    %% Activation
    Fatima->>OHC: Toggles "Accepting Pre-Orders" ON

    %% Retention (Daily Operations)
    loop Lunch Rush
        Customer->>OHC: Pre-orders Falafel Wrap (Paid)
        Op-->>Fatima: High-Volume Push Alert "New Order!"
    end

    %% Revenue
    Fatima->>OHC: Hits "Print Daily Order List"

    %% Referral
    Op->>Op: End of Day
    Op-->>Customer: "Hope you enjoyed the food! Review us on Google."
```

### Mobile UX Flow (375px First)
1. **Acquisition (Link-in-bio / Ad)**: User taps "Start my business".
2. **Onboarding (Conversational)**: A single screen with native mobile keyboard. Prompt: "What do you do?" User inputs 1-2 sentences.
3. **Generation**: Loading screen with micro-animations. Agents generate the storefront, tagline, and initial product drafts in parallel.
4. **Activation / Dashboard**:
   - Top: "Agent Actions Today" Feed (e.g., "The Promoter drafted a welcome post").
   - Middle: "Quick Actions" (Add Product, View Live Site).
   - Bottom: Native mobile navigation bar (Home, Orders, Chat, Reports).
5. **Approval Workflow**: Tapping an agent draft opens a modal with "Edit" or "Approve & Send" buttons. Minimum touch target 44x44px.

## Implementation Prompt
Implement the end-to-end "Business Journey Architecture" by creating the core orchestration flows for the 5 persona journeys defined in this doc.
1. Build the Conversational Onboarding UI component (Slint/Flutter, targeting 375px) that captures the single text input and triggers the generation pipeline.
2. Implement the "Agent Activity Feed" on the mobile dashboard.
3. Wire the Teammate Mesh to route `MessageReceived` and `OrderPlaced` events to the appropriate AI departments (Customer Success, Operations) to populate the feed with draft actions.
4. Ensure all interactions utilize the OHC Premium Token library (Glassmorphism, 20px blur) and are fully functional on a 375px width.

## Priority
P0

## Estimated Scope
Large
