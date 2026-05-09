# Instant Storefront Generation (Onboarding)

## Title
Instant Storefront Generation

## Problem Statement
Small business owners drop off during onboarding because it takes over 30 minutes to set up a basic storefront. The friction is high, and the technical jargon is intimidating.

## Research Report
*   **Competitor Analysis:** Shopify takes 30m+, Wix takes 20m+. Durable offers a 30s site but lacks business tools.
*   **User Evidence:** Setup complexity is the #1 pain point (73%).
*   **OHC Differentiation:** "Instant Build" - answering 3 simple questions to generate a fully functional, vibe-based storefront in under 1 minute.

## Design Doc
*   **Architecture:** Generative AI pipeline takes user input and produces a complete storefront schema (theme, initial products, copy).
*   **Mobile UX Flow (375px focus):** Conversational wizard -> "Generating..." animation -> Live storefront.
*   **AI Integration Points:** LLM for content generation, image generation model for initial assets.

## Implementation Prompt
Implement a generative onboarding flow that reduces the time to a live store to under 1 minute. The flow should ask no more than 3 simple, jargon-free questions.

## Priority
P1

## Estimated Scope
Large
