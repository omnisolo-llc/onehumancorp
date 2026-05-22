# Integration: Twilio WhatsApp Business API

## Title
Enable Automated Customer Communication via Twilio WhatsApp Business API

## Problem Statement
Many small business owners (like Carlos, who runs a local bakery, or Fatima, who manages an online boutique) struggle to keep up with customer inquiries, appointment reminders, and order updates. Email open rates are low, and managing SMS is expensive and often doesn't support rich media. Customers increasingly prefer to communicate via WhatsApp, but business owners cannot be glued to their phones answering individual messages 24/7. They need a way to automate basic responses, send order updates automatically, and seamlessly take over the conversation when a human touch is needed, all without leaving their main dashboard.

## Research Report
**Findings & Market Need**:
- **Customer Preference**: WhatsApp is the dominant messaging app globally, with over 2 billion active users. Open rates for WhatsApp messages often exceed 90%, vastly outperforming email.
- **Competitor Ecosystems**: Shopify, Wix, and other SMB platforms have thriving app ecosystems for WhatsApp marketing and support (e.g., Loox, Wati). Customers expect immediate, conversational commerce.
- **SMB Pain Points**: Business owners report "message fatigue" from answering the same questions about hours, location, and order status.

**Tool Evaluation: Twilio WhatsApp Business API**:
- **Capabilities**: Provides programmatic access to send and receive WhatsApp messages, including text, media, and interactive templates. It supports webhooks for incoming messages, which is perfect for routing to OHC agents.
- **Ease of Use (for Non-Technical Users)**: By abstracting the API behind an OHC integration, a business owner only needs to connect their Twilio account and select pre-built automated flows (e.g., "Send order confirmation"). The technical complexity (OAuth, webhooks, templates) is completely hidden.
- **Pricing**: Twilio offers a pay-as-you-go model. Conversation-based pricing makes it highly accessible for small businesses, as they only pay for active interactions. There is a generous free tier for testing and small volumes.
- **Reputation**: Twilio is the industry leader in CPaaS (Communications Platform as a Service) with robust documentation, high reliability (99.99% uptime SLA), and excellent support for both Cloud and Standalone (local) operating modes.

## Design Doc
**High-Level Integration**:
- **Trigger**: The integration is triggered by specific events within OHC (e.g., an order status change, a new booking) or by incoming messages from customers via WhatsApp.
- **Actions**:
    - *Automated Outbound*: OHC agents can send templated notifications (e.g., "Your order is ready for pickup!").
    - *Inbound Routing*: Incoming messages hit an OHC webhook, which routes the message to a specific internal agent or directly to the business owner's dashboard if human intervention is requested.
- **User Experience**:
    - The business owner connects their Twilio account via a simple settings panel in the OHC UI.
    - They can view a "Unified Inbox" where WhatsApp messages appear alongside other communication channels.
    - They can configure simple rules (e.g., "If someone asks for hours, reply with this message").

## Implementation Prompt
**User-Facing Outcome**:
As a small business owner, I want to connect my WhatsApp Business account so that I can automatically send order updates to my customers and reply to their inquiries directly from my main dashboard.

**Acceptance Criteria**:
1. A user can connect their Twilio account by providing necessary credentials in an integration settings view.
2. The system can send automated WhatsApp template messages based on defined business events (e.g., order completion).
3. Incoming WhatsApp messages are displayed in a unified communications view within the dashboard.
4. Users can manually reply to incoming WhatsApp messages from the dashboard.
5. The integration functions correctly in both cloud-native and standalone modes.

## Priority
P1 (High)

## Estimated Scope
Medium
