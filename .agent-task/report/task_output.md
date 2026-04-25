<div markdown="1" style="backdrop-filter: blur(15px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# [Business Journey] OHC End-to-End User Journey Architecture

## Problem Statement
Small business owners (bakers, handymen, tutors) are overwhelmed by the complexity of traditional platforms (Shopify, Wix). They abandon onboarding when faced with DNS settings, payment gateway configurations, or SEO optimization. OHC must guide non-technical users from idea to live business in under 10 minutes, entirely on mobile, using AI agents to handle the complexity.

## Research Report: Competitive Analysis & Pain Points
Based on the OHC market positioning against Shopify, Wix, Squarespace, and GoDaddy:
- **Shopify:** Setup time is 30-60 min. Target user is SMB/Tech-savvy.
- **Wix:** Complex editor, setup time 20-40 min.
- **Squarespace:** Portfolio first, complex store setup.
- **OHC:** Setup time < 10 min, zero technical knowledge required, mobile-first management.

### Persona-Specific Pain Points (Friction Points)
| Persona | Business | Friction Point (Abandonment Risk) |
|---|---|---|
| **Maya** (28) | Home Baker | Forcing her to configure Stripe webhooks or design a desktop layout. She runs everything from her iPhone. |
| **Carlos** (42) | Handyman | Asking him to write SEO copy or build a multi-page site. He just needs a booking form and quotes. Android only. |
| **Priya** (35) | Boutique | Manually syncing online and in-store inventory. She needs automatic sync and mobile analytics. |
| **Leo** (22) | Music Tutor | Integrating Zoom links manually into a calendar. He needs automatic lesson links and subscription billing. |
| **Fatima** (50) | Food Cart | English-only interfaces and complex POS hardware. She needs Arabic support, sold-out toggles, and low-data mobile performance. |

## Design Doc: Business Journey Architecture

### 1. Maya — The Home Baker (Physical Products)

```mermaid
sequenceDiagram
    actor Maya
    participant OHC Mobile
    participant Operations Agent
    participant Marketing Agent

    Maya->>OHC Mobile: Signs up (Phone) -> "I sell custom cakes"
    OHC Mobile->>Marketing Agent: Generate bakery storefront UI
    Marketing Agent-->>OHC Mobile: Live Storefront Draft
    Maya->>OHC Mobile: Uploads cake photos
    Maya->>OHC Mobile: Connects Bank (Stripe Connect)
    Maya->>OHC Mobile: Publishes store (< 10 min)
    Note over Maya, OHC Mobile: Activation: First cake uploaded and store live
    Maya->>OHC Mobile: Receives DM on Instagram
    Marketing Agent->>Maya: Drafts reply ("Do you do vegan cakes?")
```

### 2. Carlos — The Handyman (Services & Bookings)

```mermaid
sequenceDiagram
    actor Carlos
    participant OHC Mobile (Android)
    participant Sales Agent
    participant Operations Agent

    Carlos->>OHC Mobile (Android): Signs up -> "I fix plumbing"
    OHC Mobile (Android)->>Sales Agent: Create service listing & quote form
    Sales Agent-->>OHC Mobile (Android): Live Booking Page
    Carlos->>OHC Mobile (Android): Sets availability calendar
    Note over Carlos, OHC Mobile (Android): Activation: Booking page shared
    Carlos->>OHC Mobile (Android): Customer requests quote
    Sales Agent->>Carlos: Auto-drafts quote based on problem
    Carlos->>OHC Mobile (Android): Approves & sends
```

### 3. Priya — The Boutique Owner (Inventory & POS)

```mermaid
sequenceDiagram
    actor Priya
    participant OHC Mobile/Desktop
    participant Finance Agent
    participant Operations Agent

    Priya->>OHC Mobile/Desktop: Signs up -> "I run a clothing boutique"
    OHC Mobile/Desktop->>Operations Agent: Setup inventory catalog
    Priya->>OHC Mobile/Desktop: Adds product variants (Size/Color)
    Priya->>OHC Mobile/Desktop: Enables Tap-to-Pay (Stripe Terminal)
    Note over Priya, OHC Mobile/Desktop: Activation: First in-store tap payment
    Finance Agent->>Priya: Daily revenue push notification
```

### 4. Leo — The Music Tutor (Subscriptions & Digital)

```mermaid
sequenceDiagram
    actor Leo
    participant OHC Mobile
    participant Customer Success Agent
    participant Operations Agent

    Leo->>OHC Mobile: Signs up -> "I teach guitar online"
    OHC Mobile->>Operations Agent: Create subscription packages
    Leo->>OHC Mobile: Syncs Google Calendar
    Note over Leo, OHC Mobile: Activation: TikTok link-in-bio published
    Leo->>OHC Mobile: Student books lesson
    Operations Agent->>Leo: Generates & emails Zoom link
    Customer Success Agent->>Leo: Follows up with inactive students
```

### 5. Fatima — The Food Cart Operator (Pre-orders, Multilingual)

```mermaid
sequenceDiagram
    actor Fatima
    participant OHC Low-End Android
    participant Operations Agent
    participant Advisory Agent

    Fatima->>OHC Low-End Android: Signs up -> Selects Arabic Language -> "Halal food cart"
    OHC Low-End Android->>Operations Agent: Generate photo menu
    Fatima->>OHC Low-End Android: Adds items, toggles "Sold Out"
    Note over Fatima, OHC Low-End Android: Activation: Menu live, first pre-order received
    Fatima->>OHC Low-End Android: Receives loud push notification for order
    Advisory Agent->>Fatima: Weekly report (Arabic): "Tuesday was busiest"
```

### Implementation Prompt for Engineering Swarm
**Task:** Build the OHC unified onboarding flow and core persona journeys.
**User-Facing Outcome:** A mobile-first (375px) wizard where users select their business type (Baker, Handyman, Boutique, Tutor, Food Cart). The system automatically provisions the appropriate AI Agents (Operations, Marketing, Sales, Customer Success, Finance, Advisory) and configures the default entity structure (products, services, bookings) without asking for database fields or API keys.
**Acceptance Criteria:**
- 100% of the UI passes the grandmother test (no technical jargon).
- Visual Excellence Mandate applied: Glassmorphism tokens, minimum 44x44 touch targets.
- Multilingual support foundation established (RTL support for Arabic).
- E2E tests must cover the complete flow for each persona from login to live storefront.
**Priority:** P0
**Estimated Scope:** Large

</div>