# [Payment Processing] Integrate Mercado Pago for LATAM Expansion

## Problem Statement
While Stripe covers the US and Europe well, businesses operating in Latin America need local payment methods (like Pix in Brazil, or OXXO in Mexico). Without these, they cannot process online payments effectively, limiting their business growth.

## Research Report
**Evaluated Tool:** Mercado Pago API
**Alternatives Considered:** dLocal, EBANX
**Pros:** Dominant player in LATAM. Supports a massive variety of local payment methods (cash vouchers, local credit cards, bank transfers). Strong consumer trust in the region.
**Cons:** API documentation can be fragmented; support is localized.
**Ease of Use for Non-technical Users:** The user connects their Mercado Pago account via a simple OAuth flow, instantly enabling local payment options at checkout for their customers.
**Pricing:** Transaction percentage + fixed fee (varies heavily by country and payment method).
**Deployment:** Cloud and Standalone compatible via standard OAuth and webhooks.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer initiates checkout in a supported LATAM country.
- **Action:** OHC routes the payment intent to Mercado Pago instead of Stripe, generating a checkout session or native UI component.
- **AI Agent Interaction:** "The Accountant" logs the pending payment, monitors the Mercado Pago webhook for success, and reconciles the localized currency to the owner's dashboard.
- **User View:** A "Payments" setting allowing the owner to connect Mercado Pago. Customers see familiar local payment options at checkout.

## Implementation Prompt
Integrate the Mercado Pago API as an alternative payment gateway. Implement the OAuth connection flow for tenants. Update the checkout UI to support Mercado Pago checkout sessions and handle webhooks for payment status updates (pending, approved, rejected).

## Priority
P2

## Estimated Scope
Large
# [Shipping & Logistics] Integrate Shippo for Automated Fulfillment

