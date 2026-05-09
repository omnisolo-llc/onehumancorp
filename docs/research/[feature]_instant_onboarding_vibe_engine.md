# Feature Brief: Instant Onboarding Vibe Engine

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) face immense friction when starting an online store. They abandon platforms like Shopify because they are overwhelmed by technical jargon, domain setup, and blank-page anxiety. Setup complexity is the #1 pain point (73% of complaints). We need to get a user from "idea" to "live storefront" in under 60 seconds.

## Research Report
Our audit shows Durable creates sites in 30 seconds, but lacks deep business operations. Shopify takes hours or days. We must combine Durable's speed with OHC's operational depth. The solution is an AI-driven setup wizard that relies on natural language and "vibes" rather than technical settings.

## Design Doc
**Architecture & Integration:**
- **Entity Types:** `BusinessProfile`, `VibeTheme`
- **Integration Points:** LLM Integration (for generating initial copy and design tokens), Image Generation API.

**UX/UI Flow (Mobile-First 375px):**
1.  **Welcome Screen:** Natural language input ("What do you do?").
2.  **Vibe Selection:** User selects 1 of 3 generated aesthetic directions (e.g., "Warm & Cozy", "Sleek & Modern").
3.  **Generation:** Loading screen while AI generates the initial storefront structure, placeholder products, and copy.
4.  **Instant Live:** The store is immediately live. No complex DNS or payment gateway setup required to view it.

## Implementation Prompt
Implement the "Instant Onboarding Vibe Engine" Critical User Journey. The user should be able to type a brief description of their business and immediately receive a generated, aesthetically coherent storefront preview. The setup should avoid any technical configuration steps. Acceptance criteria: A user can complete the flow in under 3 steps; the generated output must adhere to the OHC Visual Excellence Mandate (Glassmorphism, correct typography). Do not prescribe the underlying data models or API routes; design them as needed to fulfill this CUJ.

## Priority
P0

## Estimated Scope
Large
