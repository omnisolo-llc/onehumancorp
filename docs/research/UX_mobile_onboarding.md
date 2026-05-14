# [UX] 3-Tap Mobile Onboarding Flow

## Problem Statement
Current platform setups (Shopify, Wix) assume desktop usage and require 10+ steps (tax details, shipping zones, theme selection) before a user can sell a single item. This causes massive drop-off for mobile-first solopreneurs like Fatima (food cart).

## Research Report
*   **User Pain Point:** "I just want a link to put in my Instagram bio to take orders." (Common sentiment on Twitter).
*   **Competitor Gap:** GoDaddy is the fastest but still requires manual design choices.
*   **OHC Advantage:** Agent-driven configuration.

## Design Doc
*   **Action:** The onboarding agent makes all initial decisions based on 2 inputs.
*   **UI Flow (Mobile First - 375px):**
    1.  Screen 1: "What's the name of your business?" (Text input)
    2.  Screen 2: "What do you sell?" (Select: Products, Services, Food)
    3.  Screen 3: "Generating your store..." (Agent provisions DB, sets default theme, creates dummy product based on type).
    4.  Result: Live store URL provided immediately.

## Implementation Prompt
Refactor the `StartOnboardingRequest` flow to require only `company_name` and `business_type`. The `OnboardingAgent` must automatically handle all other previously required fields (like admin user creation with a default secure auth flow, default product generation, and theme selection) without blocking the user.

## Priority
P0

## Estimated Scope
Large
