# Title: Conversational AI Setup Wizard for Instant Storefront Generation

## Problem Statement
Small business owners (like Maya, a 28-year-old baker selling via Instagram DMs) are overwhelmed by the setup complexity of existing platforms like Shopify. They are forced to navigate technical jargon (DNS, liquid templates, shipping zones) before they can even see what their store might look like. Setup complexity is the #1 pain point for SMBs, causing 73% of 1-star reviews on major platforms to mention the setup process being confusing for beginners. They want a store, but they don't want to build one from scratch.

## Research Report
Based on our 2024-2025 Market Audit:
- **Competitor Landscape**:
  - Shopify takes 30m+ with high friction and no built-in AI help for setup.
  - Wix takes 20m+ with some AI generation, but requires heavy dashboard manipulation afterward.
  - Durable generates a site in <1m but lacks robust business management backend features.
- **Pain Points**: 73% of users report "Setup Complexity" as their biggest hurdle. Real user reviews show extreme frustration with dev-speak and the requirement to use desktop environments to configure basic store settings.
- **The Opportunity**: OHC can leapfrog competitors by replacing the traditional "form-based" onboarding with a simple, conversational text message thread. Users answer 3 simple questions (What do you sell? What is the vibe? Where are you located?), and the autonomous AI agents build a fully functional storefront, pre-populate product templates, and configure basic settings in under 1 minute, entirely from a mobile device.

## Design Doc
- **High-Level Flow**:
  1. User lands on OHC app/website (Mobile-first, 375px native).
  2. A conversational UI (chat interface) greets the user: "Hi! What kind of business are we building today?"
  3. User replies with unstructured text (e.g., "I sell homemade vegan cookies in Austin").
  4. The AI extracts Entity Types: `BusinessType`, `ProductType`, `Location`, `Vibe`.
  5. The AI agent triggers the `StoreGenerator` service, selecting the optimal premium OHC theme (Glassmorphism design, Outfit/Inter typography).
  6. The system presents the generated storefront live-preview inside the chat, allowing the user to say "Make it more colorful" or "Looks great, let's launch!"
- **Key Relationships**: The `ConversationalAgent` integrates directly with the `StoreProfile` and `ProductCatalog` entities, bypassing traditional forms.
- **Mobile UX**: The entire flow must take place within a chat-like interface. Touch targets must be ≥ 44x44px. The design must pass the 'Grandmother Test' (understandable without reading, plain-language labels).

## Implementation Prompt
**User-Facing Outcome:** A brand new user should be able to create a fully styled, functional e-commerce storefront just by chatting with an AI assistant for less than 60 seconds.

**Critical User Journey (CUJ):**
1. User starts the setup process.
2. User answers 3-4 natural language questions in a chat interface.
3. The system displays a generated store preview.
4. User approves the design.
5. User is dropped into the main OHC dashboard with their store already live and 3 sample products created based on their prompt.

**Acceptance Criteria:**
- The setup process uses a conversational UI component instead of traditional multi-step forms.
- The AI correctly parses the user's intent to set the store name, category, and visual vibe.
- The storefront preview loads within 15 seconds of the final prompt.
- The generated store strictly adheres to OHC Premium Design Standards (Glassmorphism, correct typography, appropriate animations).
- The feature is fully functional and responsive on mobile viewports (375px width).
- Includes at least 5 Playwright E2E tests validating the CUJ from the home page.

## Priority
P0

## Estimated Scope
Large