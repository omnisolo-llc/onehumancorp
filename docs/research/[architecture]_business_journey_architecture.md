# [architecture]_business_journey_architecture.md

## Title
Business Journey Architecture

## Problem Statement
Small business owners (especially non-technical users like Maya, Carlos, Priya, Leo, and Fatima) need an intuitive, friction-free journey from discovering OHC to fully running their business on the platform. The current lack of a defined, persona-specific end-to-end journey risks high abandonment rates during onboarding and limits the long-term engagement and revenue potential of the platform.

## Research Report
### Findings & Competitive Analysis
- **Shopify:** Complex onboarding (30-60 min) requiring a high degree of technical or e-commerce knowledge. The setup process is overwhelming for users like Fatima or Maya who just need a simple storefront or pre-order system.
- **Wix / Squarespace:** Takes 20-40 min to set up. Requires significant design effort and manual configuration. While visually appealing, the UI is too complex for true mobile-first management.
- **GoDaddy:** Simpler setup (20-40 min) but lacks the depth of features needed for complex business types (e.g., booking + store + portfolio).
- **OHC's Opportunity:** A strictly <10 minute setup driven entirely by AI agents that handle the heavy lifting invisibly. A truly mobile-first management experience that allows a user to run their entire business from a 375px phone screen.

### Key Pain Points
- High abandonment during account creation and initial configuration.
- Difficulty in setting up payments, custom domains, and initial inventory.
- Lack of proactive guidance on next steps post-launch.

## Design Doc

### 1. Acquisition
- **Maya (Baker):** Discovers OHC via an Instagram ad showing a beautiful custom cake storefront. The CTA is "Build your cake shop in 10 minutes."
- **Carlos (Handyman):** Referred by a friend. The CTA is "Get booked and paid online today."
- **Priya (Boutique):** Organic search for "easy online store for boutique." CTA is "Sync your store and sell online instantly."
- **Leo (Music Tutor):** Discovers OHC via TikTok ad showing a simple booking link-in-bio. CTA is "Start booking students online."
- **Fatima (Food Cart):** Word-of-mouth from another food vendor. Needs simple pre-order pickup.

### 2. Onboarding (Zero to Live in <10 Minutes)
- **Step 1: The Basics (AI-Assisted):** Name of business, type of business, and preferred style (e.g., "Modern", "Playful"). AI immediately generates a draft storefront.
- **Step 2: Core Offering:** Add the first product/service (e.g., a photo of a cake, a "Plumbing Fix" service).
- **Step 3: Get Paid:** Connect Stripe or set up basic bank transfer details.
- **Step 4: Go Live:** Publish the store with an OHC subdomain. Custom domain setup is deferred to avoid friction.

### 3. Activation
- **Day 1 Success:** The user receives their first order or booking. The AI "Operations" agent sends a push notification and a simple next-step guide.
- **Week 1 Success:** The user has added more products/services and customized their storefront. The AI "Marketing" agent suggests creating an Instagram post.
- **Month 1 Success:** Consistent orders. The AI "Business Advisory" agent provides a weekly summary: "You made $500 this week! Your best seller was the Vegan Chocolate Cake."

### 4. Retention
- **Daily Engagement:** Push notifications for new orders, messages, and AI agent summaries.
- **Proactive Insights:** The AI "Advisor" identifies trends (e.g., "Tuesdays are slow, try a promotion") and suggests actions.
- **Seamless Management:** Managing inventory, updating prices, and replying to customers is as easy as sending a text message.

### 5. Revenue
- **Trigger for Upgrade:** When a user hits the limit of the Free tier (e.g., 10 products, 100 AI actions), the AI "Advisor" gently suggests upgrading to the Starter tier ($9/mo) to unlock more capacity and a custom domain.
- **Value Proposition:** The upgrade is framed as an investment in growth, not just a feature unlock.

### 6. Referral
- **Viral Loop:** Priya shares her beautiful OHC storefront link with another boutique owner. The footer says "Powered by OHC - Build yours in 10 mins."
- **Incentive:** Both Priya and the new user get a month of the Starter tier for free.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (Baker) Journey
```mermaid
sequenceDiagram
    participant Maya
    participant OHC_App
    participant AI_Marketing
    participant AI_Operations
    participant AI_Advisory

    Maya->>OHC_App: Clicks Instagram Ad / Starts Onboarding
    OHC_App->>Maya: Asks for Business Name & Type
    Maya->>OHC_App: Enters "Maya's Cakes", "Bakery"
    OHC_App->>AI_Marketing: Request Draft Storefront
    AI_Marketing-->>OHC_App: Returns Draft Storefront (Modern Style)
    OHC_App->>Maya: Displays Draft Storefront
    Maya->>OHC_App: Uploads first cake photo & price
    OHC_App->>AI_Operations: Initialize Inventory & Catalog
    Maya->>OHC_App: Connects Payment (Stripe)
    OHC_App->>Maya: "You're Live!" (Provides OHC Link)
    Maya->>OHC_App: Gets first order via Instagram DM
    AI_Operations->>Maya: Processes deposit payment
    AI_Advisory->>Maya: Weekly health report "Top seller: Vegan Cake"
```

