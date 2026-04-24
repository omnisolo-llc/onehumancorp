# OHC Scout: Tool Integration Research [Q2 2024]

## 1. Social Media Integration
**Title:** Implement Chatwoot for Unified Social Inbox
**Problem Statement:** Small business owners (like Priya and Maya) are overwhelmed juggling Instagram DMs, WhatsApp messages, and Facebook comments. Missing a DM often means losing a sale. They need a single, simple inbox that aggregates all customer inquiries across platforms, allowing AI to draft responses automatically.
**Research Report:**
- **Overview:** Chatwoot is an open-source customer engagement platform designed to unify conversations from multiple channels.
- **Competitors:** Intercom, Zendesk, Front (often too expensive or complex for micro-businesses).
- **Persona Fit:** Excellent for non-technical users if pre-configured. Provides a single pane of glass.
- **Pricing:** Generous free tier (hacker plan) and affordable standard plans starting around $19/mo per agent. Open-source version available for self-hosting.
- **Environment:** Cloud (SaaS offering) and Standalone (can be self-hosted via Docker).
- **Risks:** OAuth setup for Meta platforms (WhatsApp/Instagram) can be confusing for non-technical users; requires a seamless OHC wizard to abstract this.
**Design Doc:**
- **Trigger:** User connects social accounts via the OHC "Customer Success" dashboard.
- **Action:** OHC bridges Chatwoot APIs to sync messages bi-directionally. The "Customer Success" AI agent listens to incoming messages and suggests drafts.
- **User View:** A unified "Inbox" tab in the OHC mobile and web apps showing all conversations, badged by source.
**Implementation Prompt:**
Implement a unified inbox feature that allows users to connect Instagram and WhatsApp. Display messages in a single feed. When a new message arrives, auto-generate a suggested reply using the "Customer Success" AI agent. Ensure the connection process is a simple 2-click OAuth flow.
**Priority:** P0
**Estimated Scope:** Large

