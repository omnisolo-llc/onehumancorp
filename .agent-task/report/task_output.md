# Scout: Tool Integration Research Q3

## Executive Summary
This report evaluates third-party tools across various categories (Social Media, Calendar, Email Marketing, Payments, Shipping, SMS, and Video Conferencing) to assess their viability for integration into the OHC platform. The evaluations are conducted strictly through the lens of non-technical small business owners, focusing on ease of use, pricing, and capabilities in both Cloud and Standalone environments.

Following the evaluations, issue briefs are provided for the most promising candidates to guide implementation.

---

## 1. Social Media Integration

### Evaluation: ManyChat
* **Problem Solved**: Centralizes Instagram DMs, Facebook comments, and WhatsApp messages into a unified inbox. Small business owners often miss inquiries because they have to check 4 different apps constantly.
* **User Perspective**: The owner connects their social accounts once. After that, any DM or comment on an IG post appears in their OHC Inbox like a normal message. They reply from OHC, and it posts back natively to Instagram/Facebook.
* **Key Advantages**: Native integrations with Meta products, robust APIs, supports auto-responses (e.g., "Thanks for your message, I'll reply within 24hrs!").
* **Risks**: Meta's strict OAuth and webhook policies require careful compliance. Can be overwhelming for non-technical users if exposed to ManyChat's full builder instead of just the inbox integration.
* **Pricing Estimate**: Free tier available (up to 1,000 contacts); Pro starts at $15/month.
* **Environment**: Works seamlessly in Cloud via webhooks. Standalone mode requires a relay service to proxy webhooks to local instances.

### Evaluation: Twilio (WhatsApp Business API)
* **Problem Solved**: Direct WhatsApp integration for customer communication.
* **User Perspective**: Dedicated business phone number for WhatsApp. Customers text it, and messages land in the OHC Inbox.
* **Key Advantages**: High reliability, global reach.
* **Risks**: WhatsApp approval process can be tedious for small businesses. Pricing per conversation can escalate.
* **Pricing Estimate**: Pay-as-you-go per conversation (e.g., ~$0.015 - $0.08 depending on region/type).
* **Environment**: Cloud-friendly. Standalone requires similar webhook relays as ManyChat.

---

## 2. Calendar & Scheduling

### Evaluation: Calendly API / Nylas
* **Problem Solved**: Replaces the manual back-and-forth of "when are you free?" for scheduling client meetings, classes, or consultations.
* **User Perspective**: The owner sets their working hours in OHC. They get a unique link (`ohc.com/book/mybusiness`) to send to clients. When a client books, it automatically appears on the owner's Google/Outlook Calendar.
* **Key Advantages**: Solves timezone math automatically, robust sync across multiple calendar providers (Google, Microsoft), prevents double-booking.
* **Risks**: Calendar sync errors (e.g., deleted events not propagating) can cause missed appointments and angry clients.
* **Pricing Estimate**: Nylas: ~$1/connected account/month (at scale). Calendly: API access requires higher-tier plans ($16/user/mo).
* **Environment**: Cloud-friendly. Standalone mode may require local OAuth termination or a cloud relay for webhook notifications.

---

## 3. Email Marketing

### Evaluation: Mailchimp / SendGrid
* **Problem Solved**: Allows the owner to send newsletters, promotions, or updates to their entire customer list without getting flagged as spam.
* **User Perspective**: The owner selects a visual template, types their message, and clicks "Send to all past customers." OHC handles the formatting and delivery behind the scenes.
* **Key Advantages**: High deliverability, built-in unsubscribe management (crucial for compliance), analytics (open/click rates).
* **Risks**: Strict anti-spam rules. A business owner uploading a low-quality purchased list could get the OHC platform flagged if using shared IPs.
* **Pricing Estimate**: SendGrid: ~$20/mo for 50k emails. Mailchimp: Free up to 500 contacts, then scales rapidly.
* **Environment**: Cloud and Standalone both supported via standard API calls.

---

## 4. Payment Processing

### Evaluation: Mercado Pago (LATAM) & Razorpay (India)
* **Problem Solved**: Stripe isn't universal. Small businesses in specific regions need local payment methods (e.g., PIX in Brazil, UPI in India) to actually get paid.
* **User Perspective**: The owner connects their local bank account. When they send an invoice through OHC, the customer sees familiar local payment options (QR codes, local wallets) instead of just credit card forms.
* **Key Advantages**: Dramatically increases conversion rates in target regions. Faster settlement times for local merchants.
* **Risks**: Integrating multiple regional gateways increases maintenance burden. Handling varying settlement delays and dispute processes.
* **Pricing Estimate**: Variable by region (e.g., ~2-3% + fixed fee per transaction). No upfront cost for the merchant.
* **Environment**: Cloud and Standalone supported (API driven).

---

## 5. Shipping & Logistics

### Evaluation: Shippo / EasyPost
* **Problem Solved**: Calculating accurate shipping rates and buying postage labels is complex and time-consuming for product-based businesses.
* **User Perspective**: When an order comes in, the owner clicks "Generate Label". OHC shows the cheapest rate (USPS, FedEx, etc.), charges their card on file, and prints the label. Tracking info is automatically emailed to the customer.
* **Key Advantages**: Aggregates multiple carriers into one API. Provides discounted commercial rates to small businesses.
* **Risks**: Customs documentation for international shipping is highly error-prone for users. Address validation failures.
* **Pricing Estimate**: ~$0.05 per label created + postage costs.
* **Environment**: Cloud and Standalone supported (API driven).

---

## 6. SMS & Notifications

