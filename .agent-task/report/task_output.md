# OHC Tool Integration Research Report Q4

## Executive Summary
This report evaluates seven critical tool integrations designed to empower non-technical small business owners using the OHC platform. The tools were evaluated against the core OHC personas (Maya, Carlos, Priya, Leo, Fatima), with a strict focus on ease of use (the "grandmother test"), pricing accessibility, and compatibility with both Cloud and Standalone execution modes.

## Evaluated Categories & Candidates

### 1. Social Media Integration: Meta Graph API
*   **Problem**: Business owners struggle to manage customer inquiries scattered across Instagram DMs, Facebook comments, and WhatsApp.
*   **Solution**: Meta Graph API provides a unified gateway for the entire Meta ecosystem.
*   **User Experience**: A simple 1-click "Connect Meta" OAuth flow that unlocks a unified Inbox within OHC.
*   **Architecture Notes**: Requires a robust webhook receiver to parse payloads from different platforms. Cloud mode handles webhooks natively; Standalone mode will require an OHC relay service to bypass NAT/firewall issues for local instances.

### 2. Calendar & Scheduling: Cal.com
*   **Problem**: Service-based businesses waste time in back-and-forth scheduling and suffer from double-booking.
*   **Solution**: Cal.com, an API-first scheduling infrastructure.
*   **User Experience**: Business owners define availability in OHC; customers book via an embedded Cal.com widget on the business's public page.
*   **Architecture Notes**: OHC acts as an OAuth client. Leveraging Cal.com's "Atoms" (React components) allows seamless UI embedding without forcing users onto a third-party site.

### 3. Payment Processing (LATAM): Mercado Pago
*   **Problem**: Stripe is insufficient for Latin American markets where local payment methods (Pix, boletos) are required.
*   **Solution**: Mercado Pago Checkout Pro/Bricks.
*   **User Experience**: Merchants input credentials to enable local payment methods for their customers instantly.
*   **Architecture Notes**: Integration is straightforward via REST APIs. Webhooks will update order status in real-time. Standalone instances will need a mechanism to receive webhooks if not publicly accessible.

### 4. Shipping & Logistics: Shippo
*   **Problem**: E-commerce users struggle with calculating shipping rates and managing labels across multiple carriers.
*   **Solution**: Shippo multi-carrier API.
*   **User Experience**: A "Fulfill Order" button opens an embedded Shippo widget to compare rates, purchase postage, and print labels directly within OHC.
*   **Architecture Notes**: Utilizes Shippo's Shipping Elements for UI abstraction. Requires address validation pre-checks to ensure accurate quoting.

### 5. SMS & Notifications: Twilio
*   **Problem**: Critical notifications (order updates, reminders) are missed if only sent via email, especially for users relying primarily on mobile devices.
*   **Solution**: Twilio Programmable Messaging API.
*   **User Experience**: A simple toggle to "Enable SMS Notifications". OHC must abstract away the technical complexity of Twilio accounts and A2P 10DLC compliance.
*   **Architecture Notes**: Because SMS incurs direct hard costs, OHC must implement a usage billing/credit system to pass costs to tenants. Standalone mode should offer an "Advanced" setting to input personal Twilio credentials.

### 6. Email Marketing: Mailchimp
*   **Problem**: Keeping a customer list synced manually to send newsletters or marketing campaigns is tedious and error-prone.
*   **Solution**: Mailchimp Marketing API.
*   **User Experience**: Simple OAuth connection; users design campaigns in Mailchimp while OHC automatically syncs the contact list.
*   **Architecture Notes**: Standard OAuth flow. Requires a background job queue to sync customer additions/updates/deletions and webhook support for handling unsubscribe events.

### 7. Video Conferencing: Google Meet
*   **Problem**: Manually generating and sharing video links for online services is time-consuming.
*   **Solution**: Google Calendar API with Google Meet integration.
*   **User Experience**: Business owners connect their Google account. Booking an online service automatically generates and sends a Google Meet link.
*   **Architecture Notes**: OAuth integration with Calendar scopes. OHC backend needs to explicitly request `conferenceData` when creating the calendar event.

## Next Steps
1.  **Prioritization**: The Meta Graph API (Unified Inbox) and Shippo (Shipping) represent the highest immediate value add for our core personas (P1).
2.  **Implementation**: Detailed issue briefs have been generated and saved to `docs/research/` for the engineering team to begin technical design.
