# [Product Architecture] Business Journey Architecture

## Title
Research: Complete End-to-End Business Journey Architecture for OneHumanCorp

## Problem Statement
Small business owners (bakers, handymen, tutors, food cart operators) currently face immense friction when trying to launch a business online. Existing tools like Shopify, Wix, or Squarespace demand technical literacy, hours of setup time, and manual integration of disparate tools (website builders, calendars, payment gateways). For non-technical users, this complexity leads to high abandonment rates during onboarding. We need a unified, frictionless, mobile-first business journey that guides users from zero to live business in under 10 minutes, with AI handling all technical setup invisibly.

## Research Report
### Current Market Landscape
- **Shopify:** Powerful but overwhelming for single-person service or digital businesses. Requires extensive manual theme setup and app installations for features like bookings.
- **Wix/Squarespace:** Complex drag-and-drop builders that break easily on mobile. High friction in connecting payment gateways and setting up specific business logic.
- **Link-in-bio tools (Linktree, Stan):** Easy to set up but lack robust business features (inventory, complex scheduling, physical fulfillment).

### OHC User Needs & Friction Points
- **Friction Point 1:** Blank canvas paralysis. Users abandon setup when asked to design their site or write copy from scratch.
- **Friction Point 2:** Payment gateway setup (Stripe/PayPal integrations). Non-technical users struggle with API keys and business verification steps.
- **Friction Point 3:** Multi-tool fragmentation. Maya needs a catalog, Leo needs bookings, Fatima needs pre-orders. Setting these up manually takes days.

### Persona Analysis & Retention Drivers
- **Maya (Baker):** Needs Instagram DM integration and custom order deposits. Upgrade trigger: Custom domain requirement.
- **Carlos (Handyman):** Needs quote generation and booking calendar. Retention driver: Daily push notifications for new leads.
- **Priya (Boutique):** Needs inventory sync and in-person tap-to-pay. Referral loop: Branded invoices shared with other boutique owners.
- **Leo (Tutor):** Needs automated meeting links and subscription billing. Activation milestone: First recurring payment processed.
- **Fatima (Food Cart):** Needs ultra-simple, localized mobile UI (Arabic/English) for pre-orders. Friction point: High text density; needs icon/photo-driven UI.

## Design Doc

### Mobile UX Flow (375px First)
1. **Acquisition & Landing (0-1 min):**
   - Single, prominent CTA: "Start your business in 60 seconds."
   - User inputs just two things: "What is your business name?" and "What do you sell?" (e.g., "Maya's Cakes", "Custom Cakes").
2. **AI-Assisted Onboarding Wizard (1-3 min):**
   - **Simple Mode (Default):** Conversational UI where the AI asks 3-4 plain-language questions (e.g., "Do you take deposits?", "Where are you located?").
   - Progressive disclosure: Advanced technical settings are hidden by default.
   - The AI invisibly auto-generates the storefront, writes copy, and configures the relevant modules (catalog, calendar, menu).
3. **Activation (3-5 min):**
   - Immediate gratification: "Your store is live! Here is your link."
   - Next Best Action: "Add your first product photo" or "Connect your bank account to get paid."
4. **Retention Dashboard (Daily Use):**
   - Clean, widget-based mobile dashboard.
   - Prominent unread messages/orders.
   - "The Advisor" AI provides daily insights (e.g., "You had 10 visitors yesterday. Tap here to message them.").

### Key Design Decisions
- **No-Code Absolute:** Users never see API keys, DNS settings, or HTML.
- **Conversational Onboarding:** Replace long forms with a chat-like wizard driven by the AI Marketing & Operations departments.
- **Deferred Complexity:** Users can go live and accept orders *before* completing complex tax or compliance settings (handled later by "The Protector" agent via nudge).
- **Session-Sticky Simple Mode:** The UI always defaults to plain language.

### AI Agent Integration Points
- **The Promoter:** Generates initial website layout, copy, and SEO meta tags during onboarding.
- **The Manager:** Configures the appropriate backend modules (inventory vs. bookings) based on the user's initial prompt.
- **The Advisor:** Monitors the user's journey and sends push notifications to drive activation and retention (e.g., "Maya, you haven't added a photo to your new cake listing yet.").

