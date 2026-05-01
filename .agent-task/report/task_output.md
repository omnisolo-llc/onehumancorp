# Integration Research Report: External Tooling for Small Businesses

## Overview
This report summarizes the research and evaluation of external tool integrations designed to solve real-world problems for OneHumanCorp (OHC) small business owners. The goal of these integrations is to expand OHC's capabilities while maintaining the platform's core promise of radical simplicity for non-technical users.

---

# Social Media Unified Inbox Integration

## Title
Implement Unified Inbox using Meta Graph API for Instagram, WhatsApp, and Facebook

## Problem Statement
Small business owners (like Maya the Baker or Priya the Boutique Owner) are overwhelmed by messages across multiple platforms. They miss customer inquiries on Instagram DMs, WhatsApp, and Facebook Messenger because they have to constantly switch between apps. They need a single, unified inbox within the OHC platform to view and respond to all customer messages, and for the Customer Success AI agent to automatically draft replies or answer common questions (like "do you do vegan cakes?") while they sleep.

## Research Report
- **Tool**: Meta Graph API (Messenger API for Instagram, WhatsApp Business API, Facebook Messenger API)
- **Ease of Use for Non-Technical Users**: The end-user experience will be seamless (a single inbox in the OHC app). However, the initial setup requires connecting their Meta accounts via OAuth, which can be a friction point. We need to streamline the OAuth flow.
- **Pricing**: Facebook/Instagram messaging is generally free. WhatsApp Business API charges per conversation (first 1,000 service conversations per month are free).
- **Reputation**: Industry standard, but the API can be complex and requires strict adherence to Meta's policies (e.g., 24-hour messaging window).
- **Cloud/Standalone**: Works seamlessly in Cloud mode. In Standalone mode, requires user to supply their own Meta Developer credentials, which is complex for non-technical users. We should prioritize Cloud mode first.

## Design Doc
- **Integration Point**: OHC Unified Inbox UI and Customer Success AI Agent.
- **Trigger**: New message received via Meta webhooks.
- **Actions**:
  1. Receive incoming message payload via Meta Webhook.
  2. Normalize the message into an internal OHC `InboxMessage` format.
  3. Store in the OHC database under the user's `tenant_id`.
  4. Trigger the Customer Success AI Agent to generate a draft reply.
  5. Push the message and draft reply to the user's UI via WebSocket.
  6. When the user (or AI) sends a reply, route it back through the Meta Graph API to the original platform.
- **User Interface**: A single "Inbox" tab in the OHC app displaying conversations from all platforms, with platform icons indicating the source.

## Implementation Prompt
Implement the Meta Graph API integration to pull messages from Instagram DMs, WhatsApp, and Facebook Messenger into a unified OHC Inbox. The integration must handle Meta's OAuth flow for user onboarding, process incoming webhooks for new messages, store them securely with row-level security per tenant, and allow sending replies back to the respective platforms. The Customer Success AI agent must be able to read these messages and generate draft replies. The feature must be fully functional on mobile devices, displaying a clean, intuitive inbox without any technical jargon.

## Priority
P0 (Critical)

## Estimated Scope
Large

---

# Calendar & Scheduling Integration

## Title
Implement Booking and Scheduling using Cal.com API

## Problem Statement
Service-based small business owners (like Carlos the Handyman or Leo the Music Tutor) need a simple way for customers to book their time. They currently lose customers to back-and-forth emails or texts trying to find a time that works. They need a public booking page that automatically syncs with their personal calendar (like Google Calendar) so they don't get double-booked, and they need it to automatically generate video conferencing links or location details.

## Research Report
- **Tool**: Cal.com API (Open-source alternative to Calendly)
- **Ease of Use for Non-Technical Users**: Excellent. The business owner connects their Google/Outlook calendar once, sets their working hours (e.g., 9 AM - 5 PM), and the platform handles the rest. Customers see a simple calendar UI to pick a slot.
- **Pricing**: Cal.com has a generous free tier for individuals. Their API has a startup/platform tier that is very reasonable. Being open-source, it also offers self-hosting flexibility if needed in the future.
- **Reputation**: Highly respected in the developer community, modern API, built for integrations.
- **Cloud/Standalone**: API works perfectly for Cloud. Since it's open-source, we could theoretically package a stripped-down version for Standalone, but initially, we would rely on the public API for both modes.

