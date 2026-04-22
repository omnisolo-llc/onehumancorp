# Scout: Tool Integration Research [Q2]

## Overview
This research report evaluates tool integrations across seven core categories. The objective is to identify solutions that empower non-technical small business owners using OneHumanCorp (OHC). All integrations must align with OHC's mobile-first, "Zero Technical Knowledge" ethos.

---

## 1. Social Media Integration: ManyChat
**Title**: Unified Social Inbox via ManyChat Integration
**Problem Statement**: Maya the Home Baker receives custom cake requests across Instagram DMs, Facebook Messenger, and WhatsApp. She frequently loses track of orders because she has to constantly switch between apps, leading to lost revenue and slow response times.
**Research Report**: ManyChat is a leading chat marketing platform. It excels in aggregating messages across Meta properties (IG, FB, WhatsApp) and offers robust webhook/API capabilities.
*   **Ease of Use**: Very high for non-technical users once the initial OAuth connection is made.
*   **Pricing**: Pro plan starts at ~$15/month. OHC could potentially negotiate a white-label or partner API rate.
*   **Reputation**: Highly reliable, Meta Business Partner.
*   **Cloud/Standalone**: Primarily Cloud-based. Standalone integration might require direct Meta API connections instead if a third-party service like ManyChat isn't desired for local deployments, but ManyChat handles the complex webhook management well.
**Design Doc**:
*   User connects their social accounts via a simple OAuth flow in the OHC "Sales & Acquisition" settings.
*   ManyChat webhooks push incoming DMs to the OHC backend.
*   The Customer Success AI Agent reads the incoming messages via the unified inbox UI and drafts responses.
*   User approves/edits responses in the OHC mobile app, which sends them back via the ManyChat API.
**Implementation Prompt**: Implement an integration that allows a user to connect their Instagram and Facebook accounts. Once connected, incoming DMs should appear in a single "Inbox" screen in the OHC app. The user should be able to type a reply in the OHC app, and it should successfully deliver back to the customer on Instagram/Facebook.
**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling: Cronofy
**Title**: Native Calendar Sync via Cronofy
**Problem Statement**: Leo the Music Tutor needs to offer online booking, but if a student books a time when he has a personal doctor's appointment, he gets double-booked. He needs his OHC booking page to automatically read his Google or Outlook calendar and block out busy times.
**Research Report**: Cronofy provides a unified API for all major calendar providers (Google, Apple, Outlook, Exchange).
*   **Ease of Use**: User simply clicks "Connect Google Calendar" and approves the OAuth prompt.
*   **Pricing**: Starts around $199/month for the API, scaling with connected accounts. This is an infrastructure cost for OHC, abstracted from the user.
*   **Reputation**: Enterprise-grade reliability, used by major scheduling platforms.
*   **Cloud/Standalone**: Cloud-native API.
**Design Doc**:
*   OHC implements Cronofy's UI components or API for the OAuth flow.
*   OHC Operations Agent queries Cronofy for "free/busy" times when generating available slots for the user's booking page.
*   When a booking is confirmed, OHC creates an event in the user's primary calendar via Cronofy.
**Implementation Prompt**: Integrate a calendar connection flow where a user can authenticate their Google or Microsoft calendar. The OHC booking page must then accurately reflect the user's real-time availability by blocking out times that are marked "busy" on their personal calendar. New bookings made through OHC must automatically appear on their personal calendar.
**Priority**: P0
**Estimated Scope**: Medium

---

## 3. Email Marketing: Resend
**Title**: Automated Email Campaigns via Resend
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when a new clothing line arrives. She doesn't want to learn how to use Mailchimp or design complex HTML templates; she just wants to say "Tell my customers about the summer sale."
**Research Report**: Resend is a developer-first email API built for modern apps, but we can use it to power a simplified user-facing feature.
*   **Ease of Use**: Fully abstracted. The user never sees Resend. They only interact with the OHC Marketing Agent.
*   **Pricing**: Very generous free tier (3,000 emails/month), then $20/mo for 50k emails. Excellent unit economics for OHC.
*   **Reputation**: High deliverability, modern API, fast-growing.
*   **Cloud/Standalone**: Cloud API.
**Design Doc**:
*   OHC stores customer emails in the tenant's PostgreSQL database.
*   The user tells the Marketing Agent: "Send an email about the new summer dress."
*   The Agent generates a plain-text or beautifully simple HTML template (using OHC design tokens).
*   OHC backend sends the batch via the Resend API, tracking opens/clicks via webhooks.
**Implementation Prompt**: Create a feature where a user can select a segment of their customers (e.g., "All Past Purchasers") and type a plain-text message. The system must reliably deliver this message to all selected customers via email, and the OHC dashboard should display a simple count of how many emails were sent and opened.
**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing: Mercado Pago (LATAM Focus)
**Title**: Alternative Payments: Mercado Pago Integration
**Problem Statement**: Carlos the Handyman operates in Latin America where Stripe is not the dominant player and many customers prefer local payment methods or installments (cuotas) offered by Mercado Pago.
**Research Report**: Mercado Pago is the leading payment gateway in LATAM, supporting local cards, bank transfers, and cash payments.
*   **Ease of Use**: Standard OAuth integration for the merchant. Familiar checkout flow for the end customer.
*   **Pricing**: Standard payment gateway fees (varies by country, usually ~3-4%).
*   **Reputation**: The standard for e-commerce in LATAM.
*   **Cloud/Standalone**: Cloud API.
**Design Doc**:
*   Add Mercado Pago as a payment provider option in the Finance & Payments settings, alongside Stripe.
*   Implement Mercado Pago's Checkout Pro (redirect) or Checkout API (native UI) for the OHC storefront checkout flow.
*   Handle Mercado Pago webhooks for asynchronous payment confirmations (e.g., if the customer pays via cash at a local store).
**Implementation Prompt**: Integrate Mercado Pago as a secondary payment option. A user in a supported region should be able to connect their Mercado Pago account. When their customers check out on the OHC storefront, they should be able to successfully complete a transaction using Mercado Pago, and the OHC system must correctly record the payment status as "Paid".
**Priority**: P2
**Estimated Scope**: Large

