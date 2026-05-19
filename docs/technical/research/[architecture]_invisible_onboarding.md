# OHC Market Dominance: Small Business Platform Research Report

## Problem Statement
Setting up a functioning online store or booking system is the biggest barrier for non-technical small business owners like Maya (baker) or Carlos (handyman). They are overwhelmed by dashboards, settings, and configuration screens. They want to sell their services, not become web developers.

## Research Report
Competitor audits show that while platforms like Wix offer ADI (Artificial Design Intelligence), it is limited to visual layout. Shopify requires significant manual configuration. Real user reviews on App Stores and Reddit frequently cite "too confusing" and "abandoned during setup" as primary reasons for churn.

## Design Doc
*   **UI/UX (Mobile First):** The onboarding flow should look more like a conversational text thread than a traditional web form. (Glassmorphism design, large 44x44px touch targets).
*   **Flow:**
    1.  Splash screen: "What kind of business are you building today?"
    2.  Chat interface: User types or uses voice-to-text.
    3.  Processing state (subtle motion): "Agents are building your store..."
    4.  Reveal: A fully functional, pre-populated storefront.
*   **Architecture (High-Level):**
    *   A new `OnboardingAgent` service orchestrates the process.
    *   It interfaces with existing `Product`, `Store`, and `Tenant` entities.
    *   It utilizes LLM capabilities to parse the user's natural language input into structured JSON payload used to hydrate the database via the API.

## Implementation Prompt
Build the "Invisible Onboarding" conversational UI and connect it to a backend orchestration agent.
*   **Critical User Journey (CUJ):** A new user opens the app, types "I'm a dog walker in Seattle named Sarah," and within 3 screens is presented with a live, publishable booking page with placeholder services (e.g., "30 Min Walk", "1 Hour Walk").
*   **Acceptance Criteria:**
    *   The flow must be completed in under 3 minutes of active user time.
    *   The output must be a functional store/booking page requiring zero manual database configuration from the user.
    *   The UI must adhere to the Progressive Disclosure pattern (Simple mode by default).

## Priority
P0

## Estimated Scope
Large
