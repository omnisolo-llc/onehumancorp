<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [architecture] Business Journey Architecture

## Title
End-to-End Business Journey Architecture for OHC Personas

## Problem Statement
Small business owners—ranging from home bakers to freelance handymen—often lack the technical expertise to piece together fragmented solutions (e.g., website builder + booking calendar + CRM + AI chatbots) to run their operations. They need a simple, guided, and cohesive journey to start, operate, and grow their businesses without ever encountering complex configurations or code. The friction of setting up multi-tool workflows typically leads to abandonment. We need a unified Business Journey Architecture that works flawlessly across all key personas, particularly on a mobile 375px display, offloading all complexities to specialized AI Agent Departments.

## Research Report
Current market solutions (Shopify, Wix, Squarespace, GoDaddy) cater well to users who are somewhat tech-savvy or willing to invest 30-60 minutes in setup. However, they fall short for true non-technical users who require an instant, mobile-first experience.
- **Shopify:** Powerful but overwhelming; requires 30-60 minutes. Better suited for pure e-commerce.
- **Wix:** Highly customizable, but AI features (Wix AI) are often disjointed add-ons.
- **Squarespace:** Great for portfolios but requires a desktop for efficient initial setup.
- **GoDaddy:** Simple but lacks the depth needed for specialized businesses like service bookings or food cart pre-orders.

**OHC Differentiation:**
OHC's advantage is its invisible AI infrastructure that handles complexity from Day 1. By treating the AI as "departments," the business owner experiences a seamless journey.

## Design Doc

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    participant M as Maya (Mobile UI)
    participant MA as Marketing & Advertising
    participant CS as Customer Success
    participant Op as Operations
    participant Fin as Finance & Payments

    M->>MA: Onboarding: "I bake custom cakes"
    MA->>M: Designs Storefront & Generates Content
    Note over M: Customer browses & orders custom cake
    M->>Op: Custom Order Submitted
    Op->>Fin: Process Deposit Payment
    Fin-->>Op: Payment Success
    Op->>CS: Trigger Confirmation
    CS->>M: Sends Order Confirmation SMS/Email
```

#### 2. Carlos (The Freelance Handyman) Journey
```mermaid
sequenceDiagram
    participant C as Carlos (Mobile UI)
    participant SA as Sales & Acquisition
    participant Op as Operations
    participant Fin as Finance & Payments
    participant CS as Customer Success

    C->>SA: Onboarding: "I fix things"
    SA->>C: Generates Service Listings & Quote Form
    Note over C: Customer requests a plumbing fix
    C->>SA: Customer Inquiry
    SA->>C: Drafts Quote for Review
    C->>SA: Approves Quote
    SA->>Op: Schedules Booking
    Op->>Fin: Collects Deposit
    Fin-->>Op: Deposit Confirmed
    Op->>CS: Triggers Follow-Up
    CS->>C: Requests Testimonial Post-Job
```

#### 3. Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    participant P as Priya (Mobile & Desktop)
    participant MA as Marketing & Advertising
    participant Op as Operations
    participant Fin as Finance & Payments
    participant BA as Business Advisory

    P->>MA: Onboarding: "I sell clothes in-store and online"
    MA->>P: Builds Omni-channel Storefront
    Note over P: In-store Tap-to-Pay Transaction
    P->>Fin: Stripe Terminal Payment
    Fin-->>Op: Deducts Inventory (S/M/L)
    Op->>P: Low Stock Alert (if triggered)
    BA->>P: Weekly Report: "Red shirts are trending"
```

#### 4. Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    participant L as Leo (Mobile UI)
    participant SA as Sales & Acquisition
    participant Op as Operations
    participant Fin as Finance & Payments
    participant CS as Customer Success

    L->>SA: Onboarding: "I teach guitar"
    SA->>L: Builds Link-in-Bio & Booking Page
    Note over L: Student books a 4-lesson package
    L->>Op: Booking Received
    Op->>Fin: Sets up Monthly Subscription
    Op->>L: Generates Zoom Link & Calendar Sync
    CS->>L: Re-engages inactive students after 2 weeks
```

#### 5. Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    participant F as Fatima (Mobile UI)
    participant MA as Marketing & Advertising
    participant Op as Operations
    participant Fin as Finance & Payments
    participant CS as Customer Success

    F->>MA: Onboarding: "I sell Halal food"
    MA->>F: Creates Bi-lingual Menu
    Note over F: Customer Pre-orders Pickup
    F->>Fin: Processes Payment
    Fin-->>Op: Payment Verified
    Op->>F: Triggers High-Volume Mobile Notification
    CS->>F: Auto-updates "Sold Out" state based on stock
```

### UI Wireframes & Screen Flow (375px First)
1. **Onboarding (The 10-Minute Launch):**
   - **Screen 1:** "What do you do?" (Input: Text or Voice).
   - **Screen 2:** "What's the business name?"
   - **Screen 3:** "Connecting your AI Departments..." (Loading animation with Glassmorphism).
   - **Screen 4:** "Your business is live! Here is your link."

2. **Dashboard (The Daily Hub):**
   - **Top Card:** "Today's Action Items" (e.g., "1 New Quote to Approve", "2 Custom Cake Deposits Paid").
   - **Middle Grid:** Quick Actions (Add Product, Scan QR, New Post).
   - **Bottom List:** AI Department Updates (e.g., Business Advisory: "Yesterday was your busiest day!").

3. **Mobile UX Flow:**
   - **Navigation:** Bottom app bar with Home, Inbox (Customer Success), Orders (Operations), Settings.
   - **Forms:** Native keyboard inputs. Large touch targets (44x44px minimum).
   - **Visuals:** Outfit font for headings, Inter for body. Dark/light mode support with blur backdrops.

### AI Agent Integration Points
- **Onboarding:** "Marketing & Advertising" uses initial inputs to generate branding and structure.
- **Inbox:** "Customer Success" reads incoming DMs and drafts replies for 1-tap approval.
- **Reporting:** "Business Advisory" aggregates weekly data and pushes a natural language notification every Monday morning.

### Key Design Decisions
- **Mobile-First Everything:** Since Carlos and Fatima only use phones, all management interfaces (including adding inventory or approving quotes) must be flawless on a 375px screen.
- **1-Tap Approvals:** High-risk actions (sending quotes, drafting emails) require human oversight but minimal effort.
- **Unified Department Orchestration:** Using the KAIROS Orchestrator to route events (e.g., Order -> Payment -> Customer Success follow-up) ensures a cohesive experience rather than disjointed notifications.

## Implementation Prompt
**Task for Implementer:**
Implement the end-to-end Onboarding and Activation flow for the OHC mobile client.
- **User-Facing Outcome:** A non-technical user can input their business idea in a simple text field, and within 3 screens, reach a fully populated dashboard with their personalized storefront link ready to share.
- **CUJ:** User opens app -> Enters business description -> System orchestrates "Marketing & Advertising" AI to generate a business profile -> User lands on Dashboard seeing their first AI Advisory message.
- **Acceptance Criteria:**
  - Must display perfectly on a 375px width screen without horizontal scrolling.
  - Touch targets must be at least 44x44px.
  - The flow must communicate with the KAIROS Orchestrator to instantiate the business tenant.
  - Must include E2E Playwright tests verifying the UI journey from initial launch to the populated dashboard.

## Priority
P0

## Estimated Scope
Large

</div>