## Design Doc
- **Integration Point**: OHC Services/Booking Module and Operations AI Agent.
- **Trigger**: User creates a "Service" or "Consultation" product type in OHC.
- **Actions**:
  1. OHC provisions a virtual Cal.com event type via API behind the scenes.
  2. The OHC public storefront displays a date/time picker utilizing Cal.com's embed UI or via API endpoints.
  3. Customer selects a time; OHC creates the booking via Cal.com API.
  4. Cal.com handles calendar conflict resolution and syncs the event to the owner's connected Google/Outlook calendar.
  5. Cal.com (or OHC) sends confirmation emails/SMS to the customer.
- **User Interface**: Business owner sees a "Calendar" view in the OHC app showing upcoming bookings. Setting up availability is a simple day-of-week toggle with time ranges.

## Implementation Prompt
Integrate the Cal.com API to enable automated scheduling for service-based businesses. The system must allow business owners to connect their existing Google or Outlook calendars. When a customer books a service on the public storefront, the integration must check for conflicts, create the calendar event, and trigger any necessary Operations AI agent workflows (like sending a quote or prep materials). The setup process for the business owner must be entirely jargon-free, focusing simply on "When are you available to work?"

## Priority
P0 (Critical)

## Estimated Scope
Medium

---

# Email Marketing Integration

## Title
Implement Automated Email Campaigns and Transactional Emails using Resend

## Problem Statement
Small business owners (like Priya the Boutique Owner) need a way to reliably communicate with their customers. They need to send automated transactional emails (like order confirmations, shipping updates) and occasional marketing blasts (like "New Summer Collection Arrived!"). Traditional email marketing tools like Mailchimp are too complex and expensive, and setting up SPF/DKIM records is technically impossible for a non-technical user.

## Research Report
- **Tool**: Resend
- **Ease of Use for Non-Technical Users**: The complexity is entirely abstracted by OHC. The user just types an email draft or tells the Marketing AI Agent what they want to say, and OHC uses Resend to deliver it. We handle the domain authentication for them under a shared OHC subdomain or managed custom domain.
- **Pricing**: Very developer-friendly. Free tier up to 3,000 emails/month, then extremely cheap per 1,000 emails. Much more cost-effective than standard marketing platforms.
- **Reputation**: Modern, highly reliable, developer-focused API, built on top of AWS SES but with vastly superior developer experience and deliverability optimization.
- **Cloud/Standalone**: Perfect for Cloud. For Standalone, users could theoretically plug in their own Resend API key or use a generic SMTP fallback.

## Design Doc
- **Integration Point**: OHC Notification Service and Marketing/Customer Success AI Agents.
- **Trigger**: System events (order placed, shipped) or manual marketing campaigns created by the user/AI.
- **Actions**:
  1. OHC constructs the email payload using clean, responsive HTML templates (perhaps generated via React Email, which Resend supports nicely).
  2. The Marketing AI Agent can draft the content of the email based on user intent.
  3. OHC sends the payload to the Resend API.
  4. OHC listens to Resend webhooks for delivery status, opens, and clicks.
  5. Metrics are fed back into the Business Advisory Agent.
- **User Interface**: A simple "Broadcast" or "Announcements" section where users can type a message, optionally attach images, and hit send to their customer list. No complex drag-and-drop builders—just clean, text-forward templates.

## Implementation Prompt
Integrate the Resend API to handle both transactional and marketing emails. The system must automatically send beautifully formatted order confirmations, receipts, and shipping notifications. Additionally, build a simple interface for users to blast announcements to their customer list. The Marketing AI agent should be able to assist in drafting these announcements. The technical complexity of domain authentication (SPF/DKIM/DMARC) must be completely hidden from the user, managed automatically by the OHC platform.

## Priority
P1 (High)

## Estimated Scope
Medium

---

# Payment Processing Integration

## Title
Implement Mercado Pago Integration for LATAM Markets

## Problem Statement
While OHC relies primarily on Stripe, Stripe is not available or fully localized in all global markets, particularly in Latin America where alternative payment methods (like Pix in Brazil or OXXO in Mexico) are dominant. Small business owners in these regions need a localized, trusted payment processor that their customers actually use. Without this, conversion rates will be extremely low in LATAM.

