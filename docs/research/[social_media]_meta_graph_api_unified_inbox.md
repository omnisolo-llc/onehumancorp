# Meta Graph API (Instagram/Facebook/WhatsApp) - Unified Inbox

## Problem Statement
Small business owners, like Fatima, are overwhelmed by messages coming from multiple channels—Instagram DMs, Facebook comments, and WhatsApp. Missing a message often means missing a sale or angering a customer. It is exhausting to constantly switch between three different apps on their phone while trying to run their physical store. They need one simple, unified place to see and reply to every customer message, regardless of where the customer sent it from.

## Research Report
The Meta Graph API is the official path to integrating Instagram, Facebook Messenger, and WhatsApp Business.
- **Ease of Use for SMBs**: High once connected. The user just sees a single "Inbox" in their OHC dashboard.
- **Pricing**: Facebook/Instagram messaging is generally free. WhatsApp Business API uses conversation-based pricing (first 1,000 service conversations are free per month, then a few cents per conversation depending on the region).
- **Reputation**: Official, reliable, but notorious for complex app review processes and sudden API deprecations.
- **Competitive Analysis**: Tools like ManyChat or Ayrshare exist, but natively integrating via Meta Graph API gives us the most control over the experience and removes third-party subscription costs for the business owner.

## Design Doc
**Trigger**: Business owner clicks "Connect Facebook/Instagram" or "Connect WhatsApp" in their OHC settings. They go through the standard Meta OAuth flow.
**Actions**:
- OHC registers a webhook to listen for new messages across connected Meta platforms.
- Incoming messages are normalized into a standard OHC "Message" record and displayed in the Unified Inbox UI.
- When the business owner replies via OHC, the system routes the reply back through the correct Meta API channel.
**User Experience**: A seamless, chat-like interface in the OHC mobile app/web dashboard. It looks like a normal texting app, but messages might have small icons indicating if they came from IG, FB, or WA.

## Implementation Prompt
**User-facing Outcome**: A business owner can connect their Facebook, Instagram, and WhatsApp accounts to OHC. They can view all incoming messages from these platforms in a single "Unified Inbox" view and reply to them directly from OHC, with the customer receiving the reply on their original platform.
**Acceptance Criteria**:
- A user can authenticate their Meta accounts via OAuth.
- Incoming Instagram DMs, Facebook Messages, and WhatsApp messages appear in real-time in the OHC Inbox.
- User replies in OHC are successfully delivered to the customer on the original platform.
- Read receipts and message status (sent, delivered) are synchronized.

## Priority
P0 (Critical)

## Estimated Scope
Large
