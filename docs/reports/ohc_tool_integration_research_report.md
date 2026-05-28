# OHC Tool Integration Research Report

## Overview
This report outlines the findings and issue briefs for integrating various tools to expand OHC's capabilities for small business owners in both Cloud and Standalone environments.

The evaluation is categorized by domain: Social Media Integration, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing.

---

## 1. Social Media Integration

### WhatsApp Cloud API Integration
**Title**: Integrate WhatsApp Cloud API for Unified Direct Messaging
**Problem Statement**: Small business owners like Fatima (Local Grocer) rely heavily on WhatsApp for customer orders. Managing these manually on a personal device leads to missed sales and slow responses. They need an automated, unified inbox within OHC to seamlessly handle inquiries.
**Research Report**:
- **Findings & Competitive Analysis**: WhatsApp Cloud API allows direct integration without third-party brokers. Compared to Twilio, direct Meta integration avoids markup fees.
- **Ease of Use**: Non-technical users just need to click "Connect WhatsApp" and log into Facebook.
- **Pricing**: First 1,000 service conversations per month are free, then standard Meta per-conversation rates apply.
- **Reputation**: Official Meta product, highly reliable.
- **Advantages**: Direct access to billions of users, high open rates.
- **Risks**: Meta's stringent business verification process might confuse some users.
- **Cloud/Standalone**: Works seamlessly in Cloud via webhooks. Standalone requires a lightweight cloud proxy to receive incoming Meta webhooks.
**Design Doc**:
- Users visit the "Operations" dashboard and click "Connect WhatsApp".
- A seamless Meta OAuth popup handles account linking.
- Incoming WhatsApp messages trigger a notification and appear in the "Unified Inbox".
- "The Ambassador" AI drafts suggested replies based on inventory and FAQs.
**Implementation Prompt**: Implement a direct Meta Graph API OAuth flow for WhatsApp Business. Create a webhook endpoint to receive incoming messages, store them in the unified inbox, and enable users to send replies natively from the OHC app.
**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

### Cal.com Integration
**Title**: Integrate Cal.com for Seamless Calendar Sync & Booking
**Problem Statement**: Leo (Music Tutor) and Carlos (Handyman) lose hours each week coordinating schedules via text. They need an automated booking page that syncs instantly with their personal calendars to prevent double-booking.
**Research Report**:
- **Findings & Competitive Analysis**: Cal.com is an open-source alternative to Calendly. It supports massive scale and provides self-hosted options.
- **Ease of Use**: Users authenticate their Google or Outlook accounts with one click.
- **Pricing**: Free for individuals, making it ideal for our free-tier users.
- **Reputation**: Highly respected in the open-source community, robust documentation.
- **Advantages**: Native white-labeling, timezone math out-of-the-box.
- **Risks**: Minor latency during peak webhook delivery.
- **Cloud/Standalone**: Perfectly compatible with both Cloud (API) and Standalone (can be embedded or run locally).
**Design Doc**:
- The "Settings" tab features a "Sync Calendar" option.
- Users connect their preferred calendar provider via Cal.com's OAuth.
- OHC automatically generates a public booking page accessible via the user's storefront.
- When a client books, the slot is instantly blocked on the user's personal calendar.
**Implementation Prompt**: Embed Cal.com's scheduling infrastructure. Provide a one-click calendar sync for users and auto-generate a public booking widget on their OHC storefront that handles conflict resolution.
**Priority**: P1
**Estimated Scope**: Medium

---

## 3. Email Marketing

### Resend Integration
**Title**: Integrate Resend for Transactional and Marketing Emails
**Problem Statement**: Maya (Home Baker) needs to send beautiful order confirmations and seasonal promo emails to her customer list without wrestling with complex tools like Mailchimp.
**Research Report**:
- **Findings & Competitive Analysis**: Resend offers a modern, developer-friendly API and a React Email component library. It is vastly simpler to integrate natively than SendGrid or Mailgun.
- **Ease of Use**: Business owners don't interact with Resend directly; they use OHC's simple "Draft Email" UI, and Resend handles deliverability under the hood.
- **Pricing**: Free up to 3,000 emails/month. Very affordable at scale.
- **Reputation**: Rapidly growing, excellent deliverability rates.
- **Advantages**: Beautiful built-in templates, fast API.
- **Risks**: Domain authentication (DNS records) might be tricky for non-technical users.
- **Cloud/Standalone**: Works flawlessly in both modes via standard REST API calls.
**Design Doc**:
- In the "Marketing" tab, users can select a segment of their customer list.
- A simple rich-text editor allows them to draft an email.
- OHC wraps the content in a beautiful, branded template and sends it via Resend.
- Delivery and open rates are displayed on a simple dashboard card.
**Implementation Prompt**: Integrate the Resend API for outbound email delivery. Create a simple marketing UI where users can draft emails to their customer segments, and display basic analytics (sent, opened) on the dashboard.
**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing

