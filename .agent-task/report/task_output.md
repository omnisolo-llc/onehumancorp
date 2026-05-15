# OHC Tool Integration Research Report

## 1. Social Media Integration: WhatsApp Business API

### Title
Enable WhatsApp Business Direct Messaging and Auto-Replies

### Problem Statement
Many small business owners communicate directly with their customers on WhatsApp, especially in regions outside the US. Currently, they have to jump between OHC and their personal or business WhatsApp app. They want to be able to see WhatsApp messages in the same inbox as their other customer communications and set up simple auto-replies for out-of-office hours or FAQs, without needing technical skills.

### Research Report
The WhatsApp Business API (now integrated into the Meta Graph API) is the industry standard for this.
- **Ease of Use for Non-Technical Users:** Connecting the API requires a Facebook Developer account and Meta Business Manager verification, which is highly complex for our average user. However, if OHC acts as an embedded Solution Provider (BSP), the user only needs to go through a simple OAuth flow (Embedded Signup).
- **Pricing:** The first 1,000 service-category conversations per month are free. After that, it costs a few cents per conversation depending on the region.
- **Reputation:** Meta's infrastructure is very reliable, but their approval policies can be strict and accounts are occasionally blocked for spam without warning.
- **Support:** Works in both Cloud and Standalone modes. In Standalone mode, the user would need to create their own Meta App or we provide an API gateway.

### Design Doc
- **Trigger:** A new WhatsApp message arrives at the webhook.
- **Action:** The message is parsed and inserted into the user's unified inbox in OHC. If an auto-reply is configured for that time/keyword, OHC immediately sends a response via the WhatsApp API.
- **User Interface:** A new "WhatsApp" tab in the unified inbox. A settings page with a "Connect WhatsApp" button that launches the Meta Embedded Signup popup. A section to define simple text auto-replies.

### Implementation Prompt
Create a module that allows a business owner to link their WhatsApp Business account via Meta Embedded Signup. Once linked, any incoming WhatsApp messages should appear in their OHC inbox. Allow the user to reply from the OHC inbox, and configure a basic "Out of Office" auto-reply that triggers outside of their configured business hours. The technical integration must handle the Meta Webhook validation and token refresh silently in the background.

### Priority
P1

### Estimated Scope
Large

---

## 2. Calendar & Scheduling: Cal.com

### Title
Automated Appointment Scheduling via Cal.com

### Problem Statement
Small business owners (consultants, tutors, local services) waste hours emailing back and forth to find meeting times. They need a simple link they can text or email to clients that shows their actual availability and lets the client book a time instantly, without overlapping with existing commitments.

### Research Report
Cal.com is an open-source, highly customizable scheduling tool that competes with Calendly.
- **Ease of Use:** Extremely intuitive for the end-user (client). The setup for the business owner is straightforward: connect a calendar and define working hours.
- **Pricing:** The core scheduling infrastructure is free and open-source. For teams and advanced features, it starts at $12/user/month.
- **Reputation:** Rapidly growing, modern API, developer-friendly, and highly regarded for privacy and flexibility.
- **Support:** Exceptional fit for OHC because it can be self-hosted (perfect for Standalone mode) or consumed via their managed API (Cloud mode).

### Design Doc
- **Trigger:** A customer clicks the booking link and selects a time slot.
- **Action:** Cal.com creates a calendar event and triggers a webhook to OHC. OHC creates/updates the customer record and logs the appointment in the CRM timeline.
- **User Interface:** A "Scheduling" settings page where the owner can paste their Cal.com API key or connect via OAuth. A dashboard widget showing "Upcoming Appointments".

### Implementation Prompt
Build an integration with Cal.com. The business owner should be able to authenticate their Cal.com account in OHC. Once connected, OHC should display a "Copy Booking Link" button on the dashboard. When a customer books a meeting through that link, the appointment details should automatically populate in the OHC customer timeline. The system must support both the managed Cal.com service and a self-hosted instance endpoint.

### Priority
P2

### Estimated Scope
Medium

---

## 3. Shipping & Logistics: Shippo

### Title
Automated Shipping Label Generation and Tracking via Shippo

