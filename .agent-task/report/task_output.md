# Scout Research Report

As a Research agent, I have evaluated multiple tool categories based on my autonomous task definition. The goal was to find tools that solve real problems for small business owners and can be seamlessly integrated into OHC. Below are the resulting issue briefs, which have also been saved in the `docs/technical/research/` directory.

## 1. Social Media Integration: Unified Inbox

**Problem Statement:** Small business owners receive customer inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Managing these scattered messages is overwhelming and leads to slow response times or missed sales opportunities. They need a single, unified inbox within OHC to read and reply to all customer messages.

**Research Report:**
*   **Evaluated Tools:** Meta Graph API (for Instagram/Facebook), WhatsApp Business API
*   **Alternatives Considered:** Smooch/Zendesk Sunshine, Twilio Conversations
*   **Pros:** Direct integration with Meta APIs ensures reliable delivery and access to rich message features (images, reactions). No middleman costs compared to aggregator services.
*   **Cons:** Meta's App Review process can be slow and strict. Requires managing long-lived page access tokens.
*   **Ease of Use for Non-technical Users:** The user clicks "Connect Instagram" or "Connect Facebook Page", completes the standard Meta OAuth flow, and instantly sees their DMs appear in the OHC Customer Success tab.
*   **Pricing:** Free for basic Graph API usage; WhatsApp pricing is conversation-based.
*   **Deployment:** Cloud-native (OAuth callbacks require a public webhook).

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A customer sends a DM on Instagram or WhatsApp.
    *   **Action:** Meta sends a webhook to OHC. OHC parses the payload and normalizes it into a standard "Message" record in the tenant's unified inbox.
    *   **AI Agent Interaction:** "The Ambassador" agent reads the incoming message, matches the sender against the customer CRM, and drafts a suggested reply based on past context and business knowledge.
    *   **User View:** A "Unified Inbox" UI showing threads from all connected platforms, with AI-drafted replies ready for approval or auto-sending.

**Implementation Prompt:** Integrate the Meta Graph API to receive and send Instagram Direct Messages and Facebook Page messages. Implement the OAuth flow for users to connect their social accounts. Create webhook handlers to ingest messages into the OHC database, and update the UI to display a unified inbox. Ensure "The Ambassador" agent is hooked into the message ingestion pipeline to draft replies.

**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling: Google Calendar Bi-Directional Sync

**Problem Statement:** Service-based businesses (like Leo the Music Tutor) manage their personal and business schedules primarily in Google Calendar. If OHC allows a booking when the owner is at a dentist appointment, it causes double-booking friction. They need OHC to read their external availability and sync new bookings back to their personal calendar.

**Research Report:**
*   **Evaluated Tool:** Google Calendar API
*   **Alternatives Considered:** Nylas, Cronofy
*   **Pros:** Native integration with the most popular calendar platform. Free API usage within standard quotas. Avoids third-party aggregator costs.
*   **Cons:** Only covers Google users (Outlook/Apple require separate integrations later). Google OAuth verification process can be tedious.
*   **Ease of Use for Non-technical Users:** The user clicks "Sign in with Google", grants calendar access, and OHC immediately blocks off time slots where the user is already busy.
*   **Pricing:** Free API tier is generally sufficient for SMB volume.
*   **Deployment:** Cloud-native (OAuth requires web redirects).

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A customer views the booking page, or a new booking is created.
    *   **Action:** OHC queries the connected Google Calendar for "busy" blocks before displaying available slots. When a booking is confirmed, OHC inserts a new event into the Google Calendar.
    *   **AI Agent Interaction:** "The Operations Manager" monitors calendar conflicts and can suggest rescheduling if the owner manually double-books themselves in Google Calendar.
    *   **User View:** A "Calendar Sync" settings page, and a calendar view in the OHC dashboard that overlays external events (read-only) with OHC bookings.

**Implementation Prompt:** Integrate the Google Calendar API for bi-directional synchronization. Implement the Google OAuth flow requesting calendar read/write scopes. Update the availability calculation logic to exclude times marked as "busy" in the connected Google Calendar. Implement a background sync to push OHC bookings to Google Calendar.

**Priority:** P0
**Estimated Scope:** Medium

---

## 3. Email Marketing: Resend for Customer Campaigns

