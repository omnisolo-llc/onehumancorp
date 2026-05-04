# 🔍 Scout: Tool Integration Research Q2

## [Social Media] Meta Graph API Integration
**Title**: Integrate Meta Graph API for Unified Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Tool**: Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Adheres to 'Radical Simplicity' by keeping the user entirely within the OHC platform. Eliminates the need for users to learn or pay for a third-party tool like Manychat. Provides direct native access to Instagram, Messenger, and WhatsApp messages.
- **Risks**: Requires building and maintaining robust webhook handlers and rate-limit management. The app must undergo Meta's app review process, which can be stringent and time-consuming.
- **Pricing**: Free for standard usage levels. No per-contact monthly fees for the business owner.
- **Compatibility**: Cloud (via webhooks/OAuth).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth directly within OHC.
- OHC registers webhooks to receive new DMs via Meta Graph API.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via the API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history natively inside OHC.
**Implementation Prompt**: Implement an OAuth flow to connect a user's Meta account via Meta Graph API. Create a webhook endpoint that receives incoming messages from Messenger, Instagram, and WhatsApp, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar] Google Calendar API Integration
**Title**: Native OHC Booking via Google Calendar API
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly on their calendar.
**Research Report**:
- **Tool**: Google Calendar API
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Aligns with 'Radical Simplicity' by providing a native, seamless booking experience without sending users to a third-party site like Calendly. Fully integrated with OHC's internal dashboards and agent triggers.
- **Risks**: Implementing complex calendar logic (e.g., timezone conversions, recurring events, conflict resolution) natively is error-prone and requires significant engineering effort.
- **Pricing**: Free tier within Google Cloud API limits. No SaaS subscription fees for the user.
- **Compatibility**: Cloud (OAuth).
**Design Doc**:
- User goes to Sales dashboard and connects their Google Calendar via OAuth.
- User sets up event types and availability natively in OHC.
- OHC checks Google Calendar API for conflicts in real-time and displays available slots on the user's public storefront.
- When a customer clicks to book, they do it directly in the OHC native widget.
- Upon successful booking, OHC pushes the event to Google Calendar.
**Implementation Prompt**: Create a native scheduling engine in OHC powered by the Google Calendar API. Implement OAuth for calendar connection, real-time availability checking, and a native booking widget for public profiles that syncs booked events back to Google Calendar.
**Priority**: P1
**Estimated Scope**: Medium

## [Email Marketing] Native Email via SendGrid/SES
**Title**: Native Customer Re-engagement via SendGrid/SES
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns. She needs an automated way to email customers without leaving the OHC app.
**Research Report**:
- **Tool**: Native Email Marketing via SendGrid or AWS SES
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Provides a radically simple, "invisible" infrastructure approach. Users don't have to learn a separate platform like Mailchimp or deal with syncing contact lists. OHC retains full control of the data and user experience.
- **Risks**: Managing email deliverability, bounce rates, and anti-spam compliance (CAN-SPAM/GDPR) natively requires dedicated infrastructure monitoring.
- **Pricing**: Infrastructure cost based on volume (e.g., fractions of a cent per email). No external SaaS tiers for the business owner.
- **Compatibility**: Cloud (API).
**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer directory with tags (e.g., "Bought: Cake").
- The Marketing agent suggests campaigns ("Send an email to past customers about your new holiday cakes").
- The user approves the AI-generated email, and OHC triggers SendGrid/SES to send it.
- The user sees open rates and clicks natively in the OHC Marketing dashboard.
**Implementation Prompt**: Build a native email marketing system in OHC using SendGrid or AWS SES as the delivery backbone. Automatically segment customers based on purchase history and allow the AI Marketing agent to draft and send campaigns natively, displaying analytics within OHC.
**Priority**: P1
**Estimated Scope**: Medium

## [Payment] Mercado Pago Integration
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil.
**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico). Good developer docs.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a secondary payment provider. Implement the checkout flow to redirect to Mercado Pago and handle the success/failure webhooks to update order status.
**Priority**: P2
**Estimated Scope**: Large

## [Shipping] Shippo Integration
**Title**: Integrate Shippo for Automated Label Generation
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label.
**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get rates.
- The Operations agent shows the cheapest shipping option.
- The user clicks "Buy Label", and OHC downloads the PDF label for printing.
- OHC automatically emails the customer the tracking number.
**Implementation Prompt**: Connect the Shippo API to fetch shipping rates based on order weight/dimensions. Allow the user to purchase a label and automatically email the tracking link to the customer.
**Priority**: P1
**Estimated Scope**: Large

## [SMS] Twilio Integration
**Title**: Integrate Twilio for SMS Order Notifications
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable SMS alerts when a new pre-order arrives so she can start cooking.
**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Global coverage, incredibly reliable. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).
**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, the Operations agent triggers a Twilio API call to send an SMS: "New order! 2x Falafel for John. Pickup in 15m."
- (Future: Customers can also receive SMS receipts).
**Implementation Prompt**: Integrate the Twilio SDK to send outbound SMS notifications. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure compliance with local messaging regulations.
**Priority**: P2
**Estimated Scope**: Medium

## [Video] Zoom Integration
**Title**: Integrate Zoom for Auto-Generated Meeting Links
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically when a lesson is booked.
**Research Report**:
- **Tool**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Strong API for meeting creation.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: Free tier (40-min limit). Pro starts at $15/mo.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).
**Design Doc**:
- User connects their Zoom account via the Sales dashboard.
- When a customer books an online service (e.g., via Calendly or native booking), OHC calls the Zoom API to create a meeting.
- The Zoom link is embedded in the automated calendar invite and confirmation email sent to the customer.
**Implementation Prompt**: Create an OAuth integration with Zoom. Automatically generate a unique Zoom meeting link when a customer books a virtual service, and include this link in the customer's confirmation email.
**Priority**: P1
**Estimated Scope**: Medium
