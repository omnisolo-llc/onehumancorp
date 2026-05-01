# [architecture] Business Journey Architecture

## Problem Statement
The OneHumanCorp (OHC) platform aims to empower non-technical users to go from zero to a live, functioning business in under 10 minutes. Without a cohesive, documented understanding of the end-to-end journey across diverse business personas, our product and engineering teams risk building isolated features that do not flow together naturally. We need a unified architecture document that maps out the exact sequence of actions for our key personas (Maya, Carlos, Priya, Leo, and Fatima) across acquisition, onboarding, activation, retention, revenue, and referral stages.

## Research Report
Based on our target personas and the OHC platform promise, the following patterns emerge:
- **Mobile-First is Mandatory:** Users like Maya (baker) and Fatima (food cart) run their operations exclusively from a mobile device (iOS and Android). Desktop is additive, not the baseline.
- **Immediate Value (Activation):** The "Aha!" moment happens when a user sees their first product live or receives their first booking/payment. The flow must aggressively prioritize this outcome over extensive configuration.
- **AI as an Invisible Guide:** Users are not configuring technical settings. Instead, AI departments (e.g., Marketing, Sales) handle the setup based on simple questions during onboarding.
- **Differentiated Needs but Unified Framework:** While a handyman (Carlos) needs a booking system and a boutique owner (Priya) needs physical inventory sync, the fundamental phases (Acquire, Onboard, Activate, Retain) remain consistent.

### Persona Analysis Summary
- **Maya (The Home Baker):** Focuses on visual catalog and custom order deposits. Heavily relies on Instagram integrations for DMs and auto-posting.
- **Carlos (The Freelance Handyman):** Needs service listings, booking with deposits, and automated quote generation based on customer descriptions. Android priority.
- **Priya (The Boutique Owner):** Requires online/in-store sync, variants, tap-to-pay (Stripe Terminal), and analytics. Omnichannel (mobile + desktop).
- **Leo (The Music Tutor):** Calendar sync, video meeting generation (Zoom/Google Meet), subscription packages, and TikTok link-in-bio presence.
- **Fatima (The Food Cart Operator):** Multi-language UI, pre-order pickup flows with notifications, and printable summaries. Low-end Android focus.

## Design Doc

### Journey Phases
1. **Acquisition:** How the user discovers OHC (e.g., social ad, referral link).
2. **Onboarding:** A conversational, AI-led wizard gathering minimum context (business name, type, primary goal) in under 3 minutes.
3. **Activation:** The system auto-generates the necessary infrastructure (storefront, booking page, AI agents) and the user takes the first meaningful action (e.g., adding a product, sharing a link).
4. **Retention:** Engagement loops powered by the Business Advisory Agent (e.g., weekly performance summaries, push notifications for new orders).
5. **Revenue:** Trigger points for upgrading from the Free tier to Starter/Pro (e.g., reaching product limits, needing custom domains).
6. **Referral:** Built-in sharing mechanics (e.g., "Powered by OHC" branding on free tier, explicit referral programs).

### Architecture Diagrams

#### Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant Web as OHC Landing Page
    participant App as OHC Mobile App
    participant AI as AI Promoter Agent
    participant AI_CS as AI Ambassador Agent
    participant Cust as Customer

    Maya->>Ad: Clicks "Start a Bakery in 10 mins"
    Ad->>Web: Redirects to App Store
    Maya->>App: Downloads & Opens App
    App->>AI: Conversational Onboarding ("What do you sell?")
    Maya-->>App: "Custom Cakes"
    AI->>App: Generates Storefront & Menu Template
    Maya->>App: Uploads 3 Cake Photos, sets deposit price
    App->>AI: Publishes Storefront to OHC subdomain
    Maya->>App: Connects Instagram Account
    Note over Maya, AI: Activation Reached (Live in < 10 mins)
    Cust->>Maya: Sends Insta DM "Do you do vegan cakes?"
    AI_CS->>Maya: Drafts reply "Yes! Here is the link to order."
    Maya->>AI_CS: Approves Draft (1-tap)
    AI_CS->>Cust: Sends DM with booking link
    Cust->>App: Places custom order & pays deposit
    App->>Maya: Push Notification "New Order!"
```

#### Carlos (The Freelance Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant WOM as Word of Mouth
    participant App as OHC App (Android)
    participant AI as AI Promoter Agent
    participant AI_Sales as AI Salesperson Agent
    participant Cust as Customer

    Carlos->>WOM: Hears about OHC from a friend
    Carlos->>App: Downloads App
    App->>AI: Onboarding ("What service do you provide?")
    Carlos-->>App: "Handyman, plumbing, painting"
    AI->>App: Generates Service Booking Page with Calendar
    Carlos->>App: Sets hourly rate and availability
    App->>AI: Publishes Booking Page
    Carlos->>Cust: Texts link to past client
    Cust->>App: Fills out "Request a Quote" form
    App->>AI_Sales: Generates Quote based on description
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: Approves Quote
    AI_Sales->>Cust: Sends Official Quote with Deposit Link
    Cust->>App: Pays Deposit
    App->>Carlos: Notification "Job Booked & Deposit Paid"
```

