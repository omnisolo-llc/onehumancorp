# Social Media Integration: Unified Inbox via Chatwoot

## Title
Integrate Unified Customer Inbox (Social Media)

## Problem Statement
Small business owners, like bakers or handymen, receive customer inquiries through Instagram DMs, Facebook Messenger, WhatsApp, and SMS. Constantly switching apps leads to missed messages and lost sales. They need a single, simple place to see and reply to all customer messages.

## Research Report
- **Tool Evaluated:** Chatwoot
- **Ease of Use:** High. Offers a clean, unified dashboard.
- **Pricing:** Open source (free to self-host), Cloud pricing starts at $19/mo. Very affordable for small businesses.
- **Reputation:** Well-regarded open-source alternative to Intercom/Zendesk.
- **Cloud/Standalone Compatibility:** Excellent. Native support for Docker/local deployment (Standalone) and scalable API (Cloud).

## Design Doc
- **Integration Point:** A new "Inbox" icon in the OHC main navigation.
- **User Experience:** The business owner connects their Facebook/Instagram/WhatsApp accounts via a simple OAuth flow. Once connected, all new messages appear in a single chat interface within OHC. They can reply directly from OHC, and the message routes back to the correct social platform.
- **System Behavior:** OHC will manage the Chatwoot instance (or API connection) behind the scenes, abstracting away channel configuration.

## Implementation Prompt
Create a unified inbox UI within OHC that allows business owners to read and reply to messages from multiple social channels. The UI should resemble a simple chat app (like WhatsApp Web). Provide a settings page with "Connect" buttons for major social networks. Ensure the design adheres to OHC Glassmorphism standards and is fully usable on mobile devices.

## Priority
P1

## Estimated Scope
Large
