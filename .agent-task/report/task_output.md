---
issue_category: docs
---

<div style="backdrop-filter: blur(15px) saturate(200%); background: rgba(255, 255, 255, 0.1); border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.2); padding: 20px;">
  <h3>Debt Report</h3>
  <p>No technical debt was introduced during this research phase. All proposed integrations will be designed as independent modules to minimize impact on core systems.</p>
</div>

# Tool Integration Research Report

## [Social Media] Unified Inbox via Meta Graph API

### Title
Integrate Instagram DMs and WhatsApp via Meta Graph API into Unified Inbox

### Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram, Facebook, and WhatsApp. They need a single place to view and respond to all customer inquiries without constantly switching apps.

### Research Report
- **Tool**: Meta Graph API (Instagram Messaging API, WhatsApp Business API)
- **Ease of Use**: High for the end user once connected via OAuth. The setup flow requires linking their Facebook page to their Instagram/WhatsApp accounts.
- **Pricing**: Free for Instagram DMs. WhatsApp Business API has conversation-based pricing (first 1,000 service conversations are free per month).
- **Reputation/Reliability**: High reliability, but Meta's API policies and webhook requirements can be strict.
- **Environment**: Works in Cloud (webhooks) and Standalone (requires ngrok or similar tunneling, or polling if supported, though webhooks are standard).

### Design Doc
- **Integration**: A new settings page allows users to "Connect Meta Accounts" using OAuth. Once connected, OHC subscribes to webhooks for incoming messages.
- **User Interface**: New messages appear in the OHC unified inbox. Users can reply directly from OHC, and the response is sent back through the Meta Graph API to the customer's native app.
- **Triggers**: Incoming webhook from Meta triggers a new message event in OHC. User reply in OHC triggers an API call to Meta.

### Implementation Prompt
Implement the OAuth connection flow for Meta Graph API. Create the webhook listener to receive incoming Instagram DMs and WhatsApp messages and store them in the unified inbox. Implement the outgoing message sender to reply to customers. Ensure the UI clearly shows the source of each message (Instagram vs. WhatsApp).

### Priority
P1

### Estimated Scope
Large

---

## [Calendar] Google Calendar Sync and Scheduling

### Title
Two-Way Google Calendar Sync for Client Bookings

### Problem Statement
Business owners often double-book themselves because their personal or business Google Calendar isn't connected to their OHC booking page. They need their availability to automatically reflect their Google Calendar events.

### Research Report
- **Tool**: Google Calendar API
- **Ease of Use**: Extremely easy for the business owner. A simple "Sign in with Google" button connects their calendar.
- **Pricing**: Free (standard Google API quotas apply, which are usually sufficient for small businesses).
- **Reputation/Reliability**: Industry standard, highly reliable.
- **Environment**: Fully supported in both Cloud and Standalone modes via standard OAuth 2.0.

### Design Doc
- **Integration**: Add a "Connect Google Calendar" option in the booking settings. When connected, OHC fetches busy slots to block off availability on the public booking page.
- **User Interface**: The booking page only shows time slots where the business owner is free. New bookings made via OHC are pushed to the Google Calendar as new events.
- **Triggers**: Scheduled background sync or real-time API queries for availability. New booking creation triggers an event creation API call to Google.

### Implementation Prompt
Build the OAuth flow for Google Calendar. Implement a service to read free/busy information to block out unavailable times on the OHC booking interface. Implement a service to push new OHC bookings directly to the user's Google Calendar.

### Priority
P0

### Estimated Scope
Medium

---

## [Email Marketing] Customer Email Campaigns via Resend

### Title
Integrated Email Marketing Campaigns using Resend

### Problem Statement
Business owners want to send newsletters or promotional updates to their customer list but find complex tools like Mailchimp overwhelming and expensive. They want a simple way to email their existing contacts directly from OHC.

### Research Report
- **Tool**: Resend API
- **Ease of Use**: Very developer-friendly for us to build a simple UI on top. The business owner won't interact with Resend directly; they'll just use a simple email editor in OHC.
- **Pricing**: Very generous free tier (3,000 emails/month free), making it ideal for small businesses.
- **Reputation/Reliability**: Excellent deliverability and modern API.
- **Environment**: Works seamlessly in both Cloud and Standalone modes.

### Design Doc
- **Integration**: Users draft an email in a simple OHC rich-text editor and select an audience (e.g., "All past customers"). OHC dispatches the emails via the Resend API.
- **User Interface**: A new "Campaigns" tab where users can write an email, preview it, and send it. Basic stats (sent, bounced) can be pulled from Resend.
- **Triggers**: User clicking "Send Campaign" triggers batch API calls to Resend.

### Implementation Prompt
Create a simple rich-text email editor in the UI. Implement the integration with Resend API to send batch emails to selected customer segments. Provide a basic dashboard showing sent emails and any bounce notifications.

### Priority
P2

### Estimated Scope
Medium

---

## [Payment Processing] Mercado Pago Integration for LATAM

### Title
Mercado Pago Checkout Integration

### Problem Statement
Stripe is not always the preferred or supported payment method in Latin America. Small business owners in LATAM need a familiar, trusted payment gateway like Mercado Pago to accept local payments easily.

