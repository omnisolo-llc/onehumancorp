# Issue Brief: Invisible AI Storefront Generator

## Problem Statement
Non-technical users (e.g., Maya the Baker) experience "Blank Page Paralysis" when trying to set up an online store. Competitor platforms like Shopify force users through tedious manual configuration (tax zones, shipping, template selection, blank product descriptions). This leads to high drop-off rates during onboarding.

## Research Report
- 73% of 1-star Shopify reviews cite the complexity of initial setup.
- Wix ADI provides a wizard but still requires heavy manual editing post-generation.
- Tools like Durable prove users want instant generation, but they lack the backend management features SMBs need.
- **Opportunity:** OHC can provide a `< 10 minute` onboarding by utilizing an AI Marketing Agent to completely bypass manual setup. The user describes their business in one sentence, and the AI handles the rest.

## Design Doc
### High-Level Architecture
- **Trigger:** Initial user registration.
- **Agent Integration:** Trigger the "Marketing & Advertising" AI Department.
- **Process:** The AI takes a single user prompt (e.g., "I bake vegan cakes in Austin. Prices start at $50.") and translates it into:
  - Base store metadata (name, tagline).
  - An initial product catalog with AI-generated descriptions and placeholder images.
  - Basic configuration (currency, default shipping/pickup logic based on business type).
- **UI Integration:** A magic loading screen during generation, followed by the generated storefront. The user is presented with a 1-tap "Looks Good, Launch" button, or an option to tweak the prompt.

### Mobile UX Flow (375px First)
- **Screen 1 (Input):** A clean, simple chat-like interface: "Tell me about your business in a few words."
- **Screen 2 (Generation):** Shimmering glassmorphic loading states explaining what the AI is building ("Writing product descriptions...", "Setting up your booking calendar...").
- **Screen 3 (Review):** A fully functional preview of the mobile site.

## Implementation Prompt
Implement the "Invisible AI Storefront Generator" flow. Create the Flutter mobile UI screens (Input, Generation Loading, Review) ensuring perfect rendering at 375px. Connect the Input screen to a backend endpoint that delegates to the "Marketing & Advertising" AI agent to generate the store schema (products, descriptions, basic settings) based on the user's prompt. The feature must remove all complex configuration steps from the critical path of launching the initial store.

## Priority
P0

## Estimated Scope
Large
