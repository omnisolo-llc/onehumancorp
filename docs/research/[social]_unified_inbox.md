# Social Media Unified Inbox Integration

## Title
Integrate Meta Graph API for Unified Social Inbox

## Problem Statement
Small business owners like Maya (The Home Baker) manage customer inquiries across multiple platforms: Instagram DMs, Facebook Messenger, and WhatsApp. Constantly switching between apps leads to missed messages, slow response times, and lost sales. They need a single, unified inbox within the OHC platform to view and respond to all customer interactions.

## Research Report
- **Tool Evaluated**: Meta Graph API (Instagram Messaging, Facebook Messenger, WhatsApp Business API).
- **Benefits for OHC Users**: Centralizes communication, allowing the Customer Success agent ("The Ambassador") to draft responses or automatically reply to common questions (e.g., "do you do vegan cakes?").
- **Ease of Use**: Transparent to the user once connected. OHC handles the API complexity. Connecting requires a standard OAuth flow (e.g., "Log in with Facebook").
- **Pricing**: WhatsApp Business API has conversation-based pricing. Messenger and Instagram messaging are generally free for standard interactions.
- **Reputation**: Industry standard for Meta platforms. Essential for reaching a broad customer base.
- **Cloud vs. Standalone**: Works well in Cloud mode via webhooks. For Standalone, requires polling or a cloud relay to handle incoming webhooks.

## Design Doc
- **User Experience**: The user navigates to the "Customer Inbox" and clicks "Connect Instagram/Facebook". After OAuth, messages from these platforms appear in the unified inbox.
- **Integration**: The OHC backend receives webhooks from Meta for incoming messages. These are processed and stored in the unified inbox. The AI agent (Customer Success) can monitor this inbox, draft replies, or auto-respond based on user settings. Outgoing messages are sent via the Meta Graph API.
- **Triggers**: Incoming webhook from Meta.
- **Actions**: Display message in OHC UI, trigger AI agent for draft response, send owner a push notification.

## Implementation Prompt
Integrate the Meta Graph API to pull in messages from Instagram, Facebook Messenger, and WhatsApp into a unified inbox within the OHC interface. The user should be able to authenticate their social accounts easily. Incoming messages should be displayed in real-time, and the user (or the AI agent) should be able to reply directly from the OHC platform. Acceptance criteria include a working OAuth connection flow, receiving incoming messages, sending outgoing replies, and displaying a unified conversation history.

## Priority
P0

## Estimated Scope
Large
