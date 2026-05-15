# Tool Integration Research Report

## 1. Social Media Integration: Meta Graph API (Instagram/WhatsApp)

### Title
`[social_media]unified_inbox`: Implement Unified Social Inbox via Meta Graph API

### Problem Statement
Small business owners often miss critical customer inquiries because they are scattered across Instagram DMs, Facebook Messenger, and WhatsApp. Manually checking each app is a massive time sink. They need a single, unified inbox where all messages appear in one place, allowing them to reply to customers quickly without switching contexts.

### Research Report
- **Tool**: Meta Graph API
- **Pros**: Direct access to the largest social platforms (Instagram, Facebook, WhatsApp). High reliability and deep feature set.
- **Cons**: Complex OAuth flow, strict app review process, and frequent API changes. WhatsApp pricing involves per-conversation charges.
- **Reputation**: Industry standard, though developer experience can be frustrating due to Meta's aggressive review policies.
- **Pricing**: Free for standard APIs, WhatsApp Business API has per-conversation costs (approx. $0.01 - $0.08 depending on region).
- **Ease of Use for Non-Technical Users**: The user simply clicks "Connect Instagram" and authorizes the app. The complexity is hidden behind the scenes.
- **Modes Supported**: Cloud (webhooks) and Standalone (local polling or proxy).

### Design Doc
- **Trigger**: The business owner connects their Meta account via an OAuth flow in the OHC UI.
- **Action**: The OHC API server registers webhooks (in Cloud mode) or sets up a local polling mechanism/proxy (in Standalone mode) to receive incoming messages. These are stored in the shared PostgreSQL (Cloud) or local SQLite (Standalone).
- **User View**: A unified "Inbox" tab in the UI displaying all messages chronologically, with indicators for the source platform.

### Implementation Prompt
Implement a unified inbox feature that allows users to connect their Meta accounts. The system must ingest incoming messages from Instagram, Facebook, and WhatsApp, and present them in a single unified view. Users must be able to reply to messages directly from the OHC interface, and the responses should be routed back to the appropriate platform. Ensure the OAuth flow is seamless and clearly explains the required permissions.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling: Google Calendar

### Title
`[calendar]automated_scheduling`: Implement Two-Way Google Calendar Sync

### Problem Statement
Business owners juggle consultations, classes, and personal appointments. Double-booking is a constant risk, and manually copying appointments from a booking page to their personal calendar is tedious. They need a system that automatically adds new bookings to their calendar and prevents customers from booking times when they are already busy.

### Research Report
- **Tool**: Google Calendar API
- **Pros**: Ubiquitous, highly reliable, excellent documentation. Supports real-time push notifications for changes.
- **Cons**: Requires Google Cloud project setup and OAuth verification, which can be daunting if not handled by the platform.
- **Reputation**: The gold standard for calendar integrations.
- **Pricing**: Free for standard usage limits (which are very high).
- **Ease of Use for Non-Technical Users**: Very intuitive. Users authenticate with their Google account and the system handles the rest.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: The business owner clicks "Connect Google Calendar" and completes the OAuth flow.
- **Action**: The OHC API server fetches existing calendar events to block out busy times on the user's OHC booking page. New bookings made via OHC are pushed to the Google Calendar.
- **User View**: A "Calendar" or "Availability" settings page showing connected calendars and options to sync specific event types.

### Implementation Prompt
Create a two-way sync integration with Google Calendar. The system must read the user's availability to prevent double-booking on their OHC booking page. When a customer books a service, the event must be automatically created on the business owner's Google Calendar. The OAuth connection process must be straightforward, handling token refresh automatically in the background.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing: Resend

### Title
`[email_marketing]customer_campaigns`: Implement Email Campaigns via Resend

### Problem Statement
Reaching out to past customers with promotions or updates is crucial for repeat business, but complex tools like Mailchimp are overkill and expensive for simple announcements. Business owners need a straightforward way to email their customer list directly from the platform they already use to manage their business.

### Research Report
- **Tool**: Resend
- **Pros**: Developer-friendly, simple API, excellent deliverability, built-in React email templates.
- **Cons**: Newer player, fewer out-of-the-box marketing features compared to legacy providers.
- **Reputation**: Highly regarded in the developer community for its modern approach and ease of use.
- **Pricing**: Generous free tier (3,000 emails/month), then very affordable ($20/month for 50,000 emails).
- **Ease of Use for Non-Technical Users**: The user only interacts with a simple composer in OHC; the complexity of SMTP and domain verification is abstracted.
- **Modes Supported**: Cloud and Standalone (via API calls).

### Design Doc
- **Trigger**: The business owner selects a list of customers and clicks "Send Email Campaign".
- **Action**: The OHC API server formats the email and dispatches it via the Resend API, tracking delivery status.
- **User View**: A simple email composer with audience selection and a basic performance dashboard (open rates, bounces).

### Implementation Prompt
Integrate Resend to enable basic email marketing capabilities. Users should be able to select segments of their customer database and send batch emails. Implement a simple, foolproof email composer. Ensure the system handles bounce and complaint webhooks to automatically clean the user's mailing list and maintain high deliverability scores.

### Priority
P2

### Estimated Scope
Medium

---

## 4. Payment Processing: Mercado Pago

### Title
`[payment]alternative_providers`: Implement Mercado Pago Integration for LATAM

### Problem Statement
Stripe is fantastic, but it's not universally adopted or preferred in all markets. In regions like LATAM, local providers like Mercado Pago are essential for offering familiar payment methods (like Pix in Brazil or OXXO in Mexico). Without local options, business owners lose sales.

