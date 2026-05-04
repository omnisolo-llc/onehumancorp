# Social Media Integration

## Title
[Social Media] Unified Inbox Integration for Instagram, Facebook, WhatsApp, and TikTok

## Problem Statement
Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive inquiries across multiple social media platforms. Checking Instagram DMs, Facebook comments, WhatsApp, and TikTok messages separately is overwhelming and leads to missed sales opportunities. They need a single place to view and respond to all customer messages.

## Research Report
- **Evaluated Tools**: Meta Graph API (for FB/IG/WhatsApp), TikTok for Business API, external aggregators like MessageBird or Twilio.
- **Ease of Use**: Non-technical users struggle with complex OAuth flows (e.g., Meta Business Manager). The integration must abstract the setup into a simple "Connect with Facebook" button.
- **Pricing**: Meta APIs are mostly free for basic messaging, but WhatsApp Business API has per-conversation pricing. Twilio/MessageBird adds a per-message markup.
- **OAuth Complexity**: High for Meta due to app review requirements.
- **Message Parsing Quality**: High for text, variable for media (voice notes, images).
- **Webhook Reliability**: High for Meta, but requires strict SLA compliance to avoid app suspension.
- **Cloud vs Standalone**: Works well in Cloud mode. In Standalone mode, webhooks require a tunneling or polling mechanism, which adds complexity.

## Design Doc
- **Triggers**: A customer sends a message on a connected social platform.
- **Actions**: The system receives the webhook, maps it to the corresponding customer profile, and displays it in the OHC unified inbox. The AI Customer Success agent can automatically draft or send replies based on business context.
- **User View**: A single "Inbox" screen on their phone where messages from all platforms appear seamlessly, with the platform logo indicating the source.

## Implementation Prompt
Implement a unified inbox feature that allows users to connect their social media accounts. When a customer messages them on Instagram, Facebook, WhatsApp, or TikTok, the message should appear in a central OHC inbox. The business owner should be able to reply from OHC, and the response should be delivered back to the original platform. Ensure the setup process is a simple 1-click OAuth flow without requiring technical configuration.
- **Acceptance Criteria**: User can connect Instagram, Facebook, WhatsApp, and TikTok accounts with a simple 1-click OAuth flow. Incoming messages from all platforms appear in a unified "Inbox" screen within OHC. The business owner can reply to messages directly from the OHC Inbox, and the response is successfully delivered to the customer on the original platform. The UI clearly indicates the source platform of each message using its logo.

## Priority
P1

## Estimated Scope
Large
