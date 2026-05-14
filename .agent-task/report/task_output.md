# Scout: Tool Integration Research Q4

## 1. Social Media Integration: WhatsApp Business Cloud API
**Problem Statement:**
Fatima and many other small business owners get the majority of their bookings, questions, and customer requests directly through WhatsApp. Currently, answering these takes them away from actual work, or they lose track of messages scattered across their personal phones. They need a way to manage messages in one place without technical setup.

**Research Report:**
- **Ease of Use:** High for users. Users only need to link their phone number. No technical skills required.
- **Features:** Unified inbox, automated away messages, quick replies, label-based organization.
- **Pricing:** The Cloud API charges per conversation. Utility conversations (like booking confirmations) are cheap. Small businesses typically fall under the first 1,000 free service conversations tier.
- **Integration Risks:** Strict WhatsApp opt-in compliance required. Complex Meta developer account setup is abstracted away from the user, but challenging to orchestrate on the backend.
- **Cloud vs Standalone:** Works in Cloud. For Standalone, the user would need to create their own Meta App, which is too technical.

**Design Doc:**
- **Triggers:** Customer sends a message to the business's WhatsApp number.
- **Actions:** OHC receives the message via Webhook, attaches it to the customer profile, and surfaces it in the unified inbox. Replies from the owner are routed back to WhatsApp.
- **User Interface:** A simple "Connect WhatsApp" button in the Integrations tab that walks through the Meta OAuth popup. Messages appear alongside email and web chat.

**Implementation Prompt:**
Implement the WhatsApp Business Cloud API integration. Add a Connect WhatsApp button to the UI. Handle the OAuth flow, ingest incoming messages into the unified inbox, and route outbound messages from the inbox to the WhatsApp user.

**Priority:** P0 (Critical)
**Estimated Scope:** Large

## 2. Calendar & Scheduling: Google Calendar
**Problem Statement:**
Business owners double-book themselves because they have personal appointments in Google Calendar and business appointments in a notebook or separate system. They need one view of truth to avoid awkward scheduling conflicts.

**Research Report:**
- **Ease of Use:** Extremely high. Google OAuth is ubiquitous.
- **Features:** Bi-directional sync, conflict resolution, automatic Zoom/Meet link generation.
- **Pricing:** Free.
- **Integration Risks:** Google OAuth requires a verified app (Google Cloud Trust & Safety review) to access calendar data, which takes time.
- **Cloud vs Standalone:** Works smoothly in Cloud. Standalone requires the user to input their own Google Client ID/Secret, which creates a large UX hurdle.

**Design Doc:**
- **Triggers:** A new booking is created in OHC, or an event is added directly to Google Calendar.
- **Actions:** OHC pushes the booking to Google Calendar. OHC pulls new events from Google Calendar and marks those times as unavailable for new bookings.
- **User Interface:** "Sign in with Google" button. A toggle for "Sync Calendar".

**Implementation Prompt:**
Integrate Google Calendar sync. Allow users to connect their Google account, push new OHC bookings to their Google Calendar, and block out OHC availability when personal events are scheduled in Google Calendar.

**Priority:** P1 (High)
**Estimated Scope:** Medium

## 3. Payment Processing: Mercado Pago (LATAM Focus)
**Problem Statement:**
Stripe is not dominant or supported in many Latin American countries. Small business owners in these regions need a reliable way to accept local payment methods (like PIX in Brazil or local credit cards) to close sales.

**Research Report:**
- **Ease of Use:** High for Latin American users who are already familiar with the platform.
- **Features:** Local payment methods, easy checkout, quick settlement.
- **Pricing:** Variable by country, typically ~3.99% to 5.99% per transaction depending on settlement speed.
- **Integration Risks:** Documentation is fragmented across regions. Webhook reliability can vary.
- **Cloud vs Standalone:** Works well in both. Standalone users can easily paste an Access Token.

**Design Doc:**
- **Triggers:** A customer reaches the checkout step for an invoice or booking.
- **Actions:** OHC generates a Mercado Pago preference and redirects the user to the Mercado Pago checkout flow, or uses the Checkout Pro integration.
- **User Interface:** In settings, a "Connect Mercado Pago" section. Customers see Mercado Pago as a payment option alongside or instead of Stripe.

**Implementation Prompt:**
Implement Mercado Pago as an alternative payment gateway. Allow owners to input their Mercado Pago credentials. Route customer checkout flows through Mercado Pago when selected, and process payment success webhooks to mark invoices as paid.

**Priority:** P1 (High)
**Estimated Scope:** Medium

## 4. SMS & Notifications: Twilio
**Problem Statement:**
Not all customers use email, and emails often go to spam. Business owners need to send crucial reminders (like "Your appointment is in 1 hour") via SMS to reduce no-shows.

