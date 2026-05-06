# Scout 🔍: Tool Integration Research Q3

## [Social Media] Issue Brief: Meta API for Unified Inbox

**Title**: Scout 🔍: Integrate Meta Graph API for Unified Direct Messages
**Problem Statement**:
Small business owners receive customer inquiries across multiple platforms (Instagram, Facebook Messenger, WhatsApp). Managing these separately causes missed messages, delayed responses, and lost sales. A unified inbox with AI assistance would solve this.
**Research Report**:
- **Tool**: Meta Graph API (Instagram Direct & Messenger) or a managed wrapper like ManyChat.
- **Evaluation**: The Meta API allows full programmatic access to read and reply to DMs. By integrating this, OHC's "Customer Success" AI agent can draft and send replies based on the business's existing catalog, FAQs, and business hours.
- **Ease of Use**: Very easy for the user. They simply click "Log in with Facebook/Instagram" to grant permissions. No API keys to manage.
- **Pricing**: Free to use the Meta API, though WhatsApp integration has per-conversation pricing.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode (OHC manages the Meta App and Webhooks). In Standalone mode, it would be complex as the user would need to create their own Meta App.
**Design Doc**:
- The user navigates to a "Social Inbox" tab and clicks "Connect Instagram".
- Uses OAuth to grant OHC permission to read/write messages.
- OHC registers a centralized webhook for the tenant.
- Incoming messages are routed to the AI Agent (Customer Success).
- The agent formulates a response based on the tenant's context (products, availability) and sends it back via the Meta API.
**Implementation Prompt**:
Implement the Instagram/Messenger integration. Provide a UI for the user to connect their Meta account. Set up a secure webhook endpoint to receive incoming DMs, route them to the LLM with the user's business context, and send the generated reply back to the customer. Ensure the user can toggle the AI on/off or set it to "draft only" mode.
**Priority**: P1
**Estimated Scope**: Medium

## [Calendar & Scheduling] Issue Brief: Native Booking Engine & Google Calendar Sync

**Title**: Scout 🔍: Integrate Cal.com API for Conflict-Free Booking
**Problem Statement**:
Service providers like Leo (Music Tutor) and Carlos (Handyman) need clients to book their time. Doing this manually via text or email leads to double-booking and timezone confusion. They need a simple booking widget that automatically respects their personal calendar availability.
**Research Report**:
- **Tool**: Cal.com API (or direct Google Calendar API).
- **Evaluation**: Cal.com offers an open-source, API-first scheduling infrastructure. It handles timezone math, calendar conflict checks, and event generation seamlessly.
- **Ease of Use**: High. The business owner connects their Google/Outlook calendar with one click, sets their working hours (e.g., 9 AM - 5 PM), and the platform generates a beautiful booking interface.
- **Pricing**: Cal.com API is highly scalable; basic usage can be very cheap or self-hosted. Direct Google Calendar API is free but requires more engineering effort to handle timezone and scheduling logic.
- **Cloud vs. Standalone**: Cal.com works well in Cloud. For Standalone, direct Google Calendar OAuth is more appropriate to avoid third-party API dependencies.
**Design Doc**:
- A new "Booking & Schedule" module in the OHC platform.
- User sets "Service Types" (e.g., 1-hour lesson).
- User connects external calendars to block out busy times.
- The storefront displays available time slots to the end customer.
- Upon booking, an event is created in the user's calendar.
**Implementation Prompt**:
Build a booking system that allows business owners to define services with durations and prices. Integrate with Google Calendar via OAuth to check for conflicts before displaying available times to the customer. When a customer books, automatically add the event to the business owner's calendar and send a confirmation email.
**Priority**: P0
**Estimated Scope**: Large

## [Email Marketing] Issue Brief: AI-Generated Customer Broadcasts

**Title**: Scout 🔍: Integrate Resend for AI-Powered Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling.
**Research Report**:
- **Tool**: Resend.
- **Evaluation**: Resend provides a developer-friendly, reliable email API. Instead of giving users a complex drag-and-drop builder, OHC can use the "Marketing" AI agent to generate beautiful HTML emails based on a simple text prompt from the user.
- **Ease of Use**: Zero-friction. The user types "Tell my customers about the new summer dress collection," and the AI generates the subject line, body, and inserts product photos automatically.
- **Pricing**: Resend charges around $20/mo for up to 50k emails, very economical to bundle into an OHC premium tier.
- **Cloud vs. Standalone**: Cloud mode uses OHC's centralized Resend account. Standalone mode requires the user to input their own SMTP credentials.
**Design Doc**:
- "Marketing" tab -> "Send a Broadcast".
- User provides a 1-sentence prompt.
- The AI Agent generates a responsive HTML email preview.
- User clicks "Send to all customers".
- The system chunks the customer list and sends via the Resend API.
**Implementation Prompt**:
Create a feature where the user can prompt the AI to draft an email blast. Use the business's product catalog to enrich the email. Provide a preview UI. Once approved, queue the emails to be sent out via the Resend API to all opted-in customers, handling rate limits and basic bounce tracking.
**Priority**: P2
**Estimated Scope**: Medium

## [Payment Processing] Issue Brief: Localized Payments for LATAM

