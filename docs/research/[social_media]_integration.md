# [social_media] Unified Inbox & Social Integration

## Title
Implement Unified Inbox & Social Media Comment/DM Integration

## Problem Statement
Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive inquiries across Instagram DMs, Facebook Comments, WhatsApp, and TikTok. Monitoring multiple apps is overwhelming, leads to missed sales, and takes time away from their core business. They need a single place to see and reply to all customer messages, and they want AI agents (like "The Ambassador") to automatically draft replies or handle routine FAQs (e.g., "Do you do vegan cakes?") while they sleep or work.

## Research Report
### Market Evaluation
- **Meta Graph API (Instagram & Facebook)**: The industry standard for accessing DMs and comments. Supports Webhooks for real-time message delivery.
    - *Ease of use (for user)*: Requires Facebook Login and Meta Business Suite connection. Often confusing for non-technical users due to Meta's complex permission scopes and page linking requirements.
    - *Pricing*: Free API access, but subject to rate limits.
- **WhatsApp Cloud API**: Official Meta API for WhatsApp Business.
    - *Ease of use (for user)*: Requires a dedicated phone number.
    - *Pricing*: Conversation-based pricing (first 1,000 service conversations/month free).
- **TikTok for Business API**: Allows reading comments and DMs.
    - *Ease of use (for user)*: OAuth flow similar to Meta.
- **Aggregators (e.g., Twilio Conversations, MessageBird)**:
    - *Pros*: Single API for multiple channels.
    - *Cons*: High per-message cost, abstracting too much can break platform-specific features (like Instagram product shares).

### Integration Risks & Considerations
- **OAuth Complexity**: Guiding a non-technical user through Meta's OAuth flow without them getting stuck on "Page not linked to Instagram account" errors is historically the biggest drop-off point.
- **Webhook Reliability**: The system must handle duplicate webhooks, out-of-order delivery, and downtime gracefully.
- **Cloud vs. Standalone**: In Cloud mode, webhooks are straightforward to route to tenants. In Standalone mode, exposing a local instance to Meta Webhooks requires a tunneling solution (like ngrok, Cloudflare Tunnels, or an OHC relay server) which complicates setup.

## Design Doc
### User Experience
1. **Connection**: The user goes to the "Customer Success" department tab and clicks "Connect Social Media". They are guided through a simplified, wizard-like OAuth flow for Meta (Instagram/Facebook) and WhatsApp.
2. **Unified Inbox**: A new "Inbox" view in the OHC app aggregates all messages. Each thread clearly shows the source icon (Instagram, WhatsApp, etc.).
3. **AI Drafting**: When a new message arrives, "The Ambassador" agent reads it, checks the business's knowledge base (products, policies, hours), and generates a draft reply. The user sees the draft in the Inbox and can tap "Send" or edit it.
4. **Auto-Reply Mode**: The user can enable "Auto-Reply While Sleeping" or "Auto-Reply to FAQs", allowing the agent to send responses immediately if confidence is high.

### System Flow
- User completes OAuth → OHC stores short-lived/long-lived tokens and subscribes to Meta webhooks for the user's Page/Account.
- Meta sends a webhook for a new DM → OHC webhook gateway receives it, identifies the tenant, and publishes an event to the internal event mesh.
- The `Customer Success` agent picks up the event, processes it through the LLM with context (past interactions via pgvector, product catalog), and generates a draft response.
- The Inbox UI updates via WebSocket/SSE with the new message and AI draft.
- User approves draft → OHC calls the respective platform API to send the message.

## Implementation Prompt
Implement a unified Inbox feature that allows users to connect their Meta (Instagram/Facebook) accounts via OAuth. Webhooks from these platforms should populate a single Inbox UI within OHC. Ensure the "Customer Success" AI agent is integrated to automatically generate draft replies to incoming messages based on the tenant's context. The UI must be mobile-first and clearly indicate the source of each message. Do not prescribe specific database schemas or API endpoints; focus on delivering the end-to-end user experience of connecting an account, receiving a message, and approving an AI-drafted reply.

## Priority
P0

## Estimated Scope
Large