### Mercado Pago Integration
**Title**: Integrate Mercado Pago for LATAM Market Expansion
**Problem Statement**: Business owners in LATAM cannot easily use Stripe and need to accept local payment methods like PIX, Boleto, and local credit cards to operate their storefronts effectively.
**Research Report**:
- **Findings & Competitive Analysis**: Mercado Pago is the dominant payment provider in Latin America, widely trusted by consumers over international alternatives.
- **Ease of Use**: Users log into their existing Mercado Pago account to link it to OHC.
- **Pricing**: Standard localized transaction fees (~3-4%), no monthly cost to OHC.
- **Reputation**: Industry standard in LATAM.
- **Advantages**: Unlocks the entire South and Central American market.
- **Risks**: Asynchronous payment flows (e.g., waiting for a Boleto to clear) require robust webhook handling.
- **Cloud/Standalone**: Works natively in both Cloud and Standalone modes via API.
**Design Doc**:
- The "Payments" settings page includes a "Connect Mercado Pago" button.
- The storefront checkout flow dynamically displays local payment options if the user's currency/region matches.
- Order statuses remain "Pending" until Mercado Pago webhooks confirm asynchronous payments.
**Implementation Prompt**: Implement Mercado Pago as an alternative payment gateway. Update the checkout flow to support local payment methods and ensure asynchronous webhook confirmations correctly update order statuses.
**Priority**: P0
**Estimated Scope**: Large

---

## 5. Shipping & Logistics

### Shippo Integration
**Title**: Integrate Shippo for Automated Label Generation and Tracking
**Problem Statement**: Priya (Boutique Owner) struggles to manually calculate shipping rates and buy labels at the post office. She needs an automated way to print labels and send tracking numbers directly from her OHC dashboard.
**Research Report**:
- **Findings & Competitive Analysis**: Shippo provides a unified API across 85+ global carriers (USPS, FedEx, DHL). It is more user-friendly and often cheaper than EasyPost.
- **Ease of Use**: The business owner clicks "Generate Label" on an order, prints it, and Shippo automatically emails the customer the tracking link.
- **Pricing**: Pay-as-you-go ($0.05 per label) with deeply discounted USPS rates.
- **Reputation**: Highly reliable, standard for many modern e-commerce platforms.
- **Advantages**: Instant access to carrier discounts without negotiating contracts.
- **Risks**: International customs forms still require some manual data entry.
- **Cloud/Standalone**: Works seamlessly in both environments via REST API.
**Design Doc**:
- The "Orders" dashboard features a "Fulfill" button.
- Clicking it opens a modal to verify package weight/dimensions.
- OHC fetches live rates via Shippo and lets the user purchase the label natively.
- A printable PDF is generated, and tracking info is auto-attached to the order.
**Implementation Prompt**: Integrate the Shippo API to fetch real-time shipping rates and generate purchasable labels. Add a fulfillment flow to the order details page that outputs a printable label PDF and attaches tracking metadata.
**Priority**: P2
**Estimated Scope**: Large

---

## 6. SMS & Notifications

### Twilio Integration
**Title**: Integrate Twilio for Reliable SMS Notifications
**Problem Statement**: Fatima (Local Grocer) serves a customer base with low internet literacy who rely on SMS rather than email for order updates and delivery notifications.
**Research Report**:
- **Findings & Competitive Analysis**: Twilio is the global leader in programmatic SMS. While slightly more expensive than SNS, its global carrier routing and deliverability are unmatched.
- **Ease of Use**: Completely invisible to the business owner. They just toggle "Send SMS Updates" in their settings.
- **Pricing**: ~$0.0079 per message in the US, varies globally.
- **Reputation**: Gold standard for telecom APIs.
- **Advantages**: Massive global reach, reliable delivery.
- **Risks**: Strict A2P 10DLC compliance rules in the US require careful registration.
- **Cloud/Standalone**: Works perfectly in both modes via API.
**Design Doc**:
- Users can toggle "Enable SMS Notifications" in the global settings.
- When an order is dispatched, OHC triggers a Twilio SMS to the customer's phone number.
- "The Ambassador" agent can be configured to handle basic SMS replies.
**Implementation Prompt**: Implement Twilio API integration for outbound SMS. Add order lifecycle hooks that trigger SMS notifications for key events (e.g., "Order Confirmed", "Out for Delivery") and provide a toggle in the settings UI.
**Priority**: P1
**Estimated Scope**: Medium

---

## 7. Video Conferencing

### Zoom API Integration
**Title**: Integrate Zoom API for Auto-Generated Consultation Links
**Problem Statement**: Leo (Music Tutor) needs to automatically generate and send secure video meeting links when a student books an online lesson, avoiding the hassle of manual link creation.
**Research Report**:
- **Findings & Competitive Analysis**: Zoom remains the most widely recognized video tool for consumers. While Google Meet is good, Zoom's API allows for deeper integration and recording management.
- **Ease of Use**: The user authorizes Zoom once. Links are automatically added to calendar invites.
- **Pricing**: Free for standard 40-minute meetings, requires paid Zoom plan for longer API-generated meetings.
- **Reputation**: Universal consumer familiarity.
- **Advantages**: Users trust it, robust connection quality.
- **Risks**: OAuth flow requires users to have an existing Zoom account.
- **Cloud/Standalone**: Compatible with both via standard OAuth and REST APIs.
**Design Doc**:
- "Integrations" page features a "Connect Zoom" button.
- When an online service is booked via the storefront, OHC calls the Zoom API to generate a unique meeting room.
- The join link is automatically emailed to the customer and added to the user's dashboard agenda.
**Implementation Prompt**: Integrate Zoom's OAuth and meeting creation API. When a booking is confirmed for a virtual service, automatically generate a Zoom link and embed it into the confirmation emails and calendar events.
**Priority**: P2
**Estimated Scope**: Medium