## Implementation Prompt
**For Implementer Agents:**
Implement the core conversational onboarding flow and initial AI template generation.

**Critical User Journey (CUJ):**
1. A new user opens the mobile application for the first time.
2. The user is presented with a conversational UI (chat interface) powered by the AI Agent.
3. The AI asks three essential questions: Business Name, Business Category (Product, Service, Food, etc.), and Primary Goal (e.g., "Sell online", "Take bookings").
4. Upon completion, the system must invoke the Orchestrator to generate the initial required database records (Tenant, initial generic Products/Services, Agent configurations).
5. The user is transitioned to their dashboard, where a "Getting Started" checklist is populated based on their category.

**Acceptance Criteria:**
- The onboarding flow must operate entirely within the 375px mobile viewport constraint without horizontal scrolling.
- The UI must use the defined design tokens (Outfit/Inter typography, appropriate spacing).
- A full E2E Playwright/mocked-AI test must exist covering the complete flow from app launch, through the 3 questions, ending at the populated dashboard.

## Priority
P0

## Estimated Scope
Large

#### Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant App as OHC App
    participant AI as AI Promoter Agent
    participant AI_Adv as AI Advisor Agent
    participant Term as Stripe Terminal
    participant DB as Inventory

    Priya->>App: Onboarding ("Clothing boutique")
    AI->>App: Generates Omnichannel Storefront
    Priya->>App: Connects POS / Stripe Terminal
    Priya->>App: Syncs physical inventory
    App->>DB: Updates stock levels
    Priya->>Term: Taps customer card in-store
    Term->>App: Payment Success Event
    App->>DB: Deducts 1 unit from inventory
    DB->>App: Syncs stock to online storefront
    AI_Adv->>Priya: Weekly alert "Low stock on Red Dress (size M)"
```

#### Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant App as OHC App
    participant AI as AI Promoter Agent
    participant AI_CS as AI Ambassador Agent
    participant Cal as Booking Calendar
    participant Student as Student

    Leo->>App: Onboarding ("Music Tutoring")
    AI->>App: Generates Booking Page & Subscription Link
    Leo->>App: Sets calendar availability & Zoom integration
    Leo->>Student: Shares TikTok link-in-bio
    Student->>App: Buys Monthly Lesson Package
    App->>Cal: Books 4 weekly slots
    Cal->>App: Generates meeting links
    App->>Student: Sends Zoom links
    AI_CS->>Student: "Reminder: Guitar lesson tomorrow!"
    Student->>AI_CS: Misses class
    AI_CS->>Leo: Drafts follow-up: "Missed you! Want to reschedule?"
```

#### Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant App as OHC App
    participant AI as AI Promoter Agent
    participant Cust as Customer

    Fatima->>App: Onboarding ("Food cart, pick-up only, Arabic UI")
    AI->>App: Generates Menu Page (Bilingual)
    Fatima->>App: Uploads menu items & sets quantities
    App->>AI: Publishes to local web link
    Fatima->>App: Turns "Live/Taking Orders" toggle ON
    Cust->>App: Browses menu, pays for Falafel Plate
    App->>Fatima: Loud push notification "Order: Falafel Plate - Pickup in 15m"
    Fatima->>App: Taps "Order Ready"
    App->>Cust: SMS "Your food is ready for pickup!"
```

### Friction Point Analysis
- **Onboarding Drop-off:** Non-technical users like Maya and Fatima might abandon the flow if the AI asks more than 3-4 questions or requires complex technical jargon (e.g., "DNS", "Webhooks"). The conversational setup MUST remain extremely simple.
- **Payment Setup Friction:** Setting up Stripe (especially for POS like Priya) requires KYC verification, which can be overwhelming. The app must guide this process gently, perhaps allowing "Test Mode" activation before requiring full KYC to preserve the "Aha!" moment.
- **Inventory & Calendar Sync:** Leo and Priya will abandon the platform if syncing their existing tools (Google Calendar, existing POS) fails or causes double-booking/overselling. The sync must be robust and error-tolerant.
- **Mobile Usability:** Fatima uses a low-end Android device with potentially slow data. If the app is bloated, slow, or requires horizontal scrolling, she will abandon it. Strict adherence to mobile-first performance and 375px breakpoints is critical.
