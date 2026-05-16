# Title: Integrate Manychat for Unified Social Media Inbox

## Problem Statement
Small business owners often miss important customer messages because they are spread across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Managing these separate platforms is overwhelming and leads to lost sales. Owners need a single place to view and respond to all customer interactions without switching between apps.

## Research Report
Manychat is a leading platform for social media messaging automation. It supports direct integrations with Instagram, Facebook Messenger, WhatsApp, and Telegram.
- **Ease of Use:** Highly intuitive visual builder for non-technical users to set up auto-replies, but for OHC's use case, we primarily want to sync messages into our unified inbox. The setup for connecting accounts is straightforward via standard OAuth.
- **Pricing:** Has a generous free tier (up to 1,000 contacts), with Pro plans starting at $15/month, making it very accessible for small business owners.
- **Reputation:** Highly trusted, official Meta Business Partner.
- **Competitors:** Chatfuel, MobileMonkey. Manychat offers the most seamless multi-platform support and stable webhooks.
- **Cloud vs Standalone:** Works well in Cloud mode via webhooks. In Standalone mode, users would need a public tunneling service (like ngrok) or polling mechanism to receive webhooks, which adds complexity.

## Design Doc
When a business owner connects their Manychat account via our settings page, OHC will automatically receive incoming messages from their connected social channels.
- **Trigger:** A new message arrives on Instagram/Facebook/WhatsApp.
- **Action:** Manychat forwards the message payload to OHC. OHC creates a new notification or unread message in the unified inbox.
- **User Interface:** The user sees a new tab or section in their OHC dashboard labeled "Social Messages" with indicators showing the source platform (e.g., an Instagram icon next to the DM). Replies sent from OHC are pushed back out through Manychat.

## Implementation Prompt
Create a "Social Inbox" feature that allows users to connect their Manychat account. Once connected, all incoming messages from Instagram, Facebook, and WhatsApp should appear in a unified conversation list within OHC. Users must be able to read and reply to these messages directly from our dashboard. Ensure the UI clearly distinguishes between different messaging platforms so the user knows where the conversation is happening.

## Priority
P1

## Estimated Scope
Medium