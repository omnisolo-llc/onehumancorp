# Tool Integration Research Report [quarter]

## Executive Summary
This report outlines the comprehensive evaluation of key tool categories essential for empowering small business owners through the OHC platform. By integrating these specific tools, we address critical pain points across communication, scheduling, logistics, and payments, enabling business owners to save time and increase revenue.

### Research Methodology
- Analyzed top pain points from existing user feedback.
- Evaluated leading tools in each category based on API maturity, pricing, ease of use, and multi-tenant cloud readiness.
- Developed architectural integration plans that prioritize a seamless, unified user experience.

---

## Detailed Tool Issue Briefs

### Issue Brief: [social_media] Integrate Buffer Unified API for Cross-Platform Social Inbox

**Problem Statement:**
Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp messages, and TikTok comments. Non-technical users like Fatima lose potential sales because they miss messages.

**Research Report:**
The Buffer Unified API provides a single endpoint for receiving and replying to messages across all major social platforms. Buffer is well-regarded for developer experience and stable webhook reliability. Once integrated, the business owner sees a single, intuitive chat interface. They don't need to authenticate multiple times daily. Pricing is approx. $15-50/month per tenant depending on message volume. Cloud vs Standalone: Fully functional in Cloud mode. Standalone mode might require individual OAuth proxy setups to bypass platform callback URL restrictions.

**Design Doc:**
1. Trigger: Customer comments on TikTok or sends an Instagram DM.
2. Action: Buffer sends a webhook to the OHC backend. The backend maps the external user ID to a unified customer profile.
3. User View: The message appears in the Unified Inbox with a platform icon. The business owner replies, and OHC routes it back.

**Implementation Prompt:**
Create a unified inbox interface and integrate it with a unified social API. Acceptance Criteria: The user can connect Instagram and Facebook. Messages appear in a single chronological feed. Users can reply natively. UI must be mobile-responsive.

**Priority:** P0
**Estimated Scope:** Large

---

### Issue Brief: [calendar] Integrate Cal.com API for Automated Scheduling

**Problem Statement:**
Business owners spend hours playing email ping-pong to schedule consultations. Double bookings damage their reputation, and manual time zone conversions lead to missed meetings.

**Research Report:**
Cal.com provides a white-label, open-source friendly scheduling infrastructure. It allows embedding a booking widget directly into the business's storefront and handles calendar syncing. It is highly respected in the developer community for its robust API and open-source nature. Business owners just need to connect their calendar once. Pricing is open source (free if self-hosted) or ~$12/user/month. Cloud vs Standalone: Works exceptionally well in both. Standalone users can theoretically self-host the Cal.com instance, while Cloud users use the managed service.

**Design Doc:**
1. Trigger: A customer visits the booking page and selects an available time slot.
2. Action: Cal.com API creates the event, checks for conflicts, and generates a video link.
3. User View: The business owner receives a push notification and the time slot is blocked.

**Implementation Prompt:**
Implement a scheduling system powered by the Cal.com API. Acceptance Criteria: User can define weekly availability. Customers can visit a public booking page. System prevents double booking. Automated email confirmations sent.

**Priority:** P1
**Estimated Scope:** Medium

---

### Issue Brief: [email_marketing] Integrate Resend for High-Deliverability Email Campaigns

**Problem Statement:**
Small businesses struggle to re-engage past customers. Setting up traditional tools like Mailchimp is too complex, and sending bulk emails from a personal Gmail results in spam.

**Research Report:**
Resend is a developer-first email platform focusing on high deliverability, clean APIs, and React Email templates. It allows programmatic creation of beautiful campaigns. SMBs never see Resend; they see a simple 'Send Newsletter' button in OHC. Pricing is $20/mo for 50,000 emails. Very cost-effective. Cloud vs Standalone: Primarily Cloud. Standalone users would need to provide their own Resend API key or fallback to a local SMTP server.

**Design Doc:**
1. Trigger: The business owner schedules an email campaign.
2. Action: The OHC backend batches the emails, renders templates, and queues them via Resend API.
3. User View: The owner sees a progress bar, then later sees open and click rates.

**Implementation Prompt:**
Build a simple email campaign manager powered by Resend. Acceptance Criteria: Users can draft a simple rich-text email, select a list of customers, and reliably deliver emails. Basic open-rate tracking visible.

**Priority:** P2
**Estimated Scope:** Medium

---

