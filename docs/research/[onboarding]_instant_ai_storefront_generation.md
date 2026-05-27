# Issue Brief: Instant AI Storefront Generation

## Title
[Onboarding] Instant AI Storefront Generation

## Problem Statement
The "Blank Page" Paralysis (Onboarding Fatigue): Non-technical SMB owners sign up for platforms like Shopify, face a complex dashboard asking for tax settings, shipping zones, and SKUs, and abandon the setup. 73% of 1-star Shopify reviews mention the initial setup being confusing for beginners. Traditional onboarding is a high-friction process that actively discourages solopreneurs.

## Research Report
- **Competitor Landscape**: Legacy platforms (Shopify, Wix) treat onboarding as a manual configuration process. AI tools like Durable generate a site fast but lack the deep operational layers needed for actual business management.
- **User Needs**: Users want to skip the "building" phase and go straight to "selling". They need an automated system that infers the heavy lifting from minimal input.
- **AI Differentiation**: Competitors use AI as a "Copilot" (you drive, it helps). OHC uses AI as a "Department" (it drives, you approve). The Builder autonomously creates the entire setup based on a few sentences of input.

## Design Doc
### High-Level Architecture
- **Trigger**: User signs up and provides a short natural language description of their business (e.g., "I am Maya. I bake vegan cakes in Austin. Prices start at $50.").
- **Agent Action**: The AI Builder Agent takes over.
  - Provisions a Stripe account (or equivalent integration).
  - Builds the storefront UI structure in < 30 seconds.
  - Writes SEO-optimized copy.
  - Generates or selects placeholder images based on "vibe".
  - Configures default tax and shipping settings based on the provided location and business type.
- **UI Flow**: The user is presented with a fully built storefront draft and asked for a "1-Tap Approval" to go live.

### Mobile UX Flow (375px First)
1. **Input Screen**: A simple chat-like interface asking: "Tell us about your business in a few sentences."
2. **Loading State**: "The Builder is setting up your store... (Setting up payments, writing copy, adding images)"
3. **Review Screen**: A mobile-optimized preview of the complete storefront.
4. **Action**: User taps "Approve & Launch" to go live instantly.

## Implementation Prompt
Implement the "Instant AI Storefront Generation" feature for the onboarding flow. Replace the traditional multi-step setup wizard with a single text input field. Use The Builder agent to parse the input and autonomously generate a complete storefront in < 30 seconds, including basic configurations, copy, and placeholder assets. Present the final draft to the user for a simple 1-tap approval.

## Priority
P0

## Estimated Scope
Large
