# Title: Unified Social Media Inbox Integration

## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Switching between apps causes delayed responses, lost sales, and poor customer service. They need a single place to view and reply to all messages.

## Research Report
*   **Tool Candidates**: ManyChat, Meta Business Suite API, Twilio Conversations.
*   **Evaluation**: Twilio Conversations provides a robust API for WhatsApp and SMS but requires more setup for IG/FB. Meta Business Suite API is free and covers IG/FB directly. ManyChat is user-friendly but adds a subscription cost. Meta's official API is the most direct route, though the OAuth flow is complex.
*   **Ease of Use**: Once connected, the user never has to leave OHC. The initial setup requires logging into Meta.
*   **Pricing**: Meta APIs are mostly free for standard usage; WhatsApp Business has conversation-based pricing.
*   **Modes**: Cloud (OAuth redirects work well). Standalone (OAuth redirects need local handling or proxying).

## Design Doc
*   **Integration Trigger**: User connects their Meta/WhatsApp accounts via a "Connect Socials" button in OHC Settings.
*   **Action**: Webhooks receive incoming messages and route them to a unified "Inbox" view in the OHC app. Replies sent from OHC are pushed back to the respective platform.
*   **User Interface**: A chat-like interface displaying the source of the message (an icon for IG, WhatsApp, etc.).

## Implementation Prompt
Implement a unified inbox feature where users can connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages should appear in a single chronological feed. The user should be able to type a reply and have it sent back to the customer on the original platform. Acceptance criteria include successful account connection, receiving a message, and sending a reply.

## Priority
P1

## Estimated Scope
Large
