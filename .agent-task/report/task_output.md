# 🔍 Scout: Tool Integration Research Report

# Integrate MessageBird for Unified Multichannel Inbox

## Problem Statement
Small business owners like Priya (Boutique) and Maya (Baker) receive customer inquiries across Instagram, WhatsApp, and Facebook. Switching between apps causes missed messages and lost sales. They need all DMs in one unified OHC inbox so their Customer Success AI can instantly reply while they sleep.

## Research Report
- **Tool Evaluated**: MessageBird (by Bird)
- **Ease of Use**: Provides a unified API for WhatsApp, Instagram, Messenger, and SMS.
- **Pricing**: Pay-per-message model. Competitive and transparent.
- **Standalone/Cloud**: Works perfectly in both modes via REST API.
- **Persona Fit**: Perfect for non-technical users who just want "all messages in one place".

## Design Doc
- **Integration Point**: Customer Success Agent and Unified Inbox UI.
- **Trigger**: Incoming webhook from MessageBird.
- **Action**: Store message in OHC unified inbox, trigger Customer Success agent for automated draft reply.
- **User View**: A simple "Connect Instagram/WhatsApp" button in OHC Settings. Messages appear in the OHC Inbox like standard texts.

## Implementation Prompt
Create a unified webhook handler for MessageBird that parses incoming messages from WhatsApp/IG/FB and stores them in the OHC tenant inbox. Implement the settings UI to allow users to connect their social channels via MessageBird OAuth. Ensure the Customer Success AI agent is triggered on new message arrival.

## Priority
P1

## Estimated Scope
Medium

---

# Integrate Cal.com for White-Labeled Booking

## Problem Statement
Service-based businesses like Leo (Music Tutor) and Carlos (Handyman) need a way for customers to book time slots without back-and-forth emails. They need a simple link that syncs with their personal calendars.

## Research Report
- **Tool Evaluated**: Cal.com
- **Ease of Use**: Very user-friendly, open API, supports white-labeling out of the box.
- **Pricing**: Free tier available, highly SMB-friendly.
- **Standalone/Cloud**: Excellent for Cloud (API) and Standalone (self-hosted or direct API).
- **Persona Fit**: Ideal for Leo and Carlos to share a booking link without needing technical setup.

## Design Doc
- **Integration Point**: Operations Agent, Sales Agent.
- **Trigger**: Agent identifies booking intent or user sets up scheduling.
- **Action**: Generate single-use or reusable booking links via Cal.com API.
- **User View**: A scheduling widget on the OHC website where customers pick dates. Business owner connects Google/Apple Calendar once.

## Implementation Prompt
Build an integration module with the Cal.com API. Add a "Scheduling" component to the drag-and-drop website builder that embeds the Cal.com widget. Ensure booked events trigger OHC webhook handlers to update the business owner's dashboard.

## Priority
P0

## Estimated Scope
Large

---

# Integrate Brevo for Automated Email Campaigns

