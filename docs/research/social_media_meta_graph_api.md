# Integrate Meta Graph API for Unified Social Inbox

## Problem Statement
Small business owners like Maya (the baker) receive orders and customer queries across Instagram DMs, Facebook Messenger, and WhatsApp. Checking multiple apps constantly is overwhelming, error-prone, and leads to missed sales. They need a single, unified place to see and reply to all customer messages.

## Research Report
- **Tool**: Meta Graph API (Instagram Messaging API, Messenger API, WhatsApp Business API)
- **Evaluation**: Meta's APIs are the official and most reliable way to integrate with their platforms. It allows third-party apps to read and send messages.
- **Ease of Use for Persona**: Non-technical users only need to click "Connect with Facebook/Instagram" and go through a standard OAuth consent screen. Once connected, it works invisibly.
- **Pricing**: Free for standard usage. WhatsApp Business API has conversation-based pricing, but the first 1,000 service conversations per month are free.
- **Reputation**: Official API, highly reliable.

## Design Doc
- **Integration Point**: "Customer Success" department.
- **Trigger**: User connects their Meta accounts via OAuth in the settings page.
- **Actions**:
  - Webhooks receive real-time incoming messages from IG/FB/WhatsApp.
  - Messages are stored in OHC's unified inbox database.
  - AI Agent (The Ambassador) drafts suggested replies based on business context.
- **User View**: A single "Inbox" tab on the mobile app and web dashboard showing threads from all connected platforms, with AI-drafted reply suggestions.

## Implementation Prompt
Add a "Connect Instagram & WhatsApp" button in the Settings menu. Implement the Meta OAuth flow to authorize messaging access. Create a unified "Inbox" UI that displays incoming messages from these platforms. Allow the user to send replies directly from the OHC Inbox, routing the message back to the correct platform.

## Priority
P0

## Estimated Scope
Large
