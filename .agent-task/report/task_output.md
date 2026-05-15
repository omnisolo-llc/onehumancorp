# 🔎 Scout: Tool Integration Research [quarter]

## Title
Business Journey Architecture Review & UX Blueprint

## Problem Statement
Small business owners (bakers, handymen, boutique owners, tutors) find traditional website builders and business management tools overwhelming. They need a zero-to-live solution in under 10 minutes that handles complex tasks (inventory, bookings, CRM) invisibly, relying heavily on AI agents to manage operations while they focus on their craft. The current fragmented approach causes drop-offs during onboarding and failure to reach full activation.

## Research Report
- **Competitive Landscape**:
  - Shopify: Powerful, but requires extensive setup and add-on apps. Too complex for simple service businesses (like Carlos).
  - Wix/Squarespace: Great for basic brochure sites, but lack deep, out-of-the-box operational features (like Maya's custom cake deposit workflow or Fatima's sold-out toggles).
  - Link-in-bio (Linktree): Too simple; cannot handle real transactions or inventory natively.
- **Pain Points Identified**:
  - Non-technical users struggle with API keys, DNS settings, and payment gateways.
  - Managing multiple inboxes (Instagram, SMS, Email) leads to missed leads.
  - "Blank canvas" syndrome when designing storefronts.
- **Opportunity**: A conversational, AI-driven setup process combined with a unified inbox and "Agent Departments" mapping to real-world roles.

## Design Doc

### Real User Personas Addressed
- **Maya (Baker, 28)**: Needs deposit-based custom orders and automated IG DM replies.
- **Carlos (Handyman, 42)**: Needs service listings, booking calendar, and AI quotes.
- **Priya (Boutique Owner, 35)**: Needs storefront + POS inventory sync and variant management.
- **Leo (Music Tutor, 22)**: Needs lesson booking, auto-meeting links, and follow-ups.
- **Fatima (Food Cart, 50)**: Needs photo menu, pre-orders, and multi-lingual UI (Arabic/English).

### Architecture Map (Mermaid)
```mermaid
graph TD
    User([Business Owner]) --> MobileApp[OHC Mobile App / Web]
    MobileApp --> API[Tauri v2 / Next.js API]
    API --> AgentHub[AI Agent Orchestrator]

    AgentHub --> Ops[Operations Manager]
    AgentHub --> Sales[Salesperson]
    AgentHub --> Support[Customer Success]

    Ops --> DB[(Main Database)]
    Sales --> CRM[(CRM DB)]

    Customer([End Customer]) --> Storefront[Live Business URL]
    Storefront --> API
```

### Mobile UX Flow (375px First)
- **Home Dashboard**: Unified inbox merging IG, Web, and SMS.
- **Visual Mandate**: Glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`), Touch targets >= 44x44px.
- **Typography**: Outfit for headings, Inter for body text.
- **Motion**: Entrance <= 300 ms, exit <= 200 ms.

### Key Design Decisions
- **Unified Inbox Architecture**: Centralized event bus for all inbound messages to simplify user workflow.
- **Agent Roles**: Represented as "Departments" (The Manager, The Promoter) to abstract complex automation logic into relatable concepts.
- **Deferred KYC**: Delay banking setup until after the first sale to optimize for the 10-minute go-live metric.

## End-to-End User Journey Maps

### 1. Maya (Baker) Journey Sequence
```mermaid
sequenceDiagram
    participant M as Maya
    participant IG as Instagram Ad
    participant OHC as Mobile Web App
    participant AI as Setup Agent

    M->>IG: Clicks Ad
    IG->>OHC: Redirects to Onboarding
    OHC->>M: Prompts for Business Name
    M->>OHC: "Maya's Custom Cakes"
    OHC->>AI: Trigger Setup
    AI-->>OHC: Gen Bakery Template
    OHC->>M: Template Preview
    M->>OHC: Upload 1 Photo & Price
    OHC->>M: Generates Live URL
    M->>IG: Link in Bio
```

### 2. Carlos (Handyman) Journey Sequence
```mermaid
sequenceDiagram
    participant C as Carlos
    participant SMS as SMS Invite
    participant App as Android App
    participant AI as Ops Agent

    C->>SMS: Receives invite link
    SMS->>App: Downloads app
    App->>C: Prompts for Service
    C->>App: "Plumbing Repair, $100/hr"
    App->>AI: Configure Booking Agent
    AI-->>App: Generates Calendar Link
    App->>C: Provides Booking Page
    C->>C: Shares link directly with clients
```

### 3. Priya (Boutique) Journey Sequence
```mermaid
sequenceDiagram
    participant P as Priya
    participant App as iOS App
    participant AI as Inventory Agent
    participant Sync as DB Sync

    P->>App: Opens App
    App->>P: Setup "In-Store & Online"
    P->>App: Scans Barcode or takes photo
    App->>AI: Identify Clothing Item
    AI-->>App: Drafts Title, Description, Tags
    App->>Sync: Saves to DB
    Sync-->>App: Syncs to Web Storefront
    App->>P: Item Live for Sale
```

### 4. Leo (Music Tutor) Journey Sequence
```mermaid
sequenceDiagram
    participant L as Leo
    participant Web as OHC Desktop
    participant AI as Sales Agent
    participant Meet as Auto-Meeting Gen

    L->>Web: Configures availability
    Web->>L: Connects to Zoom/Meet
    Web->>AI: Configures Follow-up logic
    L->>Web: Shares subscription package
    Web->>Meet: Gen recurring links
    Web->>L: Student booked notification
```

### 5. Fatima (Food Cart) Journey Sequence
```mermaid
sequenceDiagram
    participant F as Fatima
    participant App as Android App (Arabic)
    participant AI as Translation/Ops
    participant Cust as Customer

    F->>App: Opens App, UI in Arabic
    App->>F: Uploads photo of Halal Cart Plate
    App->>AI: Auto-generates English & Arabic description
    AI-->>App: Confirms Menu Item
    Cust->>App: Orders in English
    App->>AI: Translate to Arabic
    AI-->>F: Push notification in Arabic (New Order)
    F->>App: Marks "Ready for Pickup"
```

## Implementation Prompt
**To Implementer Agents:**
Implement the "Unified Inbox" core UI and data model.
- **Outcome**: A mobile-first dashboard where a business owner can view and reply to a message from Instagram and a message from their website contact form in the same thread view.
- **CUJ**: Carlos receives an SMS lead and an email inquiry. He opens the OHC app, sees both in the "Inbox" tab, and can reply to both using the same UI interface.
- **Acceptance Criteria**:
  - UI strictly follows the Visual Excellence Mandate (Glassmorphism, 44px touch targets).
  - Must render perfectly on a 375px viewport.
  - Includes a "Drafted by Agent" indicator if the Customer Success agent has prepared a response.
- **Note**: Do not build the actual IG/SMS API integrations yet; mock the inbound data streams. Focus on the UI and internal data models.

## Priority
P0 (Critical Path for Onboarding & Activation)

## Estimated Scope
Large
