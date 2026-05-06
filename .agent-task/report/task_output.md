# Tool Integration Research Q4

## [Social Media Integration] Unified Inbox via Meta Graph API
**Title**: Integrate Meta Graph API for Instagram & Messenger Unified Inbox
**Problem Statement**: Small business owners (like bakers or consultants) receive customer inquiries across Instagram DMs, Facebook comments, and WhatsApp. Switching between apps causes them to miss messages and lose sales. They need a single place to view and reply to all customer messages without navigating multiple platforms.
**Research Report**:
- **Tool Evaluated**: Meta Graph API (Messenger, Instagram Direct, WhatsApp Business).
- **Benefits**: Official API, no middleman SaaS fees (like ManyChat), direct real-time webhooks.
- **Ease of Use**: Once connected via OAuth, the business owner simply sees messages appear in the OHC unified inbox. No technical configuration required post-login.
- **Pricing**: Free for standard messaging limits; WhatsApp has per-conversation pricing after the first 1,000.
- **Cloud vs Standalone**: Works well in Cloud mode via central OAuth app. In Standalone mode, requires user to supply their own Meta Developer credentials or use a cloud-relay service.
**Design Doc**:
- **Trigger**: User clicks "Connect Facebook/Instagram" in OHC settings.
- **Action**: Completes Meta OAuth flow. OHC subscribes to webhooks for DMs and comments. Incoming messages create threads in the OHC Inbox UI. Replies from the OHC Inbox are sent back via the Meta API.
- **User Experience**: A seamless "Connect" button in settings. A unified chat interface where Instagram and Facebook icons denote message origin.
**Implementation Prompt**: Implement the Meta OAuth flow to allow users to connect their Instagram and Facebook pages. Establish webhook listeners to ingest incoming messages into the OHC Inbox and allow outbound replies from the same UI. Success is defined by a user successfully receiving an Instagram DM and replying to it directly from OHC.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar & Scheduling] Automated Booking via Calendly API
**Title**: Integrate Calendly API for Automated Consultation Scheduling
**Problem Statement**: Business owners waste hours going back and forth over email or text to find a meeting time. They need a way to let clients book available slots directly without manual coordination, respecting their existing personal calendars.
**Research Report**:
- **Tool Evaluated**: Calendly API
- **Benefits**: Industry standard, handles complex timezone math, automatically syncs with Google/Outlook, and generates meeting links.
- **Ease of Use**: Very familiar to most users. Once connected, OHC can automatically pull the user's scheduling links and embed them in client portals or chat.
- **Pricing**: Basic is free; API access requires Professional tier ($12/mo).
- **Cloud vs Standalone**: Works in both modes via OAuth. Standalone will need a standard OAuth proxy or direct token entry.
**Design Doc**:
- **Trigger**: User connects Calendly in OHC Settings.
- **Action**: OHC fetches the user's active event types and scheduling links. When a new prospect is added, OHC can automatically suggest sending the primary booking link.
- **User Experience**: User sees their Calendly event types synced in the OHC dashboard. In the unified chat, they have a 1-click "Send Scheduling Link" button that drops their specific Calendly URL into the conversation.
**Implementation Prompt**: Build an OAuth integration with Calendly to fetch the user's event types. Add a UI component in the customer messaging view to allow the business owner to quickly insert their Calendly link into active conversations. Success is measured by a user successfully authenticating and sending a fetched link via chat.
**Priority**: P1
**Estimated Scope**: Medium

## [Email Marketing] Sync Customer Lists with Mailchimp
**Title**: Integrate Mailchimp for Seamless Email Campaign Sync
**Problem Statement**: Small businesses capture leads and customer details in OHC, but must manually export/import CSV files to send newsletters or marketing blasts. This is tedious and error-prone.
**Research Report**:
- **Tool Evaluated**: Mailchimp API
- **Benefits**: Widely used by SMBs, great drag-and-drop builder, strong deliverability.
- **Ease of Use**: OHC automatically keeps the OHC contact list and Mailchimp audience in sync in the background. The user just goes to Mailchimp to design their email.
- **Pricing**: Free tier up to 500 contacts, then starts at $13/mo.
- **Cloud vs Standalone**: Fully supported in both modes via standard OAuth 2.0.
**Design Doc**:
- **Trigger**: User connects Mailchimp via OHC settings.
- **Action**: OHC performs an initial two-way sync of contacts. OHC listens for new contacts added or updated in the CRM and pushes them to the Mailchimp audience list automatically via background jobs.
- **User Experience**: User sees a "Syncing to Mailchimp" indicator on their CRM list. No manual CSV handling is ever required again.
**Implementation Prompt**: Create a background synchronization worker that pushes new or updated OHC contacts to a connected Mailchimp account's primary audience list. Include an OAuth connection flow. Success is defined by a newly created OHC contact appearing in Mailchimp within 1 minute.
**Priority**: P2
**Estimated Scope**: Medium

