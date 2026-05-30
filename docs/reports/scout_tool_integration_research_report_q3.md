# Tool Integration Research Q3

<div style="backdrop-filter: blur(15px) saturate(200%); -webkit-backdrop-filter: blur(15px) saturate(200%); background-color: rgba(255, 255, 255, 0.45); border-radius: 12px; border: 1px solid rgba(209, 213, 219, 0.3); padding: 1.5rem; margin-bottom: 2rem;">
  <h2 style="font-family: 'Outfit', sans-serif; font-size: 1.5rem; margin-top: 0;">Debt Report</h2>
  <p style="font-family: 'Inter', sans-serif; margin-bottom: 0;">
    No technical debt generated in this research. The output consists of integration evaluations that will be addressed in future sprints to enhance user experience without compounding existing technical burdens.
  </p>
</div>

## Issue Briefs

### [Social Media Integration] Unified Inbox with Meta API
**Title**: Integrate Meta API for Unified Instagram/Facebook Inbox
**Problem Statement**: Small business owners like Fatima struggle to keep up with customer messages scattered across Instagram, Facebook, and WhatsApp. Replying late costs them sales, and constantly switching apps is exhausting. They need a single, simple place to see and reply to all customer inquiries.
**Research Report**:
- **Tool Evaluated**: Meta Graph API (Messenger/Instagram Direct/WhatsApp Business)
- **Ease of Use**: Once connected via OAuth, the user experience is seamless. All messages appear in one inbox.
- **Pricing**: Facebook/Instagram messaging is free. WhatsApp Business API charges per conversation (varies by region, typically $0.01 - $0.08).
- **Reputation**: Official, reliable, but Meta's API updates require maintenance.
- **Hybrid Support**: Works in both Cloud and Standalone (requires redirect URIs to be handled via custom scheme or public auth proxy for Standalone).
**Design Doc**:
- The user clicks "Connect Instagram/Facebook" in the OHC Settings.
- They are redirected to Meta's secure login and approve OHC to access their messages.
- New messages trigger a webhook to OHC, appearing instantly in the OHC "Unified Inbox" tab.
- Replies sent from OHC are pushed back via Meta API to the customer's original app.
**Implementation Prompt**: Build a unified inbox interface and connect it to Meta's API. The user should be able to log in to Meta, select their business page, and start receiving/sending messages within OHC without technical configuration.
**Priority**: P0
**Estimated Scope**: Large

---

### [Calendar & Scheduling] Google Calendar Two-Way Sync
**Title**: Two-Way Google Calendar Sync for Client Bookings
**Problem Statement**: Service-based businesses (consultants, salons) get double-booked when clients schedule meetings through OHC but the owner manually adds personal appointments to Google Calendar. They need a system that prevents double-booking automatically.
**Research Report**:
- **Tool Evaluated**: Google Calendar API
- **Ease of Use**: Familiar OAuth flow for users. High trust.
- **Pricing**: Free tier covers most SMB usage (Standard API limits).
- **Reputation**: Industry standard, extremely reliable.
- **Hybrid Support**: Fully supported in Cloud. Standalone requires a companion cloud-proxy for the OAuth redirect or local loopback handling.
**Design Doc**:
- User connects their Google account via the Settings > Calendar tab.
- OHC reads existing Google Calendar events and blocks out those times on the OHC public booking page.
- When a client books via OHC, the event is automatically added to the user's Google Calendar.
**Implementation Prompt**: Implement OAuth2 connection to Google Calendar. Ensure the public booking page accurately reflects busy times from Google, and new OHC bookings sync to Google Calendar instantly.
**Priority**: P0
**Estimated Scope**: Medium

---

### [Email Marketing] Mailchimp Audience Sync
**Title**: Automated Customer Sync to Mailchimp
**Problem Statement**: Business owners collect customer emails in OHC but have to manually export and import CSV files into Mailchimp to send newsletters or promotions. This manual work means they often skip marketing altogether.
**Research Report**:
- **Tool Evaluated**: Mailchimp Marketing API
- **Ease of Use**: Easy OAuth connection. Mailchimp interface is widely understood by SMBs.
- **Pricing**: Free up to 500 contacts, then tiered pricing.
- **Reputation**: Market leader, reliable webhooks.
- **Hybrid Support**: Works well in Cloud. Standalone can easily push data out via API.
**Design Doc**:
- User connects Mailchimp via Settings > Integrations.
- User selects a default Mailchimp "Audience" (List).
- Whenever a new customer is added or makes a purchase in OHC, their email and name are silently pushed to the selected Mailchimp audience.
**Implementation Prompt**: Create a one-way sync from OHC customers to a chosen Mailchimp audience. Provide a simple toggle to enable/disable the sync, and handle API rate limits gracefully in the background.
**Priority**: P1
**Estimated Scope**: Medium