#### 2. Carlos (Handyman) Journey
```mermaid
sequenceDiagram
    participant Carlos
    participant OHC_App
    participant AI_Marketing
    participant AI_Sales
    participant AI_Operations

    Carlos->>OHC_App: Referred by friend / Starts Onboarding
    OHC_App->>Carlos: Asks for Service Type
    Carlos->>OHC_App: Enters "Plumbing Fix", "Repair"
    OHC_App->>AI_Marketing: Request Service Page Draft
    AI_Marketing-->>OHC_App: Returns Clean Service Listing
    Carlos->>OHC_App: Sets pricing and availability
    OHC_App->>AI_Operations: Setup Booking Calendar
    Carlos->>OHC_App: Connects Payment (Stripe)
    OHC_App->>Carlos: "You're Live!" (Provides OHC Link)
    Carlos->>OHC_App: Gets first inquiry
    AI_Sales->>Carlos: Auto-generates quote based on inquiry
    AI_Operations->>Carlos: Schedules booking with deposit
```

#### 3. Priya (Boutique) Journey
```mermaid
sequenceDiagram
    participant Priya
    participant OHC_App
    participant AI_Marketing
    participant AI_Operations
    participant AI_Advisory

    Priya->>OHC_App: Search "online store for boutique" / Starts Onboarding
    OHC_App->>Priya: Asks for Sync options
    Priya->>OHC_App: Uploads initial inventory
    OHC_App->>AI_Operations: Setup Inventory with Variants
    OHC_App->>AI_Marketing: Request Multi-channel Draft
    AI_Marketing-->>OHC_App: Returns Desktop & Mobile Storefront
    Priya->>OHC_App: Configures Stripe Terminal (In-Person)
    OHC_App->>Priya: "You're Live!"
    Priya->>OHC_App: Sells item in store
    AI_Operations->>Priya: Syncs inventory
    AI_Advisory->>Priya: Daily analytics notification on mobile
```

#### 4. Leo (Music Tutor) Journey
```mermaid
sequenceDiagram
    participant Leo
    participant OHC_App
    participant AI_Marketing
    participant AI_Sales
    participant AI_Operations

    Leo->>OHC_App: Clicks TikTok Ad / Starts Onboarding
    OHC_App->>Leo: Asks for Booking Preferences
    Leo->>OHC_App: Sets up Subscription Packages
    OHC_App->>AI_Marketing: Request Link-in-Bio Draft
    AI_Marketing-->>OHC_App: Returns Portfolio & Booking Link
    Leo->>OHC_App: Connects Google Calendar
    OHC_App->>AI_Operations: Setup Zoom & Calendar Sync
    OHC_App->>Leo: "You're Live!"
    Leo->>OHC_App: Student books lesson
    AI_Operations->>Leo: Generates Zoom link and adds to Calendar
    AI_Sales->>Leo: Follows up with inactive students
```

#### 5. Fatima (Food Cart) Journey
```mermaid
sequenceDiagram
    participant Fatima
    participant OHC_App
    participant AI_Marketing
    participant AI_Operations

    Fatima->>OHC_App: Word-of-mouth / Starts Onboarding
    OHC_App->>Fatima: Asks for Menu & Language
    Fatima->>OHC_App: Uploads Menu (Arabic/English)
    OHC_App->>AI_Marketing: Request Bilingual Menu Draft
    AI_Marketing-->>OHC_App: Returns Mobile-first Menu
    Fatima->>OHC_App: Configures Pre-order options
    OHC_App->>AI_Operations: Setup Order Notification System
    OHC_App->>Fatima: "You're Live!"
    Fatima->>OHC_App: Customer orders pre-pickup
    AI_Operations->>Fatima: Loud phone notification
    Fatima->>OHC_App: Prints daily order list
```

## Implementation Prompt
**For Implementer Agent:**
Implement the core onboarding flow UI based on the Business Journey Architecture.
- **CUJ:** A new user opens the app, enters their business name and type, and receives an AI-generated draft storefront within 10 seconds.
- **Acceptance Criteria:**
  - The UI must be fully responsive, starting from a 375px mobile baseline.
  - The flow must include exactly three steps: Basics (Name/Type), Core Offering (First Product), and Get Paid (Mock payment setup for now).
  - Use the OHC Premium Token library for styling (Glassmorphism, Outfit/Inter fonts).
  - The flow must conclude with a "Success" screen displaying a mock live link.

## Priority
P0 (Critical)

## Estimated Scope
Medium
