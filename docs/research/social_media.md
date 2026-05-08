# [Social Media] Unified Inbox with Meta Graph API

## Problem Statement
Small business owners, especially non-technical ones, struggle to manage customer inquiries across multiple platforms (Instagram DMs, WhatsApp Business, Facebook Messenger). Constantly switching between apps leads to missed messages, slow response times, and ultimately lost sales.

## Research Report
**Tool Evaluated:** Meta Graph API (WhatsApp & Instagram)

*   **Ease of Use:** For the end-user, it would be seamless once connected. The setup process, however, requires navigating Meta's OAuth and business verification, which can be complex for non-technical users.
*   **Pricing:** WhatsApp Business API uses conversation-based or message-based pricing. There is typically a free tier (e.g., first 1000 conversations/month free), making it accessible for very small businesses. Instagram and Messenger are generally free to receive/send messages via API.
*   **Reputation:** Meta is the industry standard for these channels.

## Design Doc
**Trigger:** Customer sends a message on Instagram, WhatsApp, or Facebook.
**Action:** OHC receives the message and displays it in a unified inbox.
**User Sees:** A single dashboard in OHC where they can view and reply to all incoming messages, regardless of the source platform. They should be able to connect their accounts via a simple OAuth flow.

## Implementation Prompt
Implement a unified inbox feature that connects to the Meta Graph API. The user should be able to click a "Connect WhatsApp/Instagram" button, go through the Meta authorization flow, and then see incoming messages in an OHC interface. They should be able to reply directly from OHC. Ensure the setup is as frictionless as possible.

## Priority
P1

## Estimated Scope
Large

## Mode Compatibility
*   **Cloud:** Fully supported. Webhooks from Meta can directly hit the OHC cloud endpoints.
*   **Standalone:** Requires architectural consideration. Since a local instance cannot easily receive webhooks from the internet without exposing a port or using a tunnel (like ngrok), a relay service or polling mechanism might be necessary for standalone mode to function seamlessly.
