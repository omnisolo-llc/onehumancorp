# 🔍 Scout: Tool Integration Research

This document outlines research and issue briefs for tool integrations across seven core categories, evaluated specifically for their utility to OneHumanCorp (OHC)'s non-technical small business owners (e.g., Maya the Baker, Carlos the Handyman).

---

## 1. Social Media Integration: ManyChat
### Problem Statement
Business owners like Maya the Baker receive numerous inquiries via Instagram DMs ("Do you do vegan cakes?"). Manually responding to these is time-consuming and happens 24/7, leading to missed opportunities. A non-technical owner needs a unified inbox and automated replies without configuring complex workflows.

### Research Report
*   **Target Tool**: ManyChat
*   **Persona Benefited**: Maya (Baker), Priya (Boutique)
*   **Evaluation**: ManyChat is a leading platform for Instagram/Facebook Messenger automation. It's relatively easy to use, but setting up the initial flows can be daunting for someone with zero technical knowledge.
*   **Integration Value**: By integrating ManyChat's API, OHC's "Customer Success" AI agent can automatically draft and send replies to common questions, or simply pull all DMs into OHC's unified inbox.
*   **Pricing**: Free tier exists; Pro starts at $15/mo.
*   **Mode Support**: Cloud (SaaS integration via OAuth/Webhooks) and Standalone (API keys configured by user).

