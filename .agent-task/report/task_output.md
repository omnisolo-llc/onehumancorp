# Tool Integration Research Report

This document contains tool integration research and issue briefs for OneHumanCorp (OHC) to extend capabilities in key functional areas. The integrations focus on addressing real-world problems for non-technical small business owners (like Maya the Baker or Carlos the Handyman), operating across both Cloud and Standalone environments.

---

## 1. Social Media Integration: Meta Graph API (Instagram/Facebook)

### Title
Connect Instagram DMs and Facebook Comments to OHC Unified Inbox

### Problem Statement
Business owners like Maya the Baker receive orders, questions ("do you do vegan cakes?"), and feedback across multiple platforms like Instagram DMs and Facebook. Checking multiple apps constantly is overwhelming and leads to missed sales. They need all customer messages to flow into a single, simple inbox within OHC, where the Customer Success AI can help draft replies.

### Research Report
- **Evaluated Tool**: Meta Graph API (Messenger/Instagram Direct API)
- **Problem Solved**: Centralizes customer communications from Meta platforms into OHC.
- **Benefits for OHC Users**: Prevents missed leads, saves time context-switching between apps, enables AI agents to draft replies or auto-respond to common questions while the owner sleeps.
- **Integration Risks**: Meta's App Review process can be strict and time-consuming. API access tokens expire and require a robust refresh mechanism to prevent silent disconnections. Webhooks must be handled reliably.
- **Pricing**: Free to use the API; Meta charges for certain types of automated marketing messages, but standard customer service replies within 24 hours are free.
- **Environment Support**: Works in Cloud (via central OAuth application and webhooks). Can be challenging in Standalone mode without a Cloud-hosted webhook relay service to route incoming messages to the local instance.

### Design Doc
- **User Experience**: The user goes to the "Customer Success" settings and clicks "Connect Instagram". They log in with their Meta account and grant permissions. New messages appear in the OHC Inbox. The AI "Ambassador" agent drafts suggested replies that the user can approve or edit with one tap.
- **Integration Points**:
  - OAuth flow initiated from the OHC Frontend.
  - Webhook listener in the OHC Backend to receive incoming messages.
  - Event publishing to the OHC Event Mesh so the Customer Success agent is notified of new messages.
  - API calls to send replies back to the Meta platform.

### Implementation Prompt
Implement the Meta Graph API integration. Create the OAuth connection flow in the UI, allowing users to securely link their Meta accounts. Build a secure webhook endpoint to receive incoming Instagram DMs and Facebook messages, routing them into the user's unified OHC Inbox. Ensure the AI Customer Success agent is triggered to draft replies for new incoming messages. Handle token refreshes gracefully.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling: Cal.com

### Title
Enable Two-Way Calendar Sync and Smart Booking with Cal.com

### Problem Statement
Service providers like Leo the Music Tutor and Carlos the Handyman need clients to book available times without endless back-and-forth messaging. They already use personal calendars (Google or Apple) and need a system that prevents double-booking while providing a professional, easy-to-use booking link for their clients.

### Research Report
- **Evaluated Tool**: Cal.com API
- **Problem Solved**: Provides robust scheduling infrastructure, managing availability, timezones, and calendar sync (Google, Outlook, Apple).
- **Benefits for OHC Users**: Eliminates double bookings. Automatically translates timezones for online tutors. Replaces complex manual setup with a seamless booking flow.
- **Integration Risks**: Cal.com requires managing external user identities or using their Platform API. Data privacy is critical since we are touching personal calendars.
- **Pricing**: Cal.com Platform is priced per booked user/month or API usage, which must be factored into OHC's economics.
- **Environment Support**: Cloud support is excellent. Standalone support is possible since Cal.com is open source, but integrating a Cloud SaaS API from a Standalone OHC instance will require API key management.

### Design Doc
- **User Experience**: The user defines their working hours and connects their personal Google Calendar. OHC generates a beautiful booking page on their storefront. When a customer books, the event appears on the user's personal calendar, and the OHC Operations agent tracks the booking and schedules reminders.
- **Integration Points**:
  - Connect user calendars via OAuth.
  - Query availability considering existing calendar events.
  - Create booking events via the API when a customer completes the flow.

### Implementation Prompt
Integrate Cal.com's scheduling engine. Build a UI for business owners to set working hours and connect external calendars. Implement the backend logic to check real-time availability and create bookings. Ensure the booking flow on the public storefront is frictionless and mobile-optimized, allowing customers to easily select time slots and receive confirmation.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing: Resend

### Title
Simple, Beautiful Automated Email Campaigns via Resend

