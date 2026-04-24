# Tool Integration Research Report - Q3

This report evaluates key third-party tools across critical business functions to expand OneHumanCorp's (OHC) capabilities. Each evaluation focuses on the value delivered to non-technical small business owners (our core personas).

---

## 1. Social Media Integration
### [Social Media] Meta Graph API (Unified Inbox for FB, IG, WhatsApp)

**Problem Statement:**
Business owners like Maya (The Home Baker) receive customer inquiries across Instagram DMs, Facebook comments, and WhatsApp. Jumping between apps is overwhelming, and she misses messages while sleeping or baking. She needs a single, unified inbox within OHC where her "Customer Success Ambassador" AI can automatically draft replies.

**Research Report:**
- **Tool:** Meta Graph API (Instagram Messaging API, WhatsApp Business API).
- **Target Persona:** Maya (Home Baker), Priya (Boutique Owner).
- **Ease of Use:** Completely invisible to the user once authenticated via a simple OAuth flow ("Connect Instagram").
- **Key Advantages:** Native integration with the platforms our users already use. High reliability. Real-time webhooks.
- **Risks:** Meta's API review process can be stringent. Risk of token expiration requiring user re-authentication.
- **Pricing:** Free for basic messaging; WhatsApp charges per conversation (~$0.015 - $0.08 depending on region/type).
- **Cloud/Standalone Support:** Works in Cloud (webhooks to OHC Cloud). In Standalone, requires an intermediate OHC relay or polling if direct webhooks are blocked by local NATs.

**Design Doc:**
- **Trigger:** User clicks "Connect Social Media" in the Settings tab and authenticates via Meta OAuth.
- **Action:** OHC subscribes to webhooks for new DMs/comments. When a message arrives, it appears in the OHC "Unified Inbox". The Customer Success AI drafts a reply.
- **User View:** A clean "Messages" tab on the OHC mobile app where Instagram DMs, WhatsApps, and emails all appear in one continuous chat thread per customer.

**Implementation Prompt:**
Implement the Meta Graph API integration to support reading and replying to Instagram DMs and Facebook page messages. Build the OAuth connection flow in the UI. Ensure incoming messages create a notification and appear in the existing OHC Inbox interface. The AI must be able to inject drafted responses into the reply box.

**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling
### [Calendar] Cal.com API

**Problem Statement:**
Leo (The Music Tutor) and Carlos (The Freelance Handyman) need customers to book their time without the back-and-forth of "what time works for you?". They need a seamless booking widget on their OHC storefront that syncs with their personal Google or Apple calendars to prevent double-booking.

**Research Report:**
- **Tool:** Cal.com API (Open Source scheduling infrastructure).
- **Target Persona:** Leo (Music Tutor), Carlos (Handyman).
- **Ease of Use:** Users just connect their personal calendar; OHC uses Cal.com under the hood to manage availability logic.
- **Key Advantages:** Open source, robust API, handles timezone math automatically, supports multiple calendar providers (Google, Outlook, Apple).
- **Risks:** Introducing another dependency for scheduling vs building a simple custom slot system.
- **Pricing:** API access starts at a moderate tier, but self-hosting is an option.
- **Cloud/Standalone Support:** Fully compatible with both (especially if self-hosted for Standalone mode).

**Design Doc:**
- **Trigger:** User enables "Booking/Services" mode and connects their Google/Apple Calendar.
- **Action:** OHC creates a managed Cal.com link/event type in the background. The OHC storefront renders a native-looking date/time picker.
- **User View:** Business owner sets working hours (e.g., 9 AM - 5 PM). Customers see available slots in their local timezone and can book with one click.

**Implementation Prompt:**
Integrate Cal.com's API to power the underlying scheduling engine for OHC service businesses. Build a UI for the business owner to define working hours and connect their external calendar. On the public storefront, display a native OHC date/time picker that fetches available slots from the API and confirms bookings.

**Priority:** P0
**Estimated Scope:** Medium

---

## 3. Email Marketing
### [Email] Resend

