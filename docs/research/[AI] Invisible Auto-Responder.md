# [AI] Invisible Auto-Responder

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by incoming messages across Instagram DMs, WhatsApp, and email. They miss leads because they cannot reply instantly while working, and setting up complex chatbots is too technical.

## Research Report
- **Competitor Landscape**:
  - Shopify Sidekick is an internal assistant for the merchant, not an auto-responder for customers.
  - Wix and GoDaddy offer basic auto-replies, but they are not context-aware AI.
- **Pain Point Validation**: Reddit `r/smallbusiness` and App Store reviews frequently mention "I miss messages when I'm busy" and "I can't keep up with DMs."
- **Opportunity**: A zero-setup AI that reads the incoming message, understands the business context (e.g., pricing, availability), and replies automatically to capture the lead.

## Design Doc
- **Architecture**:
  - Incoming Message Webhook -> Context Retrieval (Business Profile, Pricing) -> LLM -> Reply Generation -> Outbound Message Webhook.
- **UI Wireframes (375px first)**:
  - Simple toggle: "Enable AI Auto-Responder".
  - Log of AI-handled conversations with an "Intervene" button.
- **AI Integration**: Use lightweight LLM (e.g., Claude 3 Haiku or Llama 3) for fast, context-aware replies.

## Implementation Prompt
Implement an AI Auto-Responder feature that connects to social channels and replies to customer inquiries using the business profile context. The user journey should consist of a single toggle to enable the feature, and a view to monitor AI conversations.

## Priority
P0

## Estimated Scope
Medium
