# 🔍 Scout: Tool Integration Research Q2 2024

This report evaluates and proposes integration issue briefs for top-tier tools across 7 key operational categories, carefully selected for the non-technical small business owner persona of OneHumanCorp (OHC).

---

## 1. Social Media Integration: ManyChat

### Title
Integrate ManyChat for Unified Inbox & Auto-Replies

### Problem Statement
Business owners like Maya (The Home Baker) receive countless DMs across Instagram, Facebook, and WhatsApp asking the same questions (e.g., "Do you make vegan cakes?", "What are your hours?"). Managing these scattered messages is overwhelming, leading to missed sales and slow response times. A unified inbox with AI-assisted auto-replies is critical to save time and capture leads.

### Research Report
- **Tool Evaluated**: ManyChat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Ease of Use**: Excellent visual flow builder, though OHC will abstract this so the user just types rules or uses the AI agent.
- **Pricing**: Free tier available (up to 1,000 contacts); Pro starts at $15/month. Very accessible.
- **Cloud & Standalone Capable**: Yes. Webhooks can push to OHC Cloud or Standalone via relay.
- **Pros**: Official Meta partner, supports IG, FB Messenger, and WhatsApp. Reliable webhook delivery.
- **Cons**: Initial OAuth connection can be slightly confusing for non-tech users, requiring clear guidance.

### Design Doc
- **Integration Point**: "Customer Success" & "Marketing" AI Departments.
- **User Experience**: The user links their Instagram/FB account in OHC. The AI agent asks the user what FAQs they want to automate. OHC configures ManyChat flows via API in the background. Incoming messages flow into a single OHC inbox.
- **Mechanisms**:
  - OAuth flow to link Meta/ManyChat accounts.
  - Webhooks from ManyChat to OHC to receive messages in real-time.
  - OHC API sends responses back through ManyChat to the customer.

### Implementation Prompt
Implement a ManyChat integration that allows users to link their Meta accounts via a single click. Create a unified inbox UI in the Flutter app where all DMs are visible. The Customer Success AI agent should be able to draft replies to these messages and automatically handle common FAQs based on the business's data. Provide clear UI feedback if the OAuth connection fails.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling: Cal.com

### Title
Integrate Cal.com for Seamless Booking & Calendar Sync

### Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) rely on bookings. Managing availability manually causes double-booking and endless back-and-forth messaging. They need a simple link where customers can book open slots that automatically sync with their personal Google or Apple calendars.

### Research Report
- **Tool Evaluated**: Cal.com
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Ease of Use**: Extremely clean interface. API-first design makes it invisible to the end user if embedded correctly.
- **Pricing**: Free for individuals; Team plans at $12/user/month. Open-source core.
- **Cloud & Standalone Capable**: Yes. Can be self-hosted (Standalone) or consumed as SaaS (Cloud).
- **Pros**: Open-source, extensive API, handles timezones natively, robust conflict resolution.
- **Cons**: Advanced routing features might be too complex for a solo user, so we must limit exposed options.

### Design Doc
- **Integration Point**: "Operations" AI Department.
- **User Experience**: The user connects their Google Calendar. They specify their working hours (e.g., 9 AM - 5 PM). OHC automatically generates a booking widget on their public profile. The Operations Agent adds bookings to the calendar and handles rescheduling.
- **Mechanisms**:
  - Leverage Cal.com Platform API for white-labeled booking.
  - OHC maintains the booking UI natively in Flutter, piping data to Cal.com for slot calculation and sync.

### Implementation Prompt
Integrate Cal.com API to power the OHC scheduling engine. Create a mobile-friendly booking widget for the public storefront. Build a settings screen for the business owner to define working hours and connect their external calendar. The Operations Agent must be notified of new bookings via webhook to trigger confirmation emails.

### Priority
P0

### Estimated Scope
Large

---

## 3. Email Marketing: Resend

### Title
Integrate Resend for Automated Customer Campaigns

### Problem Statement
Priya (Boutique Owner) wants to notify past customers when new inventory arrives, but traditional platforms like Mailchimp are bloated and complex. She needs an invisible tool that sends beautifully branded emails automatically when she triggers an update.

