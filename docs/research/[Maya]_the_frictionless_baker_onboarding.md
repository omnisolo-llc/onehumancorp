# Issue Brief: The 30-Second Instant Storefront (Maya)

## Problem Statement
Maya (Baker, 28) is overwhelmed by the setup process of Shopify. She sells via Instagram DMs because "it just works," even though it's chaotic. She needs a professional storefront but won't spend 2 hours configuring it. Every minute spent on setup is a minute not spent baking.

## Research Report
- **Competitor Benchmark**: Durable generates a site in 30 seconds. Shopify takes 30-60 minutes for a "good" setup.
- **Pain Point**: "The Blank Canvas Problem" - founders don't know what to write for their "About Us" or how to categorize shipping.
- **Opportunity**: OHC can use the "Oracle" intelligence to extrapolate a full business profile from a single paragraph of text.

## Design Doc
### High-Level Architecture
- **The Extrapolator**: A single-prompt onboarding that uses the Advisor agent to extract Name, Industry, and Services from a user's bio.
- **Zero-Config Shipping**: Automatic calculation based on the user's location and "Standard Baker" profiles.
- **Vibe-Based Design**: User selects one of three "Vibes" (e.g., "Warm & Rustic," "Modern & Clean") and the Promoter agent generates the rest.

### Mobile UX Flow (375px)
1. **Onboarding**: "Hi Maya! Tell me about your bakery in one sentence."
2. **Generation**: A progress bar shows agents working: "Drafting your menu... Designing your logo... Setting up your checkout..."
3. **Review**: Maya sees a live, functional storefront in under 60 seconds.

### AI Agent Integration
- **The Advisor**: Extrapolating metadata from raw text.
- **The Promoter**: Generative design and copy.
- **The Accountant**: Pre-configuring tax and payment defaults based on industry.

## Implementation Prompt
Implement a "Fast-Track Onboarding" mode in the setup flow. This mode should present a single text area for the user to describe their business. The "Advisor" agent should parse this input to populate the business name, description, and primary products/services. "The Promoter" should then use these attributes to generate a theme-appropriate storefront draft immediately. The entire process from "Enter bio" to "Live Preview" should take less than 60 seconds.

## Priority
P0

## Estimated Scope
Medium