**Problem Statement:**
Priya (The Boutique Owner) wants to notify her 500 past customers when her new summer collection drops. Writing individual emails takes too long, and complex tools like Mailchimp have too much jargon (DKIM, segments, campaigns). She needs her AI to write the email and send it to everyone automatically.

**Research Report:**
- **Tool:** Resend API.
- **Target Persona:** Priya (Boutique Owner).
- **Ease of Use:** Extremely simple for the user. They just type "Tell my customers about the summer sale", and the AI drafts the email. OHC handles the sending via Resend.
- **Key Advantages:** Developer-friendly API, excellent deliverability, modern React-email templates.
- **Risks:** Need to handle spam compliance, bounce rates, and unsubscribe links meticulously to protect the OHC shared domain reputation.
- **Pricing:** Free up to 3,000 emails/month; $20/mo for 50,000 emails. Very affordable.
- **Cloud/Standalone Support:** Works in both via API keys.

**Design Doc:**
- **Trigger:** User tells the "Promoter" AI to send a marketing blast.
- **Action:** AI drafts the HTML email. User clicks "Approve & Send". OHC dispatches the emails via Resend API.
- **User View:** A simple "Broadcasts" screen showing the drafted email, recipient count, and a big "Send" button. Post-send, it shows simple stats: "Opened by 45 people."

**Implementation Prompt:**
Integrate the Resend API for outgoing marketing emails. Create an internal mailing list model that automatically includes customers who opted in. Build the UI for the user to approve AI-generated email drafts and view basic open/click metrics. Ensure automatic inclusion of mandatory "Unsubscribe" links.

**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing
### [Payments] Mercado Pago Integration (LATAM Focus)

**Problem Statement:**
While Stripe is fantastic for the US and Europe, users in Latin America (like a local variant of Fatima or Carlos in Brazil/Mexico) heavily rely on local payment methods like PIX (Brazil) or OXXO (Mexico). Stripe's coverage here is limited. They need a payment processor their local customers trust.

**Research Report:**
- **Tool:** Mercado Pago API.
- **Target Persona:** Fatima (Food Cart - LATAM variant), Carlos (Handyman - LATAM variant).
- **Ease of Use:** Standard OAuth integration similar to Stripe Connect.
- **Key Advantages:** Dominant market share in LATAM, supports PIX, Boleto, and local credit cards natively.
- **Risks:** API documentation can be fragmented; testing requires region-specific accounts.
- **Pricing:** Varies by country (e.g., ~3.99% per transaction depending on settlement time).
- **Cloud/Standalone Support:** Works in both via API keys and webhooks. Standalone might require webhook relays.

**Design Doc:**
- **Trigger:** User in LATAM selects "Mercado Pago" in the Payments setup.
- **Action:** User redirects to Mercado Pago to authorize OHC. Storefront checkout dynamically shows PIX/Boleto options instead of Apple Pay.
- **User View:** A seamless checkout experience for their customers using familiar local payment methods, leading to higher conversion rates.

**Implementation Prompt:**
Add Mercado Pago as a secondary payment provider alongside Stripe. Implement the OAuth connect flow for business owners and integrate the Mercado Pago Checkout Pro or transparent checkout into the public storefront. Ensure idempotency and webhook handling for async payments like PIX.

**Priority:** P1
**Estimated Scope:** Large

---

## 5. Shipping & Logistics
### [Shipping] Shippo API

**Problem Statement:**
Priya (The Boutique Owner) and Maya (The Home Baker) spend hours copying and pasting customer addresses into USPS or FedEx websites to buy shipping labels. They need a magic button that buys the label and sends the tracking number to the customer automatically.

**Research Report:**
- **Tool:** Shippo API.
- **Target Persona:** Priya (Boutique), Maya (Home Baker).
- **Ease of Use:** High. Abstracted behind a "Generate Label" button in OHC.
- **Key Advantages:** Multi-carrier support (USPS, UPS, FedEx, DHL) through a single unified API. Good rates.
- **Risks:** Handling edge cases like incorrect addresses, package dimensions, and international customs forms.
- **Pricing:** $0.05 per label or flat monthly rate, plus postage cost.
- **Cloud/Standalone Support:** Fully compatible via API.

