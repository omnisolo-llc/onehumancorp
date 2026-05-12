# OHC Tool Integration: Meta Graph API Unified Inbox

## Title
Implement Meta Graph API for Unified Customer Inbox

## Problem Statement
Business owners are overwhelmed managing inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Important customer messages are missed, leading to lost sales and poor support experiences.

## Research Report
- **Tool Evaluated:** Meta Graph API
- **Why Meta?** It controls the three most important communication channels for small businesses globally (IG, FB, WA).
- **Ease of Use:** Requires a Facebook Login flow to authorize OHC to read/write messages. Once connected, all messages funnel into one place.
- **Pricing:** Mostly free for basic messaging; WhatsApp Business API has conversational costs.
- **Reputation:** Complex developer docs, but unparalleled reach.

## Design Doc
- **Trigger:** A customer sends a message to the business's connected Instagram, Facebook Page, or WhatsApp number.
- **Action:** Meta sends a webhook to OHC containing the message. OHC routes it to the business's unified inbox dashboard. When the owner replies, OHC uses the Graph API to send the message back to the native platform.
- **User View:** A single "Inbox" tab in the OHC dashboard where messages from all three platforms appear in unified threads.

## Implementation Prompt
Integrate the Meta Graph API to create a unified messaging inbox. Implement the OAuth flow for merchants to connect their Facebook/Instagram accounts. Create a webhook endpoint to receive incoming messages from these platforms and store them in the OHC database. Build a unified inbox interface in the dashboard where merchants can read and reply to messages, routing outgoing replies back through the correct Meta API channel.

## Priority
P0

## Estimated Scope
Large
