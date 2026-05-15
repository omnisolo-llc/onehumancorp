# [feature] Omnichannel Auto-Responder Agent

## Title
Omnichannel Auto-Responder Agent

## Problem Statement
Founders like Priya and Leo are overwhelmed by customer messages across Instagram DMs, WhatsApp, and email. They spend hours answering basic questions (e.g., "What are your hours?", "Where is my order?") instead of growing their business.

## Research Report
- **Data:** Threads in r/ecommerce show owners spending 2+ hours daily on customer service.
- **Competitors:** Shopify requires third-party apps (e.g., Gorgias) which are expensive and complex to set up.
- **Sources:** Reddit r/ecommerce, Shopify app store reviews for customer service apps.

## Design Doc
- **High-Level Architecture:**
  - `MessageIngestionService`: Webhooks for Instagram, WhatsApp, Email.
  - `ContextEngine`: Retrieves order history, business FAQs, and product details.
  - `ResponseAgent`: LLM configured with business persona to generate replies.
- **UX Flow (375px Mobile First):**
  1. User connects social accounts.
  2. Agent handles incoming messages automatically if confidence > 90%.
  3. Escalates complex issues to the human inbox with a drafted response.
- **AI Integration Points:**
  - RAG system to query business knowledge base and active orders.

## Implementation Prompt
**User-Facing Outcome:** An invisible AI assistant that automatically answers routine customer questions across all channels, escalating only complex queries to the business owner.
**Critical User Journey:**
1. Customer asks "When do you close today?" on Instagram.
2. Agent reads business hours and replies immediately.
3. Owner sees the resolved conversation in their OHC inbox but didn't have to act.
**Acceptance Criteria:**
- Agent correctly identifies intent for top 5 query types (hours, location, order status, return policy, product availability).
- Escalation path is clear and notifies the user on mobile.

## Priority
P1

## Estimated Scope
Medium
