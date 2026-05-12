# Title: Implement Unified Social Media Inbox via Meta Graph API

## Problem Statement
Small business owners, like Fatima who runs a bakery, have to constantly switch between Facebook, Instagram, and WhatsApp to answer customer questions and take orders. This is exhausting and leads to missed messages and lost sales. They need a single place to read and reply to everything.

## Research Report
- **Tool Evaluated:** Meta Graph API (covering Facebook Messenger, Instagram DMs, and WhatsApp Business).
- **Benefits:** It provides direct, native access to the channels where customers already are. It's the industry standard.
- **Ease of Use:** For the business owner, they just click "Connect Facebook" and authenticate. The technical complexity of webhooks is hidden from them.
- **Pricing:** WhatsApp charges per conversation (pay-as-you-go), while FB/IG messaging is generally free.
- **Cloud/Standalone:** Works in both. In Cloud, OHC manages the webhooks. In Standalone, the user needs to provide their own Meta App credentials, but the data stays entirely local.

## Design Doc
1. **Trigger:** User clicks "Connect Social Accounts" in the Integrations dashboard and completes the OAuth flow.
2. **Action:** OHC starts listening to incoming messages via webhooks (Cloud) or polling/direct webhooks (Standalone).
3. **UI Outcome:** A new "Unified Inbox" appears in the OHC dashboard. All incoming messages look the same, with small icons indicating the source platform. Business owners can reply directly from this inbox, and the message routes back to the correct platform.

## Implementation Prompt
Create a "Unified Inbox" feature where users can connect their Facebook, Instagram, and WhatsApp accounts. Display all incoming messages in a single feed. Allow users to reply to these messages directly from the OHC interface. The setup process must be a simple OAuth login flow with no technical configuration required for Cloud users.

## Priority
P0

## Estimated Scope
Large
