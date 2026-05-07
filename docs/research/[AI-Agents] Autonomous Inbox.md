# [AI-Agents] Autonomous Inbox

## Problem Statement
Small business owners like Priya (boutique owner) and Leo (music tutor) lose hours every day answering repetitive questions in Instagram DMs, WhatsApp, and email. They miss leads when they are busy working.

## Research Report
- **Market Data:** 73% of consumers expect a response from a business within 5 minutes. SMBs average 4+ hours.
- **Competitor Audit:** Shopify Inbox requires manual replies. Meta Business Suite has basic auto-replies, but they are robotic and not agentic.
- **Opportunity:** An invisible AI agent that understands the business's context (inventory, pricing, calendar) and answers questions conversationally.

## Design Doc
- **Architecture:** A central message broker (using Redis Pub/Sub) that ingests messages from various channels (Webhook integrations for IG, FB, WhatsApp). An AI worker service processes messages against a localized RAG context (the specific tenant's data).
- **UX Flow:** A unified inbox view in the OHC app. AI responses are marked with a subtle spark icon. The owner can take over the chat at any time.
- **AI Integration:** LLMs configured with strict grounding to only answer based on the tenant's known data (preventing hallucinations about prices or services).

## Implementation Prompt
**Outcome:** An autonomous agent that can read incoming customer messages, determine if it knows the answer based on the store's inventory and FAQ, and reply instantly.
**Critical User Journey:** Customer sends IG DM -> OHC Agent replies -> Lead is captured -> Store owner reviews transcript in app.
**Acceptance Criteria:**
- The agent must be able to read current inventory status.
- The agent must seamlessly hand off to the human if it cannot answer the question confidently.
- Multi-tenant isolation must be strictly enforced so Agent A never uses Store B's data.

## Priority
P1

## Estimated Scope
Medium
