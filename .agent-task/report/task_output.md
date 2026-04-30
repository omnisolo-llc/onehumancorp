# OHC Tool Integration Research Report

This document outlines the evaluation and issue briefs for tools across 7 key categories to expand the capabilities of OneHumanCorp (OHC) for small business owners.

## 1. Social Media Integration: Meta Graph API (Instagram & Facebook)

### Problem Statement
Business owners like Maya receive customer inquiries through Instagram DMs and Facebook comments but struggle to manage them alongside emails and web chats. Checking multiple apps is time-consuming and leads to missed opportunities. They need a unified inbox that brings all customer communications into one place.

### Research Report
**Tool Analyzed**: Meta Graph API (Messenger API for Instagram / Facebook Pages API)
**Ease of Use (for non-tech users)**: Users authenticate once via standard OAuth. No technical setup is required. The complexity lies in our implementation.
**Capabilities**:
- Read/respond to Instagram DMs and Facebook comments.
- Supports rich text and attachments.
**Pricing**: Free for the standard API tier (subject to rate limits which are generally sufficient for small businesses).
**Cloud vs Standalone**: Works well in Cloud. For Standalone, we may need a proxy service to handle webhooks and route them locally or utilize polling if webhooks aren't feasible for private environments.
**Why Meta Graph API?**: Meta owns Instagram and Facebook, which are the dominant platforms for small business social selling. Direct integration is essential.

### Design Doc
**Integration flow**:
- **Trigger**: User navigates to the "Integrations" page and clicks "Connect Instagram/Facebook".
- **Action**: System initiates Meta OAuth flow. Upon success, OHC subscribes to relevant webhooks (messages, comments) for the connected page/account.
- **Data flow**: Incoming webhooks hit OHC Backend -> stored in DB -> emitted to Unified Inbox UI via WebSocket. KAIROS Orchestrator (Customer Success Agent) can be triggered to draft responses.
- **User Interface**: The unified inbox displays messages with a small icon indicating the source (IG/FB). Replies sent from OHC are routed back through the Graph API.

### Implementation Prompt
Implement the Meta Graph API integration. Add an OAuth connection flow in the settings. Create a webhook endpoint to receive incoming Instagram DMs and Facebook comments, and display them in the existing unified customer inbox. Ensure the Customer Success Agent can read these messages and draft replies. The user must be able to reply directly from the OHC dashboard.

**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling: Cal.com

### Problem Statement
Service providers like Leo need a way for clients to book appointments based on their real-time availability without endless back-and-forth emails.

### Research Report
**Tool Analyzed**: Cal.com (formerly Calendso)
**Ease of Use (for non-tech users)**: Very high. The user connects their Google/Outlook calendar and defines working hours. The booking page is auto-generated.
**Capabilities**:
- Open-source, API-first scheduling infrastructure.
- Handles timezone conversions automatically.
- Excellent support for round-robin, collective, and recurring bookings.
**Pricing**: Generous free tier for individuals. Open-source core allows self-hosting (useful for Standalone).
**Cloud vs Standalone**: Excellent for both. They offer managed cloud APIs and self-hosted options which align perfectly with OHC's dual architecture.
**Why Cal.com?**: More developer-friendly than Calendly, open-source, and allows deep white-label integration so the user never has to leave the OHC interface.

### Design Doc
**Integration flow**:
- **Trigger**: User sets up a "Service" product type.
- **Action**: OHC provisions a Cal.com booking link via API (or internal implementation) linked to the user's connected calendar.
- **User Interface**: A seamless booking widget embedded on the user's public OHC storefront. The dashboard shows upcoming bookings.
- **Agent Integration**: The Operations Agent monitors new bookings to trigger confirmation emails and calendar invites.

### Implementation Prompt
Integrate Cal.com scheduling. Add a feature to allow users to connect their external calendars (Google/Outlook). When creating a "Service" listing, generate a white-labeled Cal.com booking flow. Embed the booking widget on the public storefront. Ensure new bookings are synced to the OHC database so the Operations Agent can send confirmations.

