# Title: Instant AI Storefront Generation

## Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) find setting up a website overwhelming. They are asked confusing questions about DNS, liquid templates, or shipping zones. The industry standard platforms take too long (30m+) and create high friction, causing many owners to abandon the process and stick to Instagram DMs or word-of-mouth. They need a system that builds their online presence instantly so they can start selling without feeling "stupid" or technically inadequate.

## Research Report
- **Competitor Landscape**:
  - Shopify: Deep, but complex. Takes >30 minutes. Sidekick is reactive and not an autonomous builder.
  - Wix: ADI builds sites quickly but leaves users with a complex dashboard.
  - Durable: AI generates a site in <1 minute, winning on speed, but lacks robust business management tools.
- **User Pain Points Data**:
  - Setup Complexity is the #1 pain point (73% frequency). Users hate technical jargon.
  - 73% of 1-star Shopify reviews mention the setup being confusing for beginners.
- **Sources**: Synthesis of Reddit (r/smallbusiness, r/ecommerce), Trustpilot (Wix), App Store (Shopify).
- **Opportunity**: OHC must leapfrog Durable's speed (30-second benchmark) while retaining powerful built-in business operations.

## Design Doc
- **High-Level Architecture**:
  - Conversational SetupWizard Entity.
  - Generative AI Vibe-Based Engine for instant templating.
- **Key Relationships & Integration Points**:
  - The SetupWizard feeds directly into Business Profile and Initial Inventory/Service lists.
- **UI/UX Flow (Mobile 375px First)**:
  - Screen 1: Simple conversational prompt ("What do you do?").
  - Screen 2: Loading/Generating animation (Under 30s).
  - Screen 3: Live preview of the storefront, fully populated with AI-generated copy and stock images.
  - Screen 4: "Publish & Share" or "Tweak" option.
  - The dashboard is radically simple with no technical jargon (no CNAME, API, or DNS settings visible).
- **AI Agent Integration Points**:
  - Autodream/Generative agent creates the layout, copy, and visual identity.

## Implementation Prompt
**User-Facing Outcome:** A non-technical user can describe their business in plain language and receive a fully functional, beautiful, mobile-optimized storefront in under 30 seconds.
**Critical User Journey (CUJ):**
1. User opens the OHC app.
2. User types "I sell custom birthday cakes in Austin".
3. AI agent generates the store, products, and checkout flow.
4. User taps "Launch" and immediately gets a sharable link.
**Acceptance Criteria:**
- Setup process completes in under 30 seconds.
- Zero technical jargon is presented to the user during the onboarding flow.
- The generated storefront is fully functional on mobile (375px width).
- Includes initial AI-generated product/service listings.

## Priority
P0

## Estimated Scope
Large