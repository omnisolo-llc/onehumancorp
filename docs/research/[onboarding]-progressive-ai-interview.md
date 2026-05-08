# Issue Brief: Progressive Onboarding via AI Interview

## Title
Progressive Onboarding via AI Interview

## Problem Statement
New small business owners, like Fatima (a food cart owner with limited English) or Carlos (a handyman), are immediately alienated by traditional SaaS onboarding flows. They are presented with massive dashboards, complex configuration menus (shipping zones, tax rates, DNS settings), and blank canvases. The "blank page syndrome" leads to high drop-off rates before the store is even live. They don't know *how* to set up a store; they just know what they sell.

## Research Report
- **Competitor Analysis:**
  - *Shopify:* Standard form-based onboarding. Requires navigating complex menus immediately. Highly intimidating for beginners.
  - *Wix:* ADI (Artificial Design Intelligence) asks a few questions and generates a site, but the post-generation experience dumps the user into a complex editor.
  - *GoDaddy Airo:* Attempts a quick AI setup but results are often generic and the subsequent management dashboard is confusing.
  - *Durable:* Fast website generation, but very thin on actual business management setup (payments, inventory).
- **User Pain Points:**
  - "I signed up for Shopify and immediately felt like I needed a degree in web development just to set up my shipping." (App Store review, 1-star)
  - "I just want to sell my cakes online. Why is it asking me about DNS records?" (Twitter/X complaint)
- **Data:** Industry averages suggest up to 60-70% of users who start an e-commerce free trial never launch their store.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `OnboardingSession`, `Merchant`, `StoreConfiguration`, `AIAgentProfile`.
  - **Key Relationships:** An `OnboardingSession` drives the initial creation of a `Merchant` and their `StoreConfiguration`.
  - **Integration Points:** LLM for conversational intent parsing, OHC's internal configuration API.
- **UI Wireframes/Screen Flow:**
  - *Mobile UX Flow (375px first):*
    1.  **Welcome Screen:** "Let's build your business. What do you do?" (Voice or Text input).
    2.  **Conversational Setup:** A chat-like interface. The AI agent asks 3-4 highly relevant questions based on the first answer (e.g., if "handyman", it asks about service area and hourly rate; if "baker", it asks about pickup vs. delivery).
    3.  **Invisible Configuration:** As the user chats, the AI invisibly configures tax settings, shipping zones, and generates initial product listings.
    4.  **The Reveal:** "Your store is ready. Here's what it looks like." A fully functional preview with pre-filled mock data based on the interview.
- **AI Agent Integration Points:**
  - A specialized "Onboarding Wizard" LLM agent that maps conversational input into structured JSON configuration payloads for the OHC backend.

## Implementation Prompt
**User-Facing Outcome:** A first-time user can launch a fully functional, personalized online store in under 3 minutes simply by having a natural language conversation (or answering a few dynamic questions) with an AI agent. There are no technical forms to fill out during the initial setup.

**Critical User Journey (CUJ):**
1. User downloads the OHC app and taps "Start a Business".
2. User types or speaks: "I run a food cart in Portland selling vegan tacos."
3. The AI agent asks 3 contextual follow-up questions (e.g., "Do you want people to pre-order for pickup?").
4. The AI agent generates a complete store setup, including a sample menu and pickup scheduling configuration.
5. User taps "Launch Store" and is immediately ready to accept orders.

**Acceptance Criteria:**
- The onboarding flow must adhere to the 30-second rule (usable by a first-time smartphone user without instructions).
- The system must use the Progressive Disclosure pattern (all technical settings like DNS, tax rates are handled invisibly or relegated to an 'Advanced mode' toggle).
- The flow must be entirely mobile-responsive (375px first viewport).

## Priority
P0

## Estimated Scope
Large
