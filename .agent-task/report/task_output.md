# Business Journey Architecture

## Problem Statement
The OHC platform must successfully guide a diverse range of non-technical users from initial discovery to managing a thriving online business, all from a mobile interface in under 10 minutes. The lack of a unified mapping of these end-to-end user journeys (from Acquisition to Referral) causes friction points that result in abandonment. Understanding the specific needs of diverse personas (like Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator) is critical to building intuitive AI-assisted workflows that eliminate technical complexity.

## Research Report
### Competitive Landscape
OHC targets the zero-technical-knowledge demographic, aiming for a setup time under 10 minutes from a mobile device.

| Feature | OHC | Shopify | Wix | Squarespace | GoDaddy |
|---|---|---|---|---|---|
| **Setup Time** | **< 10 min** | 30-60 min | 20-40 min | 30-60 min | 20-40 min |
| **Tech Knowledge Needed** | **Zero** | Low | Low | Low | Low |
| **Mobile-First Management**| **Yes** | Partial | Partial | No | No |
| **AI Agents (Invisible)** | **Yes (Built-in)** | Sidekick (Chat) | Wix AI | Limited | Airo (Limited) |
| **Target User** | **Non-technical** | SMB / Tech-Savvy | Semi-technical | Creative | Basic user |
| **Free Tier** | **Yes (Useful)** | No | Yes (Limited) | No | No |

### Persona Pain Points Summary
1.  **Maya (Home Baker):** Overwhelmed by complex e-commerce settings; needs mobile-only deposit payments and AI for simple DM inquiries.
2.  **Carlos (Freelance Handyman):** Relies on word-of-mouth; needs simple service listings, booking with deposits, and automated quote generation.
3.  **Priya (Boutique Owner):** Needs omni-channel inventory sync, tap-to-pay POS, and easy-to-read daily analytics.
4.  **Leo (Music Tutor):** Needs recurring subscription billing, zoom link generation, and a portfolio page for his TikTok "link-in-bio".
5.  **Fatima (Food Cart Operator):** Needs low-data mobile functionality, pre-order management, multi-language support (Arabic/English), and clear sold-out toggles.

## Design Doc: High-Level Architecture & User Journeys

### User Journey Comparisons

```mermaid
%%{init: {'theme': 'dark', 'themeVariables': {'primaryColor': '#1a1a2e', 'primaryTextColor': '#e2e8f0', 'primaryBorderColor': '#38bdf8', 'lineColor': '#38bdf8', 'secondaryColor': '#0f172a', 'tertiaryColor': '#1e293b'}}}%%
gantt
    title OHC User Journey: Setup to First Sale (Time in Minutes)
    dateFormat  mm:ss
    axisFormat  %M:%S

    section Maya (Baker)
    Sign Up & Setup     :active, m1, 00:00, 02:00
    Add Cakes (Photos)  :active, m2, 02:00, 05:00
    Set Deposit Rules   :active, m3, 05:00, 06:00
    Go Live & Share     :active, m4, 06:00, 08:00
    Receive First DM/Order: milestone, 08:00, 00:00

    section Carlos (Handyman)
    Sign Up & Setup     :active, c1, 00:00, 02:00
    List Services/Prices:active, c2, 02:00, 04:00
    Configure Calendar  :active, c3, 04:00, 06:00
    Go Live & Quote Req : milestone, 06:00, 00:00

    section Fatima (Food Cart)
    Sign Up (Arabic)    :active, f1, 00:00, 02:00
    Add Menu & Prices   :active, f2, 02:00, 06:00
    Set Pre-order Rules :active, f3, 06:00, 07:00
    Go Live & Print List: milestone, 07:00, 00:00
```

### Feature Gap Heatmap

