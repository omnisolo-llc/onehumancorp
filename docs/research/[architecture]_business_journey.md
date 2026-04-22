<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [Architecture] Business Journey Definition & Blueprint

## Problem Statement

Small business owners—bakers, handymen, boutique owners, tutors, and food cart operators—are not technical experts. Yet, existing platforms like Shopify, Wix, and Squarespace force them into steep learning curves, requiring hours to set up complex configurations, understand confusing terminology, and learn web design.

For the everyday entrepreneur, the current onboarding and operation process is overwhelming. They don't just want a "website," they need an end-to-end operational hub—a system that gets them from idea to a fully functioning, customer-ready business in minutes. They need a journey that seamlessly handles acquisition, frictionless onboarding, quick activation, persistent retention mechanisms, clear revenue paths, and organic referral loops. This journey must adapt perfectly to their unique business type while abstracting away all underlying complexity through AI agents.

## Research Report

Our analysis evaluated the standard journeys across major competitors and identified significant friction points for non-technical users.

### Competitor Analysis
*   **Shopify:** Excellent for scaling e-commerce but overwhelming initial setup. The onboarding wizard is long, and the "activation" moment (getting the first sale) takes days or weeks due to the complexity of setting up shipping, taxes, and themes. Heavy reliance on desktop for meaningful management.
*   **Wix/Squarespace:** Website builders first, business tools second. Users spend hours dragging and dropping elements instead of running their business. Not inherently mobile-first for management.
*   **GoDaddy:** Simple setup but limited functionality. Often traps users in a basic tier with a confusing upgrade path when they need actual business features (like advanced booking or inventory).

### Core Findings
1.  **Time to Value (TTV):** The single biggest predictor of success is how quickly the user experiences their first "win" (e.g., publishing the site, receiving a test order). Our target is < 10 minutes.
2.  **Mobile-First is Mandatory:** Many of our target personas (Maya, Carlos, Fatima) do not own a laptop or prefer not to use one for business tasks. The entire lifecycle must be manageable via a smartphone.
3.  **Context Switching:** Users hate jumping between an app for their website, another for payments (Stripe), another for booking (Calendly), and another for messaging (Instagram). The journey must unify these under the "OneHumanCorp" umbrella.
4.  **AI as an Invisible Guide:** Users don't want to configure an AI; they want the AI to proactively do the work. The onboarding process should feel like an interview with a helpful assistant, not a form-filling exercise.

### Friction Points
1.  **Blank Canvas Paralysis:** Asking users to "choose a theme" or "add a section" causes anxiety. Users need a fully generated, working template based on their business type immediately.
2.  **Complex Terminology:** Words like "DNS", "SEO", "Variants", and "SKUs" confuse non-technical users. The interface must use plain language (e.g., "Get found on Google", "Sizes and Colors").
3.  **Fragmented Tools:** Having to set up Stripe independently, link a domain manually, and configure email servers creates massive drop-off during onboarding.

### User Personas & The OHC Differentiator
The OneHumanCorp (OHC) platform must perfectly accommodate:
*   **Maya (The Home Baker):** Mobile-only, needs deposits, custom orders, and automated DM replies.
*   **Carlos (The Freelance Handyman):** Android user, needs service listings, booking, quotes, and a customer inbox.
*   **Priya (The Boutique Owner):** Needs mobile and desktop access, online/in-store inventory sync, and POS payments.
*   **Leo (The Music Tutor):** Needs subscription packages, calendar sync, automated Zoom links, and a link-in-bio.
*   **Fatima (The Food Cart Operator):** Needs a simple photo menu, pre-orders, and phone notifications, working on low-end hardware and multiple languages.

## Design Doc

The Business Journey Architecture defines the standard progression a user takes through the OHC platform.

### Key Journey Stages

1.  **Acquisition:** How the user discovers OHC (Organic search, social media, word of mouth, "Powered by OHC" badges on existing storefronts).
2.  **Onboarding:** The initial 10-minute flow to set up the business. Powered by a conversational AI interface.
    *   *Minimum Inputs:* Business Name, Category, Basic Contact Info.
    *   *Deferred Inputs:* Advanced branding, complex tax info, domain connection.
3.  **Activation:** The moment the business is "live" and ready to accept customers.
    *   *Success Metric:* First product added, first payment received, storefront published.
4.  **Retention:** Daily engagement drivers.
    *   *Hooks:* Push notifications for new orders, daily AI summaries, simple inbox management.