## [Payment Processing] LATAM Payment Gateway via Mercado Pago
**Title**: Integrate Mercado Pago for LATAM Invoicing & Payments
**Problem Statement**: Stripe is not available or preferred in many Latin American countries. Business owners in these regions need a localized way to generate payment links and accept local payment methods (e.g., PIX in Brazil, local credit cards) directly from their invoices.
**Research Report**:
- **Tool Evaluated**: Mercado Pago API
- **Benefits**: Dominant in LATAM, supports local currency settlement, integrates with local banking apps and PIX.
- **Ease of Use**: Generates a standard payment link that the customer clicks. For the business owner, invoices automatically mark themselves as "Paid" upon webhook receipt.
- **Pricing**: Percentage per transaction (varies by country, usually 3-5%), no monthly fee.
- **Cloud vs Standalone**: Works natively in both. Standalone can receive webhooks if exposed via tunnel, or rely on polling.
**Design Doc**:
- **Trigger**: User creates an invoice in OHC and selects "Mercado Pago" as the processor.
- **Action**: OHC generates a Mercado Pago checkout preference and creates a shareable payment link. OHC listens for IPN (Instant Payment Notification) webhooks to update invoice status.
- **User Experience**: User sees a "Generate LATAM Payment Link" button on invoices. Customers click the link and pay via their familiar local methods. Invoices automatically turn green ("Paid").
**Implementation Prompt**: Implement Mercado Pago OAuth and checkout preference generation. When an invoice is created, generate a Mercado Pago payment link and display it on the invoice. Implement an IPN webhook handler to mark the invoice as paid. Success is a generated payment link and successful webhook parsing.
**Priority**: P1
**Estimated Scope**: Large

## [Shipping & Logistics] Automated Label Generation via Shippo
**Title**: Integrate Shippo API for Automated Shipping Labels
**Problem Statement**: E-commerce and physical goods businesses manually type customer addresses into carrier websites to buy shipping labels. This takes hours and causes shipping errors. They need to buy labels and get tracking numbers instantly from their order screen.
**Research Report**:
- **Tool Evaluated**: Shippo API
- **Benefits**: Aggregates USPS, UPS, FedEx, DHL, and regional carriers. Negotiated discounts included.
- **Ease of Use**: Business owner clicks "Buy Label" on an OHC order, selects box size, and prints the PDF. Tracking is auto-emailed to the customer.
- **Pricing**: Pay-as-you-go (usually $0.05 per label) or $10/mo for no per-label fee.
- **Cloud vs Standalone**: Standard API integration, works in both modes.
**Design Doc**:
- **Trigger**: User views a "Pending Fulfillment" order and clicks "Create Shipping Label".
- **Action**: OHC sends origin and destination addresses to Shippo, retrieves rates, allows user to select rate, purchases label, and stores the tracking number and PDF URL.
- **User Experience**: A seamless inline modal showing carrier options and prices. A "Print Label" button appears after purchase.
**Implementation Prompt**: Integrate the Shippo API to validate addresses, fetch shipping rates for an order, purchase a selected rate, and return a printable PDF label. Ensure tracking numbers are saved to the order record. Success is defined by a test user successfully generating a sandbox label PDF.
**Priority**: P2
**Estimated Scope**: Large

## [SMS & Notifications] Global SMS via Twilio
**Title**: Integrate Twilio for Global SMS Notifications
**Problem Statement**: Some users, like Fatima, have low English proficiency and prefer immediate SMS alerts for critical business events (like a new booking or canceled order) rather than checking email or logging into a dashboard.
**Research Report**:
- **Tool Evaluated**: Twilio Programmable SMS
- **Benefits**: Global reach, high reliability, supports WhatsApp API as well.
- **Ease of Use**: User just enters their phone number in OHC profile. OHC handles the backend sending.
- **Pricing**: Pay-per-message (e.g., $0.0079 in US, varies globally).
- **Cloud vs Standalone**: In Cloud mode, OHC uses its master Twilio account (billing users accordingly). In Standalone, users must input their own Twilio Account SID and Auth Token.
**Design Doc**:
- **Trigger**: A critical event occurs (e.g., new order received, booking confirmed).
- **Action**: OHC checks user notification preferences. If SMS is enabled, it queues a job to send a short, localized text message to the owner's phone via Twilio.
- **User Experience**: A simple "Enable SMS Alerts" toggle in settings. The owner receives immediate text messages when important things happen, in their preferred language.
**Implementation Prompt**: Integrate the Twilio SDK to send outbound SMS messages. Add a settings toggle for users to opt-in to SMS alerts and provide their phone number. Create an event listener that triggers an SMS on new orders. Success is defined by receiving a test SMS on a verified phone number when a dummy order is placed.
**Priority**: P0
**Estimated Scope**: Medium

## [Video Conferencing] Auto-generate Meeting Links via Zoom
**Title**: Integrate Zoom API for Auto-generated Video Links
**Problem Statement**: Consultants and tutors manually create Zoom links and email them to clients for upcoming appointments. Sometimes they forget, or send the wrong link, leading to missed meetings and lost revenue.
**Research Report**:
- **Tool Evaluated**: Zoom API
- **Benefits**: Market leader in video conferencing, reliable infrastructure.
- **Ease of Use**: Completely invisible to the business owner once connected. Every new OHC calendar booking automatically includes a unique Zoom link.
- **Pricing**: Basic tier is free (40-min limit); Pro is $15/mo. API access works on both.
- **Cloud vs Standalone**: OAuth app required for Cloud; Standalone requires a Server-to-Server OAuth setup or user-provided credentials.
**Design Doc**:
- **Trigger**: A new meeting or consultation is booked in the OHC Calendar.
- **Action**: OHC calls the Zoom API to create a meeting, retrieves the join URL, and attaches it to the OHC calendar event and confirmation email sent to the client.
- **User Experience**: The user connects Zoom in settings. After that, whenever an appointment is created, a shiny "Join Video Call" button appears for both the owner and the client.
**Implementation Prompt**: Implement Zoom OAuth to allow users to link their accounts. Add a background job that fires upon appointment creation to generate a Zoom meeting link via API and saves the link to the appointment record. Success is verified by a unique Zoom URL appearing on a newly created test appointment.
**Priority**: P1
**Estimated Scope**: Medium