**Problem Statement:** Boutique owners like Priya need to notify customers when new stock arrives. Managing a separate tool like Mailchimp is complex and requires importing/exporting CSVs of customer emails. They need an automated, beautiful way to send emails directly from their customer list in OHC.

**Research Report:**
*   **Evaluated Tool:** Resend
*   **Alternatives Considered:** SendGrid, Mailgun
*   **Pros:** Developer-friendly, extremely fast, excellent deliverability out-of-the-box. Built with modern React Email components in mind, making it easy to generate beautiful, mobile-responsive templates programmatically.
*   **Cons:** Newer player, fewer legacy features compared to SendGrid.
*   **Ease of Use for Non-technical Users:** The user simply clicks "Send Campaign" or "Generate Email". The AI and Resend handle the formatting, delivery, and open-rate tracking automatically.
*   **Pricing:** Generous free tier, then volume-based. Very affordable for SMBs.
*   **Deployment:** Cloud-native. Perfect for multi-tenant.

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** "The Promoter" agent schedules an email campaign, or the user clicks "Send Newsletter".
    *   **Action:** OHC generates the email HTML (using React Email or similar) and sends it via the Resend API to the filtered customer list.
    *   **AI Agent Interaction:** "The Promoter" drafts subject lines and email body text based on new inventory or seasonal events.
    *   **User View:** A "Marketing Campaigns" tab showing draft emails, sent emails, open rates, and click rates.

**Implementation Prompt:** Integrate the Resend API to enable bulk and transactional email sending. Create a UI flow for users to select a customer segment and generate an email campaign. Ensure "The Promoter" AI can draft templates and that open/click events are tracked via Resend webhooks.

**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing: Mercado Pago for LATAM Expansion

**Problem Statement:** While Stripe covers the US and Europe well, businesses operating in Latin America need local payment methods (like Pix in Brazil, or OXXO in Mexico). Without these, they cannot process online payments effectively, limiting their business growth.

**Research Report:**
*   **Evaluated Tool:** Mercado Pago API
*   **Alternatives Considered:** dLocal, EBANX
*   **Pros:** Dominant player in LATAM. Supports a massive variety of local payment methods (cash vouchers, local credit cards, bank transfers). Strong consumer trust in the region.
*   **Cons:** API documentation can be fragmented; support is localized.
*   **Ease of Use for Non-technical Users:** The user connects their Mercado Pago account via a simple OAuth flow, instantly enabling local payment options at checkout for their customers.
*   **Pricing:** Transaction percentage + fixed fee (varies heavily by country and payment method).
*   **Deployment:** Cloud and Standalone compatible via standard OAuth and webhooks.

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A customer initiates checkout in a supported LATAM country.
    *   **Action:** OHC routes the payment intent to Mercado Pago instead of Stripe, generating a checkout session or native UI component.
    *   **AI Agent Interaction:** "The Accountant" logs the pending payment, monitors the Mercado Pago webhook for success, and reconciles the localized currency to the owner's dashboard.
    *   **User View:** A "Payments" setting allowing the owner to connect Mercado Pago. Customers see familiar local payment options at checkout.

**Implementation Prompt:** Integrate the Mercado Pago API as an alternative payment gateway. Implement the OAuth connection flow for tenants. Update the checkout UI to support Mercado Pago checkout sessions and handle webhooks for payment status updates (pending, approved, rejected).

**Priority:** P2
**Estimated Scope:** Large

---

## 5. Shipping & Logistics: Shippo for Automated Fulfillment

