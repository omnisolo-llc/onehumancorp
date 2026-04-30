# Integration Research Report: Expanding OneHumanCorp (OHC) Capabilities

This report contains research and integration briefs for 7 critical tool categories to expand OHC's capabilities for small business owners.

---

## 1. Social Media Integration

**Title**: [Social Media] Integrate Meta Graph API for Unified Instagram & WhatsApp Inbox

**Problem Statement**:
Business owners like Maya (The Home Baker) receive numerous inquiries via Instagram DMs and WhatsApp. Managing multiple apps to answer simple questions ("Do you do vegan cakes?") is overwhelming and time-consuming, leading to missed sales while they sleep.

**Research Report**:
- **Target Tools**: Meta Graph API (Instagram Messaging, WhatsApp Business API).
- **Evaluation**: Meta's APIs are the industry standard for reaching customers where they are. While the initial OAuth setup can be confusing, OHC can abstract this into a simple "Connect Facebook" button.
- **Ease of Use**: Once connected, completely invisible to the user.
- **Pricing**: WhatsApp Business charges per conversation (first 1,000 service conversations free monthly); Instagram DMs are generally free to receive/reply.
- **Reputation**: Indispensable for retail and food businesses.
- **Environment**: Works seamlessly in Cloud (webhooks). For Standalone, requires polling or a cloud-relay proxy for webhooks.

**Design Doc**:
- **Trigger**: User connects their Instagram/WhatsApp account in the OHC Dashboard under "Customer Success".
- **Action**: Inbound messages are routed to the OHC Unified Inbox. The "Customer Success" AI agent reads the message and either drafts a reply for the owner to approve or auto-replies based on the business's knowledge base.
- **User View**: A single "Messages" tab in the OHC app that shows Instagram, WhatsApp, and Email conversations in one place, alongside AI-suggested replies.

**Implementation Prompt**:
Create a unified inbox view that supports connecting Meta accounts (Instagram and WhatsApp). Implement the OAuth flow so a user can authorize OHC with one click. When a message arrives, it must appear in the unified inbox, and the AI agent must automatically draft a suggested reply based on the user's business profile.
**Acceptance Criteria**: User can connect Instagram/WhatsApp. Inbound messages appear in the OHC inbox. AI drafts a reply. User can send the reply back to the native platform directly from OHC.

**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

**Title**: [Calendar] Integrate Google Calendar for Seamless Booking & Conflict Resolution

**Problem Statement**:
Service providers like Leo (The Music Tutor) and Carlos (The Handyman) struggle with double-bookings. They need a public booking page that automatically syncs with their personal Google Calendar so they don't accidentally schedule a piano lesson during a dentist appointment.

**Research Report**:
- **Target Tools**: Google Calendar API.
- **Evaluation**: Google Calendar is the ubiquitous scheduling tool. Alternative is CalDAV for Apple/iCloud, but Google covers the vast majority of users.
- **Ease of Use**: Standard OAuth "Sign in with Google" flow. Highly familiar.
- **Pricing**: Free API usage within generous quotas.
- **Reputation**: Extremely reliable.
- **Environment**: Works in Cloud and Standalone (using standard OAuth device flows or web redirects).

**Design Doc**:
- **Trigger**: User navigates to "Operations" and clicks "Sync Personal Calendar".
- **Action**: OHC reads "busy" slots from the connected Google Calendar and removes those times from the public OHC booking page. When a customer books a service on OHC, an event is automatically created in the connected Google Calendar.
- **User View**: A simple toggle: "Block out times I'm busy on Google Calendar." No complex mapping required.

**Implementation Prompt**:
Build a Google Calendar integration that allows service-based businesses to sync their availability. The system must automatically read busy blocks from Google and hide those slots on the OHC booking page. New OHC bookings must instantly appear on the user's Google Calendar with customer details.
**Acceptance Criteria**: User connects Google Calendar. Existing Google events block OHC booking slots. New OHC bookings create Google Calendar events.

**Priority**: P0
**Estimated Scope**: Medium

---

## 3. Email Marketing

**Title**: [Email Marketing] Integrate Resend for AI-Driven Customer Campaigns

