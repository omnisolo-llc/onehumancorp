# Scout: Tool Integration Research Q4

## Executive Summary
This report evaluates 7 specific integrations to expand OHC's capabilities for small business owners, operating in both Cloud (multi-tenant) and Standalone (local) environments.

---

# [SMS & Notifications] Twilio vs MessageBird Evaluation

## Title
Automated SMS Reminders via Twilio or MessageBird

## Problem Statement
Small business owners (especially non-technical ones or those with lower English proficiency) rely heavily on SMS for critical customer communication. Missed appointments, unread emails, and delayed updates lead to lost revenue. They need an automated, reliable way to send SMS reminders globally without dealing with complex carrier compliance.

## Research Report
- **Strategy**: Direct API integration for automated SMS.
- **Persona**: Food service operators, local service providers, international merchants.
- **Advantages**: Excellent global coverage, simple API. Twilio is the industry standard; MessageBird has strong international presence.
- **Risks**: US A2P 10DLC compliance is still a hurdle for merchants sending to US numbers.
- **Pricing**: Pay-per-message (~$0.0079/msg in US for Twilio). Both are affordable.
- **Compatibility**:
  - **Cloud**: OHC manages a central account/compliance.
  - **Standalone**: Requires a guided setup wizard for the user to provide their own API key.

## Design Doc
- **Trigger**: An appointment is booked, or an order is ready for pickup.
- **Action**: OHC automatically sends a pre-configured SMS template to the customer.
- **User Interface**: Business owner sees a simple toggle: "Enable SMS Reminders". They can customize a basic text template without touching API keys.

## Implementation Prompt
Implement a notification toggle in user settings to enable SMS reminders for appointments. When enabled, send a generic text message 24 hours before the appointment. The UI should simply explain the message content and provide a field to customize the closing greeting.

## Priority
P1

## Estimated Scope
Medium


---

# [Payment Processing] Mercado Pago Evaluation for LATAM

## Title
Integrate Mercado Pago for LATAM Merchants

## Problem Statement
Small businesses in Latin America often cannot use Stripe or standard US/EU payment gateways. They rely on local payment methods (e.g., Pix in Brazil, OXXO in Mexico). Without local payment options, they lose online sales and resort to manual bank transfers.

## Research Report
- **Strategy**: API integration with Mercado Pago.
- **Persona**: LATAM-based merchants and regional e-commerce stores.
- **Advantages**: Dominant in LATAM, highly trusted, familiar to consumers, supports local cards and instant transfers (Pix).
- **Risks**: Higher percentage fees compared to US Stripe.
- **Pricing**: Varies by country (typically 3-5% + fixed fee), essential for the market.
- **Compatibility**:
  - **Cloud**: OAuth-like flow or Centralized routing.
  - **Standalone**: User supplies API keys via a wizard.

## Design Doc
- **Trigger**: Customer clicks "Pay Now" on an invoice/checkout page.
- **Action**: OHC redirects to Mercado Pago checkout or embeds their elements.
- **User Interface**: Business owner clicks "Connect Mercado Pago" to authorize OHC. "Mercado Pago" then appears as an active payment method on invoices.

## Implementation Prompt
Add Mercado Pago as a payment provider option in billing settings. Provide a "Connect Mercado Pago" button. Once connected, generated invoices must include a secure Mercado Pago payment link for customers.

## Priority
P2

## Estimated Scope
Medium


---

# [Calendar & Scheduling] Cal.com Integration Evaluation

## Title
Automated Scheduling via Cal.com

## Problem Statement
Business owners spend too much time going back and forth via email/WhatsApp to schedule meetings or services. They need a simple booking page that syncs with their personal calendar to prevent double-booking.

## Research Report
- **Strategy**: Leverage Cal.com's robust scheduling engine.
- **Persona**: Consultants, tutors, professional services, salons.
- **Advantages**: Open-source alternative to Calendly, great developer experience, very clean UI for both merchant and customer.
- **Risks**: Learning curve for initial setup; self-hosting involves maintenance overhead.
- **Pricing**: Free for individuals, reasonable team plans. Can be self-hosted.
- **Compatibility**:
  - **Cloud**: Managed integration via Cal.com API.
  - **Standalone**: Connect to a self-hosted Cal.com instance or public API.

## Design Doc
- **Trigger**: Business owner shares availability.
- **Action**: OHC provisions a Cal.com booking link or embeds the widget on the business's webpage.
- **User Interface**: Business owner connects Google/Outlook Calendar, sets working hours in OHC, and OHC generates a shareable booking link.

## Implementation Prompt
Create a "Scheduling" tab where the business owner can define weekly availability. Provide a "Share Booking Link" button that copies a unique URL. Customers visiting this URL see available slots and book a session, which appears on the owner's dashboard.

## Priority
P1

## Estimated Scope
Large


---

# [Social Media Integration] WhatsApp Business API Evaluation

## Title
Unified Inbox via WhatsApp Business API

## Problem Statement
Small business owners manage customer communications across too many platforms. Missing a WhatsApp message can mean losing a sale. They need a unified inbox where WhatsApp messages appear alongside emails and SMS.