### Problem Statement
Boutique owners like Priya need an easy way to notify their past customers when new stock arrives or a sale starts. Traditional email marketing tools (like Mailchimp) are overly complex with lists, tags, and HTML builders. She just wants to type a message, add photos, and have her "Promoter" agent send it beautifully to everyone who has bought from her before.

### Research Report
- **Evaluated Tool**: Resend API
- **Problem Solved**: Reliable, developer-friendly email sending with high deliverability and modern templating.
- **Benefits for OHC Users**: Ensures marketing and transactional emails actually reach the inbox, not the spam folder. Allows OHC to generate beautiful, on-brand emails automatically without the user needing to learn design tools.
- **Integration Risks**: Domain verification (DNS records) is typically required for good deliverability, which is a massive hurdle for non-technical users. OHC needs a strategy to handle sending on behalf of users (e.g., via a shared sending domain like `mail.onehumancorp.com` or automated DNS setup).
- **Pricing**: Generous free tier, scalable pay-as-you-go pricing.
- **Environment Support**: Works well in Cloud. Standalone mode users would need to provide their own Resend API key to enable email features.

### Design Doc
- **User Experience**: The user tells their Marketing agent, "Send an email about the new summer collection to past customers." The AI drafts the email with product images. The user reviews it and clicks "Send." No lists to manage; OHC uses the existing customer database.
- **Integration Points**:
  - Backend integration with Resend API for transactional (receipts) and marketing emails.
  - Handle bounce and complaint webhooks to clean the customer list automatically.

### Implementation Prompt
Integrate the Resend API for outbound email. Implement the backend service to dispatch emails and handle webhooks for bounces and complaints. In the UI, create a simple approval flow where business owners can review and send AI-generated email campaigns. Abstract away any complex DNS setup by using a verified OHC shared sending domain by default.

### Priority
P1

### Estimated Scope
Medium

---

## 4. Payment Processing: Mercado Pago (LATAM focus)

### Title
Enable Local Payments in LATAM with Mercado Pago

### Problem Statement
While Stripe is great, many users in Latin America operate in economies where local payment methods (like Pix in Brazil, or OXXO cash payments in Mexico) are mandatory. A user setting up a store in Brazil will lose most sales if they can only accept international credit cards.

### Research Report
- **Evaluated Tool**: Mercado Pago API
- **Problem Solved**: Provides access to vital local payment methods across Latin America.
- **Benefits for OHC Users**: Unlocks entirely new geographic markets for OHC, allowing business owners in LATAM to accept payments in ways their local customers trust and expect.
- **Integration Risks**: Managing multiple payment gateways increases the complexity of the Finance & Payments department. Refunds and dispute handling flows differ significantly from Stripe.
- **Pricing**: Percentage per transaction, varies by country and payment method.
- **Environment Support**: Cloud supported. Standalone supported via direct API key integration by the local user.

### Design Doc
- **User Experience**: During setup, if a user selects a LATAM country, Mercado Pago is offered as the primary payment processor. The storefront checkout seamlessly displays options like Pix. The OHC Finance agent tracks these payments just like Stripe transactions.
- **Integration Points**:
  - Implement a `PaymentProvider` interface in the backend to abstract Mercado Pago alongside Stripe.
  - Integrate Checkout Pro or API-based checkout.
  - Webhook handlers for asynchronous payment confirmations (e.g., waiting for cash payment at a physical store).

### Implementation Prompt
Add Mercado Pago as an alternative payment gateway. Implement the backend integration adhering to an abstract payment interface so the core system remains payment-agnostic. Update the checkout UI to dynamically present Mercado Pago (including local options like Pix) when appropriate for the merchant's region. Ensure the Finance AI agent can correctly parse and report on these transactions.

### Priority
P2

### Estimated Scope
Large

---

## 5. Shipping & Logistics: Shippo

### Title
Automated Shipping Rates and Label Generation with Shippo

### Problem Statement
Sellers of physical goods waste hours at the post office manually typing addresses and comparing shipping costs. They need the system to automatically calculate the correct shipping cost at checkout and let them print a prepaid label from their phone with one tap when an order is ready.

### Research Report
- **Evaluated Tool**: Shippo API
- **Problem Solved**: Aggregates dozens of shipping carriers (USPS, UPS, FedEx, DHL, local international carriers) into a single API.
- **Benefits for OHC Users**: Eliminates the guesswork in shipping costs. Turns fulfillment from a manual chore into a one-tap action. Provides professional tracking links automatically.
- **Integration Risks**: Accurately calculating rates at checkout requires knowing product weights and dimensions, which non-technical users often forget to input.
- **Pricing**: Small per-label fee plus postage costs.
- **Environment Support**: Cloud supported. Standalone mode can work with the user providing their own API credentials.

