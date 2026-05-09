# [research] Business Journey Architecture: End-to-End Persona Flow

## Problem Statement
Small business owners—whether they are a baker like Maya or a handyman like Carlos—struggle with the friction of moving from an initial idea to a fully operational digital business. They face overwhelming technical decisions when building a storefront, configuring booking calendars, setting up online payments, and managing customer communications. The current onboarding and lifecycle flows on existing platforms treat businesses as static websites rather than dynamic entities. There is a critical need to design a complete, end-to-end "zero → live business in under 10 minutes" journey, optimized strictly for mobile devices, where AI agents invisibly manage complexity at every stage (Acquisition, Onboarding, Activation, Retention, Revenue, Referral).

## Research Report

### Persona-Specific Pain Points
| Persona | Business Type | Key Pain Points |
|---------|---------------|-----------------|
| **Maya (28)** | Baker | Instagram DM overload; manually tracking deposits; wants a storefront but has no time to maintain it; relies exclusively on iPhone. |
| **Carlos (42)** | Handyman | Losing leads due to missing online presence; word-of-mouth is unscalable; needs quick mobile quotes and deposits; Android only. |
| **Priya (35)** | Boutique Owner | Out of sync inventory between in-store and online; needs daily analytics without using complex dashboards; wants a newsletter. |
| **Leo (22)** | Music Tutor | Manual calendar management; forgotten Zoom links; struggles to retain inactive students; needs a TikTok link-in-bio presence. |
| **Fatima (50)** | Food Cart | Language barriers (Arabic/English); managing complex pre-orders; low-end Android phone; needs simple visual interfaces and printed lists. |

### Competitive Analysis
| Feature / Platform | Shopify | Wix | Squarespace | GoDaddy | OneHumanCorp (Target) |
|--------------------|---------|-----|-------------|---------|-----------------------|
| **Setup Time** | Days / Weeks | Days | Hours / Days | Hours | **< 10 minutes** |
| **AI Integration** | Bolt-on tools | Basic generators | Bolt-on tools | Simple text gen | **Invisible AI Departments** |
| **Mobile-First** | App companion | App companion | Desktop-first | App companion | **100% Mobile Native** |
| **Complexity Level** | High | Medium | Medium | Low-Medium | **Zero Code, Zero Manuals** |

### Actionable Recommendations
1. **Defer Friction:** Minimize the number of inputs required to go live. Ask only for the business name, category, and primary goal (e.g., "sell products", "book appointments"). AI should infer or draft the rest.
2. **Invisible Agents:** Immediately assign AI "Departments" (e.g., The Promoter, The Manager) during onboarding to show value before the user completes their profile.
3. **Mobile-First Activation:** Define "Activation" as the moment the first product is added or first payment received via mobile tap.
4. **Retention Loop:** Provide daily push-notification summaries from the AI "Advisor" instead of relying on the user to open analytics dashboards.

## Design Doc

### Architecture Diagrams (User Journeys)

#### 1. Maya (Baker) Journey
```mermaid
sequenceDiagram
    autonumber
    actor Maya as Maya
    participant App as OHC Mobile App
    participant Onboarding as The Manager (AI Onboarding)
    participant Promoter as The Promoter (AI Marketing)
    participant Customer as Instagram Customer

    Note over Maya,App: Acquisition & Onboarding
    Maya->>App: Downloads OHC from Instagram Ad
    App->>Onboarding: Initiates "Zero-to-Live" Wizard
    Onboarding->>Maya: Asks: "What do you do?" (Voice/Text)
    Maya->>Onboarding: "I sell custom vegan cakes"
    Onboarding->>App: Generates draft storefront, photo placeholders, deposit policy
    App->>Maya: Presents 375px Storefront Draft
    Maya->>App: Approves (1 tap) & connects Instagram

    Note over Maya,Customer: Activation & Revenue
    Customer->>Promoter: DMs Maya's Instagram: "Do you have vegan chocolate?"
    Promoter->>Customer: "Yes! Here is the order link with deposit." (Invisible AI action)
    Customer->>App: Pays deposit
    App->>Maya: Push Notification: "New $50 deposit from Alex."

    Note over Maya,App: Retention & Referral
    App->>Maya: Day 7 Notification: "You made $300 this week! Let's offer a referral discount."
    Maya->>App: Taps "Approve Campaign"
```

