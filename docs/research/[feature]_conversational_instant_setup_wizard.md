# [Feature] Conversational Instant Setup Wizard

## Title
Conversational Instant Setup Wizard

## Problem Statement
The #1 pain point for non-technical small business owners is "Setup Complexity" (73% frequency). Users feel alienated and overwhelmed by technical jargon like DNS, CNAME, liquid templates, and complex shipping zones. They abandon platforms like Shopify because the initial hurdle to get a store live is too high.

## Research Report
*   **Competitor Landscape:**
    *   *Shopify:* High friction onboarding (30m+). Requires navigating complex menus to configure basics.
    *   *Wix:* Wix ADI helps, but still requires significant post-generation tweaking in a complex editor.
    *   *Durable:* Fast generation (30s) but very shallow business logic underneath.
*   **User Pain Points:** "Why do I need to know what a CNAME record is just to sell a t-shirt?"
*   **OHC Differentiation:** "Radical Simplicity." OHC must provide a zero-jargon, vibe-based setup experience that gets a fully operational business online in under 1 minute.

## Design Doc
*   **High-Level Architecture:**
    *   A stateful chat interface guides the user through 3-4 simple questions (What do you sell? What's your vibe? Where are you located?).
    *   The backend `OnboardingAgent` receives these inputs, generates the initial catalog schema, creates basic store policies via LLM, and provisions the default UI theme.
    *   The setup process is entirely synchronous or uses fast websockets to update the UI instantly as the agent "builds" the store in real-time.
*   **UI/UX Flow (Mobile-First 375px):**
    *   Full-screen chat interface. Large, friendly typography (Outfit font).
    *   User inputs answers.
    *   A loading state with engaging micro-animations (< 300ms transitions) shows the agent "working" ("Generating your catalog...", "Setting up your payment links...").
    *   Final screen: "You're live. Here is your link."

## Implementation Prompt
Build a mobile-first, conversational onboarding flow that completely hides technical configuration from the user. The Critical User Journey (CUJ) starts with the user opening the app for the first time, answering a few plain-language questions via a chat-like interface, and ends with a fully generated, live storefront link. The underlying system must automatically configure catalog defaults, basic policies, and a vibe-aligned theme. Ensure the UX adheres to OHC premium design standards and works perfectly on a 375px viewport. Do not prescribe specific database schemas or API contracts.

## Priority
P0

## Estimated Scope
Medium