5.  **Revenue:** The upgrade path.
    *   *Triggers:* Reaching limits on products/AI actions, needing a custom domain. The transition from Free to Starter must feel like a logical step due to business growth.
6.  **Referral:** The viral loop.
    *   *Mechanisms:* Easy sharing of the storefront link, referral programs for bringing other businesses to OHC.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Instagram
    participant OHC_App
    participant OHC_AIAgent
    participant Customer

    %% Acquisition & Onboarding
    Maya->>Instagram: Sees OHC Ad "Start your bakery online in minutes"
    Maya->>OHC_App: Downloads App, starts conversational onboarding
    OHC_AIAgent->>Maya: "What do you sell?"
    Maya->>OHC_AIAgent: "Custom cakes"
    OHC_AIAgent->>OHC_App: Auto-generates bakery storefront with deposit workflow

    %% Activation
    Maya->>OHC_App: Uploads 3 cake photos
    OHC_App->>Maya: Storefront is LIVE. Here is your link.

    %% Retention & Execution
    Customer->>Instagram: DMs Maya: "Do you do vegan cakes?"
    OHC_AIAgent->>Customer: "Yes! Here is the link to order a custom vegan cake: [Link]"
    Customer->>OHC_App: Places custom order, pays deposit via Stripe
    OHC_App->>Maya: Push Notification: "New Custom Order! $50 deposit received."

    %% Referral/Growth
    Maya->>Instagram: Shares OHC Storefront link in Bio
```

#### 2. Carlos (The Freelance Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant WordOfMouth
    participant OHC_App
    participant OHC_AIAgent
    participant Client

    %% Acquisition & Onboarding
    WordOfMouth->>Carlos: Client says, "Do you have a website to book you?"
    Carlos->>OHC_App: Downloads Android app.
    OHC_AIAgent->>Carlos: "What services do you offer?"
    Carlos->>OHC_AIAgent: "Plumbing fixes, Painting, General Repairs"
    OHC_AIAgent->>OHC_App: Auto-generates service listing & booking calendar

    %% Activation
    Carlos->>OHC_App: Sets basic pricing and availability
    OHC_App->>Carlos: Booking page is LIVE.

    %% Retention & Execution
    Client->>OHC_App: Selects "Plumbing", picks Friday 2pm, describes issue.
    OHC_AIAgent->>Client: Auto-sends generated quote based on issue description.
    Client->>OHC_App: Approves quote, pays $20 deposit.
    OHC_App->>Carlos: Notification: "New Job Booked for Friday. Quote approved."

    %% Revenue (Upgrade Trigger)
    OHC_App->>Carlos: "You've booked 10 jobs this month! Upgrade to Starter for unlimited bookings."
```

#### 3. Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant OHC_App
    participant OHC_Desktop
    participant OHC_AIAgent
    participant InStoreCustomer
    participant OnlineCustomer

    %% Onboarding & Activation
    Priya->>OHC_Desktop: Signs up, connects existing inventory list
    OHC_AIAgent->>OHC_Desktop: Auto-generates multi-page e-commerce storefront
    Priya->>OHC_App: Installs mobile app for in-store management

    %% Execution (Hybrid)
    InStoreCustomer->>Priya: Buys a red dress in-store
    Priya->>OHC_App: Uses Tap-to-Pay (Stripe Terminal)
    OHC_App->>OHC_App: Auto-deducts "Red Dress, Size M" from inventory

    OnlineCustomer->>OHC_Desktop: Browses storefront, buys blue dress
    OHC_App->>Priya: Notification: "New Online Order to ship."

    %% Retention
    OHC_AIAgent->>Priya: Daily Report: "Revenue today: $450. The Blue Dress is trending."
```

#### 4. Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant TikTok
    participant OHC_App
    participant OHC_AIAgent
    participant Student

    %% Acquisition & Onboarding
    Leo->>TikTok: Wants a link-in-bio for his guitar tutorials
    Leo->>OHC_App: Signs up, selects "Tutoring/Services"
    OHC_AIAgent->>OHC_App: Sets up subscription packages and calendar sync

    %% Activation
    Leo->>TikTok: Adds OHC profile link to bio

    %% Execution
    Student->>TikTok: Clicks link
    Student->>OHC_App: Buys "Monthly Guitar Package (4 lessons)"
    OHC_App->>Leo: Notification: "New Subscriber! Meeting links auto-generated."

    %% Retention
    OHC_AIAgent->>Student: Follow-up email after 2 weeks: "Ready for your next lesson?"
```