**Priority**: P1
**Estimated Scope**: Medium

---

## 3. Email Marketing: Resend

### Problem Statement
Business owners like Priya need to notify their customer base about new products or sales without learning complex marketing platforms like Mailchimp.

### Research Report
**Tool Analyzed**: Resend
**Ease of Use (for non-tech users)**: Totally invisible to the user. They just type an email or have the AI draft it, and it sends reliably.
**Capabilities**:
- Developer-first API for transactional and marketing emails.
- Excellent deliverability and bounce handling.
- React Email integration for beautiful templates.
**Pricing**: 3,000 free emails/month. Very affordable paid tiers.
**Cloud vs Standalone**: Cloud-native. For Standalone, users might need to provide their own API key, or OHC acts as a relay.
**Why Resend?**: Superior developer experience, speed, and modern template support compared to legacy providers like SendGrid or Mailgun.

### Design Doc
**Integration flow**:
- **Trigger**: Marketing Agent drafts an email campaign or Operations Agent sends a receipt.
- **Action**: OHC backend formats the email using predefined React Email templates and sends it via the Resend API.
- **User Interface**: Marketing dashboard shows campaign status (Sent, Delivered, Opened, Clicked) based on Resend webhooks.
- **Agent Integration**: Marketing Agent evaluates open rates to suggest future campaign timing.

### Implementation Prompt
Integrate Resend for outbound email delivery. Replace any existing basic SMTP setup with the Resend API. Implement webhook listeners to track delivery, bounces, and open rates. Create a simple UI in the Marketing department for the user to view campaign performance.

**Priority**: P1
**Estimated Scope**: Small

---

## 4. Payment Processing (Alternative): Mercado Pago

### Problem Statement
While Stripe is great, it isn't available everywhere. Business owners in LATAM need a reliable, localized payment processor that their customers trust.

### Research Report
**Tool Analyzed**: Mercado Pago
**Ease of Use (for non-tech users)**: Standard OAuth connection. Familiar to users in the region.
**Capabilities**:
- Dominant payment gateway in Latin America.
- Supports local payment methods (Pix in Brazil, OXXO in Mexico).
- Checkout Pro (hosted) and Custom Checkout (API).
**Pricing**: Variable by country, typically a percentage + fixed fee per transaction.
**Cloud vs Standalone**: Works via standard REST APIs, compatible with both.
**Why Mercado Pago?**: Essential for expanding OHC into the massive LATAM small business market where Stripe penetration is lower.

### Design Doc
**Integration flow**:
- **Trigger**: User in a supported LATAM country selects Mercado Pago in "Payment Settings".
- **Action**: OHC initiates OAuth flow. During checkout, OHC routes the payment intent to Mercado Pago instead of Stripe.
- **User Interface**: Checkout page dynamically offers Mercado Pago (and local methods like Pix) if configured.
- **Agent Integration**: Finance Agent tracks Mercado Pago settlements alongside Stripe payouts.

### Implementation Prompt
Add Mercado Pago as an alternative payment gateway to Stripe. Implement the OAuth connection flow. Update the checkout process to dynamically route payments to Mercado Pago when configured. Ensure the webhook handler processes Mercado Pago payment success/failure events and updates order status accordingly.

**Priority**: P2
**Estimated Scope**: Medium

---

## 5. Shipping & Logistics: Shippo

### Problem Statement
Product sellers need to calculate shipping costs accurately at checkout and print shipping labels without manually copying addresses into carrier websites.

### Research Report
**Tool Analyzed**: Shippo
**Ease of Use (for non-tech users)**: Seamless. Users just enter package dimensions and weight; the system handles the rest.
**Capabilities**:
- Multi-carrier API (USPS, UPS, FedEx, DHL, etc.).
- Real-time rate calculation.
- Label generation and tracking webhooks.
**Pricing**: Pay-as-you-go per label, plus carrier fees.
**Cloud vs Standalone**: Cloud API, works well for both architectures.
**Why Shippo?**: Unifies dozens of carriers behind a single API, abstracting massive complexity away from both OHC developers and users.