**Design Doc:**
- **Trigger:** User opens a "Paid" order and taps "Create Shipping Label".
- **Action:** OHC hits Shippo API with order weight/dimensions and customer address. Shippo returns a printable PDF label and tracking number.
- **User View:** A "Print Label" button directly on the Order Details screen. The system automatically triggers the "Customer Success" AI to email the tracking link.

**Implementation Prompt:**
Integrate the Shippo API to allow business owners to purchase and print shipping labels directly from the OHC order management screen. Implement address validation, shipping rate fetching, and label PDF generation. Automate tracking number assignment to the OHC order record.

**Priority:** P1
**Estimated Scope:** Medium

---

## 6. SMS & Notifications
### [SMS] Twilio

**Problem Statement:**
Fatima (Food Cart Operator) needs to know instantly when a pre-order arrives, even if she's not looking at the OHC app. Her customers also need a text message saying "Your order is ready for pickup!" because they might not check email while walking to her cart.

**Research Report:**
- **Tool:** Twilio Programmable SMS.
- **Target Persona:** Fatima (Food Cart Operator), Carlos (Handyman).
- **Ease of Use:** Invisible to the business owner. They just see a toggle: "Send SMS updates to customers."
- **Key Advantages:** Global reach, ultra-reliable delivery, supports two-way messaging if we want to expand later.
- **Risks:** SMS is expensive and heavily regulated (A2P 10DLC compliance in the US). High risk of spam blocks if not managed properly.
- **Pricing:** ~$0.0079 per outbound message in the US.
- **Cloud/Standalone Support:** Works in both via API.

**Design Doc:**
- **Trigger:** Order status changes to "Ready for Pickup" or "On the Way".
- **Action:** OHC triggers Twilio API to send a pre-configured, personalized SMS to the customer's phone number.
- **User View:** Fatima taps "Order Ready" on her phone. The customer immediately gets a text. Fatima can also opt-in to receive SMS alerts for new orders if her internet is spotty.

**Implementation Prompt:**
Integrate the Twilio SMS API to send transactional text messages (order confirmations, pickup alerts, booking reminders). Implement the necessary opt-in UI during customer checkout. Add robust error handling for invalid phone numbers and failed deliveries.

**Priority:** P0
**Estimated Scope:** Small

---

## 7. Video Conferencing
### [Video] Zoom API

**Problem Statement:**
Leo (The Music Tutor) teaches guitar lessons online. Currently, he manually creates a Zoom meeting for every booking and emails the link to the student. He needs the booking system to automatically generate the link and put it on both their calendars.

**Research Report:**
- **Tool:** Zoom API (Server-to-Server OAuth or standard OAuth).
- **Target Persona:** Leo (Music Tutor).
- **Ease of Use:** One-time OAuth connection. After that, links appear automatically on bookings.
- **Key Advantages:** Zoom is the industry standard. High user familiarity.
- **Risks:** Zoom's OAuth approval process for marketplace apps can be lengthy. Token management is historically finicky.
- **Pricing:** Free tier API access is sufficient for basic meeting generation.
- **Cloud/Standalone Support:** Works well in Cloud. In Standalone, users might need to provide their own Zoom Server-to-Server credentials if local OAuth redirection is tricky.

**Design Doc:**
- **Trigger:** A customer books a service marked as "Online Meeting".
- **Action:** OHC calls Zoom API to create a scheduled meeting. OHC saves the `join_url` and sends it to the customer via email/calendar invite.
- **User View:** Leo sees a "Join Zoom" button next to his upcoming appointment in the OHC dashboard. The customer gets the same link in their confirmation email.

**Implementation Prompt:**
Integrate the Zoom API to automatically generate meeting links for online service bookings. Create the OAuth connection flow in the Settings tab. Ensure that when a booking is created, rescheduled, or canceled, the corresponding Zoom meeting is updated or deleted via the API. Display the join link prominently on the appointment details screen.

**Priority:** P2
**Estimated Scope:** Medium