#### 5. Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant LocalCommunity
    participant OHC_App
    participant Customer

    %% Acquisition & Onboarding
    LocalCommunity->>Fatima: Customers ask to order ahead
    Fatima->>OHC_App: Opens app (Arabic UI on low-end Android)
    Fatima->>OHC_App: Takes photos of menu items
    OHC_App->>OHC_App: Auto-generates simple mobile pre-order menu

    %% Activation
    Fatima->>LocalCommunity: Puts QR code on food cart

    %% Execution
    Customer->>Customer: Scans QR code
    Customer->>OHC_App: Orders 2 Falafel wraps, pays online
    OHC_App->>Fatima: Loud Phone Notification: "New Order: 2 Falafel Wraps"
    Fatima->>OHC_App: Marks item "Sold Out" with one tap when she runs out of falafel.
```

### UI Wireframes or Screen Flow Description
1.  **Onboarding Chat Interface (375px width):** A full-screen chat interface. The AI (represented by an animated, friendly avatar) asks one question at a time. Quick-reply chips (e.g., "Retail", "Services", "Food") are positioned above the mobile keyboard. The background features a subtle glassmorphism blur.
2.  **The "Magic Moment" Screen:** After the final onboarding question, a loading screen with micro-animations showing the AI "building" the store (icons of pages, calendars, and products snapping into place).
3.  **The Mobile Dashboard (375px width):** The primary hub. A clean, tabbed interface (Home, Inbox, Orders/Bookings, Settings). The Home tab features "The Advisor" widget at the top with plain-language insights (e.g., "You have 3 new messages today.").

### AI Agent Integration Points
1.  **Marketing & Advertising Agent:** Triggered during onboarding to dynamically generate the site layout, color palette, and initial copywriting based on the user's business category and name.
2.  **Customer Success Agent:** Integrated into the shared Inbox tab. When a customer messages (via site widget or Instagram DM), the agent drafts a suggested reply visible only to the owner, who can approve or edit it.
3.  **Business Advisory Agent:** Runs a nightly batch process evaluating the day's sales and interactions to generate the plain-language daily/weekly summary widget on the dashboard.

### Key Design Decisions and Why
1.  **Conversational Onboarding over Forms:** *Why?* Forms feel like work and cause drop-off. A conversational interface feels like hiring an assistant, aligning with the "AI Does the Work" core value.
2.  **No "Draft" Mode for Initial Store:** *Why?* The store is published instantly, even with placeholder images. This forces the "Activation" moment faster. Users can always edit it, but the hurdle of "Publishing" is removed.
3.  **Unified Inbox:** *Why?* Small business owners lose track of communications scattered across platforms. Bringing SMS, email, and social DMs into one inbox, assisted by the Customer Success agent, drastically reduces response time and cognitive load.

## Implementation Prompt

**Role:** Product Engineering Swarm
**Task:** Implement the unified "First 10 Minutes" Onboarding and Activation flow based on the Business Journey Architecture.
**Goal:** Create a frictionless, mobile-first onboarding experience that allows a user to go from downloading the app to having a live, published business entity with their first product/service configured.

**Acceptance Criteria:**
1.  **Conversational Onboarding:** The initial setup must simulate a brief chat/interview with an AI assistant rather than a static form.
2.  **Dynamic Template Generation:** Based on the user's business category (e.g., Bakery, Handyman, Food Cart), the system must automatically provision the correct initial UI state (e.g., physical product catalog vs. service booking calendar).
3.  **Mobile-First Constraints:** The entire flow must be completely functional and visually perfect on a 375px width screen. Touch targets must be >= 44x44px. Native mobile keyboards must be triggered appropriately (e.g., numeric for price).
4.  **Deferred Complexity:** The flow must ONLY ask for the absolute minimum information required to go live. Advanced settings (domain, detailed taxes) must be relegated to a post-activation checklist.
5.  **Activation Celebration:** Upon completing the required steps and publishing, the user must receive a clear visual confirmation (celebratory UI) and their live link.

**Note:** Do not implement specific database schemas or API contracts. Focus on implementing the end-to-end frontend flow and integrating with the existing backend orchestration layers to trigger the necessary AI agents.

## Priority
P0 - Critical Path

## Estimated Scope
Large

</div>