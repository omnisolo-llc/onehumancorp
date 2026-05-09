# [core] One-Tap Mobile Store Setup

## Problem Statement
For a non-technical small business owner (e.g., Maya, 28, Baker), traditional platforms like Shopify or Wix are overwhelming. They demand an understanding of "DNS", "liquid templates", "collections", and "shipping zones." Setting up a store often takes over an hour of frustrating desktop work. 73% of SMBs report "Setup Complexity" as their top pain point, leading to high abandonment rates before the store even goes live.

## Research Report
*   **Competitor Baseline:** Shopify takes 30-60 minutes and is desktop-heavy. GoDaddy Airo is faster but produces shallow, generic results.
*   **User Data:** Reddit (r/shopify) is filled with complaints like: "Why do I need to know what a CNAME record is just to sell a t-shirt?"
*   **Opportunity:** OHC can reduce setup time to under 10 minutes by replacing manual configuration with an invisible AI "SetupWizard" that operates natively on mobile.

## Design Doc
*   **Core Entities:** `BusinessProfile`, `StorefrontTheme`, `ProductCatalog`.
*   **Mobile UX Flow (375px First):**
    1.  **Welcome Screen:** "Tell us what you sell" (Conversational input).
    2.  **AI Thinking State:** Premium glassmorphism spinner ("The Setup Agent is building your store...").
    3.  **Reveal Screen:** Fully generated mobile storefront preview.
    4.  **Refine Flow:** "Change the vibe" (select predefined visual themes like 'Minimal', 'Playful', 'Elegant') without touching code.
    5.  **Go Live:** 1-tap to publish.
*   **AI Integration Point:** A conversational agent that parses the user's initial description to generate copy, select a design template, and structure initial product categories automatically.

## Implementation Prompt
Implement a "10-Minute Setup Wizard" for the OHC mobile app. The critical user journey starts when a new user opens the app for the first time. They should interact with a conversational UI to describe their business, and the app must generate a complete, functional mobile storefront preview. The user must be able to switch visual themes with a single tap and publish the store without encountering any technical jargon (no mention of DNS, domains, or layout engines). Acceptance criteria include the storefront being generated entirely via the AI integration point based on a single text prompt from the user, and the entire flow must be fully functional on a 375px viewport.

## Priority
P0

## Estimated Scope
Large