**Title**: Scout 🔍: Integrate Mercado Pago for LATAM Market Expansion
**Problem Statement**:
While Stripe is fantastic for the US/EU, it is not the dominant or most accessible payment method in Latin America. Business owners in LATAM need to accept local payment methods like PIX (Brazil), Boleto, and local credit cards to effectively run their businesses.
**Research Report**:
- **Tool**: Mercado Pago API.
- **Evaluation**: Mercado Pago is the standard in LATAM. Integrating it allows OHC to serve a massive demographic of small businesses in South and Central America.
- **Ease of Use**: Similar to Stripe, the user connects their Mercado Pago account via an OAuth flow or by pasting their secure keys.
- **Pricing**: Standard payment gateway fees per transaction (~3-4%), no monthly cost to OHC.
- **Cloud vs. Standalone**: Works natively in both Cloud and Standalone modes.
**Design Doc**:
- "Settings" -> "Payments".
- Add a "Connect Mercado Pago" button alongside Stripe.
- On the storefront checkout page, dynamically display Mercado Pago if configured.
- Handle Mercado Pago webhooks for asynchronous payment confirmations (e.g., when a user pays a Boleto offline).
**Implementation Prompt**:
Implement an alternative payment gateway using Mercado Pago. Allow users to connect their account. Update the checkout flow to support Mercado Pago's hosted checkout or API-based payment intents. Ensure the order status accurately reflects asynchronous payment settlements via webhooks.
**Priority**: P1
**Estimated Scope**: Medium

## [Shipping & Logistics] Issue Brief: Automated Label Generation

**Title**: Scout 🔍: Integrate Shippo for 1-Click Shipping Labels
**Problem Statement**:
Sellers of physical goods (like Maya shipping cookies or Priya shipping clothes) spend too much time calculating postage, going to the post office, and manually entering tracking numbers. They need to print labels directly from their phone.
**Research Report**:
- **Tool**: Shippo API (or EasyPost).
- **Evaluation**: Shippo aggregates multiple carriers (USPS, UPS, FedEx) and provides discounted rates without the user needing to negotiate their own carrier accounts.
- **Ease of Use**: Very easy. On the order details screen, the user clicks "Buy Label", confirms the box weight, and gets a PDF label to print.
- **Pricing**: 5¢ per label + the actual cost of postage.
- **Cloud vs. Standalone**: Cloud-friendly using a master Shippo account with sub-accounts. Standalone would require the user to create their own Shippo account and provide an API key.
**Design Doc**:
- In the "Orders" view, add a "Fulfill & Ship" flow.
- Call Shippo API to get live rates based on the customer's shipping address.
- Business owner selects a rate and purchases the label.
- OHC automatically emails the tracking number to the customer.
**Implementation Prompt**:
Integrate the Shippo API to allow users to purchase and generate shipping labels directly from an order page. The flow should fetch rates, capture payment for the label (either via OHC billing or direct), generate the PDF label, and automatically update the order status with the tracking URL.
**Priority**: P1
**Estimated Scope**: Medium

## [SMS & Notifications] Issue Brief: High-Reliability SMS Alerts

**Title**: Scout 🔍: Integrate Twilio for Critical Order SMS Alerts
**Problem Statement**:
Users like Fatima (Food Cart Operator) work in fast-paced, noisy environments where they might not hear a standard push notification, or they may be in areas with poor internet connectivity. They need highly reliable SMS notifications when a new order arrives.
**Research Report**:
- **Tool**: Twilio Programmable SMS.
- **Evaluation**: Twilio is the industry standard for SMS. It guarantees delivery and provides global carrier coverage.
- **Ease of Use**: The user simply toggles "Send me SMS alerts for new orders" and verifies their phone number.
- **Pricing**: ~$0.01 per message. Because of this cost, it should be restricted to Premium users or metered.
- **Cloud vs. Standalone**: Cloud uses OHC's Twilio account. Standalone would require the user to configure their own Twilio SID and Auth Token.
**Design Doc**:
- "Notifications" setting panel.
- User enters their phone number and verifies it via a one-time code.
- When the backend processes a new paid order, an event is emitted.
- The Notification worker picks up the event and dispatches an SMS via Twilio.
**Implementation Prompt**:
Integrate Twilio to send SMS notifications to the business owner when critical events occur (e.g., new pre-order received). Add a settings UI for the user to verify their phone number and opt-in. Ensure the backend handles Twilio rate limits and securely stores the business owner's verified phone number.
**Priority**: P2
**Estimated Scope**: Small

## [Video Conferencing] Issue Brief: Auto-Generated Meeting Links

**Title**: Scout 🔍: Integrate Google Meet for Automated Online Lessons
**Problem Statement**:
For digital service providers like Leo (Music Tutor), manually creating Zoom or Google Meet links for every booked lesson and emailing them to the student is prone to human error (e.g., forgetting to send the link or sending the wrong one).
**Research Report**:
- **Tool**: Google Workspace API (Google Meet) or Zoom API.
- **Evaluation**: Google Meet is often preferred as it can be automatically attached to any Google Calendar event created during the booking process. Zoom requires a separate OAuth flow.
- **Ease of Use**: Zero extra effort if the user has already connected Google Calendar for availability syncing. The system automatically provisions the link.
- **Pricing**: Free if using the user's existing Google Calendar/Meet integration.
- **Cloud vs. Standalone**: Works natively in both.
**Design Doc**:
- When setting up a service, the user toggles "This is an online meeting".
- When a customer books the service, the OHC backend creates a Google Calendar event.
- The calendar event is configured to auto-generate a Google Meet conference link.
- The confirmation email sent to the customer includes this generated Meet link.
**Implementation Prompt**:
Extend the calendar booking flow to support online meetings. When a service is marked as "online", ensure the Google Calendar event creation request includes the conference data parameters to auto-generate a Google Meet link. Extract this link from the response and include it in the customer's confirmation email and the business owner's dashboard.
**Priority**: P1
**Estimated Scope**: Small