### Design Doc
**Integration flow**:
- **Trigger**: Customer enters shipping address at checkout; User clicks "Fulfill Order" in dashboard.
- **Action**: OHC queries Shippo for rates during checkout. Upon fulfillment, OHC calls Shippo to generate a printable PDF label.
- **User Interface**: Order detail page shows a "Print Label" button. Tracking number is auto-saved.
- **Agent Integration**: Customer Success Agent listens to Shippo tracking webhooks and emails the customer when the package is "Out for Delivery".

### Implementation Prompt
Integrate the Shippo API for order fulfillment. Add fields for product weight and dimensions. During checkout, fetch real-time shipping rates based on the customer's address. Add a "Print Shipping Label" button to the order details page that generates a label via Shippo. Listen for tracking updates and update the order status.

**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications: Twilio

### Problem Statement
Users like Fatima (food cart) need immediate notifications when an order arrives, and her customers need SMS updates because they might not check email while waiting for food.

### Research Report
**Tool Analyzed**: Twilio Programmable SMS
**Ease of Use (for non-tech users)**: Invisible. The business owner just provides their phone number.
**Capabilities**:
- Global SMS delivery.
- Reliable API with extensive documentation.
**Pricing**: Pay-per-message (fractions of a cent depending on destination).
**Cloud vs Standalone**: Cloud API.
**Why Twilio?**: The industry standard for programmatic SMS. Extremely reliable and scalable.

### Design Doc
**Integration flow**:
- **Trigger**: Urgent events (new food order) or customer updates (order ready for pickup).
- **Action**: OHC backend sends a request to Twilio API to dispatch an SMS to the configured number.
- **User Interface**: Notification settings allow the owner to toggle "SMS Alerts for New Orders". Customers see an option for "Send me text updates" at checkout.
- **Agent Integration**: Operations Agent triggers SMS dispatch based on order state changes.

### Implementation Prompt
Integrate Twilio for SMS notifications. Add a setting for business owners to receive SMS alerts for new orders. Add an opt-in checkbox at checkout for customers to receive SMS order updates. Implement the backend logic to dispatch these SMS messages via Twilio when order statuses change (e.g., "Order Received", "Ready for Pickup").

**Priority**: P1
**Estimated Scope**: Medium

---

## 7. Video Conferencing: Zoom API

### Problem Statement
Tutors and consultants like Leo need unique video call links automatically generated and attached to calendar invites when a client books a session.

### Research Report
**Tool Analyzed**: Zoom Meeting API
**Ease of Use (for non-tech users)**: User authenticates via Zoom OAuth once. Links are generated silently.
**Capabilities**:
- Auto-generate unique meeting URLs.
- Configure meeting settings (waiting room, mute on entry).
**Pricing**: Free tier API access (meetings limited to 40 mins for free users).
**Cloud vs Standalone**: Standard OAuth API.
**Why Zoom?**: Most widely recognized video conferencing tool. Most clients already have it installed.

### Design Doc
**Integration flow**:
- **Trigger**: A "Service" booking is confirmed.
- **Action**: OHC calls Zoom API (via user's OAuth token) to create a meeting scheduled for the booking time.
- **User Interface**: The booking confirmation page and email display the "Join Zoom Meeting" button.
- **Agent Integration**: Operations Agent attaches the Zoom link to the calendar invite and sends a reminder 15 minutes before the start time.

### Implementation Prompt
Integrate the Zoom API. Add an OAuth connection for Zoom in the user settings. When a new service booking is created, automatically generate a unique Zoom meeting link. Display this link in the booking details on the dashboard and include it in the confirmation emails sent to the customer.

**Priority**: P2
**Estimated Scope**: Medium
