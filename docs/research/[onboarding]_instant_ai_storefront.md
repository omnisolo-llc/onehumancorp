# [Onboarding] Instant AI Storefront

## Problem Statement
For non-technical founders like Maya the baker or Fatima the food cart owner, setting up an online store is the biggest barrier to entry. Platforms like Shopify or Wix require 30-60 minutes of configuration, template selection, domain linking, and writing copy. The "Setup Complexity" causes many users to abandon the process before making their first sale.

## Research Report
- **Source**: Shopify App Store 1-star reviews, YouTube tutorials ("Why I left Shopify").
- **Data Point**: 73% of 1-star Shopify reviews mention the setup being confusing for beginners.
- **Competitor Landscape**:
  - Shopify and Wix are legacy leaders with high complexity.
  - Durable offers fast AI generation but lacks deep business management features.
- **Recommendation**: OHC should implement an Instant AI Storefront generator because eliminating setup friction allows non-technical users to go live in under 10 minutes, directly attacking the #1 pain point of our target market.

## Design Doc
- **Core Entities**: Storefront, Prompt, Theme, Products, Policies.
- **Key Relationships**: A User has a Storefront. A Storefront has a Theme, Products, and Policies generated from a Prompt.
- **UI Wireframes/Flow**:
  - **Mobile First (375px)**: A simple conversational interface. "What kind of business are you starting?"
  - **Input**: User enters a single sentence (e.g., "I sell homemade vegan cookies in Austin").
  - **Generation Screen**: A loading animation showing AI agents "working" (building the site, writing descriptions, setting up policies).
  - **Review Screen**: The generated storefront is presented for review. User can tap to edit or "Go Live".
- **AI Integration**:
  - A specialized agent pipeline takes the initial prompt and parallelizes tasks: generating layout, writing copy, creating placeholder products, and drafting standard policies.

## Implementation Prompt
Create an onboarding flow where a new user can generate a complete, functional storefront from a single text prompt. The user journey starts immediately after signup. They provide a brief description of their business. The system must then automatically generate the storefront layout, basic product listings with descriptions, and standard business policies. The user should be able to review the generated store and publish it with a single action. The entire flow must be optimized for mobile screens.

## Priority
P0

## Estimated Scope
Large
