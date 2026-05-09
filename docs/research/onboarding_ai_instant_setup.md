# Instant AI Storefront Setup

## Problem Statement
Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by standard platform setups. They don't know what pages they need, what their color scheme should be, or how to write compelling product descriptions. They want to answer a few questions and have a professional, functional business presence live instantly. Current platforms like Shopify require hours of manual configuration, design selection, and content creation before making a first sale.

## Research Report
**Findings:**
*   73% of 1-star Shopify reviews from beginners mention the initial setup being too confusing or time-consuming.
*   Users frequently abandon setup during the theme customization phase.
*   "I just want it to look good without me having to learn web design" is a common sentiment in r/smallbusiness.
*   **Competitor Comparison:**
    *   **Shopify:** Complex. High friction. High drop-off.
    *   **Wix ADI:** Generates a site, but post-generation editing is manual and confusing.
    *   **Durable:** Very fast AI generation, but lacks deep business management tools post-launch.
*   **Opportunity:** OHC can provide the speed of Durable with the power of a real business platform by using agents to not just generate the site, but continuously manage it.

## Design Doc
**Architecture / Entities:**
*   `BusinessProfile`: Core entity storing the user's plain-text answers to onboarding questions (e.g., "I'm a baker in Austin selling sourdough").
*   `BrandKit`: AI-generated colors, typography, and logo concepts.
*   `StorefrontTemplate`: A dynamic, agent-assembled layout.

**Mobile UX Flow (375px first):**
1.  **Welcome Screen:** "What do you do?" (Text input or voice memo).
2.  **Processing Screen:** "Our agents are building your business..." (Visual feedback showing logo creation, copywriting, and layout assembly).
3.  **Reveal Screen:** The fully functional storefront is presented. "Here is your new business."
4.  **Quick Edits:** Simple toggle options to change the "vibe" (e.g., Professional, Playful, Minimalist) which instantly regenerates the brand kit.
5.  **Go Live:** One tap to publish.

**AI Agent Integration Points:**
*   **Copywriting Agent:** Takes the raw business description and generates hero text, about us, and initial service/product descriptions.
*   **Design Agent:** Selects a coherent color palette and typography based on the industry and description.
*   **Layout Agent:** Assembles the Slint UI components into a cohesive template.

## Implementation Prompt
Build the "Instant Setup" onboarding flow. The user should be able to input a single paragraph describing their business, and the system should automatically generate a complete, published storefront. The generated storefront must include a branded hero section, an "About Us" section derived from their input, and placeholder product/service listings relevant to their industry.

**Critical User Journey:**
1. User downloads OHC app.
2. User types "I offer mobile dog grooming in Chicago. I focus on anxious dogs."
3. App displays a loading state for < 10 seconds.
4. User sees a live, professional storefront with a custom color palette, generated copy focusing on "stress-free grooming," and sample booking slots.

**Acceptance Criteria:**
*   Onboarding requires less than 3 inputs from the user.
*   The generated store is immediately functional (e.g., a dummy product can be "purchased" or a dummy booking can be made).
*   All AI generation happens transparently without requiring the user to approve intermediate steps (colors, fonts, etc.) unless they choose to edit them later.

## Priority
P0

## Estimated Scope
Large