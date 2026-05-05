# Title: Social Media Integration via Meta Graph API

## Problem Statement
Maya, the home baker, receives numerous custom cake inquiries via Instagram DMs and Facebook comments while she sleeps. Because she cannot reply instantly, she loses potential customers to competitors. Unifying messages from Instagram, Facebook, and WhatsApp into a single inbox—and allowing her AI agent to automatically draft replies based on her catalog and availability—would save her hours of manual work and capture more sales.

## Research Report
The Meta Graph API provides comprehensive access to Instagram Direct Messages, Facebook Page comments/messages, and WhatsApp Business interactions.
- **Ease of Use for Non-Technical Users**: The user experience would be a simple "Connect Facebook/Instagram" OAuth button in the OHC UI. The complexity of webhooks and API limits is entirely hidden.
- **Pricing**: Basic messaging via Facebook and Instagram is free. WhatsApp Business uses conversation-based pricing (typically a few cents per 24-hour window), which is affordable for SMBs.
## Risks
- **Risks**: API changes, webhook delivery failures, and compliance with strict platform policies like the 24-hour messaging window.

## Reliability & Reputation**: Meta's APIs are the industry standard for these platforms, though they require strict compliance with the 24-hour standard messaging window and app review processes.
- **Environment Support**: Works seamlessly in Cloud via webhooks. For Standalone mode, a cloud relay or polling mechanism would be required to receive real-time webhook events.

## Design Doc
The "Customer Success" (The Ambassador) agent handles social media communication.
1. **Trigger**: A customer sends a DM on Instagram.
2. **Action**: The Meta webhook notifies OHC. The Customer Success agent reads the message, consults Maya's product list (e.g., vegan cake availability), and drafts a polite response.
3. **User View**: Maya opens her OHC app and sees a unified "Inbox" with the Instagram icon next to the message. She can see the AI's drafted reply and choose to auto-send it or edit it before sending.

## Implementation Prompt
Implement an OAuth connection flow for Meta platforms so users can link their Instagram and Facebook Business accounts. Create a "Unified Inbox" screen in the UI that lists incoming DMs and comments. Enable the Customer Success AI agent to read unread messages, generate contextual replies based on the user's business data, and allow the business owner to review and send those replies directly from the OHC app.

## Priority
P0

## Estimated Scope
Large