### Architecture Diagrams (Mermaid.js)

#### Maya's Journey (Baker - Physical Products & Custom Orders)
```mermaid
sequenceDiagram
    actor Maya
    participant Marketing (Instagram)
    participant OHC Landing
    participant AI Promoter
    participant AI Manager
    participant AI Advisor

    %% Acquisition
    Maya->>Marketing (Instagram): Sees OHC Ad "Sell Cakes via DM"
    Maya->>OHC Landing: Clicks Link
    OHC Landing->>Maya: "What do you sell?"
    Maya->>OHC Landing: "Custom Cakes"

    %% Onboarding & Setup
    OHC Landing->>AI Promoter: Generate Site Draft
    AI Promoter-->>Maya: Presents generated cake storefront
    Maya->>AI Manager: Sets up deposit rule

    %% Activation
    AI Manager-->>Maya: "Store is Live. Add first cake."
    Maya->>OHC Landing: Uploads Cake Photo

    %% Retention
    Marketing (Instagram)->>AI Manager: Customer DMs Maya
    AI Manager-->>Maya: Push Notification "New Custom Order Inquiry"

    %% Revenue & Upgrade
    AI Advisor->>Maya: "Want 'mayascakes.com'? Upgrade to Starter."
    Maya->>OHC Landing: Upgrades to Starter ($9/mo)

    %% Referral
    Maya->>Marketing (Instagram): Shares OHC success story on Instagram
    Marketing (Instagram)->>OHC Landing: New baker clicks Maya's referral link
```

#### Carlos's Journey (Handyman - Services & Bookings)
```mermaid
sequenceDiagram
    actor Carlos
    participant Referral
    participant OHC Landing
    participant AI Manager
    participant AI Salesperson
    participant AI Advisor

    %% Acquisition
    Referral->>Carlos: Word of mouth from another contractor
    Carlos->>OHC Landing: Enters "Carlos Handyman Services"

    %% Onboarding
    OHC Landing->>AI Manager: Configure Service/Booking Module
    AI Manager-->>Carlos: Asks "What are your working hours?"
    Carlos->>AI Manager: "Mon-Fri 9-5"

    %% Activation
    AI Manager-->>Carlos: "Booking page live."

    %% Retention & Daily Use
    AI Salesperson->>Carlos: Push: "New quote request for plumbing."
    Carlos->>AI Salesperson: Taps "Approve $150 Quote"
    AI Salesperson-->>Referral: Sends Quote to Customer

    %% Revenue & Upgrade
    AI Advisor->>Carlos: "You've booked 10 clients. Upgrade to Pro for unlimited bookings."
    Carlos->>OHC Landing: Upgrades to Pro ($29/mo)

    %% Referral
    Carlos->>Referral: Shares custom OHC branded invoice
    Referral->>OHC Landing: Other contractor scans QR code on invoice
```

#### Priya's Journey (Boutique Owner - Physical Retail & Online)
```mermaid
sequenceDiagram
    actor Priya
    participant Organic Search
    participant OHC Landing
    participant AI Manager
    participant AI Promoter
    participant AI Advisor

    %% Acquisition
    Priya->>Organic Search: Searches "how to sell clothes online easy"
    Organic Search->>OHC Landing: Clicks OHC result
    OHC Landing->>Priya: "What is your business name?"
    Priya->>OHC Landing: "Priya's Boutique"

    %% Onboarding
    OHC Landing->>AI Manager: Sets up physical + online catalog
    AI Manager-->>Priya: "Do you have a physical store?"
    Priya->>AI Manager: "Yes"

    %% Activation
    AI Manager-->>Priya: "Your store is live. Add your first item."
    Priya->>OHC Landing: Adds "Summer Dress" variant sizes

    %% Retention
    AI Promoter->>Priya: Push: "You have 5 online visitors. Send a discount code?"
    Priya->>AI Promoter: "Yes, 10% off"

    %% Revenue
    AI Advisor->>Priya: "To connect your custom domain, upgrade to Starter."
    Priya->>OHC Landing: Upgrades to Starter ($9/mo)

    %% Referral
    Priya->>OHC Landing: Uses Tap-to-Pay in store
    OHC Landing->>AI Promoter: Generates receipt with referral link
```