**Problem Statement**:
Boutique owners like Priya want to let their customers know when new stock arrives, but building email templates in Mailchimp is too complex and expensive. They need a simple way to send beautiful updates to past customers without knowing what "HTML" or "bounce rates" are.

**Research Report**:
- **Target Tools**: Resend (API-first email platform) vs. SendGrid.
- **Evaluation**: Resend offers a modern, developer-friendly API with excellent deliverability and simple React Email templates (which OHC can generate). SendGrid is legacy and complex.
- **Ease of Use**: OHC abstracts Resend completely. The user just types "Tell my customers about the new summer dresses" and OHC does the rest.
- **Pricing**: Resend is very cost-effective ($20/mo for 50k emails), allowing OHC to bundle email marketing into premium tiers easily.
- **Reputation**: High deliverability, modern architecture.
- **Environment**: Cloud-native via API. For Standalone, requires an API key or OHC cloud relay.

**Design Doc**:
- **Trigger**: User goes to "Marketing" and types a prompt: "Send an email to everyone who bought a dress last month about the new summer collection."
- **Action**: The "Marketing" AI generates a beautiful email with product photos from the inventory, selects the correct customer segments, and sends it via the Resend API.
- **User View**: A chat-like interface to create the campaign, a preview of the beautiful email, and a simple "Send" button. Later, a plain-text report: "150 people opened your email, and 3 bought a dress."

**Implementation Prompt**:
Integrate an email sending provider (like Resend) to support AI-generated marketing blasts. The feature must allow the AI to generate a styled email layout containing inventory items, present a preview to the user, and send the email to a filtered list of customers.
**Acceptance Criteria**: User prompts the AI to create an email. AI generates a preview with images. User clicks send. Emails are delivered. System tracks open/click metrics in plain language.

**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing

**Title**: [Payments] Integrate Mercado Pago for LATAM Market Expansion

**Problem Statement**:
While Stripe is excellent, it lacks deep penetration in Latin America. Users in these regions need local payment methods (like PIX in Brazil) to successfully run their businesses, as credit card penetration is lower.

**Research Report**:
- **Target Tools**: Mercado Pago API.
- **Evaluation**: Mercado Pago is the dominant gateway in LATAM, supporting local methods (PIX, Boleto, OXXO).
- **Ease of Use**: Requires merchant account creation, but the checkout experience for the end-customer is native and highly trusted in the region.
- **Pricing**: Varies by country, typically ~3.99% + fixed fee. No monthly costs.
- **Reputation**: The "Stripe of LATAM". Essential for doing business in South/Central America.
- **Environment**: Cloud API. Webhook based status updates.

**Design Doc**:
- **Trigger**: User in a supported LATAM country sets up their store and selects "Mercado Pago" in the "Finance & Payments" settings.
- **Action**: Replaces the default Stripe checkout with a Mercado Pago Checkout Pro redirect or transparent checkout, handling local currency and payment methods automatically.
- **User View**: A simple toggle in payments: "Accept local payments (PIX, Boleto) via Mercado Pago."

**Implementation Prompt**:
Add Mercado Pago as an alternative payment gateway for OHC checkouts. The integration must support generating a checkout link for physical/digital goods and handle the webhook callbacks to mark OHC orders as "Paid".
**Acceptance Criteria**: User can connect a Mercado Pago account. Customers checking out can pay using Mercado Pago. Order status updates automatically upon successful payment.

**Priority**: P2
**Estimated Scope**: Large

---

## 5. Shipping & Logistics

**Title**: [Shipping] Integrate Shippo for Automated Label Generation & Tracking

**Problem Statement**:
Users selling physical goods (like Maya's shipped cookies or Priya's clothes) waste hours copying addresses into post office websites to buy shipping labels. They need a way to hit "Print Label" right from the OHC app.

**Research Report**:
- **Target Tools**: Shippo API vs. EasyPost.
- **Evaluation**: Shippo offers excellent default rates (USPS, UPS, FedEx) without requiring the user to have their own carrier accounts.
- **Ease of Use**: Extremely high. OHC can completely white-label the label generation.
- **Pricing**: Pay-as-you-go ($0.05 per label) plus postage costs. OHC can pass postage costs directly to the user's card.
- **Reputation**: Reliable API, good international support.
- **Environment**: Cloud API.

