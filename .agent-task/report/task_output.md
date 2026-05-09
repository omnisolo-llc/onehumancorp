# 🔎 Scout: Tool Integration Research Q4

## 1. Social Media Integration: Unified Inbox for Meta

### Title
Integrate Meta Messaging (Instagram/Facebook/WhatsApp) to Unified Inbox

### Problem Statement
Small business owners receive customer inquiries scattered across Instagram DMs, Facebook Messenger, and WhatsApp. Checking multiple apps constantly takes time away from running the business and leads to missed sales opportunities. They need all customer messages to flow into one unified inbox.

### Research Report
Meta's official APIs support connecting Instagram, Facebook Messenger, and WhatsApp Business accounts.
*   **Ease of Use:** Moderate setup (requires Meta Business Suite linking), but highly intuitive once active.
*   **Pricing:** Free for Instagram/Messenger; WhatsApp Business API has per-conversation pricing.
*   **Reputation:** Essential. Meta platforms are where small businesses connect with consumers.
*   **Cloud vs. Standalone:** Meta Webhooks require a public endpoint, making this primarily functional in Cloud mode. Standalone mode might require polling or a cloud-relay service.

### Design Doc
A "Connect Socials" page will let users authenticate with Meta via OAuth. OHC will listen to Meta webhooks for incoming messages and display them in the OHC unified inbox. Replying from OHC will send the message back via the Meta Graph API to the customer's original platform.

### Implementation Prompt
Create an OAuth flow for Meta Business accounts. Implement a unified inbox view that aggregates incoming messages from Instagram, Facebook, and WhatsApp. Ensure the business owner can reply directly from OHC and the message correctly routes back to the customer on the original app.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling: Calendly Integration

### Title
Integrate Calendly for Automated Scheduling

### Problem Statement
Business owners spend hours going back-and-forth via email or SMS to find a time to meet with clients, leading to missed opportunities and double bookings. They need a simple way to share a link where customers can book time directly into their calendar.

### Research Report
Calendly is a widely recognized scheduling tool that syncs with personal calendars.
*   **Ease of Use:** Extremely high for both the business owner and the customer.
*   **Pricing:** Free basic tier; paid tiers starting around $10-15/month.
*   **Reputation:** Industry standard for scheduling.
*   **Cloud vs. Standalone:** Calendly is cloud-based. Cloud mode integrates via webhooks; Standalone mode can fetch updates via API polling.

### Design Doc
The integration will allow the business owner to connect their Calendly account. OHC will display upcoming Calendly events on the dashboard and automatically create a corresponding task/event within the OHC system when a new booking occurs.

### Implementation Prompt
Implement a Calendly integration where the business owner can authenticate their Calendly account. OHC should display upcoming events on the dashboard and provide a quick way to copy their scheduling link. When an event is booked, it should immediately reflect in OHC.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing: Mailchimp Sync

### Title
Integrate Mailchimp for Customer Email Campaigns

### Problem Statement
Small businesses want to send newsletters and promotions but struggle to manually export customer lists from their management system into their email tool. They need their customer database to sync automatically with their email marketing platform.

### Research Report
Mailchimp is the standard for small business email marketing.
*   **Ease of Use:** User-friendly drag-and-drop builder for the business owner.
*   **Pricing:** Generous free tier for small lists; scalable paid options.
*   **Reputation:** Trusted and reliable.
*   **Cloud vs. Standalone:** Fully supported in both modes via REST API.

### Design Doc
The integration will sync the OHC customer list (names, emails) directly to a Mailchimp audience. Business owners can map OHC tags to Mailchimp tags to enable targeted campaigns.

### Implementation Prompt
Build a one-way sync from OHC's customer directory to Mailchimp. The owner should authenticate via OAuth. Provide a toggle to automatically add new OHC customers to the connected Mailchimp audience. Show high-level campaign metrics (open rate) in OHC.

### Priority
P2

### Estimated Scope
Medium

---

## 4. Payment Processing: Mercado Pago Integration

### Title
Integrate Mercado Pago for LATAM Payments

