# Research Report

[category] Social Media Integration
## Issue Brief: Unified Inbox for Social Channels
**Problem Statement:** As a small business owner, managing messages across Instagram, Facebook, WhatsApp, and TikTok is overwhelming. I lose track of customer inquiries, leading to missed sales and slow responses. I need all my messages in one place.
**Research Report:**
- **Tool:** Buffer (Engage) or Sprout Social (too expensive). Focused on simpler, API-first tools like **Chatwoot** or **Respond.io**. Respond.io is highly rated but can get pricey. Chatwoot has a solid free tier and is open-source (great for Standalone).
- **Ease of Use:** Chatwoot has a straightforward setup but connecting specific channels (like WhatsApp) can be complex for non-tech users without clear guides.
- **Pricing:** Chatwoot starts free; Respond.io starts around $79/mo.
- **Cloud vs. Standalone:** Chatwoot works perfectly in both (can be self-hosted in Standalone mode or consumed via API in Cloud).
**Design Doc:**
- The user connects their social accounts via a simple OAuth flow in the OHC dashboard.
- Incoming messages from any connected platform appear in a single unified inbox within OHC.
- The user replies directly from OHC, and the message is routed back to the appropriate platform.
- OHC agents can optionally draft replies based on previous conversations.
**Implementation Prompt:** Add a "Connect Channels" button in the settings. Upon connection, show a unified chat interface where all messages land. Replies sent from this interface must reach the customer on their original platform.
**Priority:** P0
**Estimated Scope:** Large

[category] Calendar & Scheduling
## Issue Brief: Seamless Booking & Calendar Sync
**Problem Statement:** Booking appointments involves a lot of back-and-forth emails. I need a way for clients to book time directly on my calendar without double-booking me, and I want a meeting link generated automatically.
**Research Report:**
- **Tool:** **Calendly** vs. **Cal.com**.
- **Ease of Use:** Both are very intuitive for business owners and their clients.
- **Pricing:** Calendly has a free tier; Cal.com is open-source and free for individuals.
- **Cloud vs. Standalone:** Cal.com is preferable due to its open-source nature, fitting perfectly into the Standalone deployment model, while Calendly works well via API in Cloud mode.
**Design Doc:**
- The user authorizes their Google or Outlook Calendar in OHC.
- The user defines their availability schedule and meeting types (e.g., "30-min consultation").
- OHC generates a booking link that the user can share.
- When a client books, an event is created on the user's calendar with an auto-generated Zoom/Meet link.
**Implementation Prompt:** Integrate calendar auth in settings. Create a booking page generator that checks for conflicts in real-time. Automatically attach a video conferencing link to successful bookings.
**Priority:** P1
**Estimated Scope:** Medium

[category] Email Marketing
## Issue Brief: Integrated Customer Email Campaigns
**Problem Statement:** I want to send newsletters and promotions to my customer list, but exporting contacts from my store/CRM and importing them into another tool is annoying and error-prone.
**Research Report:**
- **Tool:** **Mailchimp** vs. **Brevo** (formerly Sendinblue).
- **Ease of Use:** Mailchimp is iconic for ease of use but gets expensive quickly. Brevo is extremely user-friendly and offers great value.
- **Pricing:** Brevo's free tier allows 300 emails/day; Mailchimp is restrictive.
- **Cloud vs. Standalone:** Both are API-driven SaaS; no true local Standalone version, but API integration works seamlessly across OHC modes.
**Design Doc:**
- The user selects a segment of their customer list directly within OHC.
- The user drafts an email using a simple WYSIWYG editor.
- OHC syncs the selected contacts to the email provider and triggers the campaign via API.
- Open and click stats are displayed back in the OHC dashboard.
**Implementation Prompt:** Build an email composer interface. Allow selecting contacts from the unified list. Send the email via the integrated provider and display basic analytics (sent, opened, clicked) on a dashboard widget.
**Priority:** P2
**Estimated Scope:** Medium

