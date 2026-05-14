# Architectural Research Report

## Business Journey Architecture

### Sequence Diagrams for Personas

#### Maya (Baker)
```mermaid
sequenceDiagram
    participant Maya as Maya (Baker)
    participant IG as Instagram
    participant OHC as OneHumanCorp (OHC)
    participant AI as AI Manager

    Maya->>IG: Posts new custom cake on IG
    IG->>Maya: Customer asks "Do you do vegan cakes?"
    Maya->>OHC: Configures AI Manager with vegan options
    OHC-->>AI: Updates knowledge base
    AI->>IG: Replies "Yes! Here are our vegan options..."
    IG->>Customer: Delivers AI reply
    Customer->>IG: Places custom order with deposit
    IG->>OHC: Order details synced via webhook
    OHC->>Maya: Notifies Maya of new deposit-based order
```

#### Carlos (Handyman)
```mermaid
sequenceDiagram
    participant Carlos as Carlos (Handyman)
    participant OHC as OneHumanCorp
    participant Web as OHC Storefront
    participant Customer as Customer

    Carlos->>OHC: Sets up service listings and prices
    OHC->>Web: Publishes storefront
    Customer->>Web: Views services and selects a time slot
    Customer->>Web: Pays deposit
    Web->>OHC: Syncs booking and deposit
    OHC->>Carlos: Sends push notification for new booking
```

#### Priya (Boutique Owner)
```mermaid
sequenceDiagram
    participant Priya as Priya
    participant POS as In-person POS
    participant OHC as OneHumanCorp
    participant Web as Online Store

    Priya->>OHC: Adds new clothing variants
    OHC->>Web: Updates online inventory
    Customer->>Web: Buys item online
    Web->>OHC: Deducts from inventory
    Customer2->>POS: Buys same item in-store
    POS->>OHC: Deducts from inventory
    OHC->>Priya: Sends daily mobile analytics report
```

#### Leo (Music Tutor)
```mermaid
sequenceDiagram
    participant Leo as Leo
    participant OHC as OneHumanCorp
    participant Web as Portfolio Page
    participant AI as AI Assistant
    participant Student as Student

    Leo->>OHC: Sets up portfolio and lesson packages
    OHC->>Web: Publishes portfolio link-in-bio
    Student->>Web: Books 5-lesson package
    Web->>OHC: Processes subscription
    OHC->>Leo: Generates meeting link
    OHC->>Student: Sends meeting link
    AI->>Student: Follows up after lesson 1 for feedback
```

#### Fatima (Food Cart)
```mermaid
sequenceDiagram
    participant Fatima as Fatima
    participant OHC as OneHumanCorp
    participant Web as Arabic/English Menu
    participant Customer as Customer

    Fatima->>OHC: Updates photo menu (low-end Android)
    OHC->>Web: Publishes bilingual menu
    Customer->>Web: Pre-orders halal food
    Web->>OHC: Processes payment
    OHC->>Fatima: Sends phone notification + printable order
```

## Data Model Architecture

```mermaid
erDiagram
    TENANT ||--o{ BUSINESS : owns
    BUSINESS ||--o{ PRODUCT : offers
    BUSINESS ||--o{ ORDER : receives
    BUSINESS ||--o{ CUSTOMER : serves
    BUSINESS ||--o{ AGENT : utilizes
    BUSINESS ||--o{ PAGE : publishes
    BUSINESS ||--o{ BOOKING : schedules
    PRODUCT ||--o{ VARIANT : has
    ORDER ||--o{ ORDER_ITEM : contains
    ORDER }|--|| CUSTOMER : placed_by
    BOOKING }|--|| CUSTOMER : booked_by

    TENANT {
        string id PK
        string email
        string plan_tier
    }
    BUSINESS {
        string id PK
        string tenant_id FK
        string name
        string industry
        string custom_domain
    }
    PRODUCT {
        string id PK
        string business_id FK
        string name
        string type "physical/digital/service"
        float price
    }
    CUSTOMER {
        string id PK
        string business_id FK
        string name
        string phone
        string email
    }
    ORDER {
        string id PK
        string business_id FK
        string customer_id FK
        float total
        string status
    }
```

### Key Invariants
* **Tenant Isolation**: A TENANT can only read/write data associated with their own BUSINESS entities. Queries must always filter by tenant_id.
* **Mobile-First Data Access**: APIs returning ORDER or BOOKING data must support pagination and lightweight projections to perform well on low-end Android devices.

### Migration Strategy
* Schema changes must be backward compatible.
* Add columns with default values; do not drop columns until the application layer is fully migrated.

## AI Agent Department Architecture

