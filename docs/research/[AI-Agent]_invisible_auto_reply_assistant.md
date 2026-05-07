# Invisible Auto-Reply Assistant

## Problem Statement
Service providers (like Carlos, 42, handyman) miss leads because they are busy working and cannot reply to customer inquiries instantly. Manual follow-ups are inefficient and lead to lost revenue.

## Research Report
*   **Competitor Analysis:**
    *   *Shopify:* Sidekick is for the merchant, not the customer.
    *   *Wix:* Basic autoresponders, but not context-aware AI.
*   **User Pain Points:** "I lose customers because I can't reply while on a ladder."
*   **Opportunity:** An AI agent that intercepts incoming messages (SMS, Web Chat, IG DMs), understands context, and replies instantly, even scheduling appointments automatically.

## Design Doc
*   **Architecture:**
    *   Webhook integrations for messaging channels.
    *   Context-aware NLP engine trained on the business's FAQ and availability.
*   **UI:** Simple toggle to turn "Auto-Reply" on/off, with a log of AI-handled conversations for review.

## Implementation Prompt
Develop an invisible AI assistant that automatically responds to customer inquiries across multiple channels. The agent should parse the customer's intent, provide accurate information based on the business profile, and route complex queries to the owner. The goal is zero missed leads.

## Priority
P1

## Estimated Scope
Medium
