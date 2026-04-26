# [business] End-to-End Business Journey Architecture

## Title
Design the Complete End-to-End User Journey for Real Small Business Owners

## Problem Statement
Small business owners, especially those who are non-technical (e.g., Maya the Baker, Carlos the Handyman), experience significant friction when onboarding to existing platforms like Shopify, Wix, or Squarespace. These platforms often require hours of setup, technical configuration, and manual data entry. Our users need a platform where they can go from "zero" to a "live business" in under 10 minutes, entirely from a mobile phone, with AI agents invisibly handling the complexity. The current flow lacks a seamless, mobile-first, zero-jargon journey that natively integrates AI from the moment of onboarding through continuous business growth.

## Research Report
### Competitive Analysis
- **Shopify:** Powerful but overwhelming for non-technical users. Setup takes 30-60 minutes minimum. Requires understanding of "Themes," "Liquid," and various plugin integrations. Mobile app is functional for management, but onboarding is heavily desktop-oriented.
- **Wix:** Easier drag-and-drop, but still requires 20-40 minutes and design sensibility. The AI features (Wix ADI) often feel like a starting point that still needs manual tweaking rather than an invisible, autonomous agent.
- **Squarespace:** Beautiful templates, but geared towards creative professionals. Lacks specialized tools for local services or simple food pre-orders out of the box.

### Pain Points for OHC Personas
- **Maya (Baker):** Overwhelmed by "inventory management" jargon. Needs simple deposit flows and Instagram DM automation.
- **Carlos (Handyman):** Has no website and uses an Android phone. Needs instant service listings, deposit booking, and quote generation without navigating complex dashboards.
- **Priya (Boutique):** Needs storefront and inventory sync. Also needs in-person tap-to-pay functionality to smoothly integrate physical and online spaces.
- **Leo (Music Tutor):** Needs seamless lesson booking synchronized with Google Calendar, Zoom link generation, and subscription models.
- **Fatima (Food Cart):** Needs a low-data, multi-lingual, extremely simplified pickup/pre-order flow.

### Key Opportunities
The primary opportunity is to shift the paradigm from a "Software Dashboard" to an "AI Business Partner." The journey must feel less like configuring a software tool and more like answering a few simple questions from a business consultant (the AI).

## Design Doc

### User Journey Sequence Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker)
```mermaid
sequenceDiagram
    participant U as User (Maya)
    participant OHC as OHC Mobile App
    participant AI as AI Agents (Marketing/Ops)
    participant DB as OHC Platform (PostgreSQL)

    %% Acquisition & Onboarding
    U->>OHC: Downloads App, Taps "Start My Business"
    OHC->>U: Asks 3 plain-language questions (Name, What you sell, Vibe)
    U->>OHC: Answers: "Maya's Cakes", "Custom Cakes", "Fun & Colorful"
    OHC->>AI: Trigger "Marketing & Advertising" AI
    AI-->>OHC: Generates storefront design, product placeholders
    OHC->>DB: Saves Business & Renders Preview
    OHC->>U: Shows Storefront Preview

    %% Activation
    U->>OHC: Taps "Add my first cake" (Uploads photo)
    OHC->>AI: Trigger "Operations" AI
    AI-->>OHC: Auto-generates description, suggests price & deposit options
    U->>OHC: Approves and taps "Go Live"
    OHC->>DB: Publishes Storefront, Provisions Subdomain

    %% Retention & Revenue
    loop Daily Operations
        U->>OHC: Checks Dashboard
        DB->>AI: Trigger "Business Advisory" AI
        AI-->>OHC: Generates daily plain-language report
        OHC->>U: Push Notification: "You have 2 new custom cake requests. Tap to reply."
        U->>OHC: Replies via AI-drafted response
    end

    %% Referral
    U->>OHC: Uses link-in-bio feature to promote store on Instagram
    OHC->>U: Generates beautiful sharing card and referral link
```

