# Research Report

## Business Journey Architecture

### Overview
This report outlines the end-to-end user journey for non-technical business owners using OneHumanCorp (OHC). We will focus on two key personas: Maya (baker, Instagram-based) and Carlos (handyman, word-of-mouth-based). The goal is to provide a seamless transition from discovery to a fully functional online business in under 10 minutes.

### 1. Maya (Baker, Instagram-Based)

#### Sequence Diagram

```mermaid
sequenceDiagram
    participant Maya
    participant Instagram
    participant OHC_Landing
    participant OHC_Onboarding
    participant OHC_AI_Ops
    participant OHC_AI_CS

    Maya->>Instagram: Sees OHC ad ("Turn DMs into Deposits")
    Instagram->>OHC_Landing: Clicks Ad CTA
    OHC_Landing->>OHC_Onboarding: "Start Free - No Code Needed"
    OHC_Onboarding->>Maya: Asks: Business Name?
    Maya-->>OHC_Onboarding: "Maya's Cakes"
    OHC_Onboarding->>Maya: Asks: What do you sell? (Physical Products, Services, etc.)
    Maya-->>OHC_Onboarding: "Physical Products - Baked Goods"
    OHC_Onboarding->>Maya: "Connect Instagram to auto-import photos?"
    Maya-->>OHC_Onboarding: Connects Instagram
    OHC_Onboarding->>OHC_AI_Ops: Import photos & create draft products
    OHC_AI_Ops-->>OHC_Onboarding: Draft catalog ready
    OHC_Onboarding->>Maya: "Set your deposit rules (e.g., 50% upfront)"
    Maya-->>OHC_Onboarding: Sets deposit rule
    OHC_Onboarding->>Maya: "Your storefront is live! Share this link in your bio."
    Maya->>Instagram: Updates Link-in-Bio

    %% Customer interaction
    participant Customer
    Customer->>Instagram: DM "Do you do vegan cakes?"
    Instagram->>OHC_AI_CS: Forward DM via integration
    OHC_AI_CS-->>Customer: "Yes! Maya makes vegan cakes. See the menu and order here: [Link]"
```

#### Journey Breakdown
*   **Acquisition:** Maya discovers OHC via a targeted Instagram ad highlighting the pain point of managing orders via DMs.
*   **Onboarding:** The wizard minimizes friction by importing her existing Instagram photos. It focuses on the core need: taking deposits.
*   **Activation:** Success is measured by publishing the link-in-bio and receiving the first automated deposit.
*   **Retention:** Push notifications on her phone for new orders and deposits keep her engaged.
*   **Revenue:** Upgrades from Free to Starter when she exceeds the 10-product limit or wants a custom domain (`mayascakes.com`).

### 2. Carlos (Handyman, Word-of-Mouth)

#### Sequence Diagram

```mermaid
sequenceDiagram
    participant Carlos
    participant Friend
    participant OHC_Referral
    participant OHC_Onboarding
    participant OHC_AI_Sales
    participant OHC_AI_Ops

    Friend->>Carlos: "You need a website to look professional. Try OHC."
    Carlos->>OHC_Referral: Clicks referral link
    OHC_Referral->>OHC_Onboarding: "Start Free - Get Booked Today"
    OHC_Onboarding->>Carlos: Asks: Business Name?
    Carlos-->>OHC_Onboarding: "Carlos Fixes It"
    OHC_Onboarding->>Carlos: Asks: What do you sell?
    Carlos-->>OHC_Onboarding: "Services & Bookings"
    OHC_Onboarding->>Carlos: "List 3 common services (e.g., TV Mount, Faucet Repair)"
    Carlos-->>OHC_Onboarding: Enters services and base prices
    OHC_Onboarding->>Carlos: "Connect Google Calendar to sync availability?"
    Carlos-->>OHC_Onboarding: Connects Calendar
    OHC_Onboarding->>Carlos: "Your booking page is live! Text this link to clients."

    %% Customer interaction
    participant Client
    Client->>OHC_AI_Sales: "Can you mount a 65-inch TV tomorrow?" via SMS/Web Widget
    OHC_AI_Sales->>Client: "Yes, Carlos has availability tomorrow at 2 PM. Quote: $150. Book here: [Link]"
    Client->>OHC_AI_Ops: Books and pays deposit
    OHC_AI_Ops->>Carlos: Push notification: "New Booking! Tomorrow at 2 PM."
```

#### Journey Breakdown
*   **Acquisition:** Word-of-mouth referral from another small business owner.
*   **Onboarding:** Focuses on listing services and syncing his existing calendar. Simple, text-based inputs.
*   **Activation:** Success is the first booked appointment via the link he texts to clients.
*   **Retention:** Daily SMS/push summaries of his upcoming schedule.
*   **Revenue:** Upgrades when he wants to use the "AI Salesperson" to automatically generate quotes from SMS inquiries.

---

## Data Model Architecture

### Overview
The OHC data model must support strict multi-tenancy, offline-first mobile synchronization, and flexible entity relationships to accommodate various business types.

### Entity-Relationship Diagram