## Problem Statement
Sellers of physical products (like Priya's Boutique) struggle with calculating accurate shipping rates at checkout and manually copying addresses to print labels. They need an automated way to charge customers the right shipping fee and print labels with one click.

## Research Report
**Evaluated Tool:** Shippo API
**Alternatives Considered:** EasyPost, ShipEngine
**Pros:** Excellent API design, strong network of global carriers, built-in address validation. Often provides discounted USPS/UPS rates out of the box without requiring the user to negotiate their own carrier accounts.
**Cons:** Customer support can be slow on lower tiers.
**Ease of Use for Non-technical Users:** The user enters the weight of their product. When an order arrives, they click "Buy Label", and a printable PDF appears. Shippo's default discounted rates mean the user doesn't need to configure carrier accounts.
**Pricing:** Pay-as-you-go (per label fee) or monthly subscriptions.
**Deployment:** Cloud-native.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer enters their shipping address at checkout (for live rates), or the business owner clicks "Fulfill Order".
- **Action:** OHC queries Shippo for shipping rates, or generates a shipping label transaction.
- **AI Agent Interaction:** "The Operations Manager" automatically fetches the tracking number from Shippo and triggers "The Ambassador" to email the customer the tracking link.
- **User View:** A "Fulfillment" screen on the order details page showing a generated label PDF and tracking status.

## Implementation Prompt
Integrate the Shippo API to provide real-time shipping rate calculation at checkout and shipping label generation in the order management dashboard. Ensure tracking webhooks are processed to update order statuses and trigger customer notifications automatically.

## Priority
P1

## Estimated Scope
Medium
# [SMS & Notifications] Integrate MessageBird for Global SMS

## Problem Statement
For users with limited English or low internet connectivity (like Fatima the Food Cart Operator), push notifications and emails are unreliable. They need immediate SMS alerts when a new order arrives, and their customers need SMS order confirmations.

## Research Report
**Evaluated Tool:** MessageBird API (now Bird)
**Alternatives Considered:** Twilio, Vonage
**Pros:** Excellent global coverage, competitive pricing outside the US, and a unified API that also handles WhatsApp. Strong omnichannel capabilities.
**Cons:** Less market dominance in the US compared to Twilio; recent rebranding may cause minor API documentation confusion.
**Ease of Use for Non-technical Users:** The user simply provides their phone number and toggles "SMS Alerts" on. No technical setup required.
**Pricing:** Pay-per-message, varies by destination country.
**Deployment:** Cloud-native.

## Design Doc
**Integration with OHC:**
- **Trigger:** A critical event occurs (e.g., new paid order, pickup ready).
- **Action:** OHC sends a templated SMS via the MessageBird API to the business owner or the customer.
- **AI Agent Interaction:** "The Operations Manager" decides when an SMS is necessary (vs. email) based on user preferences and urgency.
- **User View:** A simple toggle in settings: "Send me an SMS for new orders", and a field in checkout for customers to opt-in to SMS updates.

## Implementation Prompt
Integrate the MessageBird API for sending transactional SMS messages. Add preference toggles in the tenant dashboard for receiving SMS alerts. Ensure checkout flows capture customer phone numbers and opt-in consent, and trigger SMS confirmations for pickups/deliveries.

## Priority
P1

## Estimated Scope
Small
# [Social Media] Integrate Zernio for Unified Inbox

## Problem Statement
Small business owners like Priya (Boutique Owner) and Maya (Home Baker) receive inquiries across Instagram DMs, Facebook Comments, TikTok, and WhatsApp. Managing these separately means delayed responses and lost sales. They need a single, unified inbox to view and reply to all customer messages, and an AI agent to handle common questions ("do you do vegan cakes?") seamlessly across platforms.

## Research Report
**Evaluated Tool:** Zernio (Unified Social Media API)
**Alternatives Considered:** Native APIs (Meta Graph, X, TikTok), Ayrshare
**Pros:** Zernio aggregates multiple platforms into a single API endpoint, reducing OAuth complexity and the need to maintain multiple webhook structures. Excellent parsing quality for DMs and comments.
**Cons:** Third-party dependency, potential rate limits.
**Ease of Use for Non-technical Users:** Transparent. The user simply connects their social accounts once and all messages flow into the OHC unified inbox.
**Pricing:** Estimated at ~$50-100/mo base + volume pricing, scalable for multi-tenant SaaS.
**Deployment:** Works well in Cloud. For Standalone, OAuth callback handling will require specific configuration or proxying.

## Design Doc
**Integration with OHC:**
- **Trigger:** A new message arrives via Zernio webhooks.
- **Action:** The system parses the message and routes it to the tenant's unified inbox database.
- **AI Agent Interaction:** The Customer Success agent ("The Ambassador") receives the incoming message context, drafts a reply, and (if auto-reply is enabled) posts the response back through Zernio.
- **User View:** A unified "Inbox" screen in the OHC mobile and desktop apps.

## Implementation Prompt
Implement the backend integration with Zernio to receive webhooks for incoming social messages and send outgoing replies. Create the frontend UI for a unified inbox where users can view and reply to cross-platform messages. Ensure "The Ambassador" AI agent can draft replies within this interface.

## Priority
P1

## Estimated Scope
Large
# [Calendar & Scheduling] Integrate Nylas for Booking Sync

## Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) struggle with double bookings. They need a simple booking page where customers can choose a time, and it must sync seamlessly with their existing personal calendars (Google, Outlook) to block out unavailable times automatically.

## Research Report
**Evaluated Tool:** Nylas Calendar API
**Alternatives Considered:** Cronofy, Cal.com
**Pros:** Highly reliable, broad support for almost all calendar providers (Google, Exchange, Office365, generic IMAP/CalDAV). Provides excellent unified data models and handles timezones gracefully.
**Cons:** Can be expensive at high volume.
**Ease of Use for Non-technical Users:** Simple "Connect Calendar" OAuth flow. Once connected, sync is automatic and invisible.
**Pricing:** Volume-based, typically per connected account.
**Deployment:** Fully functional in Cloud mode. Standalone may require BYO API keys.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer visits a tenant's booking page.
- **Action:** OHC queries the Nylas API for the tenant's free/busy schedule to render available slots.
- **AI Agent Interaction:** "The Operations Manager" uses this availability to schedule, reschedule, or cancel bookings.
- **User View:** A clean "Booking Configuration" screen where the owner sets working hours, and a public booking page showing available time slots.

## Implementation Prompt
Integrate the Nylas API to enable bi-directional calendar sync. Implement an OAuth flow for tenants to connect their Google/Outlook calendars. Build a frontend scheduling component that calculates and displays available time slots based on the synced calendar data and predefined working hours.

## Priority
P0

## Estimated Scope
Medium
# [Email Marketing] Integrate Resend for Customer Campaigns

## Problem Statement
Boutique owners like Priya need to notify customers when new stock arrives. Managing a separate tool like Mailchimp is complex and requires importing/exporting CSVs of customer emails. They need an automated, beautiful way to send emails directly from their customer list in OHC.

## Research Report
**Evaluated Tool:** Resend
**Alternatives Considered:** SendGrid, Mailgun
**Pros:** Developer-friendly, extremely fast, excellent deliverability out-of-the-box. Built with modern React Email components in mind, making it easy to generate beautiful, mobile-responsive templates programmatically.
**Cons:** Newer player, fewer legacy features compared to SendGrid.
**Ease of Use for Non-technical Users:** The user simply clicks "Send Campaign" or "Generate Email". The AI and Resend handle the formatting, delivery, and open-rate tracking automatically.
**Pricing:** Generous free tier, then volume-based. Very affordable for SMBs.
**Deployment:** Cloud-native. Perfect for multi-tenant.

## Design Doc
**Integration with OHC:**
- **Trigger:** "The Promoter" agent schedules an email campaign, or the user clicks "Send Newsletter".
- **Action:** OHC generates the email HTML (using React Email or similar) and sends it via the Resend API to the filtered customer list.
- **AI Agent Interaction:** "The Promoter" drafts subject lines and email body text based on new inventory or seasonal events.
- **User View:** A "Marketing Campaigns" tab showing draft emails, sent emails, open rates, and click rates.

## Implementation Prompt
Integrate the Resend API to enable bulk and transactional email sending. Create a UI flow for users to select a customer segment and generate an email campaign. Ensure "The Promoter" AI can draft templates and that open/click events are tracked via Resend webhooks.

## Priority
P1

## Estimated Scope
Medium
# [Video Conferencing] Integrate Zoom API for Auto-Meeting Links

## Problem Statement
Service providers who teach or consult online (like Leo the Music Tutor) currently have to manually create a Zoom link, copy it, and email it to the student after they book. They need unique meeting links generated automatically and attached to the calendar invite.

## Research Report
**Evaluated Tool:** Zoom Meeting API
**Alternatives Considered:** Google Meet API, Daily.co
**Pros:** Ubiquitous adoption—almost all customers know how to use Zoom. Robust API for generating scheduled meetings programmatically.
**Cons:** Requires the tenant to have a paid Zoom account to avoid 40-minute limits. OAuth approval process for the app marketplace can be stringent.
**Ease of Use for Non-technical Users:** User clicks "Connect Zoom". When a customer books an "Online Lesson", a unique Zoom link appears on the confirmation screen and calendar invite.
**Pricing:** Free API usage; requires tenant to have a Zoom subscription.
**Deployment:** Cloud-native (Server-to-Server OAuth or standard OAuth).

## Design Doc
**Integration with OHC:**
- **Trigger:** A new booking is created for a service marked as "Online Meeting".
- **Action:** OHC calls the Zoom API to create a meeting associated with the tenant's Zoom account, retrieving the `join_url`.
- **AI Agent Interaction:** "The Ambassador" includes the `join_url` in the booking confirmation email and reminder notifications.
- **User View:** A "Video Conferencing" settings panel to connect Zoom, and an "Online Meeting" toggle when creating a service offering.

## Implementation Prompt
Integrate the Zoom Meeting API via OAuth. When an online service is booked, automatically generate a unique Zoom meeting link. Display this link in the user's booking dashboard, the customer's confirmation page, and include it in automated email/SMS reminders.

## Priority
P2

## Estimated Scope
Medium
