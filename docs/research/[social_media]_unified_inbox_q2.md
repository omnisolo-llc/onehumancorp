# [social_media] Issue Brief: Unified Customer Inbox

**Title**: Implement Unified Customer Inbox for Meta Channels (Instagram, Facebook, WhatsApp)
**Problem Statement**: As a small business owner like Maya (the baker), I get customer messages across Instagram DMs, Facebook Messenger, and WhatsApp. It's overwhelming to constantly switch between apps on my phone, and I sometimes miss custom order requests or take too long to reply. I need all my customer messages in one simple inbox so I (and my AI Customer Success agent) can reply quickly and never lose a sale.
**Research Report**:
- Evaluated Tools: Meta Graph API (Instagram Messaging, Facebook Messenger, WhatsApp Business API), Twilio Conversations, Zendesk Smooch.
- Ease of Use: Using the native Meta Graph API + Webhooks requires the user to go through an OAuth flow to connect their Facebook Business page. This is a standard flow many users are familiar with. Twilio/Smooch adds a layer of abstraction but increases cost.
- Pricing: Meta APIs are mostly free for standard messaging (WhatsApp has some per-conversation costs, but the first 1000 service conversations are often free or very cheap). Twilio charges per active user and message, which hurts our free tier.
- Reputation: Meta API is the industry standard. It can be complex to verify apps, but once approved, it's reliable.
- Environment: Works seamlessly in Cloud via webhooks. For Standalone mode, users would need to configure their own Meta Developer App or rely on an OHC cloud-relay service, making it more Cloud-oriented.
- Recommendation: Direct integration with Meta Graph API for lowest cost to users and direct control.
**Design Doc**:
- **Integration Flow**: The business owner clicks "Connect Instagram/Facebook" in the Operations or Customer Success tab. An OAuth popup guides them to grant OHC access to their pages.
- **Data Flow**: OHC registers a centralized webhook endpoint. When a customer messages the Instagram page, Meta posts to the webhook. OHC routes it to the specific tenant's inbox.
- **User Interface**: A new "Inbox" screen in the Flutter app. Shows a list of conversations (like a standard messaging app) with icons indicating the source (IG, FB, WA).
- **AI Integration**: The "Ambassador" (Customer Success Agent) monitors this inbox. If configured, it drafts a reply or auto-replies to FAQs (like "do you do vegan cakes?").
**Implementation Prompt**: Create a "Unified Inbox" feature where users can connect their Meta accounts (Facebook, Instagram, WhatsApp). Customers' messages from these platforms should appear in a single scrolling list in the OHC app. The business owner must be able to read and reply to messages directly from OHC, and the replies must show up in the customer's native app. Acceptance criteria include a working OAuth connection flow, real-time message receipt, and successful reply delivery.
**Priority**: P0
**Estimated Scope**: Large