### Research Report
- **Tool**: Mercado Pago API
- **Pros**: Dominant market share in LATAM, supports local payment methods that Stripe misses.
- **Cons**: Documentation can be fragmented; API design is sometimes inconsistent compared to Stripe.
- **Reputation**: The undisputed leader in Latin American e-commerce payments.
- **Pricing**: Varies by country and payment method (typically 3-5% + fixed fee).
- **Ease of Use for Non-Technical Users**: Users link their existing Mercado Pago account. The checkout experience is highly trusted by local consumers.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: The business owner configures their Mercado Pago credentials in the "Payments" tab.
- **Action**: The OHC API server generates payment preferences and handles incoming IPN (Instant Payment Notification) webhooks to update order status.
- **User View**: Customers see Mercado Pago as a checkout option. Business owners see unified transaction logs alongside Stripe payments.

### Implementation Prompt
Add Mercado Pago as a supported payment gateway. This must include creating payment intents and handling IPN webhooks to confirm payment success. The checkout flow must support local payment methods relevant to the user's region. Ensure the system gracefully handles pending payments (e.g., waiting for cash deposits) and updates the order status asynchronously.

### Priority
P1

### Estimated Scope
Medium

---

## 5. Shipping & Logistics: Shippo

### Title
`[shipping]automated_labels`: Implement Shipping Label Generation via Shippo

### Problem Statement
For product-based businesses, manually calculating shipping rates and copying addresses to carrier websites to buy labels is a massive bottleneck. They need a way to automatically offer accurate shipping rates at checkout and print labels with one click after an order is placed.

### Research Report
- **Tool**: Shippo API
- **Pros**: Aggregates dozens of carriers (USPS, UPS, FedEx, international) into a single API. Excellent rate calculation.
- **Cons**: Support can be slow for complex carrier account issues.
- **Reputation**: Very strong, widely used by major e-commerce platforms.
- **Pricing**: Pay-as-you-go (5¢ per label) or flat monthly fees for high volume.
- **Ease of Use for Non-Technical Users**: Users enter package dimensions and click "Buy Label". Rates are shown instantly.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: An order is placed, and the business owner clicks "Fulfill Order".
- **Action**: The OHC API server requests label generation from Shippo using the saved package dimensions and customer address.
- **User View**: A "Fulfillment" page showing order details, a "Buy Shipping Label" button, and a tracking link generator.

### Implementation Prompt
Integrate Shippo to provide real-time shipping rates at checkout and automated label generation for the business owner. The system should allow the owner to define standard package sizes. Upon order completion, provide a one-click flow to purchase and print the shipping label, and automatically email the tracking number to the customer.

### Priority
P2

### Estimated Scope
Large

---

## 6. SMS & Notifications: Twilio

### Title
`[sms]global_notifications`: Implement SMS Notifications via Twilio

### Problem Statement
Email open rates can be low. For urgent updates (like appointment reminders or order pickups), SMS is far more effective. Business owners, especially those serving demographics with lower tech literacy, need a reliable way to send text messages directly to their customers.

### Research Report
- **Tool**: Twilio API
- **Pros**: Global reach, unmatched reliability, comprehensive API for SMS and Voice.
- **Cons**: Can be expensive at scale. Strict compliance rules (A2P 10DLC) require complex registration for US numbers.
- **Reputation**: The industry standard for programmatic SMS.
- **Pricing**: Pay-as-you-go (approx. $0.0079 per SMS in the US, higher internationally).
- **Ease of Use for Non-Technical Users**: Users just type a message or configure a template. The platform handles carrier routing.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: System events (e.g., 24 hours before an appointment) or manual user action trigger a notification.
- **Action**: The OHC API server formats the message and sends it via the Twilio API.
- **User View**: A toggle in settings for "Enable SMS Reminders" and a simple text input for custom messages.

### Implementation Prompt
Implement automated SMS notifications using Twilio. Focus on high-value transactional messages like appointment reminders and order readiness alerts. The integration must handle the complexities of phone number formatting (E.164) and provide clear error messages if a delivery fails. Ensure there is a robust opt-out (STOP) handling mechanism to maintain compliance.

### Priority
P1

### Estimated Scope
Medium

---

## 7. Video Conferencing: Google Meet

### Title
`[video]embedded_consultations`: Implement Auto-Generated Google Meet Links

### Problem Statement
Virtual services (tutoring, consulting) are booming. Currently, owners have to manually create a Zoom or Meet link and email it to the client after they book. This manual step often leads to forgotten links, confused clients, and lost revenue.

### Research Report
- **Tool**: Google Meet API (via Google Workspace/Calendar integration)
- **Pros**: Free, ubiquitous, requires no software installation for the client.
- **Cons**: Requires the business owner to have a Google account (very common, though).
- **Reputation**: Highly reliable and trusted by consumers.
- **Pricing**: Free with standard Google accounts.
- **Ease of Use for Non-Technical Users**: Completely invisible. The link just appears on the booking confirmation.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: A customer books a service defined as "Virtual" or "Online".
- **Action**: The OHC API server requests a meeting link via the Google API (often bundled with the Calendar event creation) and saves the URL.
- **User View**: The booking confirmation page and email automatically display the "Join Meeting" button.

### Implementation Prompt
Enhance the booking flow to automatically generate Google Meet video conferencing links for virtual appointments. This should be tightly coupled with the Google Calendar integration. Ensure the meeting link is prominently displayed in the customer's confirmation email, the business owner's schedule view, and accessible via the API for any custom frontend clients.

### Priority
P2

### Estimated Scope
Small
