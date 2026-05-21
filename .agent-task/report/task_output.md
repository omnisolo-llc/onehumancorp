# Gorgias Integration

## Title
Implement Gorgias Unified Inbox Integration for Omnichannel Customer Support

## Problem Statement
Small business owners suffer from "Operational Fatigue" (ranked #2 in SMB Pain Points) and "Communication Lag" (ranked #8). They struggle to manage customer inquiries spread across email, Instagram DMs, Facebook Messenger, WhatsApp, and SMS. Constantly switching between apps leads to missed messages, lost sales, and extreme frustration. They need a single, unified inbox to manage all customer communications efficiently, allowing them to focus on running their business rather than acting as full-time customer support agents.

## Research Report
*   **Tool Evaluated:** Gorgias
*   **Market Position:** Gorgias is a leading helpdesk solution built specifically for ecommerce (highly popular on Shopify and Magento). It unifies email, live chat, voice, SMS, and social media messaging into one dashboard.
*   **Ease of Use for Non-Technical Users:** Gorgias is designed with ecommerce merchants in mind. The interface is intuitive, bringing all customer data (order history, tracking info) directly alongside the conversation. This context significantly reduces the need for the merchant to cross-reference multiple systems.
*   **Pricing:** Starts at $10/month (Starter plan, up to 50 billable tickets), scaling up based on ticket volume (Basic at $50/mo for 300 tickets). This is a manageable entry point for small businesses, though high volume can get expensive.
*   **Capabilities & Limits:**
    *   **API Quality & Docs:** Excellent REST API documentation.
    *   **OAuth & Webhooks:** Supports OAuth 2.0 and robust webhooks for real-time synchronization of customer data and order events.
    *   **SaaS Viability:** Cloud-native SaaS model. Can operate in a multi-tenant environment.
*   **Reputation:** Highly rated (4.5+ on Shopify App Store, Trustpilot) for its deep ecommerce integrations and time-saving automation features (macros, rules).
*   **Why Gorgias over competitors (Zendesk, Freshdesk)?** Gorgias is specifically tailored for ecommerce, meaning it naturally understands concepts like "orders," "tracking numbers," and "refunds" out-of-the-box, unlike generic ticketing systems that require heavy customization.

## Design Doc
*   **Integration Trigger:** The integration is activated via the OHC app marketplace/settings. The merchant authenticates Gorgias via OAuth.
*   **Data Sync:**
    *   OHC pushes customer data, order history, and product catalog to Gorgias.
    *   When an order is placed, updated, or cancelled in OHC, a webhook fires to update the customer profile in Gorgias.
*   **User Interface:**
    *   The merchant will see a unified "Inbox" tab within the OHC dashboard (embedded Gorgias view or a simplified native OHC view pulling via API).
    *   Incoming messages from any connected channel (Social, Email) appear in this inbox.
    *   The merchant can reply directly from OHC, and the response is routed through Gorgias back to the original channel.
    *   Automated responses (e.g., "Where is my order?") can be configured within OHC using Gorgias's macro engine, surfaced simply to the merchant.

## Implementation Prompt
Integrate Gorgias to provide a unified customer support inbox for merchants.
*   **Acceptance Criteria 1 (Authentication):** A non-technical merchant can connect their Gorgias account to their OHC store with a simple 1-click OAuth flow.
*   **Acceptance Criteria 2 (Contextual Inbox):** The merchant can view and respond to customer inquiries (from email and social channels connected to Gorgias) directly within the OHC dashboard.
*   **Acceptance Criteria 3 (Order Context):** When viewing a conversation, the merchant instantly sees the customer's recent OHC order history alongside the chat, without leaving the screen.
*   **Acceptance Criteria 4 (Sync):** Real-time synchronization of order status from OHC to Gorgias via webhooks, enabling automated "Where is my order?" responses.

## Priority
P1 (High) - Directly addresses top SMB pain points regarding communication and operational overhead.

## Estimated Scope
Medium