```mermaid
%%{init: {'theme': 'dark', 'themeVariables': {'primaryColor': '#1a1a2e', 'primaryTextColor': '#e2e8f0', 'primaryBorderColor': '#38bdf8', 'lineColor': '#38bdf8'}}}%%
pie title Feature Criticality by Persona
    "Mobile-Only Management (Maya, Fatima, Carlos)" : 45
    "Booking/Deposits (Carlos, Leo, Maya)" : 25
    "Inventory Sync & POS (Priya)" : 15
    "Multi-language/Low Data (Fatima)" : 15
```

### Persona Sequence Diagrams

#### 1. Maya (The Home Baker)
```mermaid
sequenceDiagram
    autonumber
    actor Maya
    participant App as OHC Mobile App
    participant AI_Marketing as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Stripe as Payments
    participant Instagram as IG Integration

    Maya->>App: Signs up, selects "Home Bakery"
    App->>AI_Marketing: Generate Storefront Draft
    AI_Marketing-->>Maya: Suggests 3 Glassmorphism designs
    Maya->>App: Approves Design 1
    Maya->>App: Uploads Cake Photos (auto-WebP)
    Maya->>App: Sets custom order logic (50% deposit)
    Maya->>App: Publishes Store
    App->>Instagram: AI_Marketing auto-posts "We are live!"

    Note over Maya, Instagram: A customer DMs on Instagram
    Instagram->>AI_Ops: "Do you do vegan cakes?"
    AI_Ops-->>Maya: Drafts reply: "Yes! Here is the link..." (Review Required)
    Maya->>App: Taps "Approve & Send"
    AI_Ops->>Instagram: Sends reply with booking link
```

#### 2. Carlos (The Freelance Handyman)
```mermaid
sequenceDiagram
    autonumber
    actor Carlos
    participant App as OHC Mobile App (Android)
    participant AI_Sales as Sales Agent
    participant AI_Ops as Operations Agent
    participant DB as OHC-SIP DB

    Carlos->>App: Signs up, selects "Handyman Services"
    App->>AI_Sales: Generate Service Listings
    AI_Sales-->>Carlos: Suggests standard plumbing/repair templates
    Carlos->>App: Modifies prices, sets availability
    Carlos->>App: Publishes page

    Note over Carlos, App: Customer requests a quote via site
    App->>AI_Sales: Analyze request: "Fix leaking pipe under sink"
    AI_Sales->>DB: Fetch Carlos pricing rules
    AI_Sales-->>Carlos: Drafts Quote: "$150 + Parts" (Review Required)
    Carlos->>App: Approves Quote
    AI_Ops->>App: Sends quote to customer
    App->>AI_Ops: Customer accepts & pays deposit
    AI_Ops->>Carlos: Push Notification: "New Job Booked!"
```

#### 3. Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    autonumber
    actor Priya
    participant App as OHC App (Mobile + Web)
    participant POS as Stripe Terminal (In-Store)
    participant AI_Finance as Finance Agent
    participant AI_Marketing as Marketing Agent
    participant DB as OHC-SIP DB

    Priya->>App: Logs in, adds new "Summer Dress" variant (Red/M)
    App->>DB: Updates unified inventory

    Note over Priya, POS: In-store customer buys dress
    Priya->>POS: Tap-to-Pay (Stripe)
    POS->>DB: Decrements inventory (Red/M: 4 -> 3)

    Note over Priya, App: End of Day
    AI_Finance->>DB: Calculate daily revenue across online/in-store
    AI_Finance-->>Priya: Sends push: "Daily Sales Report ready"
    Priya->>App: Reviews analytics (Mobile view)

    Note over Priya, App: Next Morning
    AI_Marketing->>DB: Identify trending items (Summer Dress)
    AI_Marketing-->>Priya: Drafts Email Newsletter: "Trending Now!"
    Priya->>App: Approves Newsletter
