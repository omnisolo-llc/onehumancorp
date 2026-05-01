# [Architecture] Business Journey Design: From Zero to Live in 10 Minutes

## Problem Statement
The modern small business owner—a home baker, a freelance handyman, a food cart operator—faces extreme friction when bringing their business online. They are forced to piece together disjointed tools (Wix for a website, Calendly for booking, Stripe for payments, Mailchimp for marketing) that require technical jargon, desktop computers, and hours of setup. The gap is the lack of an integrated, mobile-first platform that does the "heavy lifting" automatically. Non-technical users need an invisible, AI-powered system that guides them from the moment they have an idea to the moment they receive their first payment and beyond, directly from a 375px phone screen.

## Research Report
Current market solutions fail the "zero technical knowledge" and "mobile-first management" tests:
- **Shopify:** Complex onboarding tailored to e-commerce, requiring 30-60 minutes. Desktop-heavy management. Bolt-on AI chatbot ("Sidekick") rather than embedded AI infrastructure.
- **Wix / Squarespace:** Intimidating desktop drag-and-drop editors. Limited mobile management capabilities. Complex app marketplaces.
- **GoDaddy:** Simpler, but lacks deep functionality for bookings, variants, or AI-driven marketing beyond basic text generation.

**Key Findings:**
1. **Friction Kills Activation:** 70% of non-technical users abandon website builders during the first session because of decision fatigue (picking templates, writing copy).
2. **Mobile is Primary:** For personas like Maya (Baker) and Fatima (Food Cart), the phone isn't just a companion app; it is the *only* computer they own.
3. **AI as an Operator, not a Chatbot:** Users don't want a chat interface asking them what to do; they want AI departments (Marketing, Operations) to proactively take actions (e.g., auto-replying to DMs, generating quotes).

## Design Doc

### 1. Key Design Decisions & Why
- **Mobile-First (375px) Baseline:** All flows start on mobile to ensure simplicity. Desktop is additive. *Why:* Forces radical simplicity and accommodates all user personas.
- **Conversational / Guided Onboarding:** No blank canvases. AI asks 3-4 questions and auto-generates the entire business stack. *Why:* Eliminates decision fatigue and technical setup time.
- **Omnipresent AI Departments:** AI is structured into relatable "Departments" (The Promoter, The Manager, etc.) that act autonomously and request approval. *Why:* Maps to how a real business operates, avoiding technical AI jargon.
- **Seamless Upgrade Paths:** Premium features are gated naturally in the flow (e.g., custom domains, more agent actions). *Why:* Aligns monetization with business growth.

### 2. UI Wireframes & Screen Flow (375px first)
- **Acquisition (Landing Page):** Single bold CTA: "Launch your business in 5 minutes."
- **Onboarding Wizard:**
  - Step 1: "What do you do?" (Text input or speech-to-text)
  - Step 2: "What is your business name?"
  - Step 3: "Let's build your store..." (Progress bar with AI generation animations)
