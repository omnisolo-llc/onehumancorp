# Social Media Integration Research

## [Social Media] Instagram DM & Facebook Comments Unified Inbox

**Problem Statement:**
As a small business owner, I get customer inquiries across Instagram DMs, Facebook comments, and WhatsApp. It is exhausting to constantly check different apps, and I often miss messages or reply late, costing me sales. I need one single place to see and reply to all my customers, no matter where they message me.

**Research Report:**
- **Market/Competitors:** Tools like ManyChat, Chatwoot (which we already integrate), Buffer, and Hootsuite offer unified inboxes.
- **Evaluation:**
  - **Chatwoot:** Since we already have Chatwoot in our Docker stack, expanding its use to handle social channels is a natural fit. It supports Facebook, Instagram, and WhatsApp via official APIs.
  - **Ease of Use:** High. Once connected, the user sees a simple chat interface.
  - **Pricing:** Free tier available for self-hosted; Cloud pricing is reasonable ($19/mo per agent).
  - **Reputation:** Well-regarded open-source tool.
  - **Cloud vs Standalone:** Works perfectly in both. In Standalone, it connects to cloud APIs (Meta) directly.

**Design Doc:**
- **Integration Point:** Expand the existing Chatwoot integration in OHC to surface a "Connect Social Accounts" wizard.
- **Triggers:** New message received on Instagram/FB -> Meta Webhook -> Chatwoot -> OHC Unified Inbox UI.
- **User View:** A single "Inbox" tab in the OHC dashboard showing messages from all channels, with simple text input to reply.

**Implementation Prompt:**
Create a "Unified Inbox" view in the OHC dashboard that aggregates messages from Instagram, Facebook, and WhatsApp. Include a settings wizard that guides the business owner through authenticating with Meta to connect their pages. The inbox must allow reading and replying to messages directly.

**Priority:** P0
**Estimated Scope:** Large

---

# Calendar & Scheduling Research

## [Calendar] Auto-Generate Booking Pages & Meeting Links

**Problem Statement:**
Scheduling consultations with clients involves too much back-and-forth over email ("What time works for you?"). I need a simple webpage I can send clients where they can pick a time, and it automatically puts it on my Google Calendar and generates a video link.

**Research Report:**
- **Market/Competitors:** Calendly, Cal.com, Acuity Scheduling.
- **Evaluation:**
  - **Cal.com:** Open-source, developer-friendly API, excellent UI.
  - **Ease of Use:** Very high for the business owner (just set availability) and the client (just pick a slot).
  - **Pricing:** Free for individuals, affordable team plans.
  - **Reputation:** Strong open-source community, highly reliable.
  - **Cloud vs Standalone:** Works in both. Can be integrated via API or self-hosted.

**Design Doc:**
- **Integration Point:** A new "Scheduling" tab in OHC.
- **Triggers:** Client books slot -> Cal.com webhook -> OHC creates "Meeting" record -> OHC emails client confirmation.
- **User View:** Business owner sets working hours in OHC. Gets a unique URL to share. Dashboard shows upcoming meetings.

**Implementation Prompt:**
Implement a scheduling feature that allows users to connect their Google Calendar and set their availability. Generate a shareable booking link. When a client books a time, automatically add it to the OHC "Upcoming Meetings" list and ensure a Zoom/Meet link is created and emailed to both parties.

**Priority:** P1
**Estimated Scope:** Medium

---

# Payment Processing Research

## [Payments] Global Payment Gateways for Emerging Markets

**Problem Statement:**
Stripe doesn't work well or is too expensive for my customers in Latin America or India. I need a way to accept payments using local methods (like PIX in Brazil or UPI in India) so I don't lose sales at checkout.

**Research Report:**
- **Market/Competitors:** Mercado Pago (LATAM), Razorpay/Paytm (India), dLocal.
- **Evaluation:**
  - **Mercado Pago & Razorpay:** Dominant in their respective regions. High trust among local consumers.
  - **Ease of Use:** Moderate (requires local business registration), but essential for conversion.
  - **Pricing:** Transaction-based, generally lower than Stripe for local methods.
  - **Cloud vs Standalone:** API-based, works in both.

**Design Doc:**
- **Integration Point:** "Payment Settings" in OHC.
- **Triggers:** Checkout initiated -> OHC routes to regional provider based on customer IP -> Payment success webhook -> OHC marks invoice paid.
- **User View:** A simple toggle to "Enable Mercado Pago" or "Enable Razorpay" alongside Stripe.

**Implementation Prompt:**
Add support for regional payment providers (specifically Mercado Pago for LATAM and Razorpay for India) in the payment configuration settings. Provide a clear UI for the business owner to input their API keys for these services and a toggle to activate them for customer checkouts.

**Priority:** P2
**Estimated Scope:** Medium

---

# Shipping & Logistics Research

## [Shipping] Automated Shipping Rates & Label Generation

**Problem Statement:**
Calculating shipping costs manually for every order is tedious, and going to the post office to buy labels wastes hours of my day. I need the system to calculate shipping automatically at checkout and let me print labels from home.

