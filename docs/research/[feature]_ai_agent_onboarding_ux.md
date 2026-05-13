# Feature Mission: AI Agent Onboarding UX (The 10-Minute Launch)

## Problem Statement
The "First Hour" of a small business platform is where most founders quit. Shopify and Wix require 30-60 minutes of manual configuration (shipping, taxes, themes) before a store is "real." For a non-technical founder like Fatima or Maya, this technical wall feels insurmountable. They need a "Launch" experience that feels like a conversation, not a configuration.

## Research Report
- **Competitive Benchmark**: Durable.co generates a site in 30 seconds, but it's "shallow." Shopify takes ~60 minutes.
- **User Pain Point**: 73% of SMB owners cite "Setup Complexity" as their #1 frustration.
- **The Gap**: No platform currently uses AI to *autonomously* configure the business logic (shipping rules, tax categories, initial product catalog) based on a simple natural language description.

## Design Doc
### Screen Flow (375px Mobile-First)
1. **Identity Hook**: "What's your business name?" -> AI generates taglines.
2. **Persona Mapping**: Icons for "Food," "Retail," "Services." AI sets the "Agent Department" configuration.
3. **Context Injection**: "Tell me about one thing you sell." -> AI drafts the entire product entry and business settings.
4. **Vibe Selection**: 3 dynamic Generative UI tiles.
5. **Team Activation**: Animation of agents "clocking in."

### Mobile UX (375px)
- **Zero Keyboards**: After the name, use multi-choice cards and voice to minimize typing.
- **Progressive Disclosure**: Hide all "Advanced" settings behind the "The Advisor" agent.
- **Haptic Success**: Vibrate the phone when a department is activated.

### AI Agent Integration
- **Onboarding Agent**: Orchestrates the initial setup by calling the specialized agents (Manager, Promoter, etc.) to prepare their respective data silos.

## Implementation Prompt
Implement a "10-Minute Launch" onboarding flow. The UI must be optimized for 375px mobile screens and prioritize "Decisions over Labor." The backend must use an "Onboarding Agent" to parse the user's natural language business description and autonomously populate the `products`, `agents`, and `organization_settings` tables with sensible defaults. Success is defined as a user having a live storefront and their first "Action Feed" item ready within 600 seconds.

## Priority
P0

## Estimated Scope
Large
