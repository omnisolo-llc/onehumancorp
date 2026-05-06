# 🔍 Scout: The Silent Ambassador (Customer Success)

## Title
The Silent Ambassador (Customer Success)

## Problem Statement
Solopreneurs like Maya (The Home Baker) lose up to 30% of sales due to slow response times in Instagram DMs or Facebook Messenger. They cannot be online 24/7 to answer the same repetitive questions about pricing, allergies, or delivery zones. They need a system that instantly drafts accurate replies based on their business knowledge, ready for them to approve or send automatically.

## Research Report
- **Strategy**: Event-driven DM reading and draft generation.
- **Target Persona**: Maya (Home Baker), Carlos (Handyman)
- **Advantages**: Increases conversion rates by reducing response latency. Reclaims personal time for the owner.
- **Risks**: Hallucinating incorrect business information (e.g., saying a cake is vegan when it isn't). Requires strict guardrails and reliance on the business memory layer.
- **Competitor Gap**: Existing platforms treat this as a separate chatbot widget. OHC integrates it directly into a unified inbox, acting on background events rather than waiting for a widget prompt.
- **Data**: 40% of users experience "Communication Lag" as a major issue.

## Design Doc
- **High-Level Architecture**:
  - The agent ("The Ambassador") listens for `MessageReceived` events from social channels.
  - The agent queries the business's Persistent Memory Layer (RAG) to find relevant policies, pricing, or product details.
  - The agent drafts a reply that matches the owner's tone.
  - The draft is placed in the unified inbox for review, or sent immediately if the user has enabled "Auto-Pilot" for that specific type of question.
- **UI Flow**:
  - Customer messages Maya: "Do you deliver to downtown?"
  - The Ambassador agent drafts a reply: "Hi there! Yes, we deliver downtown for a $5 fee. Let me know if you'd like to place an order!"
  - Maya sees the draft in her OHC inbox, taps "Send" from her lock screen.

## Implementation Prompt
Implement "The Ambassador" customer success agent. The agent should subscribe to incoming message events, query the RAG system (VectorRepository) for relevant business context, and generate a draft response using the LLM routing gateway. The drafted response should be stored and linked to the message in the unified inbox UI. Include a mechanism to flag high-confidence answers for potential future auto-sending.

## Priority
P0

## Estimated Scope
Large
