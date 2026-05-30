# Issue Brief: Unified Omni-Channel Inbox

## Title
[Communication] Unified Omni-Channel Inbox

## Problem Statement
Communication Fragmentation: Small business owners interact with customers across multiple disconnected platforms—Instagram DMs, Facebook Messenger, WhatsApp, Email, and SMS. This fragmentation forces them to constantly switch apps, leading to missed messages, lost context, dropped leads, and a poor customer experience. A non-technical user is completely overwhelmed trying to manage 5 different inboxes from their phone.

## Research Report
- **Market Reality:** Customers expect to communicate on their preferred channel. Businesses that respond faster win more deals.
- **Competitor Weakness:** Traditional platforms (like Shopify) have rudimentary inboxes that primarily focus on email or basic web chat. They require complex, third-party app integrations to pull in social DMs, which non-technical users struggle to configure.
- **User Pain Points:**
  - Opening Instagram, then WhatsApp, then Email every morning to check for orders or questions.
  - Forgetting which app a specific customer used to ask a question.
  - Inability to link a DM conversation to an actual order or customer profile.
- **OHC Solution:** Centralize all incoming communications into a single, beautifully designed, mobile-first inbox within the OHC platform. Make the channel source invisible to the workflow.

## Design Doc
### High-Level Architecture
- **Ingestion Layer:** Build specialized webhook receivers and polling mechanisms for supported platforms:
  - Meta Graph API (Instagram DMs, Facebook Messenger).
  - WhatsApp Business API.
  - Email (via SendGrid/AWS SES inbound parsing).
  - Web/App Native Chat.
- **Normalization:** Transform all incoming messages (regardless of source) into a standard internal `OmniMessage` protocol format.
- **Identity Resolution:** Use the "Universal Cross-Channel Identity Resolution Engine" to match an incoming Instagram handle or phone number to an existing customer profile in the CRM.
- **Routing:** Route the normalized message to the appropriate tenant's inbox and trigger real-time updates (via WebSocket/SSE) to the connected clients.

### Mobile UX Flow (375px First)
- **Unified List:** A single list view displaying all conversations, sorted by recency. Small, elegant icons indicate the source channel (e.g., a tiny Instagram logo next to the timestamp).
- **Thread View:** A standard chat interface. The user types a reply and hits send. The backend handles routing that reply back to the correct original channel (so the customer receives it as an IG DM, WhatsApp message, etc.).
- **Context Panel:** While in a chat, the user can pull down or tap a top-bar icon to instantly see the customer's profile, past order history, and lifetime value without leaving the conversation.

## Implementation Prompt
Build a single, unified inbox within the OHC app that aggregates messages from various channels (Instagram, WhatsApp, Email, Web). Implement the backend normalization layer to handle different external API payloads and standardize them. Create the real-time routing mechanisms to deliver messages to the frontend. On the frontend, design and implement a premium, mobile-first chat interface that masks the complexity of the underlying channels and deeply integrates with the CRM to display customer context alongside the conversation.

## Priority
P1

## Estimated Scope
Extra Large
