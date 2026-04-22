# [architecture] Business Journey Architecture

## Title
Business Journey Architecture: End-to-End Persona Workflows and Growth Loops

## Problem Statement
Small business owners (our core personas like Maya the baker, Carlos the handyman) face immense cognitive load and friction when setting up a digital presence. Existing platforms (Shopify, Wix) are designed for tech-savvy users and require multiple manual steps (domain setup, theme customization, payment gateway configuration). We need to map out the complete, frictionless, zero-code, AI-driven business journey for our core personas—from their first interaction with OneHumanCorp to becoming retained, paying, and referring users. If our onboarding and lifecycle flows contain friction, non-technical users will abandon the platform before reaching activation.

## Research Report
### Competitive Analysis
- **Shopify**: High activation energy. Requires understanding of themes, plugins, and separate payment gateways. Time to first sale is typically days/weeks. Focuses on tech-savvy or established SMBs.
- **Wix/Squarespace**: Template-driven but still requires manual dragging, dropping, and copywriting. Often leads to choice paralysis for non-designers.
- **GoDaddy/Zyro**: Faster setup but limited functionality and poor mobile management experience.
- **OHC's Differentiation**: Absolute zero technical knowledge required. The AI "Promoter" and "Manager" departments build the store, write the copy, and manage the backend. Management is truly mobile-first (100% functionality on a 375px screen).

### Journey Stages Analyzed
1. **Acquisition**: How users find us (TikTok, Instagram, referrals).
2. **Onboarding**: The < 10 minute wizard to get live. Minimal inputs required (Business Name, Type, 1-3 Photos).
3. **Activation**: First product/service added, first deposit/payment received.
4. **Retention**: AI-driven daily/weekly push notifications (e.g., "The Advisor" sending weekly health reports, "The Manager" sending order updates).
5. **Revenue**: Free -> Starter upgrade trigger (e.g., needing custom domain or exceeding 100 products/1,000 AI actions).
6. **Referral**: Viral loop (e.g., "Powered by OHC" link, referral program for other business owners).

## Design Doc

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (Baker) - Custom Orders & Deposit Flow
```mermaid
sequenceDiagram
    participant C as Customer
    participant M as Maya (OHC Mobile App)
    participant OHC as OHC Platform
    participant Promoter as AI Promoter
    participant Manager as AI Manager

    M->>OHC: Complete 3-step Onboarding (Name, "Custom Cakes", Upload 3 photos)
    OHC->>Promoter: Generate Storefront, Copy, & Pricing Draft
    Promoter-->>M: Push Notification: "Your store is ready to review!"
    M->>OHC: Approve Storefront
    C->>Promoter: Instagram DM: "Do you do vegan cakes?"
    Promoter-->>C: "Yes we do! Here is the custom order link." (AI replies while Maya sleeps)
    C->>OHC: Submits Custom Order form & pays deposit via Stripe
    OHC->>Manager: Process Order & Deposit
    Manager-->>M: Push Notification: "New $50 deposit from Sarah for a Vegan Cake!"
    M->>Manager: Schedule Delivery Date
    Manager-->>C: Email/SMS confirmation of date & remaining balance
```

#### 2. Carlos (Handyman) - Service Booking Flow
```mermaid
sequenceDiagram
    participant C as Customer
    participant C_App as Carlos (OHC Android App)
    participant OHC as OHC Platform
    participant Salesperson as AI Salesperson
    participant Manager as AI Manager

    C_App->>OHC: Sign up via Google, Select "Home Repairs"
    OHC->>Salesperson: Generate Service Menu Draft
    Salesperson-->>C_App: Suggests: Plumbing, Painting, General. (Carlos taps 'Approve')
    C->>OHC: Visits Carlos's profile, requests "Leaky sink repair"
    OHC->>Salesperson: Analyze request
    Salesperson-->>C_App: Drafts Quote for $150 based on problem description
    C_App->>OHC: Approves Quote
    OHC-->>C: Sends quote with booking calendar link
    C->>OHC: Picks Wednesday 2PM, pays $50 deposit
    OHC->>Manager: Add to Carlos's Google Calendar & send reminder
    Manager-->>C_App: Push: "New Booking: Leaky Sink on Wed 2PM. $50 deposit secured."
```

