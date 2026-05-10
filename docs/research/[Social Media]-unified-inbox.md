# Social Media Integration: Unified Inbox

## Title
Connect Instagram, Facebook, WhatsApp, and TikTok to OHC Unified Inbox

## Problem Statement
Small business owners often miss customer inquiries and sales opportunities because they have to constantly switch between Instagram DMs, Facebook comments, WhatsApp Business, and TikTok. It is overwhelming to manage multiple apps on their phones, especially when dealing with high message volumes, leading to lost revenue and poor customer service.

## Research Report
- **Tools Evaluated:** Meta Graph API (Instagram/FB), WhatsApp Cloud API, TikTok Business API, Chatwoot (open source core), Twilio (for WhatsApp).
- **Ease of Use:** Meta's native business suite is clunky. Chatwoot provides a unified UI but requires technical setup. Integrating these directly into OHC via OAuth provides the best non-technical user experience (just click "Connect Meta").
- **Pricing:** Meta APIs are generally free for basic messaging; WhatsApp charges per conversation (~$0.01-$0.07). TikTok API is free but requires app approval.
- **Reputation:** Meta APIs are standard but can have complex approval processes. Twilio is reliable but adds cost.
- **Cloud vs Standalone:** Works in Cloud mode well. In Standalone, OAuth callbacks to localhost can be tricky but solvable using OHC Cloud as a relay or deep links.

## Design Doc
- **Trigger:** User navigates to Settings -> Integrations and clicks "Connect Social Media".
- **Action:** User completes an OAuth flow. OHC backend receives webhooks for new messages and routes them to the unified inbox.
- **User View:** A new "Unified Inbox" section appears in the OHC dashboard, displaying conversations from all connected platforms in a single thread list.

## Implementation Prompt
Implement a unified inbox interface that allows users to connect their social media accounts via OAuth. Once connected, new messages from these platforms should appear in a single view. Users should be able to reply directly from OHC, and the response should be sent back to the original platform. Ensure the setup process is a simple click-through flow without requiring API keys from the user.

## Priority
P0

## Estimated Scope
Large