**Research Report:**
- **Ease of Use:** Backend handles complexity. Users just toggle "Send SMS Reminders".
- **Features:** Global SMS delivery, high reliability.
- **Pricing:** ~$0.0079 per message in the US, higher internationally.
- **Integration Risks:** A2P 10DLC compliance in the US requires business verification, which can be slow and confusing for small business owners.
- **Cloud vs Standalone:** Works in Cloud (OHC manages the Twilio account). Standalone requires users to make a Twilio account and add API keys.

**Design Doc:**
- **Triggers:** 24 hours before an appointment, or when an urgent update occurs.
- **Actions:** OHC formats a reminder text and dispatches it via Twilio API.
- **User Interface:** A simple toggle in booking settings: "Send SMS reminders to customers".

**Implementation Prompt:**
Integrate Twilio for outbound SMS notifications. Add a setting for owners to enable SMS reminders. Create a scheduled job that triggers 24 hours before appointments to send an SMS reminder to the customer's phone number.

**Priority:** P2 (Medium)
**Estimated Scope:** Small

## 5. Email Marketing: Mailchimp
**Problem Statement:**
Business owners have a list of customers in OHC but struggle to send them newsletters or promotions. They need a simple way to keep their customer list synchronized with an email marketing tool to send targeted campaigns.

**Research Report:**
- **Ease of Use:** High. Mailchimp is widely known and has a user-friendly drag-and-drop builder.
- **Features:** Audience management, automated campaigns, open/click analytics.
- **Pricing:** Free tier up to 500 contacts, which fits many small businesses. Paid tiers scale with audience size.
- **Integration Risks:** API changes and strict API key management. Bounce handling needs to be robust to maintain list health.
- **Cloud vs Standalone:** Works well in Cloud. Standalone users would need to provide their own Mailchimp API key, which is relatively straightforward to find in their Mailchimp dashboard.

**Design Doc:**
- **Triggers:** A new customer is added or updated in OHC.
- **Actions:** OHC syncs the customer's email and basic tags (e.g., "Active Customer") to a Mailchimp Audience.
- **User Interface:** A "Connect Mailchimp" button that accepts an API key or uses OAuth. A dashboard widget showing recent campaign stats.

**Implementation Prompt:**
Integrate Mailchimp for audience synchronization. Allow users to connect their Mailchimp account and map OHC customers to a Mailchimp Audience. Ensure background jobs keep the lists synchronized in real-time.

**Priority:** P2 (Medium)
**Estimated Scope:** Medium

## 6. Shipping & Logistics: Shippo
**Problem Statement:**
For businesses that sell physical goods (like artisanal products or salon supplies), calculating shipping rates and buying labels is a manual, error-prone process. They need a seamless way to generate labels from invoices.

**Research Report:**
- **Ease of Use:** Medium. Requires weighing packages and understanding box sizes.
- **Features:** Multi-carrier rate calculation, label generation, tracking webhooks.
- **Pricing:** Pay-as-you-go model (cents per label) plus carrier postage costs.
- **Integration Risks:** Address validation failures. Customs documentation for international shipping can be complex to map.
- **Cloud vs Standalone:** Works in Cloud. Standalone users can create their own Shippo account and input API credentials.

**Design Doc:**
- **Triggers:** An invoice containing physical items is marked as paid.
- **Actions:** OHC queries Shippo for rates, presents options to the owner (or uses a default), and purchases a label. Tracking info is saved.
- **User Interface:** A "Fulfill Order" button on invoices that opens a modal to select package size and carrier, returning a printable PDF label.

**Implementation Prompt:**
Integrate the Shippo API to handle physical shipping. Add functionality to physical product invoices to calculate shipping rates, purchase labels, and store tracking numbers.

**Priority:** P2 (Medium)
**Estimated Scope:** Large

## 7. Video Conferencing: Zoom
**Problem Statement:**
Owners offering online services (tutors, consultants) currently have to manually create a Zoom link, copy it, and email it to the client. They need automated link generation when a booking is confirmed.

**Research Report:**
- **Ease of Use:** High. Zoom is universally understood by customers.
- **Features:** Auto-generated meeting links, passwords, waiting rooms.
- **Pricing:** Free for 40-minute meetings. Pro plans for longer sessions.
- **Integration Risks:** Server-to-Server OAuth vs User-level OAuth. Managing token refresh lifecycles is critical.
- **Cloud vs Standalone:** Works in Cloud. Standalone requires setting up a Server-to-Server OAuth app in Zoom, which is complex for non-technical users.

**Design Doc:**
- **Triggers:** A virtual service is booked.
- **Actions:** OHC calls the Zoom API to create a meeting, retrieves the join link, and embeds it in the booking confirmation and calendar invites.
- **User Interface:** A "Connect Zoom" button. Service settings get a toggle: "This is a virtual meeting via Zoom".

**Implementation Prompt:**
Integrate Zoom for automated meeting generation. Allow users to connect their Zoom account. For services marked as virtual, automatically generate a Zoom link upon booking and include it in customer notifications.

**Priority:** P1 (High)
**Estimated Scope:** Medium