### Problem Statement
Small e-commerce businesses waste time manually copying and pasting customer addresses into carrier websites (like USPS, UPS, FedEx) to buy shipping labels. They need a way to instantly compare shipping rates, print labels, and automatically send tracking numbers to customers directly from their order dashboard.

### Research Report
Shippo is a multi-carrier shipping API and web app designed for e-commerce.
- **Ease of Use:** Very user-friendly. Once an account is connected, comparing rates and printing labels is a 1-click process.
- **Pricing:** They offer a "Starter" plan that is free to use (no monthly fee) for up to 30 labels/month or when connecting own carrier accounts. They also have a Pro plan starting at $17/mo. They offer discounted rates on major carriers.
- **Reputation:** Highly reliable, processes millions of shipments, and is a standard in the e-commerce infrastructure space.
- **Support:** Works in Cloud mode via API. For Standalone mode, the API requires internet access to connect to Shippo servers.

### Design Doc
- **Trigger:** A business owner marks an order as "Ready to Ship" in OHC.
- **Action:** OHC requests shipping rates from Shippo based on package weight/dimensions and customer address. The owner selects a rate, and OHC generates the label via Shippo and emails the tracking number to the customer.
- **User Interface:** On the Order Details page, a "Create Shipping Label" button. A modal to input weight/dimensions and select a carrier rate. A "Print Label" button that outputs a PDF.

### Implementation Prompt
Integrate the Shippo API to allow business owners to generate shipping labels directly from an order. The owner should be able to input package dimensions, see a list of rates from different carriers, purchase the label, and print it as a PDF. Automatically update the order status to "Shipped" and save the tracking URL to the order record. The technical implementation must handle address validation to prevent failed label purchases.

### Priority
P1

### Estimated Scope
Medium

---

## 4. Email Marketing: Listmonk

### Title
Integrated Email Campaigns with Listmonk

### Problem Statement
Small business owners struggle to keep their customer contact lists in sync with third-party email tools (like Mailchimp) to send newsletters or promotions. They need a built-in way to send beautiful email campaigns directly to their OHC customer base without paying expensive monthly fees based on subscriber counts.

### Research Report
Listmonk is a standalone, self-hosted newsletter and mailing list manager.
- **Ease of Use:** Has a clean dashboard for managing lists, templates, and campaigns.
- **Pricing:** 100% free and open-source. The only cost is the underlying SMTP provider (e.g., AWS SES, SendGrid) which is exponentially cheaper than Mailchimp.
- **Reputation:** Fast, reliable (written in Go), and highly scalable.
- **Support:** Native fit for Standalone mode (runs as a local binary/container). For Cloud mode, it can be provided as a managed microservice per tenant or shared infrastructure.

### Design Doc
- **Trigger:** A business owner creates a new campaign and clicks "Send to List".
- **Action:** OHC triggers the Listmonk API to queue and dispatch the emails via the configured SMTP backend. Open/click metrics are fed back into OHC.
- **User Interface:** A new "Marketing" tab featuring an email template builder, a campaign dashboard, and performance metrics (opens, clicks). Lists are automatically synced from the OHC CRM tags.

### Implementation Prompt
Integrate Listmonk to handle email marketing natively. Ensure that customer segments or tags in the OHC CRM automatically correspond to Listmonk mailing lists. Build a seamless UI within OHC where the user can design an email campaign and hit send, without realizing they are interacting with Listmonk under the hood. Display the resulting open and click rates back in the OHC dashboard.

### Priority
P2

### Estimated Scope
Large

---

## 5. Payment Processing: Mercado Pago

### Title
Expand LATAM Payment Options with Mercado Pago

### Problem Statement
While Stripe is great, it doesn't support the preferred payment methods in many Latin American countries, where credit card penetration is lower and local methods (like Pix in Brazil or OXXO in Mexico) dominate. Business owners in these regions lose sales if they can't offer local payment options.

