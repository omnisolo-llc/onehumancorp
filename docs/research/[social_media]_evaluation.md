# [Social Media Integration] WhatsApp Business API Evaluation

## Title
Unified Inbox via WhatsApp Business API

## Problem Statement
Small business owners manage customer communications across too many platforms. Missing a WhatsApp message can mean losing a sale. They need a unified inbox where WhatsApp messages appear alongside emails and SMS.

## Research Report
- **Strategy**: Direct integration with WhatsApp Cloud API.
- **Persona**: Retail stores, international merchants, service providers.
- **Advantages**: Integrates the default communication tool in many global markets into OHC, preventing dropped leads.
- **Risks**: Meta's business verification process can be tedious for merchants.
- **Pricing**: Conversation-based pricing (first 1,000 service conversations free).
- **Compatibility**:
  - **Cloud**: Managed via embedded signup.
  - **Standalone**: User provides their own Meta App credentials.

## Design Doc
- **Trigger**: Customer sends a message to the business's WhatsApp number.
- **Action**: Message is routed to OHC's Unified Inbox.
- **User Interface**: Business owner replies directly from OHC, and the message is sent back to the customer's WhatsApp.

## Implementation Prompt
Implement a WhatsApp channel integration for the Unified Inbox. Provide a setup wizard for the business owner to connect their WhatsApp Business account. Incoming messages should create a conversation thread in OHC, and replies from OHC should be routed back via the WhatsApp API.

## Priority
P1

## Estimated Scope
Large
