# [feature] Conversational "No Jargon" Setup Wizard

## Title
Implement Conversational "No Jargon" Setup Wizard for Instant Onboarding

## Problem Statement
The number one pain point for non-technical SMBs (like Fatima the Food Cart Operator) is Setup Complexity, affecting 73% of surveyed users. Legacy platforms like Shopify require users to navigate confusing dashboards and understand technical jargon (e.g., DNS, gateways, shipping zones) just to get a basic store online. This cognitive overload causes significant drop-offs during the onboarding phase, preventing users from reaching the "Aha!" moment of having a live business.

## Research Report
- **Pain Point Mapping:** Directly addresses "Setup Complexity" (Ranked #1) and "Technical Jargon" (Ranked #5).
- **Competitive Landscape:** Shopify takes 30-60 minutes with high friction. Durable offers 30-second website generation but lacks depth for actual business operations.
- **Strategic Opportunity:** OHC must bridge the gap by offering an onboarding flow that feels like a conversation with a smart assistant, rather than a data entry form. The goal is "Time to Live Store" in under 10 minutes, completely devoid of technical terminology.

## Design Doc
### High-Level Architecture
- **Interaction Model:** A chat-based or highly conversational guided UI (Progressive Disclosure Pattern).
- **Core Action:** The Wizard AI asks natural language questions ("What do you sell?", "How do you want to get paid?") and autonomously configures the complex backend settings (creating products, configuring payment gateways, setting up a default domain) based on the user's plain-English answers.
- **UI/UX Flow (Mobile First, 375px):**
  - **Screen 1:** "Welcome. What's the name of your business?"
  - **Screen 2:** "Describe what you do in one sentence." (Agent infers industry and required features, e.g., bookings vs. physical products).
  - **Screen 3:** "Do you want to accept credit cards? Yes/No." (Agent handles Stripe abstraction).
  - The default mode is "Simple." An "Advanced Toggle" is hidden away for users who later want to tweak raw configurations.

## Implementation Prompt
Build the conversational onboarding wizard using the existing OHC UI components. The wizard must collect business details through simple, jargon-free prompts and leverage an AI agent to translate these inputs into complete backend configurations (tenant setup, initial products, booking enablement, payment preferences). Ensure the entire flow is strictly mobile-optimized and persists state continuously so the user can resume if interrupted.

## Priority
P0

## Estimated Scope
Large
