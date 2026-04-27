# 🔍 Scout: Tool Integration Research

## [Social Media Integration] Unified Inbox with ManyChat
**Problem Statement:**
Business owners like Maya get DMs across Instagram, Facebook, and WhatsApp. Tracking these manually leads to missed messages, lost sales, and poor customer service. They need a single place to see and reply to all messages, and an AI to handle basic questions.

**Research Report:**
*   **Tool Evaluated:** ManyChat
*   **Why:** ManyChat is the industry leader for Instagram/Facebook DM automation and has robust APIs for WhatsApp.
*   **Ease of Use:** High. OHC can abstract away the workflow builder and use ManyChat as a headless messaging router.
*   **Pricing:** $15/mo for Pro (required for some API features), but free tier covers basic Instagram automation which is perfect for OHC's free tier.
*   **Cloud/Standalone:** Cloud-first. Standalone would require the user to bring their own ManyChat API key.
*   **Competitors:** Chatfuel (too complex), Twilio Flex (too enterprise).

**Design Doc:**
*   **Integration:** OHC connects to ManyChat via OAuth.
*   **Workflow:** Incoming DMs are webhooked to OHC's "Customer Success" AI agent. The agent drafts a reply or auto-responds based on business context.
*   **User View:** The business owner sees a unified "Inbox" in the OHC app. They don't know ManyChat exists; it's just plumbing.

**Implementation Prompt:**
Implement a unified Inbox interface in the OHC Flutter app and a Go backend service that receives webhooks from messaging platforms (simulated for now). The UI must show a list of conversations and allow the user to read and reply. The "Customer Success" agent must be able to draft replies automatically.
**Priority:** P0
**Estimated Scope:** Large

---

## [Calendar & Scheduling] Booking Sync with Cal.com
**Problem Statement:**
Service providers like Leo (Music Tutor) and Carlos (Handyman) need a way for customers to book time without double-booking over personal events. Managing multiple calendars manually is error-prone.

**Research Report:**
*   **Tool Evaluated:** Cal.com
*   **Why:** Open-source, API-first alternative to Calendly. Developer-friendly and highly customizable.
*   **Ease of Use:** Extremely high for the end-user if integrated seamlessly.
*   **Pricing:** Free for individuals. OHC could use their Platform API for seamless white-labeling.
*   **Cloud/Standalone:** Perfect for both. Open-source nature means it can be self-hosted in Standalone mode or integrated via API in Cloud mode.
*   **Competitors:** Calendly (less developer friendly, rigid UI), SavvyCal.

**Design Doc:**
*   **Integration:** OHC provisions a Cal.com sub-account for each tenant.
*   **Workflow:** "Operations" agent manages booking types. User connects Google/Apple calendar via OHC.
*   **User View:** A "Bookings" tab in OHC where the user sets their availability hours. The storefront gets a "Book Now" widget that respects this availability.

**Implementation Prompt:**
Create a booking management module where a user can define their available hours (e.g., Mon-Fri 9-5) and connect an external calendar (mocked for this implementation). Create a booking widget for the storefront that displays available slots and allows a customer to select a time.
**Priority:** P0
**Estimated Scope:** Medium

---

## [Email Marketing] Automated Campaigns with Resend
**Problem Statement:**
Boutique owners like Priya need to send beautiful emails to their customer base when new stock arrives, but platforms like Mailchimp are too complex and expensive for simple announcements.

**Research Report:**
*   **Tool Evaluated:** Resend
*   **Why:** Developer-focused, incredibly fast, and creates beautiful emails using React Email (can be adapted/pre-rendered).
*   **Ease of Use:** Invisible to the user. OHC's AI generates the email, Resend delivers it.
*   **Pricing:** Free for up to 3,000 emails/month. Excellent for small businesses.
*   **Cloud/Standalone:** Cloud. Standalone users would need their own API key.
*   **Competitors:** SendGrid (legacy, bad UX), Mailgun.

**Design Doc:**
*   **Integration:** OHC backend uses Resend SDK to send transactional and marketing emails.
*   **Workflow:** "Marketing" agent identifies a segment (e.g., past buyers) and drafts an email. User approves it with one tap.
*   **User View:** A simple "Broadcast" button in the Customers tab. The AI drafts the message, the user taps "Send to 50 customers".

**Implementation Prompt:**
Build an email broadcast feature. In the UI, the user selects an audience (e.g., 'All Customers') and inputs a prompt. The AI generates a subject line and email body. Include a 'Send' button that dispatches the emails via a backend service (mock the actual email sending via a log statement).
**Priority:** P1
**Estimated Scope:** Medium

---

## [Payment Processing] Localized Payments with Mercado Pago
**Problem Statement:**
While Stripe is great globally, small businesses in LATAM need local payment methods (like Pix in Brazil or OXXO in Mexico) with fast settlements and lower fees tailored to their region.

