# Title: Unified Social Media Inbox via Meta Graph API & TikTok API Integration

## Problem Statement
Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive inquiries across multiple platforms: Instagram DMs, Facebook comments, WhatsApp messages, and TikTok comments. Managing these channels separately is overwhelming. They need a single, unified inbox where their AI Customer Success Agent ("The Ambassador") can draft replies, and they can review and respond to everything in one place.

## Research Report
**Findings & Evaluation:**
To achieve a unified inbox, the most robust approach is integrating directly with the Meta Graph API (for Facebook, Instagram, and WhatsApp) and the TikTok API.
- **Meta Graph API:** Provides webhooks for real-time message events across IG DMs, FB Messenger, and WhatsApp Business. It's an industry standard. However, the OAuth flow for businesses requires careful UX design to keep it simple for non-technical users.
- **TikTok API:** TikTok provides direct messaging and comment APIs.
- **Alternatives evaluated:** Aggregator services like MessageBird or Twilio Conversations. While these simplify the engineering effort, they add significant per-message costs which conflicts with our goal of an accessible free/low-cost tier for our users. Direct integration reduces variable costs.
- **Ease of Use for Non-Technical Users:** The technical complexity will be entirely hidden. Users will simply click "Connect Instagram" and go through the standard Meta OAuth flow.
- **Cloud vs Standalone:** Works well in Cloud. In Standalone, OAuth redirect URIs require a proxy service or dynamic configuration to route callbacks locally.

## Design Doc
**Integration with OHC:**
The user links their social accounts via a simple "Connect Accounts" settings screen in the OHC mobile app or web dashboard. Once connected, incoming messages from these platforms trigger webhooks to the OHC backend.
These messages are normalized into a standard "Inbox Event" and routed to the Customer Success Agent ("The Ambassador"). The Agent drafts a suggested reply based on the business's knowledge base (e.g., answering "Do you do vegan cakes?").
The user sees a unified chat interface in the app, with badges indicating the source platform (Instagram, WhatsApp, etc.). They can tap to send the AI's drafted reply or type their own. The reply is dispatched back through the respective API.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- The user can connect their Instagram Business, Facebook Page, and WhatsApp Business accounts via a simple UI.
- Incoming messages from any connected platform appear in a single unified "Inbox" view on both mobile and desktop.
- The Customer Success AI agent automatically generates a suggested reply for incoming messages.
- The user can send the AI suggestion or their own custom text back to the customer on their original platform seamlessly.
- The process must feel invisible—users should not need to know what a webhook or API key is.

## Priority
P0

## Estimated Scope
Large