### Research Report
- **Tool Evaluated**: Resend
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Ease of Use**: Developer-focused, but allows OHC to build a zero-config UI on top. The business owner never sees Resend.
- **Pricing**: 3,000 free emails/month. $20/month for 50,000 emails. Ideal for small businesses.
- **Cloud & Standalone Capable**: Yes. Cloud uses Resend API; Standalone can fallback to SMTP.
- **Pros**: Blazing fast API, excellent deliverability, modern React-email templates (translatable to HTML).
- **Cons**: Lacks a built-in WYSIWYG editor for end-users, requiring OHC to build the UI or use AI generation.

### Design Doc
- **Integration Point**: "Marketing & Advertising" AI Department.
- **User Experience**: The AI asks Priya, "Do you want to announce your new summer collection to your 200 customers?" Priya says "Yes." The AI generates the email content, shows a preview, and sends it upon approval.
- **Mechanisms**:
  - OHC uses Resend API to send transactional and bulk emails.
  - Webhooks track opens and clicks, feeding data back to the Business Advisory Agent.

### Implementation Prompt
Implement an email marketing engine powered by Resend. Create a clean Flutter UI where the user can view AI-generated draft campaigns, approve them, and see open/click rates. Ensure the Marketing AI can automatically generate these drafts based on new inventory or events. Implement a webhook receiver to update campaign metrics.

### Priority
P1

### Estimated Scope
Medium

---

## 4. Payment Processing: Mercado Pago

### Title
Integrate Mercado Pago for LATAM Payment Processing

### Problem Statement
While Stripe covers many regions, business owners in Latin America heavily rely on local payment methods (Pix, Boleto, local credit cards) that Stripe either doesn't support or prices uncompetitively. We need a localized payment processor to empower LATAM businesses.

### Research Report
- **Tool Evaluated**: Mercado Pago
- **Target Persona**: All personas operating in LATAM.
- **Ease of Use**: Well-known brand in LATAM, increasing buyer trust.
- **Pricing**: Percentage per transaction (varies by country, typically 3-5%). No fixed monthly fees.
- **Cloud & Standalone Capable**: Yes via standard REST APIs.
- **Pros**: Dominant in LATAM, supports Pix (Brazil) which is mandatory, supports local installments (cuotas).
- **Cons**: API documentation can be fragmented. Dispute resolution takes time.

### Design Doc
- **Integration Point**: "Finance & Payments" AI Department.
- **User Experience**: In the payments setup screen, if the user's country is in LATAM, Mercado Pago is offered as a 1-click connect option. Checkout flows dynamically offer Pix or local cards.
- **Mechanisms**:
  - OAuth flow to connect Mercado Pago seller accounts.
  - Generate Payment Preferences for checkout links.
  - Webhook listener for payment status updates (approved, pending, rejected).

### Implementation Prompt
Integrate the Mercado Pago API as an alternative payment provider to Stripe. Build a setup flow for LATAM users to connect their accounts. Update the checkout UI to support Mercado Pago's Web Tokenize Checkout or Checkout Pro. Ensure all transactions are recorded in the OHC ledger and visible to the Finance Agent.

### Priority
P1

### Estimated Scope
Medium

---

## 5. Shipping & Logistics: Shippo

### Title
Integrate Shippo for Automated Label Generation

### Problem Statement
Product sellers like Priya (Boutique) spend hours manually entering addresses into carrier websites to buy shipping labels. This manual data entry leads to errors, wrong shipments, and wasted time. They need 1-click label purchasing directly from their order dashboard.

### Research Report
- **Tool Evaluated**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker - for shipped items)
- **Ease of Use**: API abstracts multiple carriers (USPS, UPS, FedEx, DHL) into one interface.
- **Pricing**: Free tier (pay per label + postage). Excellent pre-negotiated carrier rates.
- **Cloud & Standalone Capable**: Yes, standard REST API.
- **Pros**: Massive time saver, great shipping discounts natively included, handles international customs forms automatically.
- **Cons**: Some obscure local carriers may not be supported.