#### 3. Priya (Boutique) - In-Store & Online Sync Flow
```mermaid
sequenceDiagram
    participant P as Priya (OHC iOS/Desktop)
    participant OHC as OHC Platform
    participant Manager as AI Manager
    participant Accountant as AI Accountant

    P->>OHC: Scans new dresses via mobile camera
    OHC->>Manager: Auto-tag variants (Size S/M/L, Colors), update inventory
    Manager-->>OHC: Sync inventory online and to in-store POS
    C_InStore->>P: Buys dress in store using Tap-to-Pay on Priya's iPhone
    OHC->>Accountant: Process POS payment
    Accountant->>Manager: Deduct 1 dress from global inventory
    P->>OHC: Checks end-of-day analytics
    Accountant-->>P: Displays: "Revenue: $450. Best seller: Red Dress. 2 left in stock!"
```

### UI Wireframes & Mobile UX Flow (375px First)

- **Onboarding Flow (The "10-Minute" Wizard):**
  - **Screen 1**: "What's the name of your business?" (Large text input, native keyboard).
  - **Screen 2**: "What do you do?" (Pill selections: Baking, Handyman, Tutor, Retail, etc.).
  - **Screen 3**: "Upload 1-3 photos of your work." (Native image picker).
  - **Screen 4 (Loading):** "Our AI agents are building your business..." (Glassmorphism progress indicator, smooth animations).
  - **Screen 5**: "You're Live!" (Confetti animation, big "View My Store" button).

- **Daily Dashboard (The Hub):**
  - **Top Card**: "Today's Revenue" with big typography (Outfit font).
  - **Middle Section (The Inbox)**: Unified messages from Instagram, WhatsApp, Email, with "AI Drafts Ready" badges.
  - **Bottom Section**: Agent activity feed ("The Advisor suggests: Add a Mother's Day promo").

### AI Agent Integration Points
- **Onboarding (Promoter & Salesperson)**: Instantly generates website structure, service lists, and copy based on minimal inputs.
- **Daily Operations (Manager & Success)**: Drafts responses to customer inquiries, auto-updates inventory, coordinates bookings.
- **Growth (Advisor)**: Pushes actionable notifications (e.g., "You hit 100 sales! Let's ask them for reviews.").

### Key Design Decisions
1. **Deferred Configuration**: We do not ask for Stripe credentials or custom domain setup during onboarding. The goal is "Store Live" in under 10 minutes. Payments and domains are prompted contextually when they are actually needed (e.g., when the first order is placed or traffic increases).
2. **Unified Inbox**: Non-technical users struggle with managing 5 different apps. The OHC app aggregates Instagram DMs, SMS, and emails into one view where AI drafts responses.
3. **Conversational Interface Alternative**: Instead of standard settings menus, users can chat with "The Manager" to change things (e.g., "Add a 10% discount for next week").
4. **Push Notification Reliance**: Mobile users don't constantly check dashboards. We rely on intelligent, low-frequency, high-value push notifications to bring them back to the app.

## Implementation Prompt
**Objective**: Build the user onboarding flow (Acquisition -> Activation) reflecting the Business Journey Architecture.

**Acceptance Criteria**:
1. Implement the 3-step mobile-first onboarding wizard in Flutter (`//srcs/app/`).
2. Integrate with the backend `Tenant` and `BusinessProfile` creation APIs.
3. Trigger an asynchronous AI Job (via PostgreSQL `SKIP LOCKED` queue) to generate the initial storefront configuration using the "Promoter" agent.
4. Display a loading screen with appropriate micro-animations while the AI job processes, transitioning to the "You're Live!" screen upon completion.
5. Ensure 100% functional layout on a 375px viewport (no horizontal scrolling).
6. Implement full E2E test verifying a user can sign up, complete the wizard, and view the generated storefront.

## Priority
P0 (Critical)

## Estimated Scope
Large
