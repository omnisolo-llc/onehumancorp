# [communication] AI Unified Inbox & Auto-Receptionist

## Problem Statement
Service providers (like Carlos the handyman) lose leads because they are busy working and miss messages coming in across multiple fragmented channels (Instagram DMs, SMS, Email, WhatsApp). They cannot act as their own full-time receptionist.

## Research Report
Feedback from r/Entrepreneur and App Store reviews highlights that "managing messages" is a major pain point. While Shopify offers Inbox apps, they often require manual configuration and don't natively integrate deeply with personal channels like WhatsApp without expensive third-party tools.

## Design Doc
**Architecture & Key Relationships:**
*   **Omnichannel Ingest:** Webhooks/API integrations with Meta (IG/FB/WA), Twilio (SMS), and Email providers.
*   **AI Triage Engine:** Analyzes incoming messages for intent (FAQ, Booking Request, Support Issue).
*   **Auto-Responder:** Drafts and sends replies based on the OHC business knowledge base (hours, location, pricing).
*   **Unified Mobile UI:** A single chronological feed of all customer interactions.

**UX Flow:**
1.  Customer sends an Instagram DM: "Are you open tomorrow at 9?"
2.  OHC Agent intercepts the message, checks the business calendar.
3.  OHC Agent auto-replies: "Yes! Would you like me to book that slot for you?"
4.  The business owner sees the summarized interaction in their Unified Inbox without having to type the reply themselves.

## Implementation Prompt
Build the AI Unified Inbox. The system must aggregate messages from at least 3 distinct channels into a single mobile view. The Critical User Journey is an AI agent successfully answering a routine customer FAQ (like business hours) without the owner's intervention. Acceptance criteria: Messages from different channels appear in one thread per customer, and the AI can automatically resolve basic queries based on the business profile.

## Priority
P1

## Estimated Scope
Medium