```

#### 4. Leo (The Music Tutor)
```mermaid
sequenceDiagram
    autonumber
    actor Leo
    participant App as OHC Web App
    participant AI_Sales as Sales Agent
    participant Calendar as Google Calendar Sync
    participant Zoom as Meeting Gen
    participant Stripe as Payments

    Leo->>App: Sets up "Monthly Lesson Package" (Subscription)
    Leo->>App: Connects Google Calendar
    Leo->>App: Publishes TikTok Link-in-Bio

    Note over Leo, App: Student buys subscription
    App->>Stripe: Initiates recurring billing
    App->>Calendar: Books weekly 4 PM slot
    Calendar->>Zoom: Auto-generates meeting link
    App->>Student: Emails confirmation & Zoom link

    Note over Leo, App: 2 Months Later (Student misses 2 weeks)
    AI_Sales->>App: Detects inactivity
    AI_Sales-->>Leo: Drafts check-in email: "Hey, missing you at lessons!"
    Leo->>App: Approves email
```

#### 5. Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    autonumber
    actor Fatima
    participant App as OHC Android App (Low Data)
    participant AI_Ops as Operations Agent
    participant DB as OHC-SIP DB

    Fatima->>App: Opens app (Arabic Language)
    Fatima->>App: Uploads daily menu photos (compressed on device)
    Fatima->>App: Sets "Chicken Over Rice" to 50 portions available
    Fatima->>App: Enables Pre-orders

    Note over Fatima, App: Lunch Rush Starts
    App->>DB: Customer places order online
    DB->>AI_Ops: Process Order Event
    AI_Ops->>Fatima: High-priority Push Alert (Loud Notification)
    Fatima->>App: Marks order as "Preparing"

    Note over Fatima, App: Portion 50 is sold
    App->>DB: Decrements inventory to 0
    DB->>App: Auto-toggles item to "Sold Out" on live menu

    Note over Fatima, App: End of Shift
    Fatima->>App: Generates Daily Order Summary for printing
```

### Key Design Decisions
1.  **Mobile-First is Mandatory:** All critical setup and management flows must be fully functional and optimized for a 375px screen size. Horizontal scrolling is prohibited.
2.  **AI as Background Infrastructure:** AI departments (Marketing, Sales, Ops, Finance) are event-driven and require 1-tap approvals ("Draft-for-Review") for high-risk external actions, building trust with non-technical users.
3.  **Unified State via KAIROS:** A single source of truth (OHC-SIP DB) ensures that an in-store POS sale instantly updates the online storefront inventory, triggering appropriate AI alerts if stock runs low.
4.  **Low-Data & Resilience Requirements:** For personas like Fatima, critical functionality (menu updates, offline order viewing) must work on slow networks with heavy media compression applied immediately.

### Actionable Recommendations
- **Implement 1-Tap Approvals:** Prioritize the mobile UX for the AI "Draft-for-Review" queue. Business owners must be able to confidently approve generated quotes, emails, or social posts with a single tap.
- **Develop Granular Onboarding:** Create branching onboarding flows. Maya's setup (deposits/images) is distinct from Fatima's (menu/sold-out toggles). Limit initial inputs to achieve "Live" status in under 10 minutes.
- **Enforce UI Standards:** Apply the Glassmorphism design system (`backdrop-filter: blur(20px) saturate(200%)`, Outfit/Inter fonts) across all persona interfaces to ensure a premium feel regardless of the business type.

## Implementation Prompt
**To the Implementer:**
Implement the branching onboarding flow and the unified mobile dashboard that supports the five distinct business types (Physical Product, Digital Product, Service/Booking, Food/Beverage, Subscription). Focus on the Critical User Journey (CUJ) where a new user selects their business type and completes the minimum necessary inputs to publish a live storefront in under 10 minutes. Ensure all UI components adhere to the OHC Premium Token library (Glassmorphism, 375px base breakpoint) and utilize Riverpod for state management. E2E tests must cover the successful setup for at least two diverse personas (e.g., Maya the Baker and Carlos the Handyman).

```yaml
issue_id: "arch-001-business-journeys"
priority: "P1"
estimated_scope: "Large"
```
