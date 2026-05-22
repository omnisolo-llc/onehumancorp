# 🔍 Scout: Tool Integration Research - Twilio for WhatsApp

## Title
Integrate Twilio for WhatsApp Business to Enable Conversational Commerce and Automated Support

## Problem Statement
Small business owners (like Carlos the fitness coach or Priya the salon owner) increasingly interact with their customers where they spend the most time: WhatsApp. However, managing customer inquiries, booking confirmations, and sales directly from a personal or unmanaged WhatsApp account creates massive friction. It lacks shared team visibility, makes automated follow-ups impossible, and leaves a fragmented customer history. They need a way to seamlessly connect their business WhatsApp presence to the OHC platform so that customer conversations, appointments, and support queries are centralized, enabling both automated AI responses and human handover.

## Research Report
*   **Tool:** Twilio API for WhatsApp Business
*   **Market Position:** Twilio is the industry leader in programmable communications, providing the most robust API gateway for the WhatsApp Business API.
*   **Capabilities & Limits:**
    *   **Conversational API:** Enables sending and receiving messages, rich media, and interactive templates (buttons, lists).
    *   **Automation:** Perfect for AI-driven conversational commerce, abandoned cart reminders, and instant booking confirmations.
    *   **Limitations:** WhatsApp requires pre-approved message templates for outbound notifications outside of the 24-hour customer service window, which adds a layer of compliance overhead.
*   **SaaS Viability & Pricing:**
    *   **Pricing Model:** Pay-as-you-go per conversation (inbound/outbound) and per message for Twilio's infrastructure. It is highly scalable for SMBs, starting with a negligible cost for low volume.
    *   **Modes:** Perfect for Cloud (multi-tenant) where OHC brokers the Twilio connection, but also viable for Standalone modes if the user provides their own Twilio credentials.
*   **Reputation & Ease of Use:** While the API is highly technical, integrating it natively into OHC abstracts away the complexity. The end-user (SMB owner) simply authenticates their WhatsApp Business account and immediately gets a unified inbox.

## Design Doc
*   **Trigger:**
    *   *Inbound:* A customer sends a message to the business's WhatsApp number.
    *   *Outbound:* A business event occurs in OHC (e.g., an appointment is booked, or an order is shipped).
*   **Action:**
    *   *Inbound:* Twilio triggers a webhook to OHC, pushing the message into the Omnichannel AI Inbox. OHC's AI can automatically respond or flag for human review.
    *   *Outbound:* OHC dispatches a pre-approved template message via the Twilio API to the customer's WhatsApp.
*   **User Experience (OHC Dashboard):**
    *   A "Channels" settings page where the merchant connects their WhatsApp Business profile via Twilio.
    *   A "Unified Inbox" where WhatsApp messages appear alongside email and SMS, allowing the business owner to reply directly from OHC.
    *   Automated workflows (e.g., "Send WhatsApp confirmation when booked") that the owner can toggle on/off.

## Implementation Prompt
Implement a Twilio for WhatsApp integration that connects to the OHC Omnichannel Inbox and supports automated event-driven notifications.
*   **Acceptance Criteria 1 (Connection):** A merchant can configure their Twilio credentials and link their WhatsApp Business sender profile within the OHC settings.
*   **Acceptance Criteria 2 (Inbound Sync):** Incoming WhatsApp messages are received via webhook and displayed in the OHC unified inbox in real-time.
*   **Acceptance Criteria 3 (Outbound Replies):** Merchants can reply to WhatsApp messages directly from the OHC inbox, utilizing the 24-hour conversational window.
*   **Acceptance Criteria 4 (Automated Notifications):** The system can trigger pre-approved WhatsApp template messages for core platform events (e.g., order confirmations, appointment reminders).

## Priority
P0 (Critical)

## Estimated Scope
Large
