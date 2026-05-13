# Strategic Risk Assessment and Mitigation

## Introduction
While the OHC vision of an "Agent-First Platform" provides a massive competitive advantage, it also introduces novel risks that traditional CRUD-based e-commerce platforms do not face. This document outlines the primary strategic and technical risks associated with our AI differentiation strategy and proposes mitigation protocols.

## 1. The Risk of "Autonomous Errors" (The Rogue Agent)
### The Threat
If an autonomous agent makes a mistake, the consequences are immediate and financial.
- Example: The Growth Agent hallucinates and sends an email offering a 90% discount instead of a 9% discount.
- Example: The Support Agent incorrectly tells a customer an item is eligible for return when it is final sale.
### Mitigation Strategy
- **Human-in-the-Loop (HITL) by Default:** For the first 6 months of a user's lifecycle, all high-stakes agent actions (sending mass emails, changing prices, issuing refunds) must default to "Draft Mode." The agent prepares the action, but it requires a 1-tap human approval via the Activity Feed.
- **Strict Bounding Boxes:** Agents must operate within hardcoded constraints that the LLM cannot override. E.g., The system must enforce a rule: `MAX_DISCOUNT_PERCENT = 20%`. If the LLM requests a 50% discount, the backend firmly rejects the API call.

## 2. The Risk of Platform Trust Degradation
### The Threat
If users feel they do not understand what the AI is doing "behind their back," they will turn it off, degrading the platform back to a traditional SaaS tool.
### Mitigation Strategy
- **Radical Transparency Log:** Every action, decision, or draft created by an AI must be visible in an immutable "Agent Activity Log." The UI must clearly explain *why* the agent did something (e.g., "I drafted this reply because the customer asked about shipping, and our policy says we ship in 2 days.")
- **Easy Undo:** Every agent action must have a 1-click "Undo" button available for a short window.

## 3. The "Cold Start" Problem for RAG
### The Threat
Agents (like the Support Agent) rely on Retrieval-Augmented Generation (RAG) to answer questions accurately. If a new user provides very little context during onboarding, the agent will hallucinate or fall back to "I don't know."
### Mitigation Strategy
- **Proactive Interrogation Agent:** During onboarding, if the system detects low context, an internal agent proactively interviews the user via SMS/Push. E.g., "Hey Maya, I noticed we don't have a return policy. Do you accept returns? Reply Yes or No." It builds the RAG database asynchronously.

## 4. Latency in the "Conversational UI"
### The Threat
Users expect instant responses from traditional UIs (clicking a button). If they have to wait 5 seconds for an LLM to generate a response or process an image every time they interact, the platform will feel broken.
### Mitigation Strategy
- **Optimistic UI Updates:** The UI must respond instantly, showing processing states elegantly.
- **Small, Fast Models for Routing:** Use smaller, highly optimized models (e.g., Llama 3 8B, or specialized classification models) for fast tasks like routing a message or determining intent, reserving large slow models (GPT-4) only for complex reasoning or drafting.
- **Background Processing:** The Magic Image Content feature must happen entirely in the background. The user takes the photo and immediately moves on; the agent notifies them when the optimized listing is ready.

## 5. Cost of Intelligence (COGS)
### The Threat
LLM API calls are significantly more expensive than traditional database queries. High volume of agent activity could ruin the unit economics of the $29/mo pricing tier.
### Mitigation Strategy
- **Caching Aggressively:** Implement semantic caching. If a customer asks "What are your hours?", and the LLM generated the answer yesterday, serve the cached answer if the store hours haven't changed.
- **Tiered Intelligence:** Use cheaper models for the Starter tier and reserve state-of-the-art models for the Premium tiers.