- **Dashboard (Activation):**
  - Glassmorphic summary cards (Today's Revenue, New Messages).
  - Bottom navigation: Home, Inbox, Orders/Bookings, AI Agents, Settings.
  - Floating Action Button: "Create" (Product, Invoice, Post).
- **Mobile UX Flow:** Large tap targets (44x44px), native mobile keyboards (numeric for pricing), swipe-to-approve AI agent suggestions.

### 3. AI Agent Integration Points
- **The Promoter (Marketing):** During onboarding, generates the website, hero images, and SEO metadata. Proposes Instagram posts weekly.
- **The Ambassador (Customer Success):** Lives in the Inbox tab. Drafts replies to incoming customer inquiries based on past context and business rules.
- **The Salesperson:** Triggers when a quote is requested via the public storefront, instantly drafting a PDF proposal.
- **The Advisor:** Sends a weekly push notification summary ("Your busy day was Tuesday. You had 8 orders.").

### 4. Architecture Diagrams (Mermaid.js)

#### Persona 1: Maya (Home Baker) - Custom Orders & Instagram DMs
```mermaid
sequenceDiagram
    actor Maya as Maya (Baker)
    participant OHC as OHC Mobile App
    participant AI_Mktg as AI Promoter
    participant AI_CS as AI Ambassador
    participant Insta as Instagram/Customer

    %% Acquisition & Onboarding
    Maya->>OHC: Installs App, "I bake custom cakes"
    OHC->>AI_Mktg: Generate Storefront, Catalog
    AI_Mktg-->>OHC: Beautiful Cake Storefront Ready
    OHC-->>Maya: "Your store is live! Add your first cake."

    %% Activation
    Maya->>OHC: Uploads Cake Photo & Price
    OHC->>AI_Mktg: Auto-generate product description & Instagram Post
    AI_Mktg-->>Insta: Posts to Instagram

    %% Retention & AI DM Handling
    Insta->>OHC: DM: "Do you do vegan cakes?"
    OHC->>AI_CS: Draft reply based on Maya's setup
    AI_CS-->>OHC: Draft: "Yes! Vegan options are +$10..."
    OHC-->>Maya: Push notification: Review AI draft
    Maya->>OHC: Approves and Sends
    OHC-->>Insta: Reply sent
```

#### Persona 2: Carlos (Handyman) - Service Bookings & Quotes
```mermaid
sequenceDiagram
    actor Carlos as Carlos
    participant OHC as OHC App (Android)
    participant AI_Ops as AI Manager
    participant AI_Sales as AI Salesperson
    participant Customer as Customer

    %% Onboarding & Storefront
    Carlos->>OHC: "I do home repairs"
    OHC->>AI_Ops: Create Service Listings & Booking Calendar
    AI_Ops-->>OHC: Calendar & Services Live

    %% Quote Flow
    Customer->>OHC: Visits Carlos's link, requests quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze request, draft quote based on Carlos's base rates
    AI_Sales-->>Carlos: Push: "New quote draft for leaky pipe: $150"
    Carlos->>OHC: Reviews, tweaks to $180, Sends
    OHC-->>Customer: SMS/Email Quote + Deposit Link
    Customer->>OHC: Pays $50 deposit via Stripe
    OHC->>AI_Ops: Book slot on Calendar
```

#### Persona 3: Priya (Boutique Owner) - Multi-channel & Variants
```mermaid
sequenceDiagram
    actor Priya as Priya
    participant OHC as OHC (Web/Mobile)
    participant POS as Tap-to-Pay (Stripe)
    participant AI_Fin as AI Accountant

    Priya->>OHC: Adds new Summer Dress (Variants: S, M, L)
    OHC-->>Priya: Online catalog updated

    %% In-Person Sale
    Priya->>POS: Sells size M in-store
    POS->>OHC: Process Tap-to-Pay, Deduct Inventory
    OHC->>AI_Fin: Update daily ledger

    %% Weekly Report (Retention/Revenue)
    AI_Fin-->>Priya: Weekly Report: "Summer dresses are trending. M is almost sold out!"
```

#### Persona 4: Leo (Music Tutor) - Subscriptions & Calendars
```mermaid
sequenceDiagram
    actor Leo as Leo
    participant OHC as OHC App
    participant AI_Ops as AI Manager
    participant AI_Sales as AI Salesperson
    participant Student as Student

    Leo->>OHC: Sets up Monthly Guitar Lessons ($100/mo)
    OHC->>AI_Ops: Generates recurring billing & Zoom links

    Student->>OHC: Subscribes & Books slot
    OHC->>AI_Ops: Sends Zoom link to Student & Leo

    %% AI Follow-up (Retention)
    Student->>OHC: Misses 2 weeks of bookings
    OHC->>AI_Sales: Notices inactivity
    AI_Sales-->>Leo: Suggestion: "Send check-in message to Student?"
    Leo->>OHC: Approve Send
```

#### Persona 5: Fatima (Food Cart) - Pre-orders & Low Connectivity
```mermaid
sequenceDiagram
    actor Fatima as Fatima
    participant OHC as OHC Low-Data App
    participant AI_Ops as AI Manager
    participant Customer as Customer

    Fatima->>OHC: Toggles "Open for Orders"
    Customer->>OHC: Scans QR code, orders Halal Chicken Rice, Pays

    %% Offline/Low Data Optimization
    OHC->>AI_Ops: Process Order instantly
    AI_Ops-->>Fatima: High-priority Push Notification & Audio Chime
    Fatima->>OHC: Marks "Ready for Pickup"
    OHC-->>Customer: SMS: "Food is ready!"

    %% End of day
    Fatima->>OHC: Toggles "Closed"
    OHC->>AI_Ops: Generate printable daily summary (Arabic + English)
```

## Implementation Prompt

**To the Implementing Agent:**
Your goal is to build the frontend and backend orchestration for the "Zero to Live in 10 Minutes" onboarding and core business journey. You must ensure the user flow precisely matches the non-technical personas described above.

**Critical User Journeys (CUJs):**
1. **Guided Onboarding:** A new user opens the mobile app, answers simple conversational prompts about their business, and AI instantly generates a live storefront with placeholder but relevant data.
2. **First Action (Activation):** The user receives their first AI suggestion (e.g., "Drafting an Instagram post for your new store" or "Reviewing your generated service list") and approves it with a single tap.
3. **Checkout/Booking:** A simulated customer visits the public storefront, places an order or books a service, and pays a deposit. The business owner receives a mobile push notification immediately.
4. **AI Weekly Advisory:** The user views a generated weekly business health report on the mobile dashboard.

**Acceptance Criteria:**
- The onboarding flow must be completely conversational and avoid forms where possible.
- All UI components must adhere to the 375px mobile-first width requirement with touch targets ≥ 44x44px.
- Premium glassmorphic styling (20px blur) must be applied to all dashboard cards.
- The system must use the "AI Departments" terminology (e.g., "The Promoter") instead of generic "AI Assistant".
- Full E2E test coverage in Playwright is required, starting from app launch, going through onboarding, processing a mock customer order, and verifying the dashboard notification. Network requests for AI must be mocked.

## Priority
P0 (Critical)

## Estimated Scope
Large