```mermaid
erDiagram
    TENANT {
        string id PK
        string name
        string tier
        string custom_domain
    }
    USER {
        string id PK
        string tenant_id FK
        string role
        string email
    }
    CUSTOMER {
        string id PK
        string tenant_id FK
        string name
        string phone
        string email
    }
    PRODUCT {
        string id PK
        string tenant_id FK
        string type "physical, digital, service"
        string name
        float price
    }
    ORDER {
        string id PK
        string tenant_id FK
        string customer_id FK
        string status
        float total
    }
    BOOKING {
        string id PK
        string tenant_id FK
        string customer_id FK
        string product_id FK
        datetime start_time
        datetime end_time
    }
    AI_AGENT {
        string id PK
        string tenant_id FK
        string department "Operations, Sales, etc."
        string status
    }

    TENANT ||--o{ USER : has
    TENANT ||--o{ CUSTOMER : owns
    TENANT ||--o{ PRODUCT : owns
    TENANT ||--o{ ORDER : owns
    TENANT ||--o{ BOOKING : owns
    TENANT ||--o{ AI_AGENT : employs

    CUSTOMER ||--o{ ORDER : places
    CUSTOMER ||--o{ BOOKING : makes
    ORDER ||--o{ PRODUCT : contains
```

### Key Invariants
1.  **Strict Multi-Tenancy:** Every operational table MUST include a `tenant_id` column. All queries MUST scope by `tenant_id` derived securely from the authenticated user's session token. Row Level Security (RLS) is mandatory at the database level.
2.  **Offline Support:** Entities accessed by mobile clients (Products, Orders, Customers) must support eventual consistency and local caching.
3.  **Agent Context:** AI Agents operate strictly within the context of a single `tenant_id`. They cannot cross-pollinate data between businesses.

### Migration Strategy
Database schemas will evolve using standard up/down migration scripts. Crucially, any new table storing tenant data MUST include the `ALTER TABLE [table_name] ENABLE ROW LEVEL SECURITY;` statement in the same migration file.

---

## AI Agent Department Architecture

### Overview
AI Agents in OHC are presented as "Departments" to non-technical users. They handle complex workflows invisibly, requiring minimal configuration.

### Agent Flow Architecture

```mermaid
graph TD
    Trigger[Event / Schedule / Demand] --> Dispatcher(Event Dispatcher)
    Dispatcher --> OpsAgent[Operations 'The Manager']
    Dispatcher --> SalesAgent[Sales 'The Salesperson']
    Dispatcher --> CSAgent[Customer Success 'The Ambassador']

    OpsAgent --> Action_Fulfill[Process Order / Update Inventory]
    SalesAgent --> Action_Quote[Generate Quote]
    CSAgent --> Action_Reply[Reply to Inquiry / Send Review Request]

    Action_Fulfill --> Memory[(Shared Tenant Memory)]
    Action_Quote --> Memory
    Action_Reply --> Memory

    Memory --> AdvisorAgent[Advisor 'The Analyst']
    AdvisorAgent --> Insight[Weekly Health Report / Suggestions]
```

### Key Principles
*   **Friendly Naming:** Agents are referred to by their roles (e.g., "The Manager", "The Promoter").
*   **Shared Memory:** All agents within a tenant read from and write to a shared memory pool (the AutoDream pipeline), ensuring consistency. If Sales promises a discount, Operations knows about it.
*   **Approval Workflows:** High-risk actions (e.g., sending a mass email, issuing a large refund) require the business owner's approval (Draft-for-Review). Low-risk actions (e.g., answering FAQ) are Auto-Execute.
*   **Budgeting:** Agent activity is metered against the tenant's tier limits (e.g., 100 AI actions/month on Free).

---

## Website & Storefront Builder Architecture

### Overview
The storefront builder must be intuitive, mobile-first, and require zero coding knowledge.

### Design Principles
*   **Block-Based:** Users construct pages using pre-defined semantic blocks (Hero, Product Grid, Testimonials, Booking Calendar).
*   **Grandmother Test:** If a user can't publish a page in 30 seconds, the UI is too complex.
*   **Progressive Disclosure:** Advanced settings (e.g., SEO metadata, custom CSS via glassmorphism tokens) are hidden behind an "Advanced Mode" toggle.

### Content Blocks
1.  **Hero:** Image/Video background, Headline, primary CTA.
2.  **Product/Service Grid:** Dynamically populated from the Product catalog.
3.  **Booking Calendar:** Integrates directly with the tenant's availability and deposit rules.
4.  **Contact/Inquiry Form:** Feeds directly into the "Salesperson" AI agent for follow-up.

---

## Multi-Tenant SaaS Tier Architecture

### Tier Breakdown

| Tier | Price | Target Persona | Key Limits | AI Usage | Custom Domain |
|---|---|---|---|---|---|
| **Free** | $0 | Hobbyist (Maya starting out) | 10 Products | 1 Dept (Ops), 100 actions/mo | No (OHC Subdomain) |
| **Starter** | $9/mo | Side-Hustle | 100 Products | 3 Depts, 1,000 actions/mo | Yes |
| **Pro** | $29/mo | Full-Time (Carlos) | Unlimited | 10 Depts, Unlimited | Yes + SSL |
| **Business** | $79/mo | Small Agency / Retail | Unlimited | Unlimited Depts | Yes + Multi-domain |

### Upgrade Flow
Upgrades are contextual. If Maya tries to add an 11th product, a modal appears: "Your business is growing! Upgrade to Starter to add unlimited products and a custom domain." The focus is on the value unlocked, not the technical limits.
