# Zero-Jargon Conversational Onboarding Wizard

## Problem Statement
The setup process on legacy platforms takes too long and introduces technical questions right away (e.g., DNS setup, payment gateways, complex templates). This "Setup Complexity" is the highest-ranked pain point for new users, leading to high drop-off rates.

## Research Report
*   **Setup Complexity (73% frequency):** The #1 reason users abandon platform onboarding. Users feel "stupid" when confronted with dev-speak.
*   **Speed to Live:** AI tools like Durable generate sites in <30 seconds, setting a new benchmark. Shopify takes 30+ minutes for basic setup.
*   **Competitor Gap:** Wix ADI attempts this but still results in a complex dashboard. OHC needs a hyper-fast, conversational flow that results in an instantly ready storefront and operations backend.

## Design Doc
*   **Architecture:** A progressive conversational flow that updates the `Tenant` and `Storefront` entities in the database without exposing any technical fields to the user.
*   **UI Flow:** A chat-like, mobile-first interface asking 3 simple questions: "What's the name of your business?", "What do you sell?", and "Describe your vibe." The system uses "Simple mode" by default.
*   **AI Integration:** LLM interprets the "vibe" and business type to auto-generate the initial design tokens, copy, and product categories.

## Implementation Prompt
Implement the Slint conversational onboarding flow. It must feel instantaneous, be fully usable on a 375px screen, and strictly avoid exposing any database or technical concepts to the user. Default to 'Simple mode' (plain language) and use the Outfit font for headings. Integrate with the backend to progressively populate the tenant setup.

## Priority
P0

## Estimated Scope
Large
