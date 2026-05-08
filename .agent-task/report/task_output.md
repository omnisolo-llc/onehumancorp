# 🔍 Scout: Tool Integration Research Q3

## 1. Social Media Integration

### Title
Integrate Meta Graph API for Unified Native Social Media Inbox

### Problem Statement
Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.

### Research Report
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).

### Design Doc
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".

### Implementation Prompt
Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
- **Acceptance Criteria**: User can connect Instagram/Facebook. Incoming messages appear in OHC unified inbox. User can reply from OHC, and it shows up on the customer's social app.
- **Priority**: P0
- **Estimated Scope**: Large

---
## 2. Calendar & Scheduling

### Title
Integrate Cal.com for Zero-Config Booking & Calendar Sync

### Problem Statement
Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs with their personal Google Calendar seamlessly.

### Research Report
- **Strategy**: Direct Cal.com integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box. It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes. Free tier available for individuals; great for our free tier users. Alternative is building from scratch, which is error-prone.
- **Risks**: Cal.com updates or breaking API changes.
- **Pricing**: Free for individuals.
- **Compatibility**: Cloud (OAuth). Standalone (Self-hosted/OAuth).

### Design Doc
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect their Google/Outlook calendar via a one-click OAuth button in the "Operations" tab.
- When a customer books a slot on the OHC public page, Cal.com manages the calendar event and conflict resolution transparently.

### Implementation Prompt
Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
- **Priority**: P0
- **Estimated Scope**: Medium

---

## 3. Email Marketing

### Title
Integrate Mailchimp for Customer Re-engagement

### Problem Statement
Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns. She needs an automated way to email customers without leaving the OHC app.

### Research Report
- **Tool**: Mailchimp
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Market leader, great API, supports tags and segments. High deliverability.
- **Risks**: Strict anti-spam policies might suspend users if they import bad lists.
- **Pricing**: Free tier available (up to 500 contacts). Essentials starts at $13/mo.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
- When a customer buys something, they are automatically added to the Mailchimp audience with tags (e.g., "Bought: Cake").
- The Marketing agent suggests campaigns ("Send an email to past customers about your new holiday cakes").
- The user approves the AI-generated email, and OHC triggers Mailchimp to send it.
- The user sees open rates and clicks in the OHC Marketing dashboard.

### Implementation Prompt
Build an integration that syncs OHC customers to a Mailchimp audience automatically after purchase. Allow the AI Marketing agent to create and send email campaigns via the Mailchimp API.
- **Priority**: P1
- **Estimated Scope**: Medium

---
## 4. Payment Processing

### Title
Integrate Mercado Pago for LATAM Payments

### Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil.

### Research Report
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico). Good developer docs.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- Webhooks update the order status in OHC when payment succeeds.

### Implementation Prompt
Add Mercado Pago as a secondary payment provider. Implement the checkout flow to redirect to Mercado Pago and handle the success/failure webhooks to update order status.
- **Priority**: P2
- **Estimated Scope**: Large

---

## 5. Shipping & Logistics

### Title
Integrate Shippo for Automated Label Generation

### Problem Statement
Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label.

### Research Report
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
- When an order is placed, OHC sends the dimensions/weight to Shippo to get rates.
- The Operations agent shows the cheapest shipping option.
- The user clicks "Buy Label", and OHC downloads the PDF label for printing.
- OHC automatically emails the customer the tracking number.

### Implementation Prompt
Connect the Shippo API to fetch shipping rates based on order weight/dimensions. Allow the user to purchase a label and automatically email the tracking link to the customer.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 6. SMS & Notifications

### Title
Integrate Twilio for SMS Order Notifications

### Problem Statement
Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable SMS alerts when a new pre-order arrives so she can start cooking.

### Research Report
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Global coverage, incredibly reliable. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

### Design Doc
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, the Operations agent triggers a Twilio API call to send an SMS: "New order! 2x Falafel for John. Pickup in 15m."
- (Future: Customers can also receive SMS receipts).

### Implementation Prompt
Integrate the Twilio SDK to send outbound SMS notifications. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure compliance with local messaging regulations.
- **Priority**: P2
- **Estimated Scope**: Medium

---
## 7. Video Conferencing

### Title
Integrate Zoom for Auto-Generated Meeting Links

### Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically when a lesson is booked.

### Research Report
- **Tool**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Strong API for meeting creation.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: Free tier (40-min limit). Pro starts at $15/mo.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

### Design Doc
- User connects their Zoom account via the Sales dashboard.
- When a customer books an online service (e.g., via Calendly or native booking), OHC calls the Zoom API to create a meeting.
- The Zoom link is embedded in the automated calendar invite and confirmation email sent to the customer.

### Implementation Prompt
Create an OAuth integration with Zoom. Automatically generate a unique Zoom meeting link when a customer books a virtual service, and include this link in the customer's confirmation email.
- **Priority**: P1
- **Estimated Scope**: Medium