### Problem Statement
Stripe is great, but it doesn't support many local payment methods essential for LATAM small businesses (like PIX in Brazil or OXXO in Mexico). Business owners need a localized payment processor to successfully capture sales in their region.

### Research Report
Mercado Pago is the dominant payment gateway in Latin America.
*   **Ease of Use:** Familiar to LATAM users; provides simple checkout links.
*   **Pricing:** Transaction-based fees varying by country.
*   **Reputation:** Highly trusted in the LATAM market.
*   **Cloud vs. Standalone:** Supported in both via API and Webhooks.

### Design Doc
OHC will allow generating Mercado Pago checkout links directly from an invoice or order. Payment status will automatically update to "Paid" in OHC when the webhook confirms the transaction.

### Implementation Prompt
Implement a Mercado Pago payment provider option alongside Stripe. Allow the business owner to connect their account and generate payment links for invoices. Ensure the invoice status automatically updates when payment succeeds.

### Priority
P1

### Estimated Scope
Large

---

## 5. Shipping & Logistics: Shippo Integration

### Title
Integrate Shippo for Label Generation

### Problem Statement
Product-based businesses waste time manually re-entering address data into carrier websites to buy shipping labels. They need a way to instantly calculate rates and print labels directly from their orders.

### Research Report
Shippo aggregates dozens of carriers (USPS, UPS, FedEx, DHL).
*   **Ease of Use:** Simple API for developers; clear label outputs for owners.
*   **Pricing:** Pay-as-you-go per label ($0.05) or monthly subscription.
*   **Reputation:** Reliable shipping API.
*   **Cloud vs. Standalone:** Works well in both modes via REST API.

### Design Doc
On any order in OHC, add a "Create Label" button. OHC will send package dimensions and weights to Shippo, return rates, and let the owner purchase and download the PDF label directly inside OHC.

### Implementation Prompt
Add a shipping module powered by Shippo. Allow the business owner to enter box dimensions and weight to fetch live rates. Provide a checkout flow to purchase the label and save the tracking number to the order record.

### Priority
P2

### Estimated Scope
Large

---

## 6. SMS & Notifications: Twilio Integration

### Title
Integrate Twilio for Global SMS Notifications

### Problem Statement
Many small business customers, especially those with lower English proficiency or in areas where email isn't the primary communication method, prefer SMS. Business owners need a reliable way to send appointment reminders and quick notifications.

### Research Report
Twilio is a robust communications platform for SMS.
*   **Ease of Use:** OHC abstracts the complexity; owner just sees "Send SMS".
*   **Pricing:** Very cost-effective per-message pricing.
*   **Reputation:** Enterprise-grade reliability.
*   **Cloud vs. Standalone:** In Standalone, user provides API keys. In Cloud, OHC manages it.

### Design Doc
OHC uses Twilio to power SMS features. The business owner can enable automated SMS reminders for appointments and send manual texts from the unified inbox.

### Implementation Prompt
Add SMS capability powered by Twilio. Provide a simple interface to send ad-hoc SMS messages to a customer directly from their profile, and a toggle for automated appointment reminders.

### Priority
P0

### Estimated Scope
Large

---

## 7. Video Conferencing: Zoom Auto-Links

### Title
Integrate Zoom for Auto-Generating Meeting Links

### Problem Statement
Consultants, tutors, and coaches waste time manually creating Zoom links for every online appointment and sending them to clients. They need meetings to automatically include a unique video link.

### Research Report
Zoom is the most popular video conferencing tool.
*   **Ease of Use:** High familiarity for end-users.
*   **Pricing:** Free for 40-min meetings; paid plans available.
*   **Reputation:** Ubiquitous and reliable.
*   **Cloud vs. Standalone:** Supports both via OAuth and REST API.

### Design Doc
When an appointment is marked as "Online/Video", OHC will automatically call the Zoom API to generate a unique meeting room and attach the link to the appointment details and calendar invite.

### Implementation Prompt
Implement an OAuth connection to Zoom. When creating a new appointment in OHC, add an option to "Make this a Zoom meeting." If selected, automatically generate a Zoom link and display it on the appointment card.

### Priority
P2

### Estimated Scope
Medium
