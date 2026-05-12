# Integrate Manychat for Unified Social Media Inbox

## Problem Statement
Small business owners often struggle to keep up with customer inquiries scattered across multiple platforms like Instagram DMs, Facebook Messenger, and WhatsApp. Missing a message means losing a potential sale. They need a single, unified inbox to view and respond to all social media messages without needing to switch between different apps throughout the day.

## Research Report
**Tool**: Manychat
Manychat is a leading platform for chat marketing and multi-channel messaging. It aggregates messages from Instagram, Facebook, WhatsApp, and SMS into a single dashboard.
- **Ease of use**: Excellent visual flow builder, very accessible for non-technical users.
- **Pricing**: Has a free tier (up to 1,000 contacts), Pro tier starts at $15/month, making it very affordable for small businesses.
- **Reputation**: Highly rated, widely used by e-commerce and local businesses.
- **Environment**: Works well in Cloud environments via API/Webhooks. For Standalone modes, it may require specific webhook tunneling or polling if direct callbacks aren't possible, but standard API integration is robust.

## Design Doc
The integration will connect the business owner's Manychat account to their OHC dashboard.
- **Trigger**: User navigates to the "Integrations" page and clicks "Connect Manychat". They will go through an OAuth flow to authorize OHC to access their Manychat account.
- **Actions**: OHC will receive incoming messages via Manychat webhooks and display them in a "Unified Inbox" widget on the OHC dashboard. When the user replies from the OHC dashboard, OHC will send the reply back through the Manychat API.
- **User View**: A simple inbox interface showing recent conversations, with the ability to read and reply directly.

## Implementation Prompt
Create a "Unified Inbox" feature that allows users to connect their Manychat account. Once connected, display a stream of recent messages from their social channels (Instagram, Facebook, WhatsApp). The user should be able to click on a message thread, read the history, and type a reply that gets sent back to the customer via the original platform. The interface should be intuitive, similar to a standard email or SMS app, with clear indicators of which platform the message originated from.

## Priority
P1

## Estimated Scope
Medium