#### 2. Carlos (The Freelance Handyman)
```mermaid
sequenceDiagram
    participant U as User (Carlos)
    participant OHC as OHC Mobile App
    participant AI as AI Agents (Sales/Ops)
    participant DB as OHC Platform

    %% Acquisition & Onboarding
    U->>OHC: Downloads App, Taps "Start My Business"
    OHC->>U: Enters "Carlos Repairs", "Handyman Services", "Reliable"
    OHC->>AI: Trigger "Marketing & Advertising" AI
    AI-->>OHC: Generates service listings template, reviews section

    %% Activation
    U->>OHC: Adds service "Plumbing Fixes" with a base price
    OHC->>DB: Saves service & booking calendar setup
    U->>OHC: Taps "Go Live"

    %% Retention & Revenue
    loop Daily Operations
        U->>OHC: Customer requests quote via site
        DB->>AI: Trigger "Sales & Acquisition" AI
        AI-->>OHC: Drafts quote based on customer's problem description
        OHC->>U: Push Notification: "Quote drafted for plumbing fix. Review and send."
        U->>OHC: Approves quote, sends to customer
    end

    %% Referral
    U->>OHC: Completes job, app prompts for review request
    OHC->>DB: Sends review link to customer
```

#### 3. Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    participant U as User (Priya)
    participant OHC as OHC Mobile App/Desktop
    participant AI as AI Agents (Ops/Finance)
    participant DB as OHC Platform

    %% Acquisition & Onboarding
    U->>OHC: Signs up on Desktop
    OHC->>U: Enters "Priya's Boutique", "Clothing", "Chic"
    OHC->>AI: Generates storefront with product grid & variants template

    %% Activation
    U->>OHC: Syncs in-store inventory via CSV or direct input
    OHC->>DB: Sets up POS configuration
    U->>OHC: Enables Tap-to-Pay on mobile

    %% Retention & Revenue
    loop Daily Operations
        U->>OHC: Processes in-person payment via Tap-to-Pay
        DB->>AI: Updates inventory instantly
        AI-->>OHC: Generates daily sales report (mobile + desktop)
    end

    %% Referral
    U->>OHC: Sets up customer loyalty program and newsletter
    DB->>AI: Auto-drafts "New Arrivals" email
```

#### 4. Leo (The Music Tutor)
```mermaid
sequenceDiagram
    participant U as User (Leo)
    participant OHC as OHC Mobile App
    participant AI as AI Agents (Customer Success/Ops)
    participant DB as OHC Platform

    %% Acquisition & Onboarding
    U->>OHC: Downloads App, Enters "Leo's Guitar", "Music Lessons", "Creative"
    OHC->>AI: Generates profile page with booking calendar and Zoom integration

    %% Activation
    U->>OHC: Sets up "Monthly Lesson Package" (Subscription)
    OHC->>DB: Configures Stripe subscription billing
    U->>OHC: Shares link-in-bio on TikTok

    %% Retention & Revenue
    loop Daily Operations
        U->>OHC: Student books a lesson
        DB->>AI: Auto-generates Zoom link and syncs Google Calendar
        DB->>AI: Trigger "Customer Success" AI to follow up on inactive students
    end

    %% Referral
    U->>OHC: Student refers a friend using referral code tracking
```

#### 5. Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    participant U as User (Fatima)
    participant OHC as OHC Mobile App
    participant AI as AI Agents (Marketing/Ops)
    participant DB as OHC Platform

    %% Acquisition & Onboarding
    U->>OHC: Downloads App, Enters details in Arabic
    OHC->>AI: Generates bilingual menu (Arabic/English)
    OHC->>U: Shows photo menu template

    %% Activation
    U->>OHC: Takes photos of menu items, AI sets up pre-order flow
    OHC->>DB: Configures low-data dashboard mode

    %% Retention & Revenue
    loop Daily Operations
        U->>OHC: Customer places pre-order
        OHC->>U: Loud Phone Notification: "New pickup order!"
        U->>OHC: Marks item as 'Sold Out' (one tap)
        DB->>AI: Auto-updates menu availability
    end

    %% Referral
    U->>OHC: Uses printable QR code flyer generated by app for the cart
```


