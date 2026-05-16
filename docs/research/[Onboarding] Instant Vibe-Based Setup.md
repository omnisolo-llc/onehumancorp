# [Onboarding] Instant Vibe-Based Setup: The 30-Second Launch

## Title
[Onboarding] Instant Vibe-Based Setup: Radical Simplicity for Launch

## Problem Statement
The "Blank Canvas" is the biggest barrier to entry for non-technical founders (e.g., Carlos the Handyman). Competitors like Shopify and Squarespace require users to choose templates, edit DNS, and learn liquid/CSS. Even "AI Builders" like Wix require a 20-minute conversational setup. OHC needs to match the 30-second benchmark of Durable while providing a deeper business foundation.

## Research Report
*   **Competitor Status**:
    *   **Durable.co**: The current gold standard for speed. Generates a site in 30 seconds from 3 questions.
    *   **Shopify**: 30-60 minute onboarding. High friction.
    *   **Wix ADI**: conversational but slow (~15 mins).
*   **User Pain Point**: 73% of SMBs cite "Setup Complexity" as their #1 fear. "DNS" and "CNAME" are the most hated words.
*   **Opportunity**: Match Durable's speed for the *website* while using OHC's "Agentic OS" to simultaneously provision the *business* (CRM, Booking, Inbox).

## Design Doc
*   **Architecture**:
    *   **Input**: "Vibe" (e.g., "Modern Artisan"), Industry ("Bakery"), and Name ("Maya's Bakes").
    *   **Agent**: The Architect Agent (Onboarding).
    *   **Action**:
        1. Instant generation of a mobile-first storefront.
        2. Auto-provisioning of a "Teammate" workforce (Promoter, Manager, Ambassador).
        3. Local-first deployment to SQLite (SIPDB) with background cloud sync.
*   **Mobile UX Flow (375px)**:
    1.  Splash screen: "What's your business name?"
    2.  Screen 2: "What's your vibe?" (Visual selection of 4 tiles: Minimalist, Bold, Cozy, Tech).
    3.  Screen 3: Shimmering AI generation animation (the "30-second rule").
    4.  Result: "Maya's Bakes is Live. Your agents are ready."
*   **AI Integration**:
    *   Generative Design for instant theme application.
    *   LLM to generate initial product catalog and "About Me" based on vibe.

## Implementation Prompt
Redesign the OHC onboarding flow to prioritize "Radical Simplicity." Create a 3-step conversational wizard that asks for Business Name, Industry, and Vibe. Upon completion, the system must use the "Architect Agent" to generate a fully functional storefront and provision the default agent workforce in under 30 seconds. Acceptance criteria: user can reach a live, functional dashboard from the splash screen in < 60 seconds; the generated site reflects the chosen "Vibe."

## Priority
P0

## Estimated Scope
Large
