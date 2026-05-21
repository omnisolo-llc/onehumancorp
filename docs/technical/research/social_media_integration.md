# [Social Media] Integrate Zernio for Unified Inbox

## Problem Statement
Small business owners like Priya (Boutique Owner) and Maya (Home Baker) receive inquiries across Instagram DMs, Facebook Comments, TikTok, and WhatsApp. Managing these separately means delayed responses and lost sales. They need a single, unified inbox to view and reply to all customer messages, and an AI agent to handle common questions ("do you do vegan cakes?") seamlessly across platforms.

## Research Report
**Evaluated Tool:** Zernio (Unified Social Media API)
**Alternatives Considered:** Native APIs (Meta Graph, X, TikTok), Ayrshare
**Pros:** Zernio aggregates multiple platforms into a single API endpoint, reducing OAuth complexity and the need to maintain multiple webhook structures. Excellent parsing quality for DMs and comments.
**Cons:** Third-party dependency, potential rate limits.
**Ease of Use for Non-technical Users:** Transparent. The user simply connects their social accounts once and all messages flow into the OHC unified inbox.
**Pricing:** Estimated at ~$50-100/mo base + volume pricing, scalable for multi-tenant SaaS.
**Deployment:** Works well in Cloud. For Standalone, OAuth callback handling will require specific configuration or proxying.

## Design Doc
**Integration with OHC:**
- **Trigger:** A new message arrives via Zernio webhooks.
- **Action:** The system parses the message and routes it to the tenant's unified inbox database.
- **AI Agent Interaction:** The Customer Success agent ("The Ambassador") receives the incoming message context, drafts a reply, and (if auto-reply is enabled) posts the response back through Zernio.
- **User View:** A unified "Inbox" screen in the OHC mobile and desktop apps.

## Implementation Prompt
Implement the backend integration with Zernio to receive webhooks for incoming social messages and send outgoing replies. Create the frontend UI for a unified inbox where users can view and reply to cross-platform messages. Ensure "The Ambassador" AI agent can draft replies within this interface.

## Priority
P1

## Estimated Scope
Large
