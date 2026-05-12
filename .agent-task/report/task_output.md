# Scout: Tool Integration Research [Q2]

## 1. Social Media Integration
### Title
Native Meta Graph API Integration for Unified Inbox

### Problem Statement
Maya (The Home Baker) receives orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. She needs a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.

### Research Report
*   **Strategy**: Direct integration with Meta Graph API.
*   **Target Persona**: Maya (Home Baker), Priya (Boutique Owner).
*   **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
*   **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
*   **Pricing**: Free API usage.
*   **Evaluation Criteria**: OAuth complexity is high (requires app review), but message parsing quality and webhook reliability are excellent since it's the native platform API.
*   **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).

### Design Doc
*   User goes to the Operations dashboard and clicks "Connect Instagram".
*   User authenticates with Facebook/Instagram via OAuth.
*   OHC registers webhooks to receive new DMs.
*   When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
*   The user sees a unified "Customer Inbox" on their phone showing the conversation history.

### Implementation Prompt
Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling
### Title
Native Calendar Sync for Automated Booking

### Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.

### Research Report
*   **Strategy**: Direct Google Calendar API / CalDAV integration.
*   **Target Persona**: Carlos (Handyman), Leo (Music Tutor).
*   **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
*   **Risks**: Handling complex timezone logic internally.
*   **Pricing**: Free API usage.
*   **Evaluation Criteria**: Calendar conflict resolution is handled natively by Google. Timezone handling is built into the API but requires careful mapping to the user's local timezone. Booking page customization is handled entirely by OHC's UI.
*   **Compatibility**: Cloud (OAuth). Standalone (OAuth).

### Design Doc
*   User goes to Sales dashboard and connects their Google account.
*   OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
*   When a customer clicks to book, they use OHC's native booking widget.
*   Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.

### Implementation Prompt
Create a native booking widget and Google Calendar OAuth integration. Calculate availability based on existing calendar events and sync new bookings directly to Google Calendar.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing
### Title
Native Email Campaign Manager (SendGrid/SES)

### Problem Statement
Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.

### Research Report
*   **Strategy**: Build a native email campaign manager utilizing a transactional email API (SendGrid or AWS SES).
*   **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor).
*   **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool. No additional SaaS subscriptions required for the user.
*   **Risks**: Requires building list management and unsubscribe logic internally.
*   **Pricing**: Included in OHC platform costs (transactional API costs scale predictably).
*   **Evaluation Criteria**: Ease of list management is high since it's native to OHC. Template quality depends on OHC's internal builder. Open rate analytics are provided by SendGrid/SES webhooks. Spam compliance must be strictly managed by OHC (e.g., handling bounces and unsubscribes).
*   **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).

### Design Doc
*   When a customer buys something, they are automatically added to the native OHC customer list with tags.
*   The Marketing agent suggests campaigns natively in the UI.
*   The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
*   The user sees open rates and clicks in the OHC Marketing dashboard.

### Implementation Prompt
Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.

### Priority
P1

### Estimated Scope
Large

---

## 4. Payment Processing
### Title
Native Integration of Local Payment Methods (Mercado Pago)

### Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

### Research Report
*   **Strategy**: Direct API integration with Mercado Pago for seamless LATAM coverage.
*   **Target Persona**: Global users outside the US/EU.
*   **Advantages**: Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools.
*   **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
*   **Pricing**: Standard transaction fees apply; merchants expect these.
*   **Evaluation Criteria**: Settlement speed is generally 14-30 days for credit cards in LATAM via Mercado Pago, but instant for Pix. Currency support is excellent for local currencies (BRL, ARS, MXN, etc.). Failure rates are lower than international processors trying to process local cards.
*   **Compatibility**: Cloud (Centralized routing). Standalone (Centralized routing).

### Design Doc
*   User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
*   User connects their Mercado Pago account.
*   Customers see a "Pay with Mercado Pago" button at checkout natively.
*   Webhooks update the order status in OHC when payment succeeds.

### Implementation Prompt
Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings. Webhooks must normalize into standard OHC order fulfillment events.

### Priority
P1

### Estimated Scope
Large

---

## 5. Shipping & Logistics
### Title
Native Shipping Rate Calculation and Label Generation (Shippo)

### Problem Statement
Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

### Research Report
*   **Strategy**: Build a native fulfillment interface powered by the Shippo API in the backend.
*   **Target Persona**: Priya (Boutique Owner), Maya (Home Baker).
*   **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC.
*   **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
*   **Pricing**: Free tier available, nominal fee per label thereafter.
*   **Evaluation Criteria**: Carrier coverage is excellent (aggregates USPS, UPS, FedEx, DHL, etc.). International support is good but requires detailed customs documentation. API reliability is very high.
*   **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
*   When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
*   The Operations agent shows the cheapest shipping option.
*   The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo and saves the tracking number.
*   OHC automatically emails the customer the tracking number.

### Implementation Prompt
Implement a native shipping and fulfillment module powered by Shippo. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.

### Priority
P1

### Estimated Scope
Large

---

## 6. SMS & Notifications
### Title
Native SMS Order Notifications (Twilio)

### Problem Statement
Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.

### Research Report
*   **Strategy**: Direct integration with the Twilio SDK for native outbound SMS.
*   **Target Persona**: Fatima (Food Cart Operator).
*   **Advantages**: Invisible to the user. They just toggle "Send SMS reminders" in their settings.
*   **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
*   **Pricing**: Pay-per-message (~$0.0079 per SMS in US). OHC will need to manage quotas or require merchants to buy "SMS Credits".
*   **Evaluation Criteria**: Global carrier coverage is industry-leading. Delivery reliability is extremely high. Opt-out compliance (STOP messages) is handled automatically by Twilio.
*   **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

### Design Doc
*   User goes to Settings and toggles "Send me SMS for new orders".
*   When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
*   The Operations Agent decides the optimal time to send the reminder.

### Implementation Prompt
Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message. Include a settings panel for merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164).

### Priority
P2

### Estimated Scope
Medium

---

## 7. Video Conferencing
### Title
Native Zoom Link Generation for Appointments

### Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.

### Research Report
*   **Strategy**: Native OAuth integration with the Zoom API.
*   **Target Persona**: Leo (Music Tutor).
*   **Advantages**: Standard OAuth connection process. Highly intuitive.
*   **Risks**: Zoom OAuth requires annual app review and compliance checks.
*   **Pricing**: API is free for Zoom users, but requires the merchant to have a Zoom account. Pro starts at $15/mo.
*   **Evaluation Criteria**: Link generation speed is instant via API. Calendar invite quality is high (includes direct join links and dial-in numbers). Join experience is ubiquitous and familiar to most users.
*   **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

### Design Doc
*   In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
*   Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
*   The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.

### Implementation Prompt
Build a Zoom integration that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account. When a customer books a service marked as "Online Meeting", the system must dynamically generate a Zoom link, store it with the booking, and share it with both the merchant and the customer.

### Priority
P2

### Estimated Scope
Medium