### Design Doc
- **User Experience**: When the user adds a physical product, the AI asks for a rough size/weight. At checkout, the customer sees live shipping rates. When the order is placed, the Operations agent notifies the owner. The owner clicks "Buy Label," the fee is deducted, and a printable PDF appears. The Customer Success agent automatically emails the tracking link to the buyer.
- **Integration Points**:
  - Live rate calculation at checkout.
  - Label purchase and PDF generation flow in the order management UI.
  - Webhook integration to track package status and trigger customer notifications.

### Implementation Prompt
Integrate Shippo for automated shipping. Add UI for users to specify product weights. Implement live rate calculation during the checkout flow. Build a frictionless "Buy Label" flow in the order management screen that purchases postage and returns a printable PDF. Ensure the Operations agent monitors tracking status and updates the customer.

### Priority
P1

### Estimated Scope
Large

---

## 6. SMS & Notifications: Twilio

### Title
Reliable Order Notifications via SMS with Twilio

### Problem Statement
Users like Fatima the Food Cart Operator may not have reliable mobile data or smart notifications active while cooking. She needs a loud, immediate text message the second a pre-order comes in so she can start preparing the food.

### Research Report
- **Evaluated Tool**: Twilio Programmable SMS
- **Problem Solved**: Delivers reliable, instantaneous text messages globally.
- **Benefits for OHC Users**: Ensures critical, time-sensitive events (like a food order or an immediate booking) are never missed. Builds trust that the system "has their back."
- **Integration Risks**: SMS regulations (A2P 10DLC in the US) are incredibly strict. Registering campaigns for small businesses automatically is complex and prone to rejection. SMS is also expensive compared to push notifications.
- **Pricing**: Pay per message (varies significantly by country).
- **Environment Support**: Cloud supported. Standalone supported via user's API key.

### Design Doc
- **User Experience**: The user goes to their notification settings and toggles on "Send me a text for new orders." OHC handles the delivery seamlessly.
- **Integration Points**:
  - API integration to dispatch SMS.
  - Preference management in the user profile to handle opt-ins and spending limits (to prevent abuse/high costs).

### Implementation Prompt
Integrate Twilio to enable critical SMS alerts. Build backend logic to dispatch SMS notifications triggered by high-priority events (e.g., new paid orders). Update the user settings UI to allow owners to opt-in to SMS alerts. Implement strict rate limiting and cost controls to ensure the feature is not abused.

### Priority
P2

### Estimated Scope
Medium

---

## 7. Video Conferencing: Zoom API

### Title
Auto-Generate Meeting Links for Services via Zoom API

### Problem Statement
When a student books a lesson with Leo the Music Tutor, he currently has to manually create a Zoom link, email it to the student, and remember to join the right meeting at the right time. This manual process looks unprofessional and leads to missed meetings.

### Research Report
- **Evaluated Tool**: Zoom API (Server-to-Server OAuth or User-Managed OAuth)
- **Problem Solved**: Programmatically creates unique video meeting rooms for scheduled events.
- **Benefits for OHC Users**: Creates a seamless, professional experience for both the provider and the client. The link is automatically embedded in calendar invites and reminder emails.
- **Integration Risks**: Zoom's OAuth approval process requires a publicly accessible application. Managing token lifecycles and ensuring the correct Zoom account limits (e.g., 40-minute limit on free accounts) are respected.
- **Pricing**: API access is included in Zoom plans, but the underlying host must have the appropriate license for extended features.
- **Environment Support**: Cloud supported. Standalone supported via direct OAuth connection if OHC provides a relay, or manual API key entry.

### Design Doc
- **User Experience**: Leo connects his Zoom account in the Operations settings. When configuring his "1-hour Guitar Lesson" service, he toggles "Online via Zoom". When a student books, OHC auto-generates the Zoom link, adds it to the calendar invite, and sends the reminder email.
- **Integration Points**:
  - OAuth flow to connect user's Zoom account.
  - API integration to create a meeting upon a successful booking event.
  - Attach the meeting URL to the OHC booking record and calendar sync system.

### Implementation Prompt
Integrate the Zoom API for automatic meeting creation. Create an OAuth flow for users to connect their Zoom accounts. Modify the booking backend so that when an "Online" service is booked, a unique Zoom meeting is generated. Ensure the resulting join link is automatically included in confirmation emails and calendar invites sent to both the owner and the customer.

### Priority
P2

### Estimated Scope
Medium