### Evaluation: Twilio / MessageBird
* **Problem Solved**: Email open rates are low. SMS ensures critical messages (appointment reminders, order ready for pickup) are actually seen, especially for users with lower English proficiency who rely heavily on texting.
* **User Perspective**: The owner checks a box that says "Send SMS reminder 24hrs before appointment." The system handles the rest seamlessly.
* **Key Advantages**: Near 100% open rate. Crucial for last-minute updates.
* **Risks**: Strict regulatory compliance (10DLC in the US, GDPR). High costs compared to email. Accidental spamming.
* **Pricing Estimate**: ~$0.0079 per SMS (US).
* **Environment**: Cloud and Standalone supported (API driven).

---

## 7. Video Conferencing

### Evaluation: Zoom API / Google Meet API
* **Problem Solved**: Generating a new meeting link for every online consultation or class is tedious and error-prone.
* **User Perspective**: When an online booking is made, a unique Zoom/Meet link is automatically generated and added to the calendar invite sent to both the owner and the customer.
* **Key Advantages**: Essential for service-based businesses (tutors, consultants). Reduces "no-shows" caused by missing links.
* **Risks**: OAuth token expiration can break link generation silently.
* **Pricing Estimate**: Included in base Zoom/Google Workspace plans, but API access limits may apply at scale.
* **Environment**: Cloud-friendly. Standalone requires OAuth flow management.

---

# Issue Briefs

## [Calendar] Unified Booking & Calendar Sync
**Priority:** P0 (Critical)
**Estimated Scope:** Large

**Problem Statement:**
Small business owners spend hours every week managing appointments via text/email, leading to double-bookings, missed meetings, and lost revenue. They need a simple way to say "here is when I am free" and have bookings automatically sync to their personal calendar.

**Research Report:**
Evaluated Calendly API and Nylas. Nylas provides a more robust, white-labeled solution for syncing across Google and Outlook, which is critical since our users don't want to manage a separate "Calendly" account—they just want it to work inside OHC. Nylas costs ~$1/user/mo at scale, which fits our pricing model. The integration must handle timezone conversions invisibly, as this is a major pain point for non-technical users.

**Design Doc:**
- **Trigger**: User navigates to Settings -> Scheduling and clicks "Connect Calendar".
- **Action**: Initiates OAuth flow for Google/Microsoft. Upon success, OHC reads free/busy times.
- **User Experience**: The user gets a customized public link (`ohc.com/book/business-name`). Customers visiting this link see available slots based on the synced calendar's free/busy data. When booked, the event is written back to the user's calendar immediately.
- **Cloud/Standalone**: OAuth must be proxied through an OHC relay service for Standalone instances to maintain security without requiring local users to register their own API keys with Google/Microsoft.

**Implementation Prompt:**
Implement a seamless calendar connection flow. The business owner should be able to connect their Google or Outlook calendar with two clicks. Provide a public-facing booking page that correctly displays available times (respecting the owner's calendar events) and automatically converts timezones for the visiting customer. Do not expose complex API keys or sync intervals to the user.

---

## [Social] Unified Social Inbox
**Priority:** P1 (High)
**Estimated Scope:** Large

**Problem Statement:**
Business owners are losing leads because they forget to check Instagram DMs, Facebook comments, or WhatsApp messages. They need one single place to read and reply to all customer inquiries.

**Research Report:**
Evaluated ManyChat and Twilio. A direct Meta Graph API integration is the most cost-effective and reliable method for IG/FB. Small businesses expect this to "just work" like their native apps. The biggest hurdle is Meta's strict business verification and OAuth requirements. We must insulate the user from Meta's complex developer console.

**Design Doc:**
- **Trigger**: User navigates to Inbox -> Connect Socials and selects "Instagram".
- **Action**: Standard Meta OAuth popup. OHC registers webhooks for `messages` and `comments` events.
- **User Experience**: A new message on Instagram appears in the OHC Inbox as a chat thread. The owner types a reply and hits send; it appears natively in the customer's IG app.
- **Cloud/Standalone**: Cloud mode handles webhooks directly. Standalone mode requires an OHC Cloud relay to receive Meta webhooks and forward them securely to the local instance (e.g., via a persistent WebSocket or polling mechanism).

**Implementation Prompt:**
Build a unified inbox view that aggregates messages from connected Meta accounts (Facebook/Instagram). The setup process must be a simple OAuth login with no developer configuration required by the user. The inbox must support basic rich media (images) and feel instantaneous when sending replies.

---

## [Payments] Regional Payment Gateway Expansion (LATAM)
**Priority:** P1 (High)
**Estimated Scope:** Medium

**Problem Statement:**
Stripe is insufficient for our growing LATAM user base. Businesses in Brazil and Mexico are losing sales because they cannot offer local payment methods like PIX or OXXO directly on their invoices.

**Research Report:**
Evaluated Mercado Pago. It is the dominant player in LATAM and essential for conversion. The API is straightforward, but the user experience must hide the complexity of currency conversion and regional tax requirements. There is no upfront cost, and it significantly increases the platform's value proposition in these regions.

**Design Doc:**
- **Trigger**: User in a supported region navigates to Billing -> Connect Payments.
- **Action**: OHC detects the region and offers Mercado Pago alongside Stripe. User authenticates via Mercado Pago OAuth.
- **User Experience**: When the owner generates an invoice, the public payment page dynamically displays local payment options (e.g., PIX QR code) instead of a standard credit card form.
- **Cloud/Standalone**: Fully supported in both modes via direct API calls from the backend to Mercado Pago.

**Implementation Prompt:**
Integrate Mercado Pago as an alternative payment provider for invoicing. The system should automatically suggest Mercado Pago during onboarding for users in applicable regions (Brazil, Mexico, Argentina). The public invoice view must cleanly present the local payment methods (like PIX) generated by the API without redirecting the user away from the OHC-branded invoice page if possible.