### UI Wireframes & Screen Flow (375px Mobile-First)

#### Screen 1: Welcome & Onboarding
- **Header:** "Welcome to OneHumanCorp."
- **Content:** Three large, tap-friendly input fields.
    1. "What is the name of your business?"
    2. "What do you sell or do?" (Dropdown/Free-text: Baked Goods, Handyman, etc.)
    3. "Describe your style in one word." (Visual chips: Elegant, Playful, Professional)
- **Action:** A prominent "Create My Business" button spanning the width (minus padding), ensuring touch target >= 44x44px.

#### Screen 2: The "Magic" Loading Screen
- **Content:** Delightful micro-animations showing AI agents "building" the store. "The Promoter is designing your site...", "The Accountant is setting up payments...".
- **Design:** Glassmorphism overlay (backdrop-filter: blur(20px)).

#### Screen 3: First Product & Activation
- **Header:** "Let's add your first item."
- **Content:** Native camera integration. User takes a photo. The screen displays the photo with an AI-generated title and price suggestion.
- **Action:** "Looks Good! Go Live."

### Mobile UX Flow
- **Input Strategy:** Rely on native mobile keyboards exclusively (e.g., numeric keypad for price).
- **Navigation:** Bottom navigation bar for core areas: Home (Dashboard), Inbox (Customer Success), Store (Products/Services), AI Advisor.
- **Feedback:** Optimistic UI updates. When Maya taps "Save Cake," it instantly appears saved, while network requests process in the background. If a network failure occurs, a subtle toast notifies the user to retry.

### AI Agent Integration Points
- **Onboarding:** "Marketing & Advertising" AI builds the initial site draft from 3 inputs.
- **Product Creation:** "Operations" AI suggests pricing, descriptions, and categories based on a single image upload.
- **Ongoing Engagement:** "Customer Success" AI drafts replies to customer inquiries; "Business Advisory" AI pushes plain-language weekly health reports.

### Key Design Decisions
1. **Zero-Jargon Promise:** Terms like "SKU", "SEO", or "Webhook" are completely hidden. Instead, we use "Product Code", "Getting Found on Google", and "Automatic Updates".
2. **AI as Default:** The user never "writes" a full product description from scratch unless they want to. The AI drafts it first.
3. **Optimistic Rendering:** To ensure a snappy feel on mobile, all UI actions assume success while the backend queues the job.
4. **Agent Departments:** Features are conceptually grouped into friendly departments ("The Promoter", "The Accountant") rather than technical modules, aligning with how small business owners think.

## Implementation Prompt
**Context:** We need to implement the onboarding and activation flow for a non-technical small business owner. The goal is to collect minimal initial data and use AI to generate the rest of the storefront and settings.

**User Journey (CUJ):**
1. User opens the app on a mobile device (375px width).
2. User enters their business name, type, and aesthetic preference.
3. The app displays a loading screen while AI agents generate the initial storefront and business settings.
4. User uploads one photo to create their first product/service.
5. AI generates the product description and suggests a price.
6. User confirms, and the business goes "Live."

**Acceptance Criteria:**
- The flow must be entirely mobile-responsive (tested at 375px width).
- Touch targets for all buttons and inputs must be at least 44x44px.
- The UI must use native mobile keyboards appropriately.
- The UI must replace all technical jargon with plain language (e.g., "Publish Store" -> "Go Live").
- Provide 100% E2E test coverage using Playwright starting from login, navigating the entire flow, and asserting the storefront is created with the AI-generated content. Mock the AI model responses during testing.

## Priority
P0

## Estimated Scope
Large