#### Leo's Journey (Music Tutor - Digital Bookings & Subscriptions)
```mermaid
sequenceDiagram
    actor Leo
    participant TikTok
    participant OHC Landing
    participant AI Manager
    participant AI Salesperson
    participant AI Advisor

    %% Acquisition
    Leo->>TikTok: Posts guitar tutorial
    TikTok->>OHC Landing: Follower clicks link-in-bio
    OHC Landing->>Leo: "Start selling lessons."

    %% Onboarding
    Leo->>OHC Landing: Sets up "Leo's Guitar Lessons"
    OHC Landing->>AI Manager: Configures subscription and calendar modules
    AI Manager-->>Leo: "How much per hour?"
    Leo->>AI Manager: "$50"

    %% Activation
    AI Manager-->>Leo: "Calendar is live. Add your Zoom link."
    Leo->>OHC Landing: Adds Zoom link

    %% Retention
    AI Salesperson->>Leo: Push: "Student hasn't booked in 2 weeks. Send follow-up?"
    Leo->>AI Salesperson: "Yes"

    %% Revenue
    AI Advisor->>Leo: "Your 10th student subscribed! Upgrade to Pro for analytics."
    Leo->>OHC Landing: Upgrades to Pro ($29/mo)

    %% Referral
    Leo->>TikTok: Shares OHC feature on how easy subscriptions are
```

#### Fatima's Journey (Food Cart - Pre-Orders & Notifications)
```mermaid
sequenceDiagram
    actor Fatima
    participant Local Flyer
    participant OHC Landing
    participant AI Manager
    participant AI Advisor

    %% Acquisition
    Fatima->>Local Flyer: Scans QR code from OHC ad
    OHC Landing->>Fatima: (Arabic UI) "What do you sell?"
    Fatima->>OHC Landing: "Halal Food"

    %% Onboarding
    OHC Landing->>AI Manager: Generates simple photo-menu interface
    AI Manager-->>Fatima: "Upload your first dish photo."
    Fatima->>OHC Landing: Uploads Chicken Over Rice photo

    %% Activation
    AI Manager-->>Fatima: "Menu is live. Customers can now pre-order."

    %% Retention
    AI Manager->>Fatima: Loud Phone Notification: "New Pre-Order for Pickup at 12 PM"
    Fatima->>AI Manager: Marks "Ready"

    %% Revenue
    AI Advisor->>Fatima: "You've reached 100 orders this month. Upgrade to Starter."
    Fatima->>OHC Landing: Upgrades to Starter ($9/mo)

    %% Referral
    Fatima->>Local Flyer: Prints OHC QR code for her own cart
```


## Implementation Prompt
**For Implementer Agent:**
Implement the end-to-end Onboarding Wizard and Home Dashboard UI supporting the defined Business Journeys.
1. Create a mobile-first wizard (375px) that collects minimal business details (Name, Type).
2. Integrate the progressive disclosure pattern: ensure all technical settings (like DNS or Stripe API keys) are hidden behind a session-sticky "Advanced Mode" toggle, keeping the default view in plain language ("Simple Mode").
3. Build the core Retention Dashboard that displays "Next Best Actions" (e.g., "Add a product", "Connect bank") and recent AI Agent activity.
4. Ensure the UI gracefully adapts to different business types (e.g., showing a "Menu" module for food carts vs. "Calendar" for tutors).
5. All UI must adhere to the Visual Excellence Mandate (Glassmorphism, Outfit + Inter typography) and pass the 30-second "grandmother test". Write at least 5 E2E Playwright tests verifying this complete flow from login to the final dashboard.

## Priority
P0

## Estimated Scope
Large
