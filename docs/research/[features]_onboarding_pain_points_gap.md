# Issue Brief: Zero-Knowledge Onboarding Flow Refactor

## Problem Statement
Small business owners (e.g., Maya the Baker, Carlos the Handyman) find e-commerce setup overwhelming because platforms assume prior technical knowledge. Our research identifies "Initial Setup Confusion" as the #1 pain point across Shopify and other major platforms. Current setup flows are reactive and expect the user to manually configure settings rather than having an AI guide the process.

## Research Report
- **Competitor Flaw:** Shopify takes 30-60 minutes and requires knowledge of themes, payment gateways, and DNS settings.
- **Competitor Flaw:** Wix ADI attempts to simplify this, but leaves the user with a static site that requires manual post-launch configuration.
- **OHC Opportunity:** Leverage our "mobile-first fast setup" advantage to create an AI-driven, conversation-based onboarding sequence. The AI should act as an interviewer, asking simple business questions ("What do you sell?", "How do you want to get paid?") and autonomously generating the store configuration in the background.

## Design Doc
### High-Level Architecture
- **Entity Update:** Introduce an `OnboardingSession` entity to track the conversational state and intermediate business configuration payload.
- **Integration Point:** Connect the new conversational flow to the existing AI Orchestrator (`src/server/services/onboarding/onboarding_agent.rs`).
- **Mobile UX Flow (375px first):**
  - Screen 1: Welcome message ("Let's get your business online in 3 minutes. What's the name of your business?").
  - Screen 2: Conversational interface where the AI asks 3-4 tailored questions based on the business type (e.g., "Do you need a booking calendar for appointments?").
  - Screen 3: "Generating your business..." loader with a glassmorphism effect.
  - Screen 4: Store dashboard ready to accept orders.

## Implementation Prompt
Refactor the initial setup wizard to be entirely conversational, driven by the AI agent. The UI must be optimized for 375px mobile screens and support native mobile keyboard inputs seamlessly. Ensure the AI correctly maps the user's plain-text answers to the required OHC database configurations (e.g., enabling the booking module if they mention appointments). All state changes must be saved asynchronously with optimistic UI updates.

## Priority
P0

## Estimated Scope
Medium