### Research Report
- **Tool**: Mercado Pago API
- **Ease of Use**: Well-known in LATAM. The checkout experience is seamless for customers.
- **Pricing**: Transparent transaction fees competitive with local alternatives.
- **Reputation/Reliability**: The dominant payment processor in Latin America.
- **Environment**: Supported in Cloud and Standalone modes.

### Design Doc
- **Integration**: Add Mercado Pago as an alternative payment provider in the billing settings. OHC generates a preference ID to launch the Mercado Pago checkout.
- **User Interface**: Customers checking out can select Mercado Pago, which opens a secure payment modal or redirects them to complete the payment.
- **Triggers**: Invoice generation or checkout initiates a Mercado Pago preference. Webhooks update the invoice status in OHC when payment succeeds.

### Implementation Prompt
Implement the Mercado Pago API integration. Allow business owners to connect their Mercado Pago credentials. Update the checkout flow to support Mercado Pago as a payment option and handle the webhook callbacks to mark invoices as paid.

### Priority
P1

### Estimated Scope
Medium

---

## [Shipping & Logistics] Automated Shipping Labels via Shippo

### Title
Automated Shipping Rate Calculation and Label Generation via Shippo

### Problem Statement
Business owners selling physical goods waste time manually entering addresses into carrier websites to buy shipping labels. They need to generate labels and track shipments directly from their OHC orders dashboard.

### Research Report
- **Tool**: Shippo API
- **Ease of Use**: The API abstracts away multiple carriers (USPS, UPS, FedEx, local carriers). Business owners just hit "Generate Label" in OHC.
- **Pricing**: Pay-as-you-go (approx $0.05 per label plus postage). Very affordable.
- **Reputation/Reliability**: Highly reliable, standard choice for e-commerce platforms.
- **Environment**: Supported in both Cloud and Standalone.

### Design Doc
- **Integration**: In the order fulfillment view, business owners can input box dimensions and weight to get real-time rates via Shippo.
- **User Interface**: A "Create Shipping Label" button on the order details page. It displays available rates, allows purchase, and provides a printable PDF label and tracking number.
- **Triggers**: User requests rates -> Shippo API call. User buys label -> Shippo API call, OHC saves tracking URL.

### Implementation Prompt
Integrate the Shippo API to fetch real-time shipping rates based on order weight and destination. Implement the label purchase flow, allowing users to download the PDF label directly from the OHC order dashboard and automatically attach tracking info to the order.

### Priority
P2

### Estimated Scope
Large

---

## [SMS & Notifications] Automated SMS Notifications via Twilio

### Title
Global SMS Notifications for Appointments and Orders

### Problem Statement
Many customers (and business owners with low English proficiency) prefer text messages over emails. Businesses need to send automated SMS reminders for appointments or order updates to reduce no-shows and keep customers informed.

### Research Report
- **Tool**: Twilio Programmable SMS
- **Ease of Use**: Users just toggle "Send SMS Reminders" in settings. They don't need to interact with Twilio directly.
- **Pricing**: Pay-as-you-go per message (pennies per SMS). Requires the business to buy a phone number.
- **Reputation/Reliability**: Industry leader, excellent global coverage.
- **Environment**: Works in Cloud and Standalone.

### Design Doc
- **Integration**: Business owners connect a Twilio account or OHC provisions sub-accounts. Automated triggers send templated SMS messages.
- **User Interface**: Settings toggles for "SMS Appointment Reminder (24h before)" and "SMS Order Shipped".
- **Triggers**: Background worker checks for upcoming appointments and fires Twilio API calls. Order status changes trigger notification API calls.

### Implementation Prompt
Integrate the Twilio SMS API. Create a background job to scan for appointments exactly 24 hours away and send a templated SMS reminder. Add a simple UI settings page for the business owner to enable/disable SMS notifications and view their message history.

### Priority
P1

### Estimated Scope
Medium

---

## [Video Conferencing] Auto-Generating Zoom Links for Consultations

### Title
Automatic Zoom Link Generation for Online Bookings

### Problem Statement
When a customer books an online consultation or lesson, the business owner currently has to manually create a Zoom link and email it to them, which is error-prone and time-consuming.

### Research Report
- **Tool**: Zoom API
- **Ease of Use**: Standard OAuth flow. The business owner clicks "Connect Zoom" and it works invisibly thereafter.
- **Pricing**: Free for the integration, requires a Zoom account (free tier works but has a 40-minute limit).
- **Reputation/Reliability**: Ubiquitous, highly reliable.
- **Environment**: Cloud and Standalone support.

### Design Doc
- **Integration**: When a booking is created for a service marked as "Online/Virtual", OHC calls the Zoom API to create a meeting.
- **User Interface**: The booking confirmation page and emails automatically include the unique Zoom join link for both the customer and the business owner.
- **Triggers**: Successful booking creation of a virtual service triggers a Zoom meeting creation API call.

### Implementation Prompt
Build the OAuth connection for Zoom. Update the booking creation service to check if the service is virtual; if so, create a Zoom meeting and save the join URL and host URL to the booking record. Ensure these links are displayed in the booking confirmation UI and emails.

### Priority
P1

### Estimated Scope
Medium
