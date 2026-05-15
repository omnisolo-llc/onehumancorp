# [AI] Auto-Reply & Follow-up Agent

## Title
AI Auto-Reply & Follow-up Agent for SMB Customer Messages

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by incoming messages across Instagram DMs, SMS, and email. They miss leads when busy and spend hours manually responding to repetitive questions about pricing, availability, and services.

## Research Report
- **Competitor Landscape**:
  - Shopify Sidekick is a chat assistant for the merchant, not the customer.
  - Wix and GoDaddy lack autonomous customer-facing reply agents.
- **User Pain Points**:
  - 73% of 1-star reviews for existing platforms mention being overwhelmed and dropping leads.
  - "I spend 3 hours a day just answering Instagram DMs about my cake prices" (Reddit r/smallbusiness).
- **Differentiation**:
  - OHC will provide an invisible, autonomous agent that auto-replies to inquiries using context from the business's inventory, pricing, and schedule, saving hours per day.

## Design Doc
- **Architecture**:
  - Entity: `AutoReplyConfig` (stores prompt context, tone, enabled channels).
  - Integration: Webhooks for incoming messages (Instagram, SMS, Email).
  - AI Agent: LLM agent (e.g., using GPT-4o) trained on business data to generate responses.
- **UI Wireframes/Flow**:
  - Mobile UX (375px): A simple "AI Auto-Reply" toggle on the dashboard.
  - Configuration screen: Three text areas: "Business Info", "Tone", "Special Instructions".
  - Inbox view: Messages handled by AI are tagged "AI Handled". Merchant can take over anytime.

## Implementation Prompt
Implement the AI Auto-Reply & Follow-up Agent. The Critical User Journey begins when a user enables the "Auto-Reply" feature from the mobile app and configures their business context. When a customer sends a message, the system should automatically generate and send a contextual response.
- **Acceptance Criteria**:
  - User can enable/disable auto-reply.
  - User can define business context and tone.
  - Incoming messages trigger an AI evaluation and response.
  - Business owner can view and override AI responses.

## Priority
P0

## Estimated Scope
Large