#### 2. Carlos (Handyman) Journey
```mermaid
sequenceDiagram
    autonumber
    actor Carlos as Carlos
    participant App as OHC Mobile App (Android)
    participant Onboarding as The Manager
    participant Sales as The Salesperson
    participant Customer as Potential Client

    Note over Carlos,App: Acquisition & Onboarding
    Carlos->>App: Downloads OHC from word-of-mouth referral
    App->>Onboarding: Initiates "Zero-to-Live" Wizard
    Onboarding->>Carlos: Asks: "What services do you offer?"
    Carlos->>Onboarding: "Plumbing repair and installation"
    Onboarding->>App: Generates service list with pricing estimates & booking calendar
    App->>Carlos: Presents Draft Booking Page
    Carlos->>App: Approves (1 tap)

    Note over Carlos,Customer: Activation & Revenue
    Customer->>App: Submits quote request via Booking Page
    App->>Sales: Analyzes request details
    Sales->>Carlos: Drafts quote: "$150 for pipe fix. Approve to send?"
    Carlos->>App: Taps "Approve & Request Deposit"
    App->>Customer: Sends Quote + Payment Link
    Customer->>App: Pays deposit
    App->>Carlos: "Deposit paid. Job booked for Tuesday 2PM."
```

#### 3. Priya (Boutique Owner) Journey
```mermaid
sequenceDiagram
    autonumber
    actor Priya as Priya
    participant App as OHC Mobile App
    participant Ops as The Manager (Ops/Inventory)
    participant Advisor as The Advisor
    participant Customer as In-store Customer

    Note over Priya,App: Acquisition & Onboarding
    Priya->>App: Downloads OHC to sync in-store and online
    App->>Ops: Initiates "Zero-to-Live" Wizard
    Ops->>Priya: "Scan your first 5 items"
    Priya->>App: Scans clothing tags/barcodes
    Ops->>App: Auto-categorizes and creates product variants (Size/Color)
    App->>Priya: Shows Draft Storefront
    Priya->>App: Approves (1 tap)

    Note over Priya,Customer: Activation & Revenue
    Customer->>App: Buys item in-store via Tap-to-Pay on Priya's phone
    App->>Ops: Instantly deducts from central inventory

    Note over Priya,App: Retention
    App->>Advisor: Daily end-of-day analytics check
    Advisor->>Priya: Push Notification: "You sold 10 dresses today. Inventory is low on Mediums. Order more?"
    Priya->>App: Taps "Yes, draft reorder email"
```

#### 4. Leo (Music Tutor) Journey
```mermaid
sequenceDiagram
    autonumber
    actor Leo as Leo
    participant App as OHC Mobile App
    participant Manager as The Manager (Calendar)
    participant Ambassador as The Ambassador (Retention)
    participant Student as Student

    Note over Leo,App: Acquisition & Onboarding
    Leo->>App: Downloads OHC to organize online lessons
    App->>Manager: Initiates Wizard
    Manager->>Leo: "When are you available to teach?"
    Leo->>Manager: "Mon-Wed afternoons"
    Manager->>App: Generates recurring booking slots, syncs to Google Calendar, creates Zoom integration
    App->>Leo: Presents TikTok Link-in-bio Draft
    Leo->>App: Approves & copies to TikTok profile

    Note over Leo,Student: Activation
    Student->>App: Books Tuesday 4PM lesson via TikTok link
    App->>Manager: Auto-generates Zoom link and calendar invite
    Manager->>Student: Sends confirmation email
    Manager->>Leo: Push Notification: "New booking for Tuesday 4PM."

    Note over Leo,App: Retention
    Ambassador->>Leo: Detects student missed 2 weeks. "Draft a check-in email to Sarah?"
    Leo->>App: Taps "Send"
```