## Problem Statement
Business owners want to email their customer list about new products or promotions (like Priya's boutique stock arrivals) without learning complex tools like Mailchimp.

## Research Report
- **Tool Evaluated**: Brevo (formerly Sendinblue)
- **Ease of Use**: Generous free tier (300 emails/day), simple API for transactional and campaign emails.
- **Pricing**: Very SMB-friendly compared to competitors.
- **Standalone/Cloud**: REST API works perfectly for both.
- **Persona Fit**: Simple enough for Maya and Priya to run automated marketing.

## Design Doc
- **Integration Point**: Marketing & Advertising Agent.
- **Trigger**: AI Agent creates a campaign or transactional event (e.g., new stock).
- **Action**: Sync OHC customer list to Brevo, generate email HTML via AI, send via Brevo API.
- **User View**: Owner types "Email all my customers about the weekend sale", the Marketing agent handles the rest.

## Implementation Prompt
Implement the Brevo API client to sync customer contacts and trigger email campaigns. Add a UI in the Marketing department for the user to review and approve AI-generated email drafts before they are sent.

## Priority
P1

## Estimated Scope
Medium

---

# Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
Stripe is great, but not supported or widely used in many LATAM countries where local payment methods (like Pix in Brazil or OXXO in Mexico) are essential for conversion.

## Research Report
- **Tool Evaluated**: Mercado Pago
- **Ease of Use**: Dominant in LATAM, supports local payment methods automatically.
- **Pricing**: Standard local processing fees, no monthly cost.
- **Standalone/Cloud**: Robust API and webhooks, works in both.
- **Persona Fit**: Essential for international businesses needing local context and trust.

## Design Doc
- **Integration Point**: Finance & Payments Agent, Checkout Flow.
- **Trigger**: Customer initiates checkout.
- **Action**: Generate Mercado Pago preference and redirect to their secure checkout or use transparent checkout.
- **User View**: Buyers in LATAM see local payment options. Owner sees funds in their OHC dashboard alongside Stripe.

## Implementation Prompt
Integrate Mercado Pago SDK/API as an alternative payment provider. Update the checkout UI to support dynamic provider selection based on the tenant's region setting. Ensure webhooks reliably update order status.

## Priority
P2

## Estimated Scope
Medium

---

# Integrate Shippo for Automated Label Generation

## Problem Statement
Shipping physical goods is a manual nightmare. Maya needs to calculate shipping rates, print labels, and send tracking numbers without leaving OHC or dealing with carrier portals.

## Research Report
- **Tool Evaluated**: Shippo
- **Ease of Use**: Connects to dozens of carriers globally (USPS, FedEx, DHL) with one API.
- **Pricing**: Pay-as-you-go, no monthly fees for basic tier.
- **Standalone/Cloud**: Cloud API works for both modes.
- **Persona Fit**: Perfect for Maya (Baker) and Priya (Boutique).

## Design Doc
- **Integration Point**: Operations Agent, Customer Success Agent.
- **Trigger**: Order paid and marked ready to ship.
- **Action**: Fetch shipping rates, purchase label via Shippo, notify Customer Success Agent to send tracking.
- **User View**: Owner clicks "Print Label" on the order page. Tracking is auto-emailed.

## Implementation Prompt
Build a Shippo integration to fetch live shipping rates during checkout and generate shipping labels from the order dashboard. Provide a UI component to display package tracking statuses.

## Priority
P0

## Estimated Scope
Large

---

# Integrate Twilio for Reliable SMS Notifications

## Problem Statement
Fatima (Food Cart) operates in a noisy environment on a slow mobile connection. She needs instant SMS alerts for new orders, and her customers need SMS pickup confirmations.

## Research Report
- **Tool Evaluated**: Twilio
- **Ease of Use**: Industry standard, highly reliable global SMS API.
- **Pricing**: Pay per message, very cheap.
- **Standalone/Cloud**: Excellent for both.
- **Persona Fit**: Critical for Fatima and local service businesses.

## Design Doc
- **Integration Point**: Operations Agent, Notification System.
- **Trigger**: Order placed or Order ready for pickup.
- **Action**: Send SMS via Twilio API to the owner or customer.
- **User View**: Fatima gets a text: "New Order: 2x Chicken over Rice. Reply READY when done."

## Implementation Prompt
Implement a Twilio SMS sender service. Add a notification preference panel where business owners can enable SMS alerts. Allow the system to send automated SMS updates to customers for order readiness.

## Priority
P1

## Estimated Scope
Small

---

# Integrate Zoom API for Auto-Generated Virtual Lessons

## Problem Statement
Leo (Music Tutor) spends too much time manually creating Zoom links for every student and pasting them into calendar invites.

## Research Report
- **Tool Evaluated**: Zoom API
- **Ease of Use**: Ubiquitous for consumers, standard OAuth flow.
- **Pricing**: Free tier supports 40-min meetings, which covers basic needs.
- **Standalone/Cloud**: Requires OAuth 2.0, well-supported in Cloud.
- **Persona Fit**: Essential for Leo and any digital service provider.

## Design Doc
- **Integration Point**: Operations Agent (Booking Flow).
- **Trigger**: New virtual service booking created.
- **Action**: Call Zoom API to generate a meeting, attach the join link to the OHC booking record and calendar invite.
- **User View**: Leo connects his Zoom account once. Every online booking auto-includes a unique Zoom link for him and the student.

## Implementation Prompt
Implement Zoom OAuth flow in the integrations settings. When a booking is confirmed for a "virtual" service, automatically generate a Zoom meeting link and append it to the confirmation email and calendar event.

## Priority
P2

## Estimated Scope
Medium
