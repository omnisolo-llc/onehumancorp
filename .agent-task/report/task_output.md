# [architecture] Business Journey Architecture for OneHumanCorp

## Problem Statement
Small business owners—whether they are a baker like Maya, a handyman like Carlos, or a boutique owner like Priya—need a frictionless path to start, run, and grow their businesses. Existing solutions are often fragmented and require significant technical knowledge. There is a need to clearly map out the end-to-end user journeys for these real personas across Acquisition, Onboarding, Activation, Retention, Revenue, and Referral, to identify friction points and ensure a cohesive, mobile-first experience powered by invisible AI agents.

## Research Report
Based on the OneHumanCorp product vision and real user personas, we have analyzed the business journeys for our key target demographics:
- **Maya (Home Baker, 28):** Mobile-first, Instagram-driven, needs simple booking and deposit payments.
- **Carlos (Freelance Handyman, 42):** Word-of-mouth, needs service listings, calendar booking, and quoting.
- **Priya (Boutique Owner, 35):** Omni-channel, needs inventory sync, POS, and analytics.
- **Leo (Music Tutor, 22):** Online/in-person, needs scheduling, recurring subscriptions, and a link-in-bio.
- **Fatima (Food Cart Operator, 50):** Low-end Android, limited English, needs simple pre-orders and notifications.

Competitor analysis (Shopify, Wix, Squarespace) shows that these platforms often require 30-60 minutes of setup and technical knowledge. OHC's goal is to reduce this to under 10 minutes with zero technical knowledge.

### Key Friction Points Identified
1. **Initial Setup Overwhelm:** Asking for too much information upfront during onboarding.
2. **Payment Configuration:** Setting up Stripe/payments is often complex.
3. **Mobile Management:** Existing platforms often have subpar mobile management experiences.
4. **Content Generation:** Writing descriptions and taking good photos is a barrier.

## Design Doc

### Business Journey Maps (Sequence Diagrams)

#### 1. Maya (The Home Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant OHC Mobile App
    participant AI Promoter (Marketing)
    participant AI Manager (Operations)
    participant AI Accountant (Finance)

    %% Acquisition & Onboarding
    Maya->>OHC Mobile App: Downloads app via Instagram ad
    OHC Mobile App->>Maya: Simple wizard (Business Name, Type: Baker)
    Maya->>OHC Mobile App: Connects Instagram account
    OHC Mobile App->>AI Promoter: Generate storefront based on IG photos
    AI Promoter-->>Maya: Storefront draft ready (under 5 mins)
    Maya->>OHC Mobile App: Approves storefront

    %% Activation
    Maya->>OHC Mobile App: Connects bank account for deposits
    OHC Mobile App-->>Maya: Store is LIVE!

    %% Retention & Revenue (Daily Use)
    note over Maya, AI Manager: Customer sends IG DM: "Vegan cakes?"
    AI Promoter (via CS)->>Maya: Drafts reply "Yes! Here's the link to order."
    Maya->>OHC Mobile App: Approves reply
    note over Maya, AI Manager: Customer places order & pays deposit
    AI Manager->>Maya: Push Notification: "New Custom Order! $50 deposit paid."
    AI Accountant->>OHC Mobile App: Updates revenue dashboard

    %% Referral
    AI Promoter->>Maya: Suggests sharing a referral code with the customer
```

#### 2. Carlos (The Freelance Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant OHC Android App
    participant AI Salesperson (Sales)
    participant AI Manager (Operations)
    participant AI Advisor

    %% Acquisition & Onboarding
    Carlos->>OHC Android App: Signs up from a friend's referral link
    OHC Android App->>Carlos: Wizard (Services offered, Pricing rough estimate)
    Carlos->>OHC Android App: Enters: Plumbing, Painting, Gen Repair
    OHC Android App->>AI Salesperson: Generate service listing page

    %% Activation
    Carlos->>OHC Android App: Sets availability calendar
    OHC Android App-->>Carlos: Booking page LIVE

    %% Retention (Daily Use)
    note over Carlos, AI Manager: Customer requests quote for leaky pipe
    AI Salesperson->>Carlos: Drafts quote based on typical plumbing jobs
    Carlos->>OHC Android App: Approves & sends quote
    note over Carlos, AI Manager: Customer accepts & books time
    AI Manager->>Carlos: Push Notification: "Job booked for Tuesday 10am."

    %% Revenue & Growth
    AI Advisor->>Carlos: Weekly report: "Plumbing jobs are your most profitable. Consider raising rates 10%."
```