## 2. Calendar & Scheduling
**Title:** Integrate Cal.com for Seamless Booking
**Problem Statement:** Service providers (like Carlos and Leo) waste time on back-and-forth emails trying to schedule appointments. They need a public booking link that syncs with their personal calendar to prevent double-booking and automates meeting links or location details.
**Research Report:**
- **Overview:** Cal.com is an open-source scheduling infrastructure platform.
- **Competitors:** Calendly, Acuity Scheduling.
- **Persona Fit:** Ideal for independent professionals. Open-source nature aligns with OHC's potential for self-hosted/standalone modes.
- **Pricing:** Free for individuals; affordable team plans.
- **Environment:** Cloud (managed API) and Standalone (self-hostable).
- **Risks:** Managing timezone conversions for remote bookings (e.g., Leo's online tutoring) requires careful handling.
**Design Doc:**
- **Trigger:** User sets availability in the OHC "Operations" dashboard.
- **Action:** OHC creates a customized Cal.com instance/link under the hood, embedding the booking widget on the user's public OHC site.
- **User View:** A calendar interface where the user defines working hours. A public-facing booking widget on their site.
**Implementation Prompt:**
Create a scheduling widget powered by Cal.com that embeds on the user's public storefront. The backend should handle creating the Cal.com user and syncing their Google/Apple calendar. Include automated email reminders 24 hours before the booked time.
**Priority:** P0
**Estimated Scope:** Medium

## 3. Email Marketing
**Title:** Integrate MailerLite for Automated Campaigns
**Problem Statement:** Store owners (like Priya) need to easily notify past customers about new stock or promotions without learning complex email builder software. They need simple, beautiful templates that send automatically.
**Research Report:**
- **Overview:** MailerLite is an email marketing tool known for its simplicity and clean interface.
- **Competitors:** Mailchimp, Klaviyo (too complex/expensive for many micro-SMBs).
- **Persona Fit:** Perfect for semi-technical and non-technical users due to its intuitive drag-and-drop editor and clear analytics.
- **Pricing:** Free tier up to 1,000 subscribers; very affordable paid plans.
- **Environment:** Cloud (SaaS API).
- **Risks:** Deliverability issues if the platform's IP reputation is affected; ensuring users comply with spam laws.
**Design Doc:**
- **Trigger:** "Marketing & Advertising" agent detects new inventory or a seasonal event.
- **Action:** AI drafts an email campaign, pushes it to MailerLite via API, and presents the draft to the user for one-click approval.
- **User View:** A "Promotions" tab where users see suggested email campaigns. One click to "Approve & Send."
**Implementation Prompt:**
Build an integration that automatically drafts an email newsletter when a user adds a new product or runs a sale. Use the MailerLite API to manage the subscriber list and send the email. Provide a simple UI for the user to review the AI-generated subject line and content before sending.
**Priority:** P1
**Estimated Scope:** Medium

## 4. Payment Processing
**Title:** Integrate Mercado Pago for LATAM Markets
**Problem Statement:** While Stripe is excellent, it is not universally available or preferred in all regions. Small businesses in Latin America need a trusted, local payment method with high conversion rates.
**Research Report:**
- **Overview:** Mercado Pago is the leading digital wallet and payment processor in LATAM.
- **Competitors:** Dlocal, Ebanx.
- **Persona Fit:** Essential for international users operating in supported LATAM countries. Familiar and trusted by local consumers.
- **Pricing:** Varies by country, typically percentage + fixed fee per transaction.
- **Environment:** Cloud (API).
- **Risks:** Complex regulatory environment per country; managing currency conversions and local payout schedules.
**Design Doc:**
- **Trigger:** User selects their country during onboarding.
- **Action:** If in a supported LATAM country, OHC suggests Mercado Pago as the primary payment gateway.
- **User View:** A seamless checkout experience for their customers, offering local payment methods (e.g., PIX in Brazil).
**Implementation Prompt:**
Add Mercado Pago as a payment gateway option alongside Stripe. Implement the checkout flow to support local payment methods (like PIX or OXXO) depending on the buyer's region. Ensure the backend correctly processes webhooks for async payment confirmations.
**Priority:** P2
**Estimated Scope:** Large

## 5. Shipping & Logistics
**Title:** Integrate Shippo for Real-Time Shipping & Labels
**Problem Statement:** Product sellers (like Maya and Priya) struggle with calculating accurate shipping costs and manually printing labels. They need an automated way to charge the right shipping fee and generate a printable label as soon as an order is placed.
**Research Report:**
- **Overview:** Shippo is a multi-carrier shipping API that provides rates, labels, and tracking.
- **Competitors:** EasyPost, ShipStation.
- **Persona Fit:** Very high. Simplifies a highly complex operational task for non-technical sellers.
- **Pricing:** Pay-as-you-go (per label) or affordable monthly plans. Often provides discounted carrier rates.
- **Environment:** Cloud (API).
- **Risks:** Ensuring physical package dimensions and weights are accurately captured from the user to prevent carrier adjustments/fees.
**Design Doc:**
- **Trigger:** Customer reaches the checkout page; order is finalized.
- **Action:** OHC queries Shippo for real-time rates at checkout. Upon order confirmation, OHC generates the label via Shippo API.
- **User View:** A "Print Label" button appears next to new orders in the Operations dashboard.
**Implementation Prompt:**
Integrate the Shippo API to provide real-time shipping rate calculation at checkout based on product weight. Add a feature in the order management view that allows the user to generate and download a printable shipping label in one click.
**Priority:** P1
**Estimated Scope:** Medium

## 6. SMS & Notifications
**Title:** Integrate Twilio for Reliable SMS Alerts
**Problem Statement:** Users in low-connectivity areas or those managing time-sensitive businesses (like Fatima's food cart) rely heavily on SMS over email. They need instant text notifications for new orders to ensure fast fulfillment.
**Research Report:**
- **Overview:** Twilio is a leading cloud communications platform providing robust SMS APIs globally.
- **Competitors:** MessageBird, Plivo.
- **Persona Fit:** Critical for users who don't monitor email constantly or have limited data access.
- **Pricing:** Pay-as-you-go per message (very cheap, fractions of a cent in many regions).
- **Environment:** Cloud (API).
- **Risks:** Managing opt-outs (STOP messages) to remain compliant with telecom regulations (e.g., A2P 10DLC in the US).
**Design Doc:**
- **Trigger:** A new order or booking is placed.
- **Action:** OHC sends a summarized SMS via Twilio to the business owner's registered phone number.
- **User View:** The owner receives a text: "New Order #102: 2x Halal Platter. Pickup at 12:30."
**Implementation Prompt:**
Implement an SMS notification service using Twilio. When a new order is received, send a concise text message to the business owner. Include settings in the dashboard for the owner to toggle SMS notifications on or off and set "do not disturb" hours.
**Priority:** P0
**Estimated Scope:** Small

## 7. Video Conferencing
**Title:** Integrate Zoom API for Auto-Generated Meeting Links
**Problem Statement:** Online service providers (like Leo) currently have to manually create Zoom meetings and email the links to clients after a booking is made. This manual step is prone to errors and looks unprofessional.
**Research Report:**
- **Overview:** Zoom offers a comprehensive API for managing meetings and users.
- **Competitors:** Google Meet (via Google Workspace integration), Microsoft Teams.
- **Persona Fit:** Very high for tutors, consultants, and online instructors. Zoom is widely recognized and trusted.
- **Pricing:** Free tier available; API access usually requires a Pro plan or higher (OAuth app integration is free to build).
- **Environment:** Cloud (API).
- **Risks:** OAuth connection maintenance; handling meeting cancellations and rescheduling robustly.
**Design Doc:**
- **Trigger:** A customer books an online service via the OHC scheduling widget.
- **Action:** OHC creates a Zoom meeting via API and attaches the join URL to the calendar invite and confirmation email.
- **User View:** The business owner connects their Zoom account once. Future online bookings automatically include a unique Zoom link.
**Implementation Prompt:**
Create an integration with Zoom via OAuth. When an appointment is booked for a service marked as "Online Video," automatically generate a Zoom meeting link and embed it in the confirmation emails sent to both the customer and the business owner.
**Priority:** P1
**Estimated Scope:** Medium