## Research Report
- **Strategy**: Direct integration with WhatsApp Cloud API.
- **Persona**: Retail stores, international merchants, service providers.
- **Advantages**: Integrates the default communication tool in many global markets into OHC, preventing dropped leads.
- **Risks**: Meta's business verification process can be tedious for merchants.
- **Pricing**: Conversation-based pricing (first 1,000 service conversations free).
- **Compatibility**:
  - **Cloud**: Managed via embedded signup.
  - **Standalone**: User provides their own Meta App credentials.

## Design Doc
- **Trigger**: Customer sends a message to the business's WhatsApp number.
- **Action**: Message is routed to OHC's Unified Inbox.
- **User Interface**: Business owner replies directly from OHC, and the message is sent back to the customer's WhatsApp.

## Implementation Prompt
Implement a WhatsApp channel integration for the Unified Inbox. Provide a setup wizard for the business owner to connect their WhatsApp Business account. Incoming messages should create a conversation thread in OHC, and replies from OHC should be routed back via the WhatsApp API.

## Priority
P1

## Estimated Scope
Large


---

# [Email Marketing] MailerLite Integration Evaluation

## Title
Native Email Campaigns via MailerLite Integration

## Problem Statement
Small business owners want to send newsletters and promotional emails to their customer base, but building a full drag-and-drop template editor internally is too complex.

## Research Report
- **Strategy**: API integration with MailerLite for subscriber management and campaign sending.
- **Persona**: E-commerce stores, content creators, boutique owners.
- **Advantages**: Offloads the complexity of email rendering and template design while keeping the trigger points natively in OHC.
- **Risks**: MailerLite API rate limits; synchronizing customer lists requires background jobs.
- **Pricing**: Free tier up to 1,000 subscribers, affordable thereafter.
- **Compatibility**:
  - **Cloud**: OAuth or API Key.
  - **Standalone**: API Key.

## Design Doc
- **Trigger**: User wants to send a campaign.
- **Action**: OHC triggers pre-built campaigns natively, making API calls to MailerLite.
- **User Interface**: Users input their API key. Customer emails collected during checkout are automatically synced to MailerLite. Basic analytics are displayed in OHC.

## Implementation Prompt
Integrate MailerLite to handle email marketing campaigns. Automatically sync OHC customer records to MailerLite subscriber groups. Provide a native UI to trigger specific email campaigns and view basic delivery analytics.

## Priority
P2

## Estimated Scope
Medium


---

# [Shipping & Logistics] ShipStation Integration Evaluation

## Title
Automated Label Generation via ShipStation Integration

## Problem Statement
Small business owners selling physical goods spend too much time copying addresses to find shipping rates and print labels. They need a single button natively within OHC to calculate rates, buy postage, and print labels.

## Research Report
- **Strategy**: API integration with ShipStation.
- **Persona**: E-commerce stores, crafters, physical goods sellers.
- **Advantages**: Aggregates carriers (USPS, UPS, FedEx) with discounted rates. Highly reliable API.
- **Risks**: ShipStation has its own monthly subscription cost, increasing merchant overhead. Complex API for international shipments.
- **Pricing**: Monthly subscription fee plus postage costs.
- **Compatibility**:
  - **Cloud**: OAuth or API Key.
  - **Standalone**: API Key.

## Design Doc
- **Trigger**: Merchant clicks "Buy Label".
- **Action**: OHC calls ShipStation to purchase the label and saves the PDF.
- **User Interface**: Merchant connects ShipStation. Live shipping rates show at checkout. Operations Agent highlights unfulfilled orders. Tracking numbers are auto-retrieved and emailed.

## Implementation Prompt
Integrate ShipStation to automate shipping label generation. The checkout flow must query live rates. The merchant order management view must allow purchasing/printing shipping labels natively. Tracking numbers must be automatically saved and sent to the customer.

## Priority
P1

## Estimated Scope
Large


---

# [Video Conferencing] Microsoft Teams/Zoom Integration Evaluation

## Title
Auto-Generate Meeting Links for Appointments

## Problem Statement
Small business owners offering virtual consultations spend unnecessary time manually creating meeting links and emailing them to clients. This workflow is error-prone and unprofessional.

## Research Report
- **Strategy**: Native OAuth integration with Zoom or Microsoft Graph API.
- **Persona**: Tutors, online consultants, B2B services.
- **Advantages**: Highly professional. Parity with industry standards. Keeps the scheduling and delivery flow entirely automated.
- **Risks**: OAuth permissions can be granular and confusing to configure for the initial developer setup.
- **Pricing**: Free tiers exist; Teams included in Microsoft 365.
- **Compatibility**:
  - **Cloud**: OAuth.
  - **Standalone**: Server-to-Server OAuth or User OAuth.

## Design Doc
- **Trigger**: Customer books an online service.
- **Action**: OHC schedules a meeting via API and attaches the join URL.
- **User Interface**: User connects their Zoom/Microsoft account. During service creation, they set location to "Online Meeting".

## Implementation Prompt
Build an integration that dynamically creates meeting links for online service bookings. Users authenticate their account. Upon booking, OHC generates a unique link and attaches it to the appointment record and outgoing notifications.

## Priority
P2

## Estimated Scope
Medium
