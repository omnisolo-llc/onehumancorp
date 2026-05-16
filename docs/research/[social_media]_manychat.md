# Title: Social Media Unified Inbox Integration

## Problem Statement
Small business owners, like boutique shops or local service providers, are overwhelmed managing customer inquiries across multiple platforms (Instagram, Facebook, WhatsApp). They lack a single place to view and respond to messages, leading to missed sales opportunities, delayed responses, and a poor customer experience. They need a simple, unified inbox.

## Research Report
**Tool Analyzed**: ManyChat
**Ease of Use**: Very high for non-technical users. It offers a visual drag-and-drop builder for basic automations and a unified dashboard for viewing conversations.
**Reputation**: Industry leader for Instagram and Messenger automation. Highly reliable.
**Pricing**: Free tier available (up to 1,000 contacts). Pro plan starts at $15/month, which is affordable for most small businesses.
**Environment**: Primarily Cloud-based, but could potentially be adapted for Standalone via localized webhooks if supported by the provider, although official support is Cloud-first.
**AI Integration**: Strong potential. Could integrate with OHC's AI agents to draft suggested responses or auto-reply to common questions (like opening hours) before the business owner steps in.

## Design Doc
**Integration Trigger**: The user links their Meta (Facebook/Instagram) or WhatsApp Business account from the OHC settings page.
**Actions Taken**:
- OHC listens for incoming messages via webhooks.
- New messages appear in a new "Inbox" tab within the OHC dashboard.
- The business owner can reply directly from the OHC dashboard, which routes the message back to the respective platform.
- Optional: AI Agent drafts a suggested reply for the owner to review before sending.
**User View**: A unified, chat-like interface in OHC that clearly labels the source of the message (e.g., an Instagram icon next to the user's avatar) and supports basic media (images/text).

## Implementation Prompt
Implement a unified inbox feature using ManyChat (or similar provider). The user should be able to connect their social accounts via a simple OAuth flow. Once connected, all incoming DMs and comments should appear in a new "Inbox" UI component in OHC. The user must be able to reply from this UI, and the reply should be delivered to the customer on the original platform. Ensure the UI clearly distinguishes between different platforms.

## Priority
P1

## Estimated Scope
Medium