**Research Report:**
*   **Tool Evaluated:** Mercado Pago
*   **Why:** Dominant payment processor in LATAM. High trust, supports all local payment methods out-of-the-box.
*   **Ease of Use:** Familiar to LATAM users.
*   **Pricing:** Varies by country, generally competitive for the region.
*   **Cloud/Standalone:** Both.
*   **Competitors:** dLocal (more enterprise), EBANX.

**Design Doc:**
*   **Integration:** Alternate payment gateway implemented alongside Stripe in the backend.
*   **Workflow:** During OHC onboarding, if the user's country is in LATAM, Mercado Pago is offered as the default or alternative to Stripe.
*   **User View:** A "Connect Mercado Pago" button in settings. Customers see local payment options at checkout.

**Implementation Prompt:**
Extend the existing payment provider interface in the Go backend to support multiple gateways. Implement a dummy `MercadoPagoProvider` that fulfills this interface. Update the checkout UI to display Mercado Pago as a payment option if the tenant is configured for a LATAM region.
**Priority:** P1
**Estimated Scope:** Medium

---

## [Shipping & Logistics] Label Generation with Shippo
**Problem Statement:**
Selling physical products requires shipping. Navigating carrier sites (USPS, UPS, FedEx) manually to buy labels is a huge time sink for creators and boutique owners.

**Research Report:**
*   **Tool Evaluated:** Shippo
*   **Why:** Excellent API for multi-carrier shipping label generation and tracking.
*   **Ease of Use:** Completely abstracted by OHC.
*   **Pricing:** Pay-as-you-go per label, often with discounted carrier rates.
*   **Cloud/Standalone:** Cloud-first.
*   **Competitors:** EasyPost (similar, slightly more complex API), ShipStation (UI-heavy, less API-centric).

**Design Doc:**
*   **Integration:** OHC backend connects to Shippo API.
*   **Workflow:** When an order is paid, OHC calculates shipping. When the user taps "Fulfill", OHC generates and charges for the label via Shippo.
*   **User View:** An "Orders" screen. User taps an order, taps "Buy Label ($4.50)", and a PDF label pops up to print.

**Implementation Prompt:**
Create an order fulfillment flow in the app. When viewing a pending order, the user should see an option to 'Purchase Shipping Label'. Clicking this should call a backend endpoint that calculates a flat rate, generates a mock tracking number, and updates the order status to 'Fulfilled'.
**Priority:** P2
**Estimated Scope:** Large

---

## [SMS & Notifications] Global Messaging with Twilio
**Problem Statement:**
Users like Fatima (Food Cart) need instant, reliable notifications on their phone when an order arrives, regardless of app connectivity. Customers also need SMS updates for order readiness.

**Research Report:**
*   **Tool Evaluated:** Twilio
*   **Why:** The gold standard for global SMS delivery. Extremely reliable.
*   **Ease of Use:** Invisible to the user.
*   **Pricing:** Pay per message. OHC would likely need to pass this cost or bundle it in premium tiers.
*   **Cloud/Standalone:** Cloud. Standalone requires BYO API key.
*   **Competitors:** MessageBird, Plivo.

**Design Doc:**
*   **Integration:** OHC backend uses Twilio SDK.
*   **Workflow:** "Customer Success" agent sends order status updates via SMS. "Operations" agent alerts the owner of new orders.
*   **User View:** A toggle in settings: "Send me an SMS for every new order". Customers receive texts like "Your order from Fatima's Cart is ready for pickup!"

**Implementation Prompt:**
Implement a notification preferences UI where a business owner can toggle 'Receive SMS for new orders'. On the backend, create a notification service that intercepts 'OrderCreated' events and, if the setting is enabled, triggers an SMS sending function (mock the Twilio API call).
**Priority:** P2
**Estimated Scope:** Small

---

## [Video Conferencing] Auto-Meeting Links with Daily.co
**Problem Statement:**
Online tutors like Leo need a video link generated automatically for every booked lesson, without having to manually copy-paste Zoom links into calendar invites.

**Research Report:**
*   **Tool Evaluated:** Daily.co
*   **Why:** Developer-first video APIs. Allows embedding the video call directly into the OHC web app, keeping the user in the ecosystem instead of kicking them out to Zoom.
*   **Ease of Use:** Seamless. The meeting happens inside OHC.
*   **Pricing:** Generous free tier (10,000 participant minutes/mo).
*   **Cloud/Standalone:** Cloud.
*   **Competitors:** Zoom API (clunky, requires external app), Google Meet API (hard to embed).

**Design Doc:**
*   **Integration:** OHC creates a Daily.co room dynamically when a booking is confirmed.
*   **Workflow:** "Operations" agent detects a virtual booking, generates a Daily.co link, and adds it to the calendar invite.
*   **User View:** The tutor and the student just see a "Join Meeting" button in the OHC app that opens a video call directly in the browser.

**Implementation Prompt:**
Build a virtual meeting integration. When a 'Virtual Service' is booked, the backend should generate a unique meeting URL (using a mock Daily.co room generator). Update the booking details UI to display a 'Join Meeting' button that links to this URL.
**Priority:** P3
**Estimated Scope:** Medium