**Problem Statement:** Sellers of physical products (like Priya's Boutique) struggle with calculating accurate shipping rates at checkout and manually copying addresses to print labels. They need an automated way to charge customers the right shipping fee and print labels with one click.

**Research Report:**
*   **Evaluated Tool:** Shippo API
*   **Alternatives Considered:** EasyPost, ShipEngine
*   **Pros:** Excellent API design, strong network of global carriers, built-in address validation. Often provides discounted USPS/UPS rates out of the box without requiring the user to negotiate their own carrier accounts.
*   **Cons:** Customer support can be slow on lower tiers.
*   **Ease of Use for Non-technical Users:** The user enters the weight of their product. When an order arrives, they click "Buy Label", and a printable PDF appears. Shippo's default discounted rates mean the user doesn't need to configure carrier accounts.
*   **Pricing:** Pay-as-you-go (per label fee) or monthly subscriptions.
*   **Deployment:** Cloud-native.

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A customer enters their shipping address at checkout (for live rates), or the business owner clicks "Fulfill Order".
    *   **Action:** OHC queries Shippo for shipping rates, or generates a shipping label transaction.
    *   **AI Agent Interaction:** "The Operations Manager" automatically fetches the tracking number from Shippo and triggers "The Ambassador" to email the customer the tracking link.
    *   **User View:** A "Fulfillment" screen on the order details page showing a generated label PDF and tracking status.

**Implementation Prompt:** Integrate the Shippo API to provide real-time shipping rate calculation at checkout and shipping label generation in the order management dashboard. Ensure tracking webhooks are processed to update order statuses and trigger customer notifications automatically.

**Priority:** P1
**Estimated Scope:** Medium

---

## 6. SMS & Notifications: MessageBird for Global SMS

**Problem Statement:** For users with limited English or low internet connectivity (like Fatima the Food Cart Operator), push notifications and emails are unreliable. They need immediate SMS alerts when a new order arrives, and their customers need SMS order confirmations.

**Research Report:**
*   **Evaluated Tool:** MessageBird API (now Bird)
*   **Alternatives Considered:** Twilio, Vonage
*   **Pros:** Excellent global coverage, competitive pricing outside the US, and a unified API that also handles WhatsApp. Strong omnichannel capabilities.
*   **Cons:** Less market dominance in the US compared to Twilio; recent rebranding may cause minor API documentation confusion.
*   **Ease of Use for Non-technical Users:** The user simply provides their phone number and toggles "SMS Alerts" on. No technical setup required.
*   **Pricing:** Pay-per-message, varies by destination country.
*   **Deployment:** Cloud-native.

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A critical event occurs (e.g., new paid order, pickup ready).
    *   **Action:** OHC sends a templated SMS via the MessageBird API to the business owner or the customer.
    *   **AI Agent Interaction:** "The Operations Manager" decides when an SMS is necessary (vs. email) based on user preferences and urgency.
    *   **User View:** A simple toggle in settings: "Send me an SMS for new orders", and a field in checkout for customers to opt-in to SMS updates.

**Implementation Prompt:** Integrate the MessageBird API for sending transactional SMS messages. Add preference toggles in the tenant dashboard for receiving SMS alerts. Ensure checkout flows capture customer phone numbers and opt-in consent, and trigger SMS confirmations for pickups/deliveries.

**Priority:** P1
**Estimated Scope:** Small

---

## 7. Video Conferencing: Zoom API for Auto-Meeting Links

**Problem Statement:** Service providers who teach or consult online (like Leo the Music Tutor) currently have to manually create a Zoom link, copy it, and email it to the student after they book. They need unique meeting links generated automatically and attached to the calendar invite.

**Research Report:**
*   **Evaluated Tool:** Zoom Meeting API
*   **Alternatives Considered:** Google Meet API, Daily.co
*   **Pros:** Ubiquitous adoption—almost all customers know how to use Zoom. Robust API for generating scheduled meetings programmatically.
*   **Cons:** Requires the tenant to have a paid Zoom account to avoid 40-minute limits. OAuth approval process for the app marketplace can be stringent.
*   **Ease of Use for Non-technical Users:** User clicks "Connect Zoom". When a customer books an "Online Lesson", a unique Zoom link appears on the confirmation screen and calendar invite.
*   **Pricing:** Free API usage; requires tenant to have a Zoom subscription.
*   **Deployment:** Cloud-native (Server-to-Server OAuth or standard OAuth).

**Design Doc:**
*   **Integration with OHC:**
    *   **Trigger:** A new booking is created for a service marked as "Online Meeting".
    *   **Action:** OHC calls the Zoom API to create a meeting associated with the tenant's Zoom account, retrieving the `join_url`.
    *   **AI Agent Interaction:** "The Ambassador" includes the `join_url` in the booking confirmation email and reminder notifications.
    *   **User View:** A "Video Conferencing" settings panel to connect Zoom, and an "Online Meeting" toggle when creating a service offering.

**Implementation Prompt:** Integrate the Zoom Meeting API via OAuth. When an online service is booked, automatically generate a unique Zoom meeting link. Display this link in the user's booking dashboard, the customer's confirmation page, and include it in automated email/SMS reminders.

**Priority:** P2
**Estimated Scope:** Medium

