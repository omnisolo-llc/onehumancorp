# [architecture]_business_journey.md

## Title
Business Journey Architecture

## Problem Statement
The OHC platform needs to clearly define the end-to-end user journeys for non-technical small business owners (our personas: Maya, Carlos, Priya, Leo, and Fatima). Currently, there's a risk of friction at acquisition, onboarding, activation, retention, revenue upgrade, and referral points, which could lead to a drop-off. We need to meticulously map out how these non-technical users interact with the system—from discovering the platform to running a successful, AI-assisted business—so that engineers and product teams can build a seamless, "zero to live in 10 minutes" experience.

## Research Report
- **Goal:** Map the complete end-to-end user journey across all key phases (Acquisition, Onboarding, Activation, Retention, Revenue, Referral) for all target personas.
- **Competitor Analysis:**
  - *Shopify:* Has a complex onboarding flow that assumes some e-commerce knowledge. Upgrades are feature-gated but often unclear until hit.
  - *Wix/Squarespace:* Template-heavy onboarding that requires manual design work, leading to high drop-off before the first sale.
  - *GoDaddy:* Fast setup but very basic; lacks the AI depth OHC promises.
- **OHC Approach:** The onboarding must be zero-jargon and AI-first. A user answers 3-5 simple questions, and the AI agent provisions the storefront, calendar, and products in the background. The mobile-first design ensures 100% of these tasks can be done on a 375px screen.

## Design Doc

### 1. Persona Journeys

#### Maya (The Home Baker)
- **Acquisition:** Sees an Instagram ad showcasing a baker setting up a shop on their phone in 3 minutes. Clicks CTA: "Start your free bakery shop."
- **Onboarding:** Opens OHC mobile web. Answers 3 questions: "What do you sell?" (Custom Cakes), "How do you want to get paid?" (Stripe/Bank), "Connect Instagram?". AI generates a beautiful glassmorphic storefront.
- **Activation:** Adds her first custom cake product with a required deposit. Shares her OHC link in her Instagram bio. First order received.
- **Retention:** Receives daily push notifications on new orders and weekly advisory reports suggesting she add a vegan option based on AI DM analysis.
- **Revenue:** Hits the 100-order limit on the Free tier. Prompted to upgrade to Starter ($9/mo) for custom domain and more AI actions.
- **Referral:** "Share OHC with another baker and get 1 month of Pro free."

#### Carlos (The Freelance Handyman)
- **Acquisition:** Word of mouth from another contractor. Searches "simple booking app for contractors".
- **Onboarding:** App asks for his services (Plumbing, General Repairs) and availability. AI generates service listing and booking calendar.
- **Activation:** Client books a Tuesday slot and pays a deposit.
- **Retention:** Checks his OHC app every morning to view his daily schedule (Operations Agent) and messages (Customer Success Agent).
- **Revenue:** Upgrades to Pro ($29/mo) to unlock unlimited AI follow-ups for leads who abandoned the booking flow.
- **Referral:** Shows his app to another contractor at the hardware store.

#### Priya (The Boutique Owner)
- **Acquisition:** Organic search for "sync in-store and online inventory easily".
- **Onboarding:** Connects existing simple POS or sets up OHC POS. AI imports her current inventory list.
- **Activation:** Completes her first tap-to-pay transaction on her iPhone using Stripe Terminal.
- **Retention:** Checks daily analytics on her phone. Gets an AI alert that red dresses are selling out fast.
- **Revenue:** Upgrades to Business ($79/mo) for unlimited AI departments to handle her growing email list and multiple domains.
- **Referral:** Mentions OHC in a local business owner WhatsApp group.

#### Leo (The Music Tutor)
- **Acquisition:** TikTok link-in-bio of another creator.
- **Onboarding:** Selects "Tutoring". Sets up subscription packages. Connects Google Calendar.
- **Activation:** First student buys a 4-lesson monthly package. Zoom link is auto-generated.
- **Retention:** AI agent follows up with students who missed a lesson. Leo reviews these drafts with 1-tap approval.
- **Revenue:** Upgrades to Starter for a custom domain (\`leoguitar.com\`).
- **Referral:** Shares a promotional link with a fellow musician.

#### Fatima (The Food Cart Operator)
- **Acquisition:** Local community outreach program for street vendors.
- **Onboarding:** Uses the Arabic UI on her Android phone. Takes photos of her food. AI auto-removes backgrounds and creates a menu.
- **Activation:** First customer scans the QR code at her cart, orders a pre-order meal, and Fatima's phone rings with a notification.
- **Retention:** Uses the printable daily order list (Operations Agent).
- **Revenue:** Remains on the Free tier as it supports her needs perfectly.
- **Referral:** Another cart owner scans her QR code and sees a "Powered by OHC" badge.

### 2. Sequence Diagrams (Mermaid.js)

\`\`\`mermaid
sequenceDiagram
    participant User as Maya (Baker)
    participant App as OHC Mobile App
    participant AI as Marketing Agent
    participant DB as OHC Platform

    User->>App: Tap "Start your shop"
    App->>User: "What do you sell?"
    User->>App: "Custom Cakes"
    App->>User: "Connect Instagram?"
    User->>App: [Connects IG]
    App->>AI: Trigger: Generate Storefront Context
    AI->>DB: Provision tenant, generate theme, add mock products
    DB-->>App: Storefront Ready
    App->>User: "Your shop is live. Add your first real cake!"
    User->>App: Uploads photo & sets price
    App->>DB: Save product
    DB-->>App: Product URL generated
    App->>User: "Share this link in your bio!"
\`\`\`

### 3. UI Wireframes & Mobile UX Flow
- **Breakpoint:** 375px native app/PWA.
- **Flow:**
  1. **Landing Screen:** "Welcome to OHC. Let's build your business." -> Big primary button.
  2. **Conversational Setup:** 3-step conversational form using the native keyboard. Large text, clear inputs.
  3. **Magic Moment:** A loading skeleton with glassmorphic shimmer while the AI agent "builds" the business.
  4. **Dashboard:** "You're live!" A prominent "Share Link" button, a "Next Action" suggestion from the Advisory Agent (e.g., "Add a product"), and an empty state for Orders/Bookings.

## Implementation Prompt
"Implement the onboarding UI flow in the Flutter application for a new user. The flow must be entirely mobile-first (375px), conversational, and consist of no more than 4 screens before reaching the generated dashboard. Use the OHC Premium Token library for glassmorphism effects. Upon completion, make an API call to the backend to trigger the Marketing Agent to generate the storefront. Ensure comprehensive E2E tests using Playwright that verify the full journey from the welcome screen to the live dashboard, mocking the backend agent response."

## Priority
P0

## Estimated Scope
Large