#### 5. Fatima (Food Cart) Journey
```mermaid
sequenceDiagram
    autonumber
    actor Fatima as Fatima
    participant App as OHC Mobile App (Arabic UI)
    participant Manager as The Manager (Menu/Ops)
    participant Customer as Hungry Customer

    Note over Fatima,App: Acquisition & Onboarding
    Fatima->>App: Downloads OHC
    App->>Manager: Initiates Wizard (Arabic voice prompt)
    Manager->>Fatima: "Take a picture of your physical menu"
    Fatima->>App: Uploads photo
    Manager->>App: OCR parses items, generates bilingual digital menu (Arabic/English)
    App->>Fatima: Presents simple visual menu with toggles
    Fatima->>App: Approves (1 tap)

    Note over Fatima,Customer: Activation & Revenue
    Customer->>App: Browses digital menu in English, orders Falafel Plate, pays online
    App->>Manager: Receives order
    Manager->>App: Loud push notification ring on Fatima's phone (in Arabic): "New Order! Falafel Plate."
    Fatima->>App: Taps "Preparing"
    Manager->>Customer: SMS: "Your order is being prepared."

    Note over Fatima,App: Retention & Operations
    App->>Manager: End of day
    Manager->>Fatima: Generates simple printable daily order summary and profit breakdown.
```


### UI Wireframes & Screen Flow (375px)
1. **Welcome Screen:** "What kind of business are we building today?" with large, touch-friendly icon buttons (Store, Service, Food, etc.).
2. **The "Magic" Loading Screen:** Glassmorphic spinner showing AI actions in real-time ("The Promoter is designing your storefront...", "The Protector is drafting your refund policy...").
3. **Activation Dashboard:** A clean, simplified home screen. A floating action button (FAB) for "Add Item/Service". A prominent "Share Store Link" card.
4. **Daily AI Briefing:** A dismissible card at the top of the dashboard: "Good morning Maya. You have 2 cakes to bake today. The Promoter replied to 3 DMs overnight."

### Mobile UX Flow
- **Input:** Relies heavily on voice-to-text or short natural language inputs to reduce typing fatigue.
- **Progressive Disclosure:** Advanced settings (like tax configuration or custom domain setup) are hidden behind the AI Advisor and only suggested when the user reaches relevant milestones (e.g., approaching Free Tier limits).
- **Offline Capabilities:** Local caching allows Carlos to draft quotes in a basement without cellular service; syncing happens automatically when connectivity is restored.

### Key AI Agent Integration Points
- **The Manager (Operations):** Takes the initial user input and scaffolds the core business entities (catalog, services) dynamically.
- **The Promoter (Marketing):** Intercepts social media DMs to answer questions and funnel users to the checkout link without the owner's intervention.
- **The Advisor (Business Advisory):** Drives retention by sending weekly plain-language summaries (e.g., "Mondays are slow. Let's run a 10% discount this Monday.").

### Key Design Decisions
- **AI as the Onboarding Guide:** Instead of a static form, onboarding is conversational and generative. *Why:* Reduces abandonment rates by eliminating the "blank page" syndrome.
- **Mobile Parity Guarantee:** No features exist on desktop that cannot be executed in <= 30 seconds on a 375px viewport. *Why:* Ensures accessibility for users like Carlos and Fatima who only have mobile devices.
- **Push-Driven Retention:** The platform pushes insights to the user rather than expecting them to pull data. *Why:* Non-technical users find traditional analytics dashboards intimidating.

## Implementation Prompt
**Task:** Implement the "Zero-to-Live" mobile onboarding wizard for new tenants.
**CUJ:** A new user opens the OHC mobile app for the first time. They are greeted by an AI assistant that asks for a plain language description of their business. The system must use this description to orchestrate multiple AI Departments to generate a draft storefront, default service/product listings, and core policies. The user reviews the draft on their 375px screen and approves it with a single tap, transitioning their business to a "Live" state.
**Acceptance Criteria:**
- Create the mobile-first onboarding UI components matching the Design System (Glassmorphism, 375px optimized).
- Implement the orchestration logic that routes the user's initial prompt to the relevant AI Departments to generate business assets.
- Ensure the onboarding process completes in under 10 minutes from a user-interaction standpoint.
- The user must be able to approve the generated storefront with a single tap.
- Provide a suite of E2E Playwright/Slint tests that navigate the entire onboarding wizard from a clean slate to a "Live" business dashboard.

## Priority
P0

## Estimated Scope
Large
