# Invisible AI Store Setup

**Title:** Invisible AI Store Setup (The "Grandmother Test" Onboarding)

**Problem Statement:**
Small business owners (like Maya the baker) are overwhelmed by standard platform onboarding. Terms like "DNS", "Theme customization", and "Payment Gateways" cause massive drop-off. Current competitors offer "AI" that simply dumps a generic template, still requiring the user to navigate a complex editor. Setup needs to be a conversation, not a configuration screen.

**Research Report:**
* **73% of 1-star platform reviews** cite setup complexity as the primary reason for churn.
* Competitor analysis reveals that "Time to Live Store" averages 2-4 hours on Wix and 1-3 days on Shopify for non-technical users.
* Users vastly prefer chat interfaces (like WhatsApp) over dashboard interfaces for initial data entry.

**Design Doc:**
* **UX Flow (Mobile First - 375px):**
  1. User opens the app. No dashboard is visible.
  2. A conversational UI (using Glassmorphism, 20px blur) greets them: *"Hi, I'm your OHC Agent. What are we building today?"*
  3. User types or uses voice: *"I sell vegan cupcakes in Austin."*
  4. Agent asks 3-4 clarifying questions sequentially (e.g., *"Do you want to offer local delivery or pickup?"*).
  5. The screen transitions to a loading state with cubic-bezier easing.
  6. The Agent presents a fully functional, branded storefront preview with placeholder products generated based on their answers.
  7. A single primary action button (Touch target >= 44x44px): *"Looks good, let's launch."*
* **Architecture Impact:**
  * Requires a new conversational onboarding Slint component.
  * Needs an AI state machine to guide the onboarding conversation and gather required structured data (business name, niche, fulfillment method) before triggering the store generation.

**Implementation Prompt:**
Implement a mobile-first, chat-based onboarding flow that gathers necessary business information from the user via a conversational interface. The system must autonomously generate the initial store configuration, theme, and basic structure based on the extracted intent, completely bypassing traditional manual form-filling and dashboard configuration. The outcome must pass the "Grandmother Test"—zero technical jargon. Ensure 100% test coverage for the new UI flow.

**Priority:** P0

**Estimated Scope:** Large