**Research Report:**
- **Market/Competitors:** Shippo, EasyPost, ShipStation.
- **Evaluation:**
  - **Shippo / EasyPost:** API-first, supports dozens of carriers (USPS, UPS, FedEx, international).
  - **Ease of Use:** High.
  - **Pricing:** Pay per label (cents) + postage.
  - **Reputation:** Highly reliable APIs.
  - **Cloud vs Standalone:** Works in both via API.

**Design Doc:**
- **Integration Point:** "Orders/Fulfillment" tab in OHC.
- **Triggers:** New order -> OHC calls Shippo API for label -> User clicks "Print Label".
- **User View:** An "Orders" list where each unfulfilled order has a one-click "Buy & Print Shipping Label" button.

**Implementation Prompt:**
Integrate an order fulfillment flow using a provider like Shippo. The UI should display pending orders with a button to instantly calculate the shipping cost based on package dimensions and generate a printable PDF shipping label.

**Priority:** P1
**Estimated Scope:** Large

---

# SMS & Notifications Research

## [Notifications] Automated SMS Alerts for Low-English Proficiency Users

**Problem Statement:**
Many of my workers or customers (like Fatima) prefer text messages over emails or complicated apps. I need a way to automatically text them updates (like "Your shift changed" or "Your order is ready") reliably, globally.

**Research Report:**
- **Market/Competitors:** Twilio, MessageBird, Plivo.
- **Evaluation:**
  - **Twilio:** Industry standard, immense global reach.
  - **Ease of Use:** Business owner just types a message or sets a template.
  - **Pricing:** Pay-per-message (fractions of a cent to a few cents depending on country).
  - **Cloud vs Standalone:** Works in both.

**Design Doc:**
- **Integration Point:** "Notifications/Messaging" tab and automated workflow triggers.
- **Triggers:** Event occurs (e.g., order ready) -> OHC calls Twilio API -> SMS delivered.
- **User View:** A simple interface to write a broadcast text message, or toggles to turn on "SMS Alerts for Orders".

**Implementation Prompt:**
Add an SMS notification capability using a provider like Twilio. Create a UI where the business owner can configure automated SMS alerts for key events (like order updates or appointments) and view a log of sent messages to ensure delivery.

**Priority:** P1
**Estimated Scope:** Medium


---

# Email Marketing Research

## [Email Marketing] Unified Email Campaigns

**Problem Statement:**
I want to send newsletters and promotional emails to my customer list to drive repeat business. I need an email marketing tool that integrates directly with my OHC contacts so I don't have to manually export and import CSV files every time I want to send an email.

**Research Report:**
- **Market/Competitors:** Listmonk, Mailchimp, SendGrid Marketing Campaigns.
- **Evaluation:**
  - **Listmonk:** Open-source, self-hosted friendly, powerful and fast.
  - **Ease of Use:** Moderate. Requires some initial setup for SMTP, but campaign creation is straightforward.
  - **Pricing:** Free (self-hosted), only pay for SMTP (e.g., AWS SES or SendGrid).
  - **Cloud vs Standalone:** Works perfectly in both. In Standalone, runs locally alongside OHC.

**Design Doc:**
- **Integration Point:** "Marketing/Email" tab in OHC.
- **Triggers:** User creates campaign -> OHC syncs contacts to Listmonk -> Listmonk sends emails.
- **User View:** A simple UI to write an email, select a segment of customers (e.g., "all past customers"), and click send.

**Implementation Prompt:**
Integrate an email marketing solution like Listmonk. Provide a UI where the business owner can draft a promotional email, select a target audience from their existing OHC contact list, and send the campaign. The system should automatically handle syncing the contacts and tracking open rates.

**Priority:** P2
**Estimated Scope:** Large

---

# Video Conferencing Research

## [Video] Auto-Generate Video Links for Consultations

**Problem Statement:**
When I schedule online consultations or remote support sessions, I have to manually create a Zoom link and email it to the client. Sometimes I forget or send the wrong link. I need a way to automatically generate and send a video meeting link whenever a client books a remote session.

**Research Report:**
- **Market/Competitors:** Google Meet, Zoom, Jitsi Meet.
- **Evaluation:**
  - **Jitsi Meet:** Open-source, no account required for guests, can be self-hosted.
  - **Ease of Use:** Very high. One-click join for clients directly in the browser.
  - **Pricing:** Free.
  - **Cloud vs Standalone:** Works in both. Can use public Jitsi server or self-host.

**Design Doc:**
- **Integration Point:** "Appointments/Meetings" tab in OHC.
- **Triggers:** Remote appointment booked -> OHC generates unique Jitsi room URL -> OHC adds URL to confirmation email and calendar event.
- **User View:** A toggle on an appointment type that says "Online Video Call". When checked, OHC automatically handles the rest.

**Implementation Prompt:**
Integrate a video conferencing solution like Jitsi Meet for online appointments. Add a feature to the scheduling system that allows business owners to designate an appointment as "Remote." When booked, automatically generate a unique video meeting URL and include it in the confirmation notifications sent to the client and the business owner.

**Priority:** P2
**Estimated Scope:** Medium