#### 3. Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant OHC App (Mobile & Web)
    participant AI Manager (Operations)
    participant AI Accountant (Finance)
    participant AI Promoter (Marketing)

    %% Acquisition & Onboarding
    Priya->>OHC App (Mobile & Web): Signs up to sync in-store and online
    OHC App (Mobile & Web)->>Priya: Wizard to import existing inventory CSV
    Priya->>OHC App (Mobile & Web): Uploads CSV

    %% Activation
    OHC App (Mobile & Web)->>AI Promoter: Automatically generate product descriptions
    Priya->>OHC App (Mobile & Web): Configures Stripe Terminal for in-store POS
    OHC App (Mobile & Web-->>Priya: Omnichannel setup complete

    %% Retention (Daily Use)
    note over Priya, AI Manager: In-store purchase via POS
    AI Accountant->>OHC App (Mobile & Web): Records transaction
    AI Manager->>OHC App (Mobile & Web): Updates central inventory count
    note over Priya, AI Manager: Online purchase
    AI Manager->>Priya: Push Notification: "New online order to ship!"

    %% Revenue & Growth
    AI Advisor->>Priya: "Blue dresses are trending online. Run a promotion?"
    Priya->>OHC App (Mobile & Web): Approves AI Promoter to send email blast
```

#### 4. Leo (The Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant OHC Mobile App
    participant AI Manager (Operations)
    participant AI Promoter (Marketing)
    participant AI Salesperson (Sales)

    %% Acquisition & Onboarding
    Leo->>OHC Mobile App: Downloads app for TikTok link-in-bio
    OHC Mobile App->>Leo: Wizard (Service: Tutor, Calendar Sync)
    Leo->>OHC Mobile App: Connects Google Calendar
    OHC Mobile App->>AI Promoter: Generate link-in-bio & lesson booking page
    AI Promoter-->>Leo: Link-in-bio ready
    Leo->>OHC Mobile App: Approves and adds to TikTok

    %% Activation
    Leo->>OHC Mobile App: Sets up recurring subscription plans
    OHC Mobile App-->>Leo: Subscriptions LIVE!

    %% Retention (Daily Use)
    note over Leo, AI Manager: Student books lesson
    AI Manager->>Leo: Push Notification: "New lesson booked! Zoom link generated."

    %% Revenue & Growth
    note over Leo, AI Salesperson: Student inactive for 2 weeks
    AI Salesperson->>Leo: Drafts follow-up message to student
    Leo->>OHC Mobile App: Approves message
```

#### 5. Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    actor Fatima
    participant OHC Android App (Low-End)
    participant AI Promoter (Marketing)
    participant AI Manager (Operations)

    %% Acquisition & Onboarding
    Fatima->>OHC Android App (Low-End): Signs up (Arabic language selected)
    OHC Android App (Low-End)->>Fatima: Wizard (Food Menu, Upload Photos)
    Fatima->>OHC Android App (Low-End): Uploads photos of dishes
    OHC Android App (Low-End)->>AI Promoter: Generate multi-language photo menu
    AI Promoter-->>Fatima: Menu ready for review

    %% Activation
    Fatima->>OHC Android App (Low-End): Approves menu, enables pre-orders
    OHC Android App (Low-End)-->>Fatima: Pre-orders LIVE!

    %% Retention (Daily Use)
    note over Fatima, AI Manager: Customer pre-orders Halal Chicken via Web
    AI Manager->>Fatima: Push Notification (Loud Tone): "New order pickup at 12:30!"

    %% Revenue
    Fatima->>OHC Android App (Low-End): Marks order "Sold Out" when running low on ingredients
```

### UI Wireframes & Mobile UX Flow
**1. Home Dashboard (375px first):**
- **Top:** Glassmorphic card summarizing today's key metric (e.g., "$150 Revenue Today").
- **Middle (Action Queue):** A prominent list of tasks drafted by AI requiring review (e.g., "Draft reply to customer DM", "Review weekly report").
- **Bottom:** Quick actions (e.g., "Add Product", "New Booking").
- **UX Flow:** All actions are presented as simple conversational prompts. Tapping an action opens a focused screen.

**2. Draft Review Screen (375px first):**
- **Content:** The AI-generated draft (e.g., a quote or email) displayed clearly.
- **Actions:** Large, thumb-friendly buttons at the bottom: "Approve" (Primary), "Edit" (Secondary).
- **UX Flow:** Tapping "Approve" executes the action and triggers a micro-animation confirming success, returning to the Home Dashboard.

**3. Onboarding Wizard (375px first):**
- **Content:** One question per screen (e.g., "What type of business do you run?").
- **Actions:** Large selection buttons or simple text inputs.
- **UX Flow:** Progress bar at the top. Skips complex setup (like bank details) until the user sees their generated storefront, reducing drop-off.

### Key Design Decisions
1. **Deferred Onboarding:** Only ask for absolute minimums (Business Type, Name) to get a storefront draft. Defer complex tasks (bank linking, domain setup) until the user has experienced value (seeing their site).
2. **AI Content Generation:** Use the AI Promoter to auto-generate descriptions and sites based on social media or minimal input, lowering the barrier to entry.
3. **Mobile-First Notifications:** Retention is driven by high-value push notifications (new orders, drafts to review) on 375px screens.
4. **Actionable Advisory:** The Business Advisor provides plain-language, actionable insights to drive revenue and upgrades.

## Implementation Prompt
**Task:** Implement the unified deferred onboarding flow and core event routing for the AI Promoter.
**CUJ:** A new user (e.g., Maya) signs up on a mobile device, provides only their business type and connects an Instagram account. The system must immediately trigger the AI Promoter to generate a draft storefront and present it to the user. The user can then approve the draft and is guided to the next activation step (e.g., connecting a bank account).
**Acceptance Criteria:**
- Create the deferred onboarding data models in the database.
- Implement the integration with the AI Promoter to generate storefront drafts from minimal input.
- Develop the mobile-first UI for reviewing and approving the generated storefront.
- Ensure E2E tests cover the full flow from sign-up to storefront approval without mocking the network (AI model responses may be mocked).

## Priority
P0

## Estimated Scope
Medium
