# Tool Integration Research Report

## [Social Media] Meta Graph API Integration
**Title**: Integrate Meta Graph API for Unified Native Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply, and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar] Google Calendar API Integration
**Title**: Native Calendar Sync for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar.
**Research Report**:
- **Strategy**: Direct Google Calendar API / CalDAV integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
- **Risks**: Handling complex timezone logic internally.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).
**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types.
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
**Implementation Prompt**: Create a native booking widget and Google Calendar OAuth integration. Calculate availability based on existing calendar events and sync new bookings directly to Google Calendar.
**Priority**: P1
**Estimated Scope**: Medium

## [Email Marketing] Native Email Campaign Manager
**Title**: Native Email Campaign Manager (SendGrid/SES)
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.
**Research Report**:
- **Strategy**: Build a native email campaign manager utilizing a transactional email API (SendGrid or AWS SES)
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool. No additional SaaS subscriptions required for the user.
- **Risks**: Requires building list management and unsubscribe logic internally.
- **Pricing**: Included in OHC platform costs (transactional API costs scale predictably).
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).
**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
**Implementation Prompt**: Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.
**Priority**: P1
**Estimated Scope**: Large

## [Payment Processing] Native Integration of Local Payment Methods
**Title**: Native Integration of Local Payment Methods (Mercado Pago)
**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods natively within the OHC platform.
**Research Report**:
- **Strategy**: Direct API integration with Mercado Pago for seamless LATAM coverage.
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Native integration within the OHC platform ensures a seamless onboarding experience.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Standard transaction fees apply.
- **Compatibility**: Cloud (Centralized routing). Standalone (Centralized routing).
**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings.
**Priority**: P1
**Estimated Scope**: Large

## [Shipping] Shippo Integration
**Title**: Native Shipping Rate Calculation and Label Generation (Shippo)
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label.
**Research Report**:
- **Strategy**: Build a native fulfillment interface powered by the Shippo API in the backend.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC.
- **Risks**: International shipping requires complex customs declarations.
- **Pricing**: Free tier available, nominal fee per label thereafter.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo.
- OHC automatically emails the customer the tracking number.
**Implementation Prompt**: Implement a native shipping and fulfillment module powered by Shippo. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly.
**Priority**: P1
**Estimated Scope**: Large

## [SMS] Native SMS Order Notifications
**Title**: Native SMS Order Notifications (Twilio)
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives.
**Research Report**:
- **Strategy**: Direct integration with the Twilio SDK for native outbound SMS.
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Invisible to the user. They just toggle "Send SMS reminders" in their settings.
- **Risks**: A2P 10DLC compliance in the US is complex.
- **Pricing**: Pay-per-message.
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).
**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- The Operations Agent decides the optimal time to send the reminder.
**Implementation Prompt**: Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message.
**Priority**: P2
**Estimated Scope**: Medium

## [Video Conferencing] Native Zoom Link Generation
**Title**: Native Zoom Link Generation for Appointments
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. He needs links to be generated automatically natively when a lesson is booked.
**Research Report**:
- **Strategy**: Native OAuth integration with the Zoom API.
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Standard OAuth connection process. Highly intuitive.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: API is free for Zoom users, but requires the merchant to have a Zoom account.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).
**Design Doc**:
- In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite.
**Implementation Prompt**: Build a Zoom integration that automatically creates meeting links for online service bookings.
**Priority**: P2
**Estimated Scope**: Medium
