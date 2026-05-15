# [feature] Zero-Click Mobile Onboarding Agent

## Title
Zero-Click Mobile Onboarding Agent

## Problem Statement
Non-technical founders (like Maya, a baker, or Carlos, a handyman) find Shopify and Wix overwhelmingly complex. They abandon setup when faced with DNS settings, theme customization, and payment gateway configurations. They need to launch their business in minutes from their phone, not days from a laptop.

## Research Report
- **Data:** 73% of 1-star Shopify App Store reviews mention the setup being too complex for beginners.
- **Competitors:** Durable generates websites quickly, but lacks deep business management logic. Wix ADI is a one-time setup that still requires manual tweaking.
- **Sources:** r/smallbusiness, Trustpilot reviews for Shopify/Wix.

## Design Doc
- **High-Level Architecture:**
  - `OnboardingAgent`: Core orchestrator.
  - `ProfileExtractor`: Pulls business data from user input or social media link.
  - `StorefrontGenerator`: Selects theme, generates copy, configures default products/services.
- **UX Flow (375px Mobile First):**
  1. User enters business name and type, or links an Instagram profile.
  2. Loading screen with "AI is building your business..."
  3. Presentation of the generated storefront.
  4. One-tap approval to go live.
- **AI Integration Points:**
  - LLM to generate business description, mission statement, and initial product categories.
  - Image generation for placeholder graphics if user has none.

## Implementation Prompt
**User-Facing Outcome:** A fully functional, beautifully designed online store generated in under 2 minutes from a mobile device, requiring zero technical decisions from the user.
**Critical User Journey:**
1. Open OHC app.
2. Tap "Start my business".
3. Provide business name and category.
4. Review generated store and tap "Publish".
**Acceptance Criteria:**
- Flow works entirely on mobile (375px viewport).
- AI agent generates valid, contextual copy based on the business category.
- Output is a deployable, responsive storefront.

## Priority
P0

## Estimated Scope
Large