### Issue Brief: [payment] Integrate Mercado Pago for LATAM Market

**Problem Statement:**
Stripe is not universally accessible in LATAM. Small businesses in Brazil, Argentina, and Mexico lose sales without local payment methods like Pix or local credit card installments.

**Research Report:**
Mercado Pago is the leading payment processor in Latin America. It supports local payment methods that international gateways miss. Ubiquitous in LATAM and highly trusted. Familiar to LATAM business owners. Pricing varies by country, ~3-5% per transaction. Cloud vs Standalone: Works in both, but Standalone users handle their own API keys.

**Design Doc:**
1. Trigger: Customer initiates checkout in a LATAM region.
2. Action: OHC requests a checkout preference ID from Mercado Pago API and renders the Checkout Pro UI.
3. User View: Business owner sees successful payments denominated in local currency.

**Implementation Prompt:**
Add Mercado Pago as an alternative payment gateway. Acceptance Criteria: User can connect Mercado Pago account. Customers can pay using local methods like Pix. Webhooks accurately update order status to 'Paid'.

**Priority:** P1
**Estimated Scope:** Large

---

### Issue Brief: [shipping] Integrate Shippo for Real-Time Label Generation

**Problem Statement:**
Fulfilling physical orders is a nightmare. Copy-pasting addresses into post office websites to buy shipping labels is error-prone and wastes hours.

**Research Report:**
Shippo provides a multi-carrier shipping API to compare rates across USPS, UPS, FedEx, DHL, and generate printable labels. Highly rated for its dashboard and API uptime. Simplifies logistics. Pricing is $0.05 per label or $10/month for advanced features. Cloud vs Standalone: Excellent in both. In Cloud, OHC can negotiate master rates.

**Design Doc:**
1. Trigger: Business owner initiates fulfillment for a paid order.
2. Action: OHC sends parcel data to Shippo. Shippo returns rates. User selects rate, OHC purchases label.
3. User View: A PDF opens for printing. Order status updates to 'Shipped'.

**Implementation Prompt:**
Integrate Shippo for shipping label generation. Acceptance Criteria: Connect to Shippo's API to fetch live rates. Allow owner to purchase label and download PDF. Automatically attach tracking number to order.

**Priority:** P2
**Estimated Scope:** Large

---

### Issue Brief: [sms] Integrate Twilio for Global SMS Alerts

**Problem Statement:**
In emerging markets, email open rates are abysmal. If an appointment reminder is only sent via email, the no-show rate remains high.

**Research Report:**
Twilio is the industry standard for programmatic SMS, Voice, and WhatsApp routing. Offers global reach and high deliverability. Zero friction for SMBs; happens automatically in background. Pricing is ~$0.0079 per message in the US. Cloud vs Standalone: Cloud is preferred due to A2P 10DLC compliance complexities. Standalone users would have a steep learning curve registering their own campaigns.

**Design Doc:**
1. Trigger: An event occurs (e.g., 24 hours before an appointment).
2. Action: A background job triggers the Twilio API with the customer's phone number and a localized template.
3. User View: Business owner sees a log indicating 'SMS sent'.

**Implementation Prompt:**
Implement automated SMS notifications via Twilio. Acceptance Criteria: Send SMS for order confirmations and reminders. Handle invalid phone numbers gracefully. Respect customer opt-out preferences.

**Priority:** P1
**Estimated Scope:** Medium

---

### Issue Brief: [video] Integrate Zoom API for Auto-Generated Consultation Links

**Problem Statement:**
Service-based businesses waste time manually creating Zoom links for every booking and emailing them to clients.

**Research Report:**
Zoom's API allows applications to dynamically create meetings via OAuth. Zoom is the default verb for video conferencing, offering highest consumer trust. Users authorize Zoom once. Free for the integration, user pays their own Zoom subscription if needed. Cloud vs Standalone: Works well in both via standard OAuth flows.

**Design Doc:**
1. Trigger: A new booking is finalized for a virtual service.
2. Action: OHC uses the user's stored OAuth refresh token to request a new meeting via Zoom API.
3. User View: The generated join URL is displayed in the booking details and emailed to client.

**Implementation Prompt:**
Integrate Zoom to auto-generate meeting links. Acceptance Criteria: Secure OAuth flow. Automatically create unique meeting link when booked. Display link clearly in client dashboard. Handle token refresh gracefully.

**Priority:** P2
**Estimated Scope:** Medium

---
