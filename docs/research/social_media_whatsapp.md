# [Social Media] WhatsApp Business API Integration

## Title
Native WhatsApp Business API Integration for Automated Customer Conversations

## Problem Statement
Fatima (Food Cart Operator) and many other SMB owners rely on WhatsApp as their primary communication channel. They manually respond to every "Are you open?" or "Where is my order?" message. They need these messages to flow into OHC so an AI agent can handle them automatically, saving them hours of manual typing and ensuring no customer is left waiting.

## Research Report
- **Strategy**: Direct integration with WhatsApp Business Platform (Meta).
- **Target Persona**: Fatima (Food Cart Operator), Maya (Home Baker).
- **Advantages**: WhatsApp is the #1 messaging app for SMBs globally. Native integration ensures no third-party markups and deep control over the AI response flow.
- **Risks**: Meta's business verification can be tedious. 24-hour customer service window requirements must be managed by the AI to maintain "Service" conversation status.
- **Pricing**: Conversation-based pricing. First 1,000 service conversations per month are free. Meta charges per 24-hour window thereafter.
- **Ease of Use**: Once connected, it is invisible. The user just sees messages in their OHC inbox.
- **Compatibility**: Cloud (Webhooks). Standalone (Requires a cloud proxy for webhooks).

## Design Doc
- **Integration with OHC**:
    - User connects their WhatsApp Business Account in the "Operations" settings.
    - OHC registers a webhook to receive incoming messages.
    - The "Ambassador" AI agent analyzes the message and drafts/sends a response based on the business profile.
    - All conversations are surfaced in the OHC unified "Customer Inbox" screen.
- **User View**: A unified thread showing WhatsApp messages alongside other channels, with AI-drafted replies ready for approval or auto-send.

## Implementation Prompt
Build a native integration for the WhatsApp Business API. Handle incoming message webhooks and implement outbound message sending. Ensure the "Ambassador" AI agent can participate in WhatsApp threads by drafting and sending replies. Normalize WhatsApp message formats into the OHC unified inbox schema.

## Priority
P0

## Estimated Scope
Large
