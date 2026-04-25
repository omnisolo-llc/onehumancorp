# Issue Brief: End-to-End Business Journey Architecture

## Title
Business Journey Architecture & Activation Workflows

## Problem Statement
Small business owners joining OneHumanCorp (OHC) range from completely non-technical bakers to semi-technical boutique owners. Existing platforms (Shopify, Wix) present immediate friction during onboarding by asking technical questions (e.g., DNS setup, payment gateway integration) before demonstrating value. If a user cannot experience their "aha" moment—seeing their live business ready to accept a real order—within the first 10 minutes, they churn. OHC needs a seamlessly guided, AI-assisted journey covering acquisition, onboarding, activation, retention, revenue, and referral that shields users from all technical complexity and operates flawlessly on mobile.

## Research Report
Based on market analysis of SMB platform onboarding and user behavior:
- **Acquisition**: Non-technical users convert best through social proof and direct link-in-bio examples (e.g., seeing another baker use OHC).
- **Onboarding**: Shopify requires ~30-60 minutes to set up a basic store. OHC targets <10 minutes. By leveraging AI to pre-fill descriptions, generate placeholder images, and design the initial layout based on just 3 inputs (Name, Business Type, Primary Goal), we eliminate the "blank canvas" paralysis.
- **Activation**: True activation happens when the first order is placed or booked. OHC must prioritize creating a functional storefront with at least one active product/service before asking for complex configurations.
- **Retention**: Push notifications showing real business activity (new orders, AI agent actions) are the strongest retention driver.
- **Friction Points Identified**:
  - Connecting payment gateways (Stripe setup can be daunting). OHC should allow deferred payout setup while instantly enabling pre-orders or deposit collection.
  - Adding inventory. OHC must support photo-to-product AI extraction.

## Design Doc
### High-Level Architecture
- **Journey Tracks**:
  - **Acquisition**: Discovery via social channels, organic search, or referral links. Landing page CTA focuses on "Start selling in 10 minutes."
  - **Onboarding**: Step-by-step wizard (375px mobile-first). AI generates the initial storefront.
  - **Activation**: First product/service creation. Success is defined by the user viewing their live, shareable storefront URL.
  - **Retention**: Push notifications for business activity, weekly plain-language AI advisory reports ("Your top seller is...").
  - **Revenue**: Trigger-based prompts to upgrade from Free to Starter when limits (e.g., product count, AI actions) approach 80%.
  - **Referral**: Viral loop where every receipt and booking confirmation includes a "Powered by OHC" link, turning customers into new tenants.

### Mobile UX Flow (375px First)
- **Onboarding Screen 1**: "What's your business called?" (Text input)
- **Onboarding Screen 2**: "What do you do?" (Grid selection: Bake, Fix, Teach, Sell Clothes, etc.)
- **Onboarding Screen 3**: "AI is building your store..." (Progress animation, background API calls to Marketing Agent)
- **Home Dashboard**: "Your store is live! Add your first item." (Card layout, large touch targets ≥ 44x44px).
- **Friction Reduction**: Use native mobile keyboards. Defer complex setups (taxes, custom domains) to a "Finish Setup" checklist that appears post-activation.

### Persona Sequence Diagrams (Mermaid)

#### Maya (The Home Baker)
```mermaid
sequenceDiagram
    actor Maya
    participant OHC Mobile
    participant Onboarding Agent
    participant Ops Agent
    participant Customer
    Maya->>OHC Mobile: Signs up (Phone number)
    OHC Mobile->>Onboarding Agent: Business Type: Bakery
    Onboarding Agent-->>OHC Mobile: Generates Bakery Storefront & Catalog Template
    Maya->>OHC Mobile: Uploads cake photo
    OHC Mobile->>Ops Agent: Create product (Custom Cake)
    Ops Agent-->>OHC Mobile: Product Live URL
    Maya->>Customer: Shares link on Instagram
    Customer->>OHC Mobile: Places custom order with deposit
    OHC Mobile->>Ops Agent: Process Deposit
    Ops Agent-->>Maya: Push Notification: "New Cake Order Received!"
```

