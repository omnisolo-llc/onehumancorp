# WhatsApp Business API Integration for Unified Inbox

## Title
Connect WhatsApp Business to Unified Inbox

## Problem Statement
Many small business owners, especially those with international customers or in regions where WhatsApp is the primary communication channel, spend hours jumping between their personal phone, WhatsApp Business app, and email. This fragmentation leads to missed messages, slow response times, and lost sales. A unified inbox would allow owners to see and reply to WhatsApp messages in the same place they handle emails and website chats, saving time and reducing stress.

## Research Report
The WhatsApp Business API (via Meta Cloud API) allows businesses to programmatically send and receive messages. Based on recent research, Meta has transitioned to a conversation-based pricing model, and starting in July 2025, they are moving to a per-message pricing model to simplify the system. The platform offers features like CRM integration and chatbot functionality through Business Solution Providers (BSPs).

For a non-technical user, setting up the API directly is complex, often requiring a BSP (like Gupshup or Sendblue) to handle the technical overhead. However, the value is immense: it provides a centralized location for customer interactions. Pricing varies by region and message type (marketing, utility, authentication), with free tiers available for utility templates. While the setup complexity is a risk, the benefit of reaching customers where they are most active makes it a highly requested feature. This tool is fully compatible with both Cloud and Standalone modes if properly routed through OHC's backend.

## Design Doc
When a business owner connects their WhatsApp Business account via the OHC dashboard, they will go through an OAuth-style flow with Meta. Once connected, incoming WhatsApp messages will appear as new threads in the OHC Unified Inbox alongside emails and web chats. The business owner can type a reply in OHC, and it will be sent back to the customer's WhatsApp seamlessly.

The integration will involve webhook receivers on the OHC backend to listen for incoming messages and an outbound messaging service to push replies via the Meta Cloud API. For the Standalone app, local webhooks or long-polling strategies might be required, but the user experience will remain identical: a single, clean inbox for all communication.

## Implementation Prompt
Implement a connection flow in the settings menu allowing users to link their WhatsApp Business account. Once linked, seamlessly display incoming WhatsApp messages in the Unified Inbox with a distinct WhatsApp icon. Ensure that replies sent from the inbox are delivered back to the customer via WhatsApp. Provide clear error messages if a message fails to send (e.g., outside the 24-hour customer service window).

## Priority
P1

## Estimated Scope
Medium
