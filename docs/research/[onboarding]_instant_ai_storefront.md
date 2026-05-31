# Issue Brief: 10-Minute Setup Wizard (Instant AI Storefront)

## Title
10-Minute Setup Wizard (Instant AI Storefront)

## Problem Statement
73% of SMBs cite setup complexity and technical jargon as their primary barrier to launching online. The "Blank Page" Paralysis affects non-technical SMB owners who sign up for platforms, face complex dashboards (asking for DNS, APIs, SKUs), and abandon setup. traditional onboarding is a high-friction process.

## Research Report
Based on the Top 10 SMB User Pain Points, users abandon platforms like Shopify due to the technical jargon. Durable offers a 30-second site generation, but it lacks operational depth. We can provide a conversational setup flow that synthesizes website structure, copy, and product catalogs in under a minute without jargon.

Users want to skip the "building" phase and go straight to "selling". OHC uses AI as a "Department" that builds the store for you to approve, not just a "Copilot".

## Design Doc
### High-Level Architecture
- **Trigger**: User signs up and provides a short natural language description of their business in a chat interface.
- **Agent Action**: The Marketing Agent (Invisible Storefront Generator) takes over.
  - Provisions payment integration (e.g., Stripe).
  - Builds storefront UI structure.
  - Writes SEO-optimized copy & generates placeholder images.
  - Configures default tax/shipping settings based on location/business type.
- **UI Flow**: The user is presented with a fully built storefront draft and asked for a "1-Tap Approval".

### Mobile UX Flow (375px First)
1. **Input Screen**: Simple chat-like interface asking: "Tell us about your business in a few sentences."
2. **Loading State**: "Our Marketing Department is building your store... (Setting up payments, writing copy...)"
3. **Review Screen**: A mobile-optimized preview of the complete storefront.
4. **Action**: User taps "Approve & Launch" to go live instantly.

## Implementation Prompt
Implement a step-by-step Setup Wizard using Riverpod/Slint. The Critical User Journey goes from "Launch App" -> "Answer 3 plain-language questions" -> "View generated site". Ensure all interactions use large touch targets and are completely jargon-free.

## Priority
P0

## Estimated Scope
Large
