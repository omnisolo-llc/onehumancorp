# Core Tool Integrations Research Report Q4

## 1. Social Media Integration

### Title
Unified Social Media Inbox Integration

### Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Switching between apps causes delayed responses, lost sales, and poor customer service. They need a single place to view and reply to all messages without needing to switch context or manage multiple apps.

### Research Report
- **Tool Candidates**: Meta Business Suite API, WhatsApp Business API.
- **Evaluation**: Connecting Meta platforms natively into OHC allows for centralising the communication for the merchant.
- **Ease of Use**: Merchants connect their accounts via OAuth. Non-technical users benefit as all interactions remain inside OHC.
- **Pricing**: Meta's standard APIs are generally free, WhatsApp conversation charges apply.
- **Compatibility**: Works well via standard OAuth flows for Cloud; standalone may require a proxy due to callback URLs.

### Design Doc
- **Integration Trigger**: Users connect accounts from Settings.
- **Action**: Webhooks feed into an internal OHC Inbox.
- **User Interface**: A unified chronological timeline.

### Implementation Prompt
Implement a unified inbox feature where users can connect their Meta platforms. Incoming messages appear in a single feed. The user types a reply and it is routed back via API.
- **Acceptance Criteria**: Connect accounts, receive a message, send reply.
- **Priority**: P1
- **Estimated Scope**: Large


## 2. Calendar & Scheduling

### Title
Native Google Calendar Sync

### Problem Statement
Service providers (like Leo the music tutor) manually manage appointments between their personal calendar and OHC, causing double bookings. They need automatic two-way sync so OHC knows when they are busy.

### Research Report
- **Tool Candidates**: Google Calendar API, Outlook Calendar API.
- **Evaluation**: Google Calendar is the most pervasive tool.
- **Ease of Use**: Standard Google OAuth connection makes it simple for users.
- **Pricing**: Free tier API limits are sufficient for standard use.
- **Compatibility**: Standard OAuth for Cloud; Standalone needs local auth handling.

### Design Doc
- **Integration Trigger**: User authenticates with Google from their profile settings.
- **Action**: OHC queries free/busy times and blocks slots natively.
- **User Interface**: OHC booking widget uses aggregated availability.

### Implementation Prompt
Create a native integration with Google Calendar API. Fetch free/busy times to update OHC booking slots, and push new OHC bookings into the Google Calendar.
- **Acceptance Criteria**: Connect Google account. Availability reflects calendar. Bookings push to calendar.
- **Priority**: P1
- **Estimated Scope**: Medium


## 3. Email Marketing

### Title
Native Email Campaign Manager

### Problem Statement
Priya wants to email her customers about new stock but finds tools like Mailchimp too complex. She needs an automated, simple way to email customers right from her sales dashboard.

### Research Report
- **Tool Candidates**: SendGrid, AWS SES.
- **Evaluation**: Using a transactional email API allows OHC to wrap the feature entirely in its own UI.
- **Ease of Use**: Completely native. The user doesn't need to learn a new tool.
- **Pricing**: Built into OHC subscription (scale-based).
- **Compatibility**: Centralized API for Cloud.

### Design Doc
- **Integration Trigger**: User creates a campaign via OHC UI.
- **Action**: OHC composes and sends via backend API.
- **User Interface**: Simple editor and metrics dashboard inside OHC.

### Implementation Prompt
Build a native email campaign management system utilizing SendGrid/SES for delivery.
- **Acceptance Criteria**: Create a campaign. AI can help generate content. Delivery succeeds. Open rates track.
- **Priority**: P1
- **Estimated Scope**: Large


## 4. Payment Processing

### Title
Native Integration of Local Payment Methods (Mercado Pago)

### Problem Statement
Small business owners outside the US/EU need trusted local payment processors to accept common local methods seamlessly.

### Research Report
- **Tool Candidates**: Mercado Pago (LATAM), Paytm (India).
- **Evaluation**: Direct API integration needed to maintain the "one platform" promise.
- **Ease of Use**: Familiar for regional merchants.
- **Pricing**: Standard payment gateway transaction fees.
- **Compatibility**: Standard API keys/OAuth.

### Design Doc
- **Integration Trigger**: Merchant adds Mercado Pago in regional settings.
- **Action**: Checkout dynamically routes payments. Webhooks update order status.
- **User Interface**: "Pay with Mercado Pago" on checkout.

### Implementation Prompt
Integrate Mercado Pago. The checkout must dynamically offer it based on merchant region, and webhooks must process order status.
- **Acceptance Criteria**: Connect Mercado Pago. Customer completes checkout. Order updates to paid via webhook.
- **Priority**: P1
- **Estimated Scope**: Large


## 5. Shipping & Logistics

### Title
Native Shipping Rate Calculation and Label Generation

### Problem Statement
Sellers waste time manually copying addresses to carrier websites. They need to generate and print labels directly from their order screen.

### Research Report
- **Tool Candidates**: Shippo, EasyPost.
- **Evaluation**: Shippo offers good API abstraction over multiple carriers.
- **Ease of Use**: High value; 1-click label purchasing.
- **Pricing**: Per-label fee + postage.
- **Compatibility**: Cloud (OAuth/Keys), Standalone (Keys).

### Design Doc
- **Integration Trigger**: Order received.
- **Action**: Query rates, purchase label, generate PDF.
- **User Interface**: "Buy Label & Print" button on order details.

### Implementation Prompt
Implement a fulfillment module powered by Shippo for live rates and label generation.
- **Acceptance Criteria**: See rates at checkout. Print PDF label. Tracking info sent to customer.
- **Priority**: P1
- **Estimated Scope**: Large


## 6. SMS & Notifications

### Title
Native SMS Order Notifications

### Problem Statement
Busy operators miss app notifications. They need reliable SMS alerts for new orders and appointments.

### Research Report
- **Tool Candidates**: Twilio.
- **Evaluation**: Twilio is the gold standard for programmatic SMS.
- **Ease of Use**: Invisible to user once toggled on.
- **Pricing**: Pay-per-message.
- **Compatibility**: Cloud centralized account; standalone requires user key.

### Design Doc
- **Integration Trigger**: Order placed / Booking made.
- **Action**: Async job dispatches SMS.
- **User Interface**: Toggle switches in Settings for SMS alerts.

### Implementation Prompt
Integrate Twilio SMS for order and appointment confirmations. Handle global formatting.
- **Acceptance Criteria**: SMS sent on order ready. SMS sent for appointment reminder.
- **Priority**: P2
- **Estimated Scope**: Medium


## 7. Video Conferencing

### Title
Native Zoom Link Generation

### Problem Statement
Tutors manually create meeting links and email them. They need this automated when a booking happens.

### Research Report
- **Tool Candidates**: Zoom API.
- **Evaluation**: Essential for online services.
- **Ease of Use**: Standard OAuth setup.
- **Pricing**: Requires merchant to have Zoom account.
- **Compatibility**: Standard OAuth flows.

### Design Doc
- **Integration Trigger**: Service booked as "Online Meeting".
- **Action**: OHC creates meeting via Zoom API and saves URL.
- **User Interface**: Link embedded in bookings and emails.

### Implementation Prompt
Build Zoom integration that dynamically creates meeting links for online bookings.
- **Acceptance Criteria**: Connect Zoom. Book online service. Link generated and emailed.
- **Priority**: P2
- **Estimated Scope**: Medium