#### Carlos (The Freelance Handyman)
```mermaid
sequenceDiagram
    actor Carlos
    participant OHC Mobile
    participant Onboarding Agent
    participant Sales Agent
    participant Customer
    Carlos->>OHC Mobile: Signs up (Android)
    OHC Mobile->>Onboarding Agent: Business Type: Handyman
    Onboarding Agent-->>OHC Mobile: Generates Service Booking Page
    Carlos->>OHC Mobile: Adds "Plumbing Fix" service + price
    OHC Mobile->>Sales Agent: Configure Booking Calendar
    Customer->>OHC Mobile: Selects time slot & pays deposit
    OHC Mobile->>Sales Agent: Sync Calendar & Generate Quote
    Sales Agent-->>Carlos: Notification: "New Booking for Plumbing Fix"
```

#### Priya (The Boutique Owner)
```mermaid
sequenceDiagram
    actor Priya
    participant OHC Mobile/Desktop
    participant Onboarding Agent
    participant Finance Agent
    participant Customer
    Priya->>OHC Mobile/Desktop: Signs up
    OHC Mobile/Desktop->>Onboarding Agent: Business Type: Retail
    Onboarding Agent-->>OHC Mobile/Desktop: Generates Storefront with Variants
    Priya->>OHC Mobile/Desktop: Adds dress (Red/Blue, S/M/L)
    Customer->>OHC Mobile/Desktop: Buys online
    OHC Mobile/Desktop->>Finance Agent: Process online payment
    Customer->>Priya: Buys in-store
    Priya->>OHC Mobile/Desktop: Tap-to-Pay POS
    OHC Mobile/Desktop->>Finance Agent: Process POS payment
    Finance Agent-->>Priya: Daily Analytics "Online vs In-Store Sales"
```

#### Leo (The Music Tutor)
```mermaid
sequenceDiagram
    actor Leo
    participant OHC Mobile
    participant Onboarding Agent
    participant Success Agent
    participant Student
    Leo->>OHC Mobile: Signs up
    OHC Mobile->>Onboarding Agent: Business Type: Tutoring
    Onboarding Agent-->>OHC Mobile: Generates Profile & Lesson Packages
    Leo->>OHC Mobile: Sets up monthly subscription package
    Student->>OHC Mobile: Books lesson & subscribes
    OHC Mobile->>Success Agent: Generate Zoom Link & Calendar Invite
    Success Agent-->>Student: Email: Lesson Details
    Success Agent-->>Leo: Notification: "New Student Subscribed!"
```

#### Fatima (The Food Cart Operator)
```mermaid
sequenceDiagram
    actor Fatima
    participant OHC Mobile
    participant Onboarding Agent
    participant Ops Agent
    participant Customer
    Fatima->>OHC Mobile: Signs up (Low-end Android, Arabic)
    OHC Mobile->>Onboarding Agent: Business Type: Food Cart
    Onboarding Agent-->>OHC Mobile: Generates Pre-order Menu UI (Arabic)
    Fatima->>OHC Mobile: Adds Falafel Plate (Photo, Price)
    Customer->>OHC Mobile: Pre-orders online
    OHC Mobile->>Ops Agent: Queue Order
    Ops Agent-->>Fatima: High-volume audio alert: "New Order!"
    Fatima->>OHC Mobile: Taps "Order Ready"
    Ops Agent-->>Customer: SMS: "Pickup your food"
```

## Implementation Prompt
Design and implement the unified Onboarding Engine and User Journey state machine for the OHC platform. The engine must track each tenant's progression through Acquisition, Onboarding, Activation, Retention, and Revenue phases. Implement the Flutter mobile UI for the step-by-step onboarding wizard (max 3 inputs before storefront generation) ensuring perfect 375px rendering. Integrate with the backend `Onboarding Agent` to auto-generate initial catalog items and storefront designs based on the user's business type. Create E2E tests validating the complete flow from sign-up to viewing a live storefront URL for at least two personas (e.g., Baker and Handyman). Do not require complex configuration (like Stripe setup or custom domains) during the initial onboarding flow; defer these to a post-activation checklist.

## Priority
P0

## Estimated Scope
Large
