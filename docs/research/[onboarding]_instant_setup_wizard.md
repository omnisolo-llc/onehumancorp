# Issue Brief: 10-Minute Setup Wizard (No Jargon)

## Problem Statement
Shopify and Wix setups overwhelm beginners with technical jargon like "DNS", "Webhooks", and complex multi-step forms. 73% of SMB owners rank "Setup Complexity" as their top pain point. OHC must enable a full business launch from a mobile phone in under 10 minutes by completely eliminating technical terminology.

## Research Report
- **Competitor Audit**: Shopify requires 30-60 minutes and PC for setup; Wix takes 20-40 minutes. Durable provides a 30-second site, but lacks depth.
- **Pain Point Mapping**: "Setup Complexity" (73%) and "Technical Jargon" (48%).
- **Persona Context**: Maya (Baker) and Carlos (Handyman) don't care about the backend. They only want to state their business type, vibe, and pricing, letting the AI generate the rest.

## Design Doc
- **Core Entities**: SetupSession, OnboardingProfile, BusinessConfig.
- **Integration Points**: Agentic generation for layout, default products, and copy.
- **UI Flow (375px Native)**:
  1. "What do you do?" (Free-text input processed by AI).
  2. "Pick a Vibe" (Visual cards: Minimal, Bold, Classic).
  3. "Generating your business..." (Loading animation while AI builds site, configures defaults).
  4. Live Preview with 1-tap "Publish".
- **AI Integration**: The agent translates the simple inputs into full technical configuration under the hood.

## Implementation Prompt
Build a mobile-first, conversational setup wizard using Slint. The wizard should consist of no more than 3 simple, jargon-free screens. Use the builtin AI agent to parse the user's natural language input and automatically generate the necessary `BusinessConfig`, initial layout, and sample product listings. Ensure the flow is completely functional on a 375px screen.

## Priority
P0

## Estimated Scope
Medium
