# Integrate WhatsApp for Unified Customer Messaging

## Problem Statement
Small business owners often find themselves juggling multiple communication channels, leading to missed inquiries and delayed responses. A significant portion of customer communication, especially internationally, happens over WhatsApp. Managing a separate device or tab for WhatsApp Business alongside other tools creates friction and reduces efficiency. Business owners need a unified inbox where they can see and respond to all customer messages, including those from WhatsApp, without leaving the One Human Corp (OHC) platform.

## Research Report
**Findings & Data**: WhatsApp is one of the most popular instant messaging applications globally, making it a critical channel for customer engagement. The WhatsApp Business API allows businesses to integrate messaging capabilities into their own platforms.
**Ease of Use**: For the end-user (business owner), the integration should be seamless. Once connected, messages appear in their unified inbox just like any other communication. The initial setup requires OAuth/QR code scanning, which is a familiar pattern for WhatsApp Web users.
**Pricing**: The WhatsApp Business API utilizes a conversation-based pricing model. The first 1,000 service conversations each month are generally free, which is very attractive for small businesses. Beyond that, rates vary by region and conversation category (utility, marketing, service).
**Reputation**: Highly reliable and trusted globally. The webhook reliability is robust, ensuring messages are delivered promptly.

## Design Doc
**Integration flow**:
1.  **Connection**: The business owner navigates to the OHC Integrations page and selects "Connect WhatsApp". They are guided through an OAuth flow or a QR code scan (similar to WhatsApp Web linking) to authenticate and link their WhatsApp Business account.
2.  **Ingestion**: OHC sets up webhooks with the WhatsApp Business API. Incoming messages to the business's WhatsApp number are routed to the OHC unified inbox.
3.  **Interaction**: The business owner reads and replies to these messages directly within the OHC inbox UI. Replies are sent back through the WhatsApp API to the customer.
4.  **Notifications**: Standard OHC notifications trigger for new incoming WhatsApp messages.

## Implementation Prompt
**User-Facing Outcome**: The user can connect their WhatsApp Business account via the Integrations page. Once connected, incoming WhatsApp messages will appear in the main OHC unified inbox. The user can click on a message, type a reply, and send it, and the customer will receive the reply on their WhatsApp.
**Acceptance Criteria**:
- A "Connect WhatsApp" option is available in the integrations UI.
- Users can successfully authenticate and link their account.
- Incoming WhatsApp text messages appear in the OHC inbox in near real-time.
- Replies sent from the OHC inbox are successfully delivered to the customer's WhatsApp.
- The integration supports both Cloud and Standalone OHC modes (Cloud utilizes central webhook endpoints, Standalone may require localized webhook proxies or polling depending on the API's constraints).

## Priority
P1

## Estimated Scope
Medium