## Research Report
- **Tool**: Mercado Pago API
- **Ease of Use for Non-Technical Users**: Creating a Mercado Pago account is common and easy for individuals in LATAM. Connecting it to OHC via OAuth is straightforward.
- **Pricing**: Varies by country, but generally competitive within the region. They charge a percentage per transaction, similar to Stripe, with varying settlement times.
- **Reputation**: The undisputed leader in LATAM payment processing. Extremely high trust and adoption among consumers.
- **Cloud/Standalone**: API works well in Cloud. Can also be supported in Standalone by having the user provide API credentials.

## Design Doc
- **Integration Point**: OHC Checkout Flow and Finance AI Agent.
- **Trigger**: User configures their store location as a supported LATAM country, or manually selects Mercado Pago as a payment provider.
- **Actions**:
  1. User authenticates with Mercado Pago via OAuth.
  2. During checkout, if the store uses Mercado Pago, the OHC backend creates a Preference via the Mercado Pago API.
  3. The customer is redirected to the Mercado Pago checkout flow (or uses a transparent checkout if PCI compliance allows, though redirect is safer and common in LATAM).
  4. OHC listens to Mercado Pago webhooks (IPN - Instant Payment Notification) to update order status.
  5. The Finance AI Agent tracks these payments alongside any other revenue streams.
- **User Interface**: A simple toggle in the "Payments" settings to "Connect Mercado Pago". The checkout screen will show Mercado Pago as the payment option, displaying local payment methods (Pix, Boleto, etc.).

## Implementation Prompt
Integrate the Mercado Pago API as an alternative payment gateway to Stripe, specifically targeting the LATAM market. The integration must support OAuth for easy onboarding, handle the creation of checkout preferences, and process IPN webhooks to reliably update order statuses. The system must gracefully handle delayed settlement methods common in LATAM (like cash payments at convenience stores), updating the OHC order status from "Pending" to "Paid" when the webhook arrives.

## Priority
P2 (Medium)

## Estimated Scope
Medium

---

# Shipping & Logistics Integration

## Title
Implement Automated Shipping Rates and Label Generation using Shippo

## Problem Statement
Small business owners selling physical products (like Maya the Baker shipping cookies, or Priya the Boutique Owner) struggle with logistics. Calculating accurate shipping rates at checkout is complicated, and manually copying addresses to buy postage at the post office is incredibly time-consuming. They need a system that calculates shipping automatically and lets them print labels directly from their phone or computer with one click.

## Research Report
- **Tool**: Shippo API
- **Ease of Use for Non-Technical Users**: Completely abstracted. The user enters their package weight/dimensions and ships-from address. OHC handles the API calls to Shippo.
- **Pricing**: Shippo has a pay-as-you-go model (cents per label) or a monthly subscription for volume. They also offer deeply discounted USPS/UPS rates out of the box, which is a massive value proposition for the business owner.
- **Reputation**: Very strong API, reliable, extensive carrier network globally.
- **Cloud/Standalone**: Excellent for Cloud. For Standalone, the user would need their own Shippo API key.

## Design Doc
- **Integration Point**: OHC Checkout Flow, Order Management UI, and Operations AI Agent.
- **Trigger**: Customer enters shipping address at checkout; Business owner clicks "Fulfill Order" in OHC.
- **Actions**:
  1. At checkout, OHC calls Shippo to get real-time rates based on cart weight and destination, presenting options to the customer.
  2. Upon order completion, OHC uses Shippo to generate a shipping label PDF and a tracking number.
  3. The Operations AI Agent automatically emails the tracking link to the customer.
  4. OHC listens to Shippo tracking webhooks to update the order status ("In Transit", "Delivered").
- **User Interface**: During fulfillment, the user sees a "Buy Label" button. Clicking it generates a printable PDF. The tracking number is automatically attached to the order.

## Implementation Prompt
Integrate the Shippo API to provide real-time shipping rate calculation at checkout and one-click label generation for physical product orders. The integration must allow users to configure default package sizes and weights. It must securely handle the purchase of the shipping label, generate a printable PDF for the user, and automatically sync tracking information to the customer via the Operations AI Agent. The UI must be dead simple: "Print Label" -> PDF opens.

## Priority
P1 (High)

## Estimated Scope
Large

---

# SMS & Notifications Integration

## Title
Implement SMS Notifications and Order Alerts using Twilio

