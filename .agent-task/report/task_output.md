# OHC Tool Integration Research Report

## Category 1: Social Media Integration

### [Social Media] Unified Inbox Integration (ManyChat Alternative)

**Problem Statement:**
Small business owners receive customer inquiries scattered across Instagram DMs, Facebook Messenger, WhatsApp, and SMS. They miss messages, lose track of customer context, and struggle to respond quickly, leading to lost sales. They need all messages in one place without having to build a complex Zapier flow or buy expensive CRM software.

**Research Report:**
Current OHC state: Basic UI stubs exist for connecting WhatsApp and Instagram (seen in `unified_inbox_screen.dart`), but there is no real backend integration or generic webhook parsing framework.
Market solutions like ManyChat and Chatfuel charge $15-$30/month for basic unified inboxes.
Integrating official APIs (Meta Graph API for IG/FB, WhatsApp Cloud API) directly into OHC provides immense value.
- **Ease of use:** High. The user simply clicks "Connect Instagram" and completes the Meta OAuth flow.
- **Pricing:** Free for the user (included in OHC), though OHC incurs server costs for webhook processing. WhatsApp charges per conversation after the first 1,000.
- **Hybrid Support:** Yes. Cloud handles webhooks centrally; Standalone requires a cloud-relay or local ngrok-style tunnel for incoming webhooks.

**Design Doc:**
- **Trigger:** User clicks "Connect [Platform]" in the OHC Unified Inbox settings.
- **Action:** OHC initiates OAuth flow. Once connected, OHC registers webhooks for incoming messages. A unified background sync daemon normalizes incoming payloads into a standard OHC message format.
- **User View:** All messages appear in the Unified Inbox tab. The user replies from OHC, and the message routes back to the correct platform transparently.

**Implementation Prompt:**
Implement the backend unified messaging service. Build the system to receive and standardize incoming messages from Meta and WhatsApp. Store messages locally in Standalone mode or in the multi-tenant DB in Cloud mode. Ensure the UI can display and send messages without knowing the underlying platform details.

**Priority:** P0
**Estimated Scope:** Large

## Category 2: Calendar & Scheduling

### [Calendar] Google Calendar & Auto-Scheduling

**Problem Statement:**
Service-based business owners (tutors, consultants, salons) waste hours playing "email ping-pong" trying to find a time to meet. They need a simple booking link they can send to customers that automatically syncs with their personal Google Calendar to prevent double-booking.

**Research Report:**
Current OHC state: "Setup Base Inventory/Calendar" is a stub in onboarding, but no real calendar sync exists.
Market solutions like Calendly or Acuity Scheduling cost $10-$15/month.
Integrating Google Calendar API directly allows OHC to offer a built-in booking page.
- **Ease of use:** High. User authenticates with Google, selects their working hours, and gets a reusable booking link.
- **Pricing:** Free (Google Calendar API has a generous free tier for standard use cases).
- **Hybrid Support:** Yes. Standard OAuth flow works in both Cloud and Standalone (saving tokens locally).

**Design Doc:**
- **Trigger:** User connects Google Calendar via OAuth in OHC settings and defines their available hours.
- **Action:** OHC reads free/busy blocks from Google Calendar. A public booking page is generated. When a customer books, OHC creates an event on the user's Google Calendar.
- **User View:** A "Share Booking Link" button in OHC. A simple public web page for customers to pick a time slot.

**Implementation Prompt:**
Implement the Google Calendar integration. Add an OAuth flow to acquire and store refresh tokens securely. Create a service to fetch free/busy slots and a public endpoint to render available times. When a slot is booked, create the event via the Google API and send a confirmation to both parties.

**Priority:** P1
**Estimated Scope:** Medium

## Category 3: Payment Processing

### [Payments] Global Payment Gateways (Stripe + Mercado Pago + Razorpay)

**Problem Statement:**
Small business owners need to get paid easily. While Stripe is popular in the US/EU, it is not supported or preferred everywhere (e.g., LATAM prefers Mercado Pago, India prefers Razorpay). Business owners need a simple way to generate payment links and track who has paid, regardless of their region.

**Research Report:**
Current OHC state: The UI mentions "Connect Stripe" in the Help Center, and telemetry redacts "stripe" keys, but no actual multi-gateway payment service exists.
- **Ease of use:** High. User enters API keys or uses standard OAuth for their preferred regional gateway.
- **Pricing:** Varies by provider (typically 2-3% + fixed fee per transaction). No additional cost from OHC.
- **Hybrid Support:** Yes. Cloud handles webhooks; Standalone can poll for payment status or use a cloud-relay.

**Design Doc:**
- **Trigger:** User selects their payment provider and connects it in OHC settings.
- **Action:** OHC provides an interface to generate an invoice or payment link. OHC abstracts the gateway API calls to create a payment session.
- **User View:** A simple "Create Invoice" screen. User inputs amount and customer email, clicks send, and sees payment status (Pending/Paid) in OHC.

**Implementation Prompt:**
Build the core payment service that can route requests to Stripe (US/EU), Mercado Pago (LATAM), and Razorpay (India). Add endpoints to generate payment links and webhooks/polling to update invoice status in the database. Ensure the UI clearly shows payment status.

**Priority:** P0
**Estimated Scope:** Large

## Category 4: SMS & Notifications

### [Notifications] Global SMS Gateway Integration

**Problem Statement:**
Many customers, especially in developing regions or older demographics, do not use email or ignore it. Small business owners need to send automated SMS confirmations (e.g., booking reminders, shipping updates) to reduce no-shows and keep customers informed.

**Research Report:**
Current OHC state: Telemetry redacts "phone", but there is no SMS sending capability.
Market solutions: Twilio, MessageBird, or Vonage.
- **Ease of use:** High. The user simply toggles "Send SMS Reminders" in OHC. The complexity of routing is handled by the backend.
- **Pricing:** SMS costs money (e.g., $0.01 - $0.05 per message depending on the country). OHC could bundle a small amount or require users to bring their own Twilio API key.
- **Hybrid Support:** Yes. API calls to SMS providers work seamlessly in both modes.

**Design Doc:**
- **Trigger:** A significant event occurs in OHC (e.g., appointment booked, order shipped).
- **Action:** OHC triggers the notification service, which looks up the customer's phone number and dispatches an SMS via the configured gateway (e.g., Twilio).
- **User View:** A toggle in settings to enable SMS notifications. A log showing sent messages.

**Implementation Prompt:**
Implement a Notification Service that supports SMS alongside email. Build the core notification service that routes requests to Twilio. Ensure phone numbers are validated and formatted (E.164) before sending. Add a UI setting for business owners to enable/disable automated SMS and input their API credentials if required.

**Priority:** P1
**Estimated Scope:** Medium
