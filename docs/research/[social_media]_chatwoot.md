# Chatwoot Unified Inbox Integration

## Problem Statement
Small business owners, especially those with low English proficiency like Fatima, struggle to manage customer communications across multiple platforms (WhatsApp, Facebook, Instagram, email). They miss messages, lose context, and find it overwhelming to switch between apps continuously. They need a single, simple unified inbox.

## Research Report
Chatwoot is an open-source omni-channel customer support system. It allows connecting multiple channels, including social media, email, and live chat, into one central dashboard.
- **Ease of Use**: Chatwoot provides a straightforward interface that feels like a modern messaging app. It is less complex than enterprise tools like Zendesk, making it suitable for small business owners.
- **Pricing**: Open-source and self-hostable for free. Cloud pricing starts around $19/agent/month for the lowest paid tier with social media channels.
- **Reputation**: Well-regarded in the open-source community as an alternative to Intercom/Zendesk.
- **Environment**: Perfect fit. Works seamlessly in Cloud (managed) and Standalone (self-hosted) modes since it's open-source.

## Design Doc
**Trigger**: Business owner navigates to "Inbox" tab and clicks "Connect Channels".
**Action**: User authorizes OHC to link their social media profiles or input webhook credentials. Chatwoot runs transparently in the background.
**User Experience**: A unified "Conversations" view inside the OHC dashboard. All incoming messages from WhatsApp, FB, IG, etc., appear here. The business owner replies here, and the message is routed back to the appropriate channel.

## Implementation Prompt
Integrate a unified inbox feature into the OHC dashboard. The business owner should see a single "Inbox" page where all customer messages from different platforms appear. They must be able to reply to any message from this single view, and the reply should be sent to the correct original platform. Provide a simple setup flow for connecting at least one social media channel (e.g., WhatsApp).

## Priority
P0

## Estimated Scope
Medium