## Problem Statement
Many small business owners and their customers, especially in emerging markets or non-technical demographics (like Fatima the Food Cart Operator), rely heavily on SMS rather than email. They need immediate text alerts when a new order arrives, and their customers need text confirmations for pickups or bookings. Relying solely on push notifications (which can fail) or email (which isn't checked frequently) leads to missed orders and unhappy customers.

## Research Report
- **Tool**: Twilio API
- **Ease of Use for Non-Technical Users**: Invisible to the user. OHC provisions the numbers and manages the API. The user just toggles "Send SMS confirmations" in settings.
- **Pricing**: Pay-per-message. Can get expensive at scale, so we must be judicious about what triggers an SMS (e.g., only critical alerts or paid opt-ins).
- **Reputation**: The industry gold standard for SMS API. Highly reliable globally.
- **Cloud/Standalone**: Perfect for Cloud. In Standalone, users would have to provide their own Twilio credentials, which is slightly technical but standard for self-hosted apps.

## Design Doc
- **Integration Point**: OHC Notification Service and Customer Success AI Agent.
- **Trigger**: Critical system events: New order placed (alert to owner), Order ready for pickup (alert to customer), Upcoming booking reminder (alert to customer).
- **Actions**:
  1. OHC formats a concise SMS string.
  2. OHC calls the Twilio Programmable SMS API.
  3. Twilio handles delivery across global carrier networks.
  4. OHC listens for delivery failure webhooks to log errors or fallback to email.
- **User Interface**: A simple preferences screen where the business owner can enter their phone number to receive alerts, and toggle whether customers receive SMS updates.

## Implementation Prompt
Integrate the Twilio SMS API to handle critical transactional text messages. Implement a notification service that sends real-time alerts to business owners for new orders or bookings, and sends confirmation/reminder texts to their customers. The system must gracefully handle opt-outs (STOP replies) and international phone number formatting (E.164). Ensure that SMS is treated as a premium or cost-controlled feature due to per-message pricing, perhaps limiting it to certain tiers or specific critical events.

## Priority
P1 (High)

## Estimated Scope
Medium

---

# Video Conferencing Integration

## Title
Implement Automated Video Link Generation using Zoom API

## Problem Statement
Service providers who teach or consult online (like Leo the Music Tutor) waste significant time manually creating Zoom links for every booking and emailing them to clients. If they forget, or if the client loses the link, the session is delayed or canceled. They need a system that automatically generates a unique video meeting link the moment a booking is made and includes it in all calendar invites and reminder emails.

## Research Report
- **Tool**: Zoom API (specifically Server-to-Server OAuth or standard User OAuth)
- **Ease of Use for Non-Technical Users**: The user connects their Zoom account via a standard OAuth flow once. After that, it's completely automated.
- **Pricing**: Zoom API is free to use for users with a paid Zoom account.
- **Reputation**: The dominant video conferencing platform.
- **Cloud/Standalone**: Works well in Cloud via OAuth. Standalone requires the user to set up their own OAuth app in Zoom, which is highly technical.

## Design Doc
- **Integration Point**: OHC Booking Module (often paired with Cal.com integration).
- **Trigger**: A new virtual appointment is booked by a customer.
- **Actions**:
  1. OHC calls the Zoom API using the business owner's OAuth token.
  2. OHC creates a new Meeting (specifically a unique meeting ID, not PMI) for the scheduled time.
  3. Zoom API returns the join URL.
  4. OHC saves the join URL in the database attached to the booking.
  5. The join URL is automatically injected into the confirmation email, SMS, and Calendar invite sent to the customer.
- **User Interface**: In the service creation setup, a simple toggle: "Location: [ ] In-person [x] Online via Zoom". A one-time "Connect Zoom" button appears if they haven't authenticated yet.

## Implementation Prompt
Integrate the Zoom API to automatically generate unique video conferencing links for booked services. The integration must handle the Zoom OAuth flow to connect the business owner's account. When a customer books a "Virtual" service, OHC must automatically create a Zoom meeting, retrieve the join link, and embed that link securely in the customer's calendar invite and confirmation notifications. Ensure tokens are refreshed automatically so the integration doesn't silently break.

## Priority
P2 (Medium)

## Estimated Scope
Medium

---

## Next Steps
1. **Review Issue Briefs**: Engineering leadership should review this report containing all 7 integrations.
2. **Prioritization**: Begin implementation of the P0 items (Meta Unified Inbox and Cal.com Scheduling) as these address the most acute pain points for our core personas.
3. **Architecture Check**: Ensure the OHC OAuth flow can gracefully handle the multi-tenant credential storage required for Meta, Cal.com, and Zoom integrations.