### Research Report
Mercado Pago is the leading payment gateway in Latin America.
- **Ease of Use:** Straightforward checkout experience for the end-user. For the merchant, it requires a standard OAuth connection or API key setup.
- **Pricing:** Variable by country and payment method, but generally standard payment processing rates for the region.
- **Reputation:** Dominant market leader in LATAM, high trust among consumers there.
- **Support:** Cloud-based API. Works in both Cloud and Standalone modes as long as there is an internet connection for processing the transaction.

### Design Doc
- **Trigger:** A customer proceeds to checkout on an invoice or storefront.
- **Action:** OHC presents Mercado Pago as a payment option. Upon selection, the user is redirected to the Mercado Pago secure checkout or an embedded iframe. Upon completion, a webhook notifies OHC of payment success.
- **User Interface:** In the "Payments" settings, add a "Connect Mercado Pago" button alongside Stripe. On the client-facing invoice, a new "Pay with Mercado Pago" button.

### Implementation Prompt
Integrate Mercado Pago as an alternative payment gateway. Allow business owners to connect their Mercado Pago credentials. Update the checkout flow for invoices and online bookings to present Mercado Pago as a payment option. Ensure Webhooks are securely handled to automatically mark OHC invoices as "Paid" once the transaction clears.

### Priority
P1

### Estimated Scope
Medium

---

## 6. SMS & Notifications: Twilio

### Title
Reliable Global SMS Notifications via Twilio

### Problem Statement
Many business owners serve clients with lower English proficiency or less reliable internet access (like Fatima's persona). These clients prefer and respond best to direct SMS text messages for appointment reminders, order updates, or quick questions, rather than email.

### Research Report
Twilio is the industry-standard API for programmatic SMS and voice communications.
- **Ease of Use:** For the business owner, zero setup if OHC handles it natively. If they bring their own account, it requires copying Account SID and Auth Token.
- **Pricing:** Very cheap, typically fractions of a cent per message in the US, varying globally.
- **Reputation:** The gold standard for reliability, global carrier coverage, and delivery rates.
- **Support:** Works seamlessly in both Cloud and Standalone modes via API.

### Design Doc
- **Trigger:** An event occurs in OHC (e.g., appointment tomorrow, order shipped).
- **Action:** OHC hits the Twilio API to dispatch a pre-configured SMS template to the customer's phone number.
- **User Interface:** A "Notifications" settings panel where owners can toggle "Send SMS Reminders" on/off and customize the text template.

### Implementation Prompt
Implement Twilio SMS integration to enable automated text notifications. Allow the business owner to enable SMS for specific events (like appointment reminders 24 hours prior). The implementation must handle phone number formatting (E.164) and provide a basic interface for the owner to customize the message content using simple variables like `[Customer Name]` and `[Time]`.

### Priority
P1

### Estimated Scope
Medium

---

## 7. Video Conferencing: Google Meet

### Title
Automated Video Link Generation with Google Meet

### Problem Statement
Business owners offering online consultations, lessons, or remote services waste time manually generating video meeting links and sending them to clients. They need a video link automatically attached to every booked appointment.

### Research Report
Google Meet is a ubiquitous, reliable, and free video conferencing tool tied to Google Workspace/Gmail.
- **Ease of Use:** Most users already have a Google account, making integration a simple 1-click Google OAuth flow. Clients just click the link to join, often without needing to download an app.
- **Pricing:** Free for basic use, which covers most small business 1-on-1 needs.
- **Reputation:** Highly reliable, secure, and familiar to most users.
- **Support:** Works via API in both Cloud and Standalone modes.

### Design Doc
- **Trigger:** A new appointment is booked (e.g., via Cal.com integration or manually).
- **Action:** If "Online Meeting" is selected, OHC calls the Google Calendar API to create an event with conference data enabled, returning a Google Meet link.
- **User Interface:** An "Integrations" page with a "Connect Google Calendar/Meet" button. On the appointment creation screen, a simple toggle: "Add Google Meet link".

### Implementation Prompt
Integrate Google Meet via the Google Calendar API. Allow the business owner to authenticate with their Google account. When a new meeting is scheduled within OHC, automatically generate a Google Meet link and attach it to the appointment record. Ensure this link is included in the confirmation email/SMS sent to the customer.

### Priority
P2

### Estimated Scope
Medium