**Design Doc**:
- **Trigger**: User views an unfulfilled order in "Operations" and taps "Buy Shipping Label".
- **Action**: OHC calculates the box size/weight based on inventory data, gets a rate from Shippo, charges the user's saved card for the postage, and returns a PDF of the label.
- **User View**: A single "Print Label for $4.50" button on the order screen. Tapping it generates a printable PDF. Customers automatically get an email with the tracking link.

**Implementation Prompt**:
Integrate a shipping API (Shippo) to allow users to generate and print shipping labels directly from the order details screen. The flow must estimate package weight, fetch the cheapest rate, purchase the label, and automatically email the tracking number to the end customer.
**Acceptance Criteria**: User sees a "Buy Label" button on a paid order. User confirms weight/dimensions. System generates a PDF label. Order status changes to "Shipped" with a tracking number.

**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications

**Title**: [SMS] Integrate Twilio for Critical Notifications & Offline Alerts

**Problem Statement**:
Users like Fatima (Food Cart Operator) are busy cooking and might miss an email or app push notification when a new pre-order comes in. They need an unavoidable SMS text message to alert them of new paid orders immediately.

**Research Report**:
- **Target Tools**: Twilio Programmable SMS.
- **Evaluation**: Industry standard, massive global reach.
- **Ease of Use**: Completely invisible to the business owner.
- **Pricing**: ~$0.0079 per SMS in the US. OHC can absorb this for premium tiers or limit free tier usage.
- **Reputation**: Gold standard for telecom APIs.
- **Environment**: Cloud API.

**Design Doc**:
- **Trigger**: A new order is placed and paid for by a customer.
- **Action**: The "Operations" agent checks the user's notification preferences. If SMS is enabled, it sends a brief text via Twilio.
- **User View**: A toggle in settings: "Text my phone when I get a new order." User receives a text: "OHC: New order from John D. for 2x Falafel Platter ($24). Tap here to view: [link]"

**Implementation Prompt**:
Add an SMS notification system utilizing Twilio to alert business owners of critical events like new orders or booking cancellations. This should be an opt-in setting in the user profile.
**Acceptance Criteria**: User toggles "SMS Notifications" on. Customer places an order. User receives an SMS on their verified phone number within 10 seconds containing order details.

**Priority**: P1
**Estimated Scope**: Small

---

## 7. Video Conferencing

**Title**: [Video] Integrate Zoom for Automated Online Lesson Links

**Problem Statement**:
Online tutors like Leo waste time manually creating Zoom links for every booked lesson and emailing them to students. This process is prone to errors and looks unprofessional.

**Research Report**:
- **Target Tools**: Zoom API vs. Google Meet API.
- **Evaluation**: Zoom is widely preferred for online tutoring/consulting due to screen sharing and recording features.
- **Ease of Use**: Requires a one-time OAuth connection to Zoom.
- **Pricing**: Free for the API, but the user needs their own Zoom account (free tier has 40-min limits).
- **Reputation**: Ubiquitous for remote work and lessons.
- **Environment**: Cloud API.

**Design Doc**:
- **Trigger**: A customer books an online service (e.g., "1hr Guitar Lesson").
- **Action**: The "Operations" agent calls the Zoom API to generate a unique meeting link for that specific date/time, and attaches it to the calendar invite and confirmation email.
- **User View**: User connects their Zoom account once. After that, they just check their OHC calendar, click the appointment, and see a "Join Zoom" button. The customer gets the same button in their email.

**Implementation Prompt**:
Integrate the Zoom API to auto-generate meeting links for services marked as "Online Meeting". The system must authenticate the user's Zoom account, create a meeting upon a successful booking, and distribute the join link to both the business owner and the customer.
**Acceptance Criteria**: User connects Zoom. Customer books an online service. Both parties receive a unique Zoom link. Meeting appears in the user's Zoom dashboard.

**Priority**: P2
**Estimated Scope**: Medium