[category] Payment Processing
## Issue Brief: Flexible Global Payment Options
**Problem Statement:** Stripe doesn't work well or is too expensive for all my customers, especially those in specific regions. I need alternative ways to accept payments easily.
**Research Report:**
- **Tool:** **Mercado Pago** (LATAM), **Razorpay** (India).
- **Ease of Use:** Very familiar to users in their respective regions; onboarding is standard.
- **Pricing:** Competitive local rates.
- **Cloud vs. Standalone:** Standard REST API integrations, fully compatible with both OHC modes.
**Design Doc:**
- In billing settings, the user selects their preferred payment gateways based on their region.
- When generating an invoice or payment link via OHC, the user can choose which gateway to use.
- Payment status (pending, paid, failed) syncs back to OHC via webhooks.
**Implementation Prompt:** Add region-specific gateway options in the payment settings. Ensure invoice generation supports selecting the gateway. Implement robust webhook handling to update invoice status securely.
**Priority:** P1
**Estimated Scope:** Large

[category] Shipping & Logistics
## Issue Brief: Automated Shipping Rates and Labels
**Problem Statement:** Calculating shipping costs and generating labels manually takes too much time. I need to know the exact shipping cost when a customer orders and print the label instantly.
**Research Report:**
- **Tool:** **Shippo** vs. **EasyPost**.
- **Ease of Use:** Both offer great APIs. Shippo often has a slightly more business-owner-friendly dashboard if they log in directly.
- **Pricing:** Both offer pay-as-you-go per label models (e.g., $0.05/label).
- **Cloud vs. Standalone:** Pure API integration; works seamlessly in both modes.
**Design Doc:**
- The user enters package dimensions and weight for an order in OHC.
- OHC fetches real-time rates from multiple carriers (USPS, UPS, FedEx).
- The user selects a rate and clicks "Buy Label".
- The label is generated as a PDF for easy printing, and tracking info is sent to the customer.
**Implementation Prompt:** Create a shipping module on the order details page. Input fields for package size/weight. Display a list of available rates. Add a "Generate Label" button that downloads a PDF and updates the order with a tracking number.
**Priority:** P2
**Estimated Scope:** Medium

[category] SMS & Notifications
## Issue Brief: Reliable SMS Customer Alerts
**Problem Statement:** Emails often get lost in spam, and many of my customers (like Fatima) prefer text messages for appointment reminders and order updates.
**Research Report:**
- **Tool:** **Twilio** vs. **MessageBird**.
- **Ease of Use:** Both are very developer-focused; OHC must abstract all complexity.
- **Pricing:** Pay-per-message (fractions of a cent).
- **Cloud vs. Standalone:** Cloud API; works in both OHC modes.
**Design Doc:**
- The user toggles "Enable SMS Notifications" in settings.
- The user can customize simple templates for specific events (e.g., "Your appointment is tomorrow at [Time]").
- OHC triggers the SMS via the API when the event occurs.
**Implementation Prompt:** Add an SMS settings tab. Provide toggle switches for different notification types (appointment, order shipped). Allow minor text template customization. Send messages automatically based on system events.
**Priority:** P0
**Estimated Scope:** Medium

[category] Video Conferencing
## Issue Brief: Auto-Generated Meeting Links
**Problem Statement:** Creating a Zoom link and manually sending it to a client for an online consultation is tedious and looks unprofessional.
**Research Report:**
- **Tool:** **Zoom API** vs. **Google Meet** (via Calendar API).
- **Ease of Use:** Google Meet is zero-friction if they already use Google Calendar. Zoom requires separate OAuth.
- **Pricing:** Google Meet (included with Workspace); Zoom (requires Pro for API access/longer meetings).
- **Cloud vs. Standalone:** API integrations; compatible with both modes.
**Design Doc:**
- The user connects their preferred video tool in OHC.
- When a calendar event is booked or manually created in OHC, a "Make it a video meeting" toggle is available.
- If toggled, a unique link is generated and attached to the invite.
**Implementation Prompt:** Integrate video provider OAuth. Add a video meeting toggle on the event creation/booking flow. Automatically append the generated link to the calendar event and notification emails.
**Priority:** P1
**Estimated Scope:** Small
