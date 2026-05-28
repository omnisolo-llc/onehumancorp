# [Social Media] ManyChat Integration

## Title
Integrate ManyChat for Unified Social Media Inbox

## Problem Statement
Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.

## Research Report
- **Strategy**: Direct API integration with ManyChat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Excellent Instagram and WhatsApp API integrations. Robust webhook support for routing messages to OHC's backend. Highly rated by SMBs (G2), clear UI.
- **Risks**: Pricing scales with contacts, which may be expensive for high-volume, low-margin businesses. Requires Meta business verification for some features.
- **Pricing**: Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy/relay).

## Design Doc
- User goes to the Operations dashboard and clicks "Connect ManyChat".
- User authenticates via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via ManyChat's API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.

## Implementation Prompt
Implement an OAuth flow to connect a user's ManyChat account. Create a webhook endpoint that receives incoming messages, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.

## Priority
P0

## Estimated Scope
Large