---

### [Payment Processing] Mercado Pago Integration for LATAM
**Title**: Native Mercado Pago Checkout Integration
**Problem Statement**: Small businesses in Latin America often cannot use Stripe. They need a trusted, local payment processor like Mercado Pago to accept customer payments via credit card, PIX, or Boleto directly within OHC invoices.
**Research Report**:
- **Tool Evaluated**: Mercado Pago API Checkout
- **Ease of Use**: Very popular and trusted by consumers in LATAM. Dashboard is easy for merchants.
- **Pricing**: Varies by country, typically 3-5% per transaction.
- **Reputation**: Leading payment processor in Latin America.
- **Hybrid Support**: Works in both Cloud and Standalone (webhooks needed for payment confirmation).
**Design Doc**:
- User enters their Mercado Pago Access Token in Settings > Payments.
- When sending an invoice from OHC, a "Pay with Mercado Pago" button is generated.
- Customers click to pay via a secure Mercado Pago hosted checkout.
- Successful payments update the invoice status to "Paid" in OHC.
**Implementation Prompt**: Add Mercado Pago as a payment provider option. Generate checkout links for invoices and process payment confirmation webhooks to automatically mark invoices as paid.
**Priority**: P1
**Estimated Scope**: Medium

---

### [Shipping & Logistics] Shippo Label Generation
**Title**: One-Click Shipping Label Generation via Shippo
**Problem Statement**: E-commerce users spend hours copying addresses from orders into carrier websites (USPS, FedEx) to print shipping labels. They need a way to buy and print labels directly from the order page.
**Research Report**:
- **Tool Evaluated**: Shippo API
- **Ease of Use**: Aggregates many carriers. Easy account setup for merchants.
- **Pricing**: 5 cents per label + postage costs. Free tier available.
- **Reputation**: Reliable, excellent developer experience.
- **Hybrid Support**: Supported in Cloud and Standalone (direct API calls).
**Design Doc**:
- User connects a Shippo account.
- On an OHC "Order" screen, a "Buy Shipping Label" button appears.
- User enters package weight, selects a carrier rate (e.g., USPS Priority), and clicks confirm.
- OHC charges Shippo, retrieves the PDF label, and displays it for printing.
**Implementation Prompt**: Integrate Shippo to allow fetching live shipping rates and purchasing labels directly from an order detail view. The end user should be able to print the resulting PDF with one click.
**Priority**: P2
**Estimated Scope**: Large

---

### [SMS & Notifications] Twilio SMS Notifications
**Title**: Automated SMS Reminders via Twilio
**Problem Statement**: Appointment no-shows cost service businesses money. Customers often ignore emails but check their text messages. Business owners need a hands-off way to text customers reminders before an appointment.
**Research Report**:
- **Tool Evaluated**: Twilio Programmable SMS
- **Ease of Use**: Complex initial setup (buying a number, A2P 10DLC registration in US) but seamless once configured.
- **Pricing**: ~$0.0079 per SMS in US. Varies globally.
- **Reputation**: Industry standard, highest reliability.
- **Hybrid Support**: Supported in Cloud and Standalone.
**Design Doc**:
- User configures Twilio credentials or buys SMS credits directly through OHC (white-labeled).
- When an appointment is booked, a scheduled job is created for 24 hours prior.
- At the scheduled time, an SMS is sent to the customer's phone number reminding them of the appointment.
**Implementation Prompt**: Build a background task that sends SMS reminders using Twilio for upcoming calendar events. Provide a settings area where the business owner can customize the reminder message template.
**Priority**: P1
**Estimated Scope**: Medium

---

### [Video Conferencing] Zoom Auto-Meeting Generation
**Title**: Auto-Generate Zoom Links for Online Consultations
**Problem Statement**: Coaches and consultants have to manually create a Zoom meeting and email the link to the client after every booking. They need the meeting link to be generated and attached to the calendar invite instantly.
**Research Report**:
- **Tool Evaluated**: Zoom API
- **Ease of Use**: OAuth flow is standard. Users are highly familiar with Zoom.
- **Pricing**: Requires a paid Zoom Pro account for API access/longer meetings.
- **Reputation**: Ubiquitous, highly reliable.
- **Hybrid Support**: Supported in Cloud and Standalone (similar OAuth proxy requirements as Google Calendar).
**Design Doc**:
- User connects their Zoom account via Settings.
- When configuring a service (e.g., "1 Hour Consultation"), the user selects "Location: Zoom".
- When a client books, OHC calls the Zoom API, creates a meeting, and includes the join URL in the confirmation email and calendar event.
**Implementation Prompt**: Integrate Zoom OAuth and meeting creation API. When a user books an online service, automatically generate a unique Zoom link and display it in the booking confirmation and notifications.
**Priority**: P2
**Estimated Scope**: Medium
