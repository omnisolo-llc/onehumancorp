# [Product] 10-Minute AI Setup Wizard

## Problem Statement
The biggest drop-off point for new SMB platforms is the initial setup. Users face a "Blank Canvas Paralysis" where they must choose themes, upload photos, and write copy before seeing value.

## Research Report
- **Competitor Landscape:** Wix ADI attempts this but requires 15+ questions and yields generic results. Durable does it in 30 seconds but produces a shell site with no backend operational tools.
- **Pain Point Data:** Setup friction is the #1 reason for trial abandonment across all major platforms.
- **Opportunity:** OHC can use its Marketing Agent to generate not just a visual storefront, but a fully configured business backend (products, pricing, policies) from a single conversational prompt.

## Design Doc
- **Core Entity:** `BusinessProfile` (Name, Industry, Vibe, Target Audience).
- **UI Flow (Mobile-First 375px):**
  1.  **Conversational Entry:** A simple chat-like interface. "Hi, I'm your OHC Assistant. What kind of business are we building today?"
  2.  **Generation State:** A premium loading screen (glassmorphism effects) showing the AI "thinking" ("Designing storefront...", "Writing product descriptions...", "Setting up payment links...").
  3.  **The Reveal:** The user is dropped directly into a fully populated, beautiful preview of their business.
  4.  **Refinement:** Simple toggles to change the "vibe" (e.g., "More playful", "More professional") which regenerates the site instantly.
- **AI Integration:** Uses Gemini Pro to take the initial prompt and output a structured JSON configuration that provisions the tenant's initial state (website layout, 3 placeholder products, AI agent system prompts).

## Implementation Prompt
Create the UI flow for the "10-Minute AI Setup Wizard" in Slint. It should consist of an initial conversational input screen, a dynamic loading state with premium visual feedback (simulating the agents working), and a final "Reveal" screen showing a mocked generated storefront. The entire flow must be designed for a 375px mobile screen.

## Priority
P0

## Estimated Scope
Large