### Core Departments
1.  **Operations ("The Manager")**: Triggered by ORDER_CREATED or BOOKING_CREATED events. Coordinates fulfillment status updates.
2.  **Marketing & Advertising ("The Promoter")**: Triggered on demand by the user or scheduled weekly. Auto-drafts social posts based on new PRODUCT additions.
3.  **Sales & Acquisition ("The Salesperson")**: Triggered by inbound messages. Auto-generates quotes and tracks lead conversions.
4.  **Customer Success ("The Ambassador")**: Triggered by ORDER_DELIVERED. Sends review request emails.
5.  **Finance & Payments ("The Accountant")**: Scheduled monthly. Aggregates ORDER totals and generates tax summaries.
6.  **Legal & Compliance ("The Protector")**: Triggered during onboarding. Generates TOS based on BUSINESS industry.
7.  **Business Advisory ("The Advisor")**: Scheduled weekly. Analyzes sales trends and suggests actionable next steps via push notification.

### Coordination
Departments communicate via an event bus (e.g., Kafka or Redis Pub/Sub). For example, when Operations completes an order, it emits an ORDER_COMPLETED event, which Customer Success listens to in order to send a follow-up.

### Memory & Approval
* Context is stored in a multi-tenant vector database.
* AI actions default to "draft-for-review" until the user explicitly enables "auto-execute" for a specific department.

## Website & Storefront Builder Architecture

### Drag-and-Drop Concept
* **Content Blocks**: Hero Image, Product Grid, Testimonial Carousel, Booking Calendar, Contact Form.
* **Templates**: Industry-specific starter themes (e.g., "Bakery", "Handyman"). Users customize colors and fonts globally.
* **Publishing**: Changes are saved as a draft JSON structure. Clicking "Publish" compiles the JSON into static assets distributed via CDN.
* **SEO**: Meta tags and structured data (JSON-LD) are auto-generated based on business type and product details.

## Mobile-First Architecture Review

### Mobile-Critical Screens
1. **Dashboard**: Daily revenue, pending orders, AI Advisor suggestions.
2. **Order Management**: Accept/reject orders, mark as fulfilled.
3. **Product Catalog**: Quick add via phone camera, toggle availability.
4. **Inbox**: Consolidated messages from IG, SMS, Email.

### Offline & Performance
* **Offline Capabilities**: Reading recent orders and drafting replies. Synchronized when reconnected.
* **Performance**: Target < 1s TTI (Time to Interactive) on 3G networks. Payloads < 50KB for core views.
* **Real-time**: WebSockets for live order notifications, falling back to FCM/APNs push notifications.

## Multi-Tenant SaaS Tier Architecture

### Tier Limits & Upgrades
* **Free ($0)**: OHC subdomain. Limits enforced gracefully (e.g., "You've reached your 10 product limit. Upgrade to add more.").
* **Starter ($9/mo)**: Custom domain unlocked.
* **Pro ($29/mo)**: SSL provisioning enabled for custom domains.
* **Business ($79/mo)**: Unlimited AI usage and multi-domain support.

### Enforcement
Limits are checked at the application boundary (API Gateway). Upgrades are presented contextually (e.g., when trying to add an 11th product on the Free tier) rather than locking the user out.

# Issue Briefs (Integrated)

## [product]_business_journey
### Problem Statement
Small business owners (bakers, handymen, boutique owners, tutors, food cart operators) currently face friction when onboarding, managing their digital storefronts, and handling customer interactions. The current flow may be too technical or lack the necessary automation to allow a non-technical user to go from zero to a live business in under 10 minutes.

### Design Doc
#### Architecture Diagram
```mermaid
sequenceDiagram
    participant User as Business Owner
    participant OHC as OneHumanCorp Platform
    participant AI as AI Agent Departments

    User->>OHC: Sign up via mobile
    OHC->>AI: Trigger Onboarding Agent
    AI-->>User: Conversational setup (industry, name)
    User->>OHC: Add first product/service (photo upload)
    OHC->>AI: Generate product description & SEO
    User->>OHC: Connect payment method
    OHC->>User: "Your business is live!" (Under 10 mins)
    User->>OHC: Share link-in-bio to Instagram/TikTok
```

#### UI Wireframes & Mobile UX Flow (375px)
1. **Welcome Screen**: Large, clear CTA "Start your business in 5 minutes".
2. **Conversational Setup**: Chat-like interface asking for business name and type.
3. **Product Addition**: Camera integration. Take a photo -> AI auto-fills title and suggests price based on industry.
4. **Go Live**: Confetti animation. Big button to "Share to Instagram".

#### Key Design Decisions
- **Mobile-First**: The entire onboarding and management flow must be 100% functional and optimized for a 375px viewport. Desktop is additive.
- **AI-Assisted Onboarding**: Replace static forms with conversational, AI-driven data collection to reduce cognitive load.
- **Immediate Value Delivery**: Focus on getting one product/service live and a payment method connected before asking for complex configurations.

### Implementation Prompt
Implement the new mobile-first onboarding flow. The user should be greeted by a conversational interface that collects their business name and industry. Then, prompt them to add their first product by taking a photo, using AI to auto-generate the description. Finally, guide them to connect a payment method and provide a shareable link. The flow must pass the 'grandmother test' (completable in < 30 seconds by a non-technical user). Ensure all screens follow the Glassmorphism design tokens and Outfit/Inter typography.

### Priority
P0

### Estimated Scope
Large