---

## 5. Shipping & Logistics: Shippo
**Title**: Simplified Shipping Rates & Labels via Shippo
**Problem Statement**: Maya the Home Baker wants to start shipping her cookies nationwide. She doesn't know how to calculate USPS rates or print labels and is afraid of undercharging for shipping and losing money.
**Research Report**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into a single API.
*   **Ease of Use**: High. Shippo abstracts carrier accounts. OHC can further abstract this so the user just enters package weight.
*   **Pricing**: Pay-as-you-go ($0.05 per label) or monthly plans. Very accessible for small businesses.
*   **Reputation**: Reliable, widely used by major e-commerce platforms.
*   **Cloud/Standalone**: Cloud API.
**Design Doc**:
*   User enters package dimensions/weight for their physical products in OHC.
*   During checkout, OHC pings Shippo API to get real-time shipping rates to display to the buyer.
*   After the order is placed, the Operations Agent presents a "Print Label" button in the OHC mobile app, which fetches the PDF from Shippo.
**Implementation Prompt**: Implement a shipping integration where physical products can have a weight assigned. During the storefront checkout, the system must calculate and display accurate shipping costs based on the customer's address. After purchase, the business owner must be able to tap a single button on their mobile app to generate and download a printable shipping label.
**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications: Twilio
**Title**: Reliable SMS Notifications via Twilio
**Problem Statement**: Fatima the Food Cart Operator doesn't always have a reliable internet connection for push notifications, but she needs to know immediately when a customer places a pre-order for pickup.
**Research Report**: Twilio is the industry standard for programmatic SMS.
*   **Ease of Use**: Fully abstracted from the user. OHC handles the Twilio account.
*   **Pricing**: ~$0.0079 per message. OHC would likely need to bundle this into a paid tier or charge usage-based fees.
*   **Reputation**: Extremely reliable, global coverage.
*   **Cloud/Standalone**: Cloud API. Standalone might require a user to bring their own Twilio API key.
**Design Doc**:
*   OHC configures a central Twilio account.
*   User enables "SMS Order Alerts" in their Operations settings and verifies their phone number.
*   When the payment webhook fires for a new order, the OHC backend enqueues an SMS job.
*   Twilio sends a concise text: "New OHC Order: $24.50 - Chicken Over Rice. Pickup in 15m."
**Implementation Prompt**: Create a notification preference that allows a business owner to enter their phone number and receive instant SMS text messages whenever a new order is paid for. The SMS must contain the order total, key items, and the customer's expected pickup/delivery time.
**Priority**: P1
**Estimated Scope**: Small

---

## 7. Video Conferencing: Zoom
**Title**: Auto-Generated Meeting Links via Zoom
**Problem Statement**: Leo the Music Tutor offers online guitar lessons. Currently, he has to manually create a Zoom link for every booking and email it to the student, which is tedious and prone to errors.
**Research Report**: Zoom provides a robust API for meeting creation, widely used in scheduling apps.
*   **Ease of Use**: Standard OAuth connection. After that, it's automatic.
*   **Pricing**: Free for the API integration itself, but the end-user (Leo) needs a Zoom account.
*   **Reputation**: The default standard for video calls.
*   **Cloud/Standalone**: Cloud API.
**Design Doc**:
*   User connects Zoom via OAuth in the Sales & Acquisition settings.
*   When configuring a service (e.g., "Online Guitar Lesson"), the user toggles "Location: Video Call (Zoom)".
*   When a customer books the service, the OHC backend calls the Zoom API to create a unique meeting URL.
*   The meeting URL is saved to the OHC database, added to the calendar event, and included in the confirmation email to the customer.
**Implementation Prompt**: Build an integration that allows a user to connect their Zoom account. When creating a service listing, they can specify it as an online meeting. Upon a successful customer booking, the system must automatically generate a unique Zoom meeting link and display it on the customer's receipt and in the business owner's upcoming appointments dashboard.
**Priority**: P2
**Estimated Scope**: Medium