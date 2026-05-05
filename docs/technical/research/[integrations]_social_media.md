# [Social Media] Integrate Unified Inbox for DMs and Comments

## Problem Statement
Small business owners receive customer inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Managing these scattered messages is overwhelming and leads to slow response times or missed sales opportunities. They need a single, unified inbox within OHC to read and reply to all customer messages.

## Research Report
**Evaluated Tools:** Meta Graph API (for Instagram/Facebook), WhatsApp Business API
**Alternatives Considered:** Smooch/Zendesk Sunshine, Twilio Conversations
**Pros:** Direct integration with Meta APIs ensures reliable delivery and access to rich message features (images, reactions). No middleman costs compared to aggregator services.
**Cons:** Meta's App Review process can be slow and strict. Requires managing long-lived page access tokens.
**Ease of Use for Non-technical Users:** The user clicks "Connect Instagram" or "Connect Facebook Page", completes the standard Meta OAuth flow, and instantly sees their DMs appear in the OHC Customer Success tab.
**Pricing:** Free for basic Graph API usage; WhatsApp pricing is conversation-based.
**Deployment:** Cloud-native (OAuth callbacks require a public webhook).

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer sends a DM on Instagram or WhatsApp.
- **Action:** Meta sends a webhook to OHC. OHC parses the payload and normalizes it into a standard "Message" record in the tenant's unified inbox.
- **AI Agent Interaction:** "The Ambassador" agent reads the incoming message, matches the sender against the customer CRM, and drafts a suggested reply based on past context and business knowledge.
- **User View:** A "Unified Inbox" UI showing threads from all connected platforms, with AI-drafted replies ready for approval or auto-sending.

## Implementation Prompt
Integrate the Meta Graph API to receive and send Instagram Direct Messages and Facebook Page messages. Implement the OAuth flow for users to connect their social accounts. Create webhook handlers to ingest messages into the OHC database, and update the UI to display a unified inbox. Ensure "The Ambassador" agent is hooked into the message ingestion pipeline to draft replies.

## Priority
P0

## Estimated Scope
Large