### Design Doc
*   **Trigger**: User connects Instagram account via OHC wizard. A new DM is received.
*   **Action**: Webhook triggers OHC. The Customer Success AI agent reads the DM, consults the business context (e.g., Maya's menu), and generates a response. The response is sent back via ManyChat API.
*   **User View**: The user sees a "Social Inbox" in OHC where all DMs are logged, and AI replies are shown with an "Auto-Replied" tag.

### Implementation Prompt
Implement a ManyChat integration where users can connect their Instagram/Facebook accounts. Create a webhook endpoint to receive incoming DMs. Route these messages to the Customer Success AI agent to generate context-aware replies. Send the replies back via ManyChat API and display the conversation in the OHC unified inbox.

*   **Priority**: P1
*   **Estimated Scope**: Large

---

## 2. Calendar & Scheduling: Calendly
### Problem Statement
Service providers like Carlos (Handyman) and Leo (Tutor) need customers to book specific time slots. Back-and-forth emails to find a time are inefficient. They need a simple, public booking page that syncs with their personal calendars (Google/Outlook) to prevent double-booking.

### Research Report
*   **Target Tool**: Calendly
*   **Persona Benefited**: Carlos (Handyman), Leo (Tutor)
*   **Evaluation**: Calendly is the industry standard for scheduling. It handles timezone conversions and calendar conflicts flawlessly. The API is robust.
*   **Integration Value**: Instead of building a complex calendar sync engine from scratch, OHC can leverage Calendly. OHC can automatically create Calendly event types for the user's services and embed the booking widget on their OHC storefront.
*   **Pricing**: Free basic tier; Standard starts at $10/mo.
*   **Mode Support**: Cloud and Standalone (via user's OAuth/API key).

### Design Doc
*   **Trigger**: User creates a "Service" offering in OHC (e.g., "1-hour Guitar Lesson").
*   **Action**: OHC automatically provisions a Calendly event type via API and retrieves the booking link.
*   **User View**: The user's public storefront displays a seamless booking widget. The user's OHC dashboard shows upcoming bookings synced from Calendly.

### Implementation Prompt
Integrate Calendly API to automatically create event types when a user defines a "Service" in OHC. Embed the Calendly booking widget on the user's generated public storefront. Sync booked events back into the OHC dashboard's calendar view.

*   **Priority**: P0
*   **Estimated Scope**: Medium

---

## 3. Email Marketing: MailerLite
### Problem Statement
Priya (Boutique Owner) wants to email her customers when a new clothing line drops. She doesn't understand "campaigns" or "DKIM settings." She just wants to write a message and hit send to everyone who bought from her.

### Research Report
*   **Target Tool**: MailerLite
*   **Persona Benefited**: Priya (Boutique), Leo (Tutor)
*   **Evaluation**: MailerLite is known for its simplicity and clean interface, making it friendlier than Mailchimp for non-technical users. It has a generous free tier and a good API.
*   **Integration Value**: OHC can sync the user's customer list to MailerLite. The "Marketing" AI agent can draft the email content, which is then sent via MailerLite's API, abstracting away the email template builder.
*   **Pricing**: Free up to 1,000 subscribers; Paid starts at $9/mo.
*   **Mode Support**: Cloud and Standalone.

### Design Doc
*   **Trigger**: User says "Send an email to my customers about the spring sale."
*   **Action**: Marketing AI drafts the email. OHC syncs the customer list to a MailerLite group and uses the MailerLite API to send the campaign.
*   **User View**: A simple "Send Announcement" UI where they approve the AI-generated text. Analytics (opens/clicks) are pulled from MailerLite and shown in plain English on the OHC dashboard.

### Implementation Prompt
Integrate MailerLite API to synchronize OHC customer lists with MailerLite subscriber groups. Build a flow where the Marketing AI drafts an email, the user approves it in OHC, and OHC triggers the campaign send via MailerLite. Fetch and display basic open/click metrics in the OHC dashboard.

*   **Priority**: P1
*   **Estimated Scope**: Medium

---

## 4. Payment Processing: Mercado Pago
### Problem Statement
While Stripe is great, small businesses in Latin America heavily rely on local payment methods (e.g., PIX in Brazil, OXXO in Mexico). Users in these regions need a payment processor that their customers actually use and trust.

### Research Report
*   **Target Tool**: Mercado Pago
*   **Persona Benefited**: Global users (e.g., a baker in Argentina or a tutor in Brazil)
*   **Evaluation**: Mercado Pago is the dominant payment gateway in LATAM. It supports local cards, bank transfers, and cash payments.
*   **Integration Value**: Essential for expanding OHC's total addressable market beyond US/Europe.
*   **Pricing**: Transaction fee based (varies by country, typically ~3-5%).
*   **Mode Support**: Cloud and Standalone.

### Design Doc
*   **Trigger**: Customer checks out on the OHC storefront from a LATAM region.
*   **Action**: OHC routes the payment intent to Mercado Pago instead of Stripe.
*   **User View**: Business owner selects "Mercado Pago" in the "Finance & Payments" settings. Customers see local payment options at checkout.

### Implementation Prompt
Add Mercado Pago as an alternative payment provider to Stripe. Implement the checkout flow for one-time payments and deposits. Ensure webhook handlers are robust for asynchronous payment confirmations (like cash payments via OXXO/Boleto). Update the Finance UI to support processor selection.

*   **Priority**: P2
*   **Estimated Scope**: Large

---

## 5. Shipping & Logistics: Shippo
### Problem Statement
Priya (Boutique) sells physical goods. When an order comes in, she needs to calculate shipping costs, print a label, and give the customer a tracking number. Navigating USPS/UPS directly is complex.

### Research Report
*   **Target Tool**: Shippo
*   **Persona Benefited**: Priya (Boutique), Maya (Baker - if shipping non-perishables)
*   **Evaluation**: Shippo aggregates multiple carriers, provides discounted rates, and has a developer-friendly API for generating labels and tracking.
*   **Integration Value**: Allows OHC to offer "Buy Postage & Print Label" directly inside the order fulfillment screen.
*   **Pricing**: Pay-as-you-go (5¢ per label) or $10/mo for professional.
*   **Mode Support**: Cloud and Standalone.

### Design Doc
*   **Trigger**: An order is placed for a physical product. User clicks "Fulfill Order".
*   **Action**: OHC requests shipping rates from Shippo based on package weight/dimensions. User selects a rate. OHC purchases the label via Shippo API and returns the PDF.
*   **User View**: A simple "Get Shipping Label" button on the order details page. The customer automatically gets an email with the tracking link.

### Implementation Prompt
Integrate Shippo API to provide real-time shipping rate estimates during customer checkout. Add functionality in the OHC Operations dashboard to purchase and download shipping labels (PDFs) for physical orders. Automatically attach tracking numbers to orders and notify the Customer Success agent to send tracking emails.

*   **Priority**: P1
*   **Estimated Scope**: Large

---

## 6. SMS & Notifications: Twilio
### Problem Statement
Fatima (Food Cart) operates in a fast-paced environment and doesn't always check email. She needs a loud SMS notification when a new pickup order arrives. Her customers also appreciate an SMS when their food is ready.

### Research Report
*   **Target Tool**: Twilio
*   **Persona Benefited**: Fatima (Food Cart), Carlos (Handyman)
*   **Evaluation**: Twilio is the industry standard for programmatic SMS. It has global reach and high reliability.
*   **Integration Value**: Crucial for real-time operational alerts for businesses that are "on the go" and for sending critical customer updates (e.g., "Your table is ready", "Carlos is 10 mins away").
*   **Pricing**: Pay-as-you-go (~$0.0079 per SMS in the US).
*   **Mode Support**: Cloud (OHC managed Twilio account) and Standalone (User provides Twilio keys).

### Design Doc
*   **Trigger**: A new order is placed (for Fatima), or an order status changes to "Ready".
*   **Action**: OHC backend calls Twilio API to send an SMS to the business owner or the customer.
*   **User View**: Fatima's phone buzzes with a text: "New order: 2 Chicken over Rice. $18." The customer gets a text: "Your order from Fatima's Cart is ready for pickup!"

### Implementation Prompt
Integrate Twilio SMS API. Add a notification preference setting for business owners to receive SMS alerts for new orders or bookings. Implement customer-facing SMS notifications for order status updates (e.g., "Ready for pickup"). Ensure phone numbers are formatted correctly and handle opt-out compliance.

*   **Priority**: P0
*   **Estimated Scope**: Medium

---

## 7. Video Conferencing: Zoom API
### Problem Statement
Leo (Music Tutor) teaches online. Manually creating a Zoom link for every booking and emailing it to the student is tedious and error-prone.

### Research Report
*   **Target Tool**: Zoom API
*   **Persona Benefited**: Leo (Tutor), Carlos (Handyman - for virtual estimates)
*   **Evaluation**: Zoom is universally recognized. The API allows for automatic meeting creation.
*   **Integration Value**: Zero-touch workflow for online services. A student books a lesson, and everything is set up automatically.
*   **Pricing**: Zoom Basic (Free, 40-min limit), Pro ($15.99/mo). API access available.
*   **Mode Support**: Cloud and Standalone.

### Design Doc
*   **Trigger**: A customer books a service marked as "Online/Virtual" (e.g., via the Calendly integration or native booking).
*   **Action**: OHC calls the Zoom API (via user's connected Zoom account) to generate a unique meeting link for that specific timeslot.
*   **User View**: Leo's calendar event automatically includes the "Join Zoom" link. The student receives a confirmation email with the same link.

### Implementation Prompt
Integrate the Zoom API. Allow users to connect their Zoom accounts. When a virtual service is booked, automatically generate a unique Zoom meeting link. Display this link in the user's dashboard and include it in the customer's booking confirmation email.

*   **Priority**: P1
*   **Estimated Scope**: Medium