### Design Doc
- **Integration Point**: "Operations" AI Department.
- **User Experience**: When an order is placed, the Operations Agent prepares a shipping label. The business owner clicks "Buy Label & Print" in the OHC app. Tracking info is automatically emailed to the customer via the Customer Success Agent.
- **Mechanisms**:
  - Address validation via Shippo API.
  - Rate quoting at checkout (optional).
  - Label generation (PDF/ZPL) via API upon order fulfillment.

### Implementation Prompt
Integrate Shippo to enable 1-click shipping label generation. Add an address validation step during checkout. In the order management UI, add a "Purchase Label" flow that shows real-time rates, purchases the label, and provides a printable PDF. Ensure the Customer Success Agent is triggered to send tracking numbers to buyers.

### Priority
P2

### Estimated Scope
Large

---

## 6. SMS & Notifications: Twilio

### Title
Integrate Twilio for Critical SMS Notifications

### Problem Statement
Fatima (Food Cart Operator) is busy cooking and may not hear an app notification or have reliable data coverage for push notifications. She needs an immediate, offline-capable SMS ping when a new pickup order arrives. Customers also appreciate SMS updates for their food orders.

### Research Report
- **Tool Evaluated**: Twilio
- **Target Persona**: Fatima (Food Cart), Carlos (Handyman)
- **Ease of Use**: Industry standard, highly reliable. OHC hides all API complexity.
- **Pricing**: ~$0.0079 per SMS. Very cheap for critical alerts.
- **Cloud & Standalone Capable**: Yes.
- **Pros**: Global reach, ultra-reliable delivery, supports WhatsApp Business API if we decide to expand.
- **Cons**: Strict A2P 10DLC compliance rules in the US require automated business registration flows.

### Design Doc
- **Integration Point**: "Operations" and "Customer Success" AI Departments.
- **User Experience**: Fatima toggles "Send me a text for new orders" in settings. Customers can opt-in to SMS updates at checkout. Twilio handles the routing silently.
- **Mechanisms**:
  - Twilio Programmable SMS API for outbound messages.
  - Phone number validation and formatting.
  - Automated A2P 10DLC registration handling for US tenants (via API).

### Implementation Prompt
Implement Twilio SMS integration for critical alerts. Add a toggle in the business owner's settings for SMS order notifications. Update the checkout flow to allow customers to opt-in for SMS order updates. The Operations Agent should dispatch SMS alerts for high-priority events (e.g., new order, cancellation). Provide fallback mechanisms if SMS fails.

### Priority
P1

### Estimated Scope
Medium

---

## 7. Video Conferencing: Daily.co

### Title
Integrate Daily.co for Frictionless Online Lessons

### Problem Statement
Leo (Music Tutor) spends 5 minutes before every online lesson manually generating a Zoom link and emailing it to his student. Students often lose the link. He needs automated, embedded video calls that live directly inside his OHC portal.

### Research Report
- **Tool Evaluated**: Daily.co
- **Target Persona**: Leo (Music Tutor), Carlos (Consultations)
- **Ease of Use**: API-first video infra. We can embed the video player directly in the OHC Flutter app.
- **Pricing**: 10,000 free participant minutes/month. Pay-as-you-go after.
- **Cloud & Standalone Capable**: Yes.
- **Pros**: Embeddable via WebRTC (no app download required for the student), highly customizable, generous free tier.
- **Cons**: Less brand recognition than Zoom, but better integrated experience.

### Design Doc
- **Integration Point**: "Operations" AI Department.
- **User Experience**: When a student books a lesson, the system auto-generates a Daily.co room. Both Leo and the student get a "Join Lesson" button in their respective portals. Clicking it opens a beautiful, OHC-branded video room inside the browser/app.
- **Mechanisms**:
  - Daily.co REST API to create isolated rooms with expiration times.
  - Embed Daily Prebuilt iframe in the Web version, and use their Flutter SDK for native mobile.

### Implementation Prompt
Integrate Daily.co to auto-generate video meeting rooms for service bookings. When an online service is booked, create a unique room URL via API. Build a "Join Meeting" UI component in the Flutter app using the Daily Flutter SDK (or WebView for Prebuilt). Ensure the Operations Agent sends calendar invites containing this specific link.

### Priority
P2

### Estimated Scope
Large
