# Integrate WhatsApp Business API for Unified DMs

## Problem Statement
Small business owners, especially in Latin America and India, handle the majority of their customer inquiries, orders, and support via WhatsApp. Currently, they have to constantly switch between their personal WhatsApp app and the OHC platform, leading to missed messages, slow response times, and disorganized order tracking. They need all their customer WhatsApp messages to flow directly into their unified OHC inbox.

## Research Report
**Tool Evaluated:** WhatsApp Business API (via Meta Graph API)
- **Ease of Use:** For the business owner, connecting WhatsApp requires a Facebook Business Manager account and phone number verification. Once connected, it's completely seamless. Messages appear in the OHC inbox like any other message.
- **Pricing:** Meta charges per conversation (24-hour window), typically $0.01 - $0.05 depending on the country and conversation type (marketing, utility, service). The first 1,000 service conversations per month are free.
- **Reputation:** It is the industry standard and absolute necessity for businesses outside the US. Reliability is high, though Meta's API updates can sometimes be breaking.
- **Deployment:** Works perfectly in Cloud mode. For Standalone mode, webhooks need to be routed through a stable public URL or relay service.

## Design Doc
- **Trigger:** Business owner navigates to "Integrations" and clicks "Connect WhatsApp". They go through an OAuth/Meta setup flow.
- **Action:** Incoming WhatsApp messages trigger a webhook to OHC, which parses the message and creates/updates a thread in the unified inbox. Outgoing replies from the OHC inbox trigger an API call to Meta to send the message to the customer's phone.
- **User View:** A simple "Connect WhatsApp" button. After connection, a new tab in the Inbox for "WhatsApp" where they can chat with customers exactly as they do with email or website live chat.

## Implementation Prompt
Implement a WhatsApp integration that allows users to connect their WhatsApp Business account. Once connected, all incoming WhatsApp messages must appear in the OHC unified inbox in real-time. When a user replies from the OHC inbox, the message must be delivered back to the customer on WhatsApp. Focus on handling text, images, and basic document attachments seamlessly.

## Priority
P0

## Estimated Scope
Large