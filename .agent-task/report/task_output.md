# Research Report: OHC External Tool Integrations

## 1. Social Media Integration: Chatwoot API

**Title**: Unified Inbox Integration via Chatwoot
**Problem Statement**: Small business owners like Priya (boutique owner) receive customer messages across Instagram DMs, Facebook Messenger, WhatsApp, and their website. Replying to each platform separately is exhausting and time-consuming. They need a single, simple inbox that consolidates all customer conversations.
**Research Report**:
- **Tool Evaluated**: Chatwoot
- **Why it fits**: Chatwoot provides an open-source, multi-channel customer engagement platform. It natively aggregates Facebook, Instagram, WhatsApp, Twitter, and website chat.
- **Ease of Use**: Once connected via OHC, the business owner simply sees a "Messages" tab. No need to understand omnichannel routing.
- **Pricing**: Chatwoot offers a generous free tier for self-hosted or basic cloud, scaling reasonably for small businesses.
- **Hybrid Support**: Works exceptionally well in both Cloud (via Chatwoot Cloud or centralized self-hosting) and Standalone (can be run via Docker alongside the OHC local backend).
**Design Doc**:
- **User Experience**: The user navigates to "Settings -> Channels" and clicks "Connect Instagram" or "Connect WhatsApp". OHC handles the OAuth flow. All incoming messages appear in the OHC "Customer Success" inbox. The "Ambassador" agent can optionally draft replies.
- **Integration Layer**: OHC backend subscribes to Chatwoot webhooks for incoming messages. Outgoing messages from OHC are sent via Chatwoot API to the respective channel.
**Implementation Prompt**: Implement the Chatwoot integration so users can connect their social channels via the UI. Ensure incoming messages appear in the OHC unified inbox and replies are successfully routed back to the native platform (e.g., Instagram DM). Add support for the Customer Success agent to draft suggested responses.
**Priority**: P1
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling: Cal.com

**Title**: Booking & Calendar Sync via Cal.com
**Problem Statement**: Service providers like Carlos (handyman) and Leo (tutor) need a frictionless way for clients to book available times without endless back-and-forth messaging. They also need to ensure bookings don't conflict with their personal Google/Apple calendars.
**Research Report**:
- **Tool Evaluated**: Cal.com
- **Why it fits**: Cal.com is an open-source scheduling infrastructure. It has robust APIs and pre-built booking components.
- **Ease of Use**: Users simply connect their existing calendar and set their working hours. The system generates a clean, mobile-friendly booking link.
- **Pricing**: Free for individuals (perfect for our core personas), affordable team tiers.
- **Hybrid Support**: Can be self-hosted for Standalone mode or consumed via their cloud API for Cloud mode.
**Design Doc**:
- **User Experience**: Under "Services", the user toggles "Enable Booking". They authorize their Google or Apple Calendar. They define service duration (e.g., "1 Hour Plumbing Consult"). The OHC public storefront displays a date/time picker.
- **Integration Layer**: OHC uses Cal.com Platform API to create managed users for each tenant. OHC embeds the Cal.com booking widget on the storefront and listens to webhooks for new bookings to update the OHC database.
**Implementation Prompt**: Integrate Cal.com so business owners can connect their personal calendars and accept bookings directly from their OHC storefront. Ensure double-bookings are prevented and new appointments appear in the OHC dashboard.
**Priority**: P0
**Estimated Scope**: Medium

---

## 3. Email Marketing: Resend

**Title**: Automated Email Campaigns via Resend
**Problem Statement**: Business owners need to engage their customer base (e.g., Priya announcing a new clothing line) but find traditional tools like Mailchimp too complex and full of marketing jargon. They just want to type a message and hit send to all past customers.
**Research Report**:
- **Tool Evaluated**: Resend
- **Why it fits**: Resend is a developer-first email API with an incredible focus on deliverability and simplicity.
- **Ease of Use**: OHC can abstract away all template management. The user just writes text; OHC wraps it in a beautiful, branded HTML template and sends it via Resend.
- **Pricing**: 3,000 free emails per month (plenty for our personas), then $20/mo.
- **Hybrid Support**: Cloud API. Standalone mode can still call the API.
**Design Doc**:
- **User Experience**: User clicks "Send Update to Customers". They type a subject and a message, and optionally attach a photo. The "Promoter" agent can help draft it. They click Send.
- **Integration Layer**: OHC maintains the customer list. When a broadcast is triggered, OHC formats the emails using React Email (or similar Rust equivalent) and dispatches them via the Resend API.
**Implementation Prompt**: Integrate Resend to allow business owners to send beautifully formatted, branded email broadcasts to their customer list directly from the OHC mobile app. The Promoter agent should be able to suggest subject lines.
**Priority**: P2
**Estimated Scope**: Medium

---

## 4. Payment Processing: Mercado Pago

**Title**: LATAM Payment Processing via Mercado Pago
**Problem Statement**: While Stripe is excellent, it is not available or widely used in many Latin American countries where a huge portion of our target demographic resides. Users need a trusted, local payment method.
**Research Report**:
- **Tool Evaluated**: Mercado Pago API
- **Why it fits**: Dominant payment gateway in LATAM (Brazil, Mexico, Argentina, etc.). Supports local payment methods like PIX in Brazil.
- **Ease of Use**: Familiar checkout experience for LATAM customers. Simple OAuth connection for the merchant.
- **Pricing**: Varies by country, typically standard processing fees (~3-4%).
- **Hybrid Support**: Cloud API. Works seamlessly from both Cloud and Standalone OHC deployments.
**Design Doc**:
- **User Experience**: In Payment Settings, a user in LATAM sees "Connect Mercado Pago". After connecting, their storefront displays local payment options (PIX, Boleta, local credit cards) at checkout.
- **Integration Layer**: Implement a new provider in the OHC payment abstraction layer. Generate preference IDs via Mercado Pago API for checkout sessions and handle IPN (Instant Payment Notification) webhooks to mark orders as paid.
**Implementation Prompt**: Add Mercado Pago as a supported payment gateway alongside Stripe. Ensure the checkout flow seamlessly handles generation of payment preferences and processes local payment methods like PIX.
**Priority**: P1
**Estimated Scope**: Large

---

## 5. Shipping & Logistics: Shippo

**Title**: Automated Shipping Labels via Shippo
**Problem Statement**: Sellers of physical goods (like Maya shipping custom cookies) waste hours manually typing addresses into carrier websites and waiting in line at the post office.
**Research Report**:
- **Tool Evaluated**: Shippo API
- **Why it fits**: Aggregates multiple carriers (USPS, UPS, FedEx, DHL, etc.) behind a single API. Provides deep discounts on labels.
- **Ease of Use**: User clicks "Fulfill Order" -> sees the cheapest shipping option -> clicks "Buy Label" -> prints it from their phone.
- **Pricing**: Free tier available (pay per label + postage cost).
- **Hybrid Support**: Cloud API.
**Design Doc**:
- **User Experience**: On the Order Details screen, user taps "Create Shipping Label". OHC automatically passes the package dimensions (pre-saved per product) and customer address to Shippo. The user selects the rate and taps "Purchase". A printable PDF label is generated.
- **Integration Layer**: OHC calls Shippo to validate addresses, get live rates during checkout (optional future feature), and purchase labels. Tracking numbers are saved to the OHC order and auto-emailed to the customer.
**Implementation Prompt**: Integrate the Shippo API to allow merchants to generate and print shipping labels directly from the order fulfillment screen. Automatically attach the resulting tracking number to the order and notify the customer.
**Priority**: P2
**Estimated Scope**: Medium

---

## 6. SMS & Notifications: Twilio

**Title**: Customer SMS Notifications via Twilio
**Problem Statement**: For businesses dealing with time-sensitive orders (like Fatima's food cart) or customers with low email usage, SMS is critical for order confirmations and pickup ready alerts.
**Research Report**:
- **Tool Evaluated**: Twilio Programmable Messaging
- **Why it fits**: Global standard for SMS. Extremely reliable and supports WhatsApp as a fallback.
- **Ease of Use**: Invisible to the merchant. They just see that the customer received a text.
- **Pricing**: Pay-as-you-go (~$0.0079 per message in the US).
- **Hybrid Support**: Cloud API.
**Design Doc**:
- **User Experience**: When Fatima marks an order as "Ready for Pickup", the system automatically sends an SMS to the customer: "Your order from Fatima's Cart is ready!".
- **Integration Layer**: OHC backend uses the Twilio API to dispatch transactional SMS messages based on order state changes. Must handle opt-outs gracefully.
**Implementation Prompt**: Integrate Twilio to send automated SMS notifications to customers for critical order lifecycle events (e.g., Order Confirmed, Ready for Pickup). Ensure compliance with standard SMS opt-out requirements.
**Priority**: P1
**Estimated Scope**: Small

---

## 7. Video Conferencing: Daily.co

**Title**: Auto-Generated Video Meeting Links via Daily.co
**Problem Statement**: Tutors like Leo shouldn't have to manually create a Zoom link, copy it, and email it to a student every time a lesson is booked. It should be automatic and frictionless.
**Research Report**:
- **Tool Evaluated**: Daily.co API
- **Why it fits**: Unlike Zoom which requires complex OAuth and app approvals, Daily provides a dead-simple REST API to instantly generate video rooms. It runs entirely in the browser without requiring the client to download an app.
- **Ease of Use**: The student and tutor just click a link and the video call opens in their browser. Zero setup.
- **Pricing**: 10,000 free minutes per month.
- **Hybrid Support**: Cloud API. WebRTC works on both Cloud and Standalone.
**Design Doc**:
- **User Experience**: When a "Virtual Lesson" is booked, the confirmation screen and email include a unique "Join Video Call" link. Clicking it opens a beautiful, branded video room directly in the browser/app.
- **Integration Layer**: OHC calls the Daily.co API `POST /rooms` to generate a unique, time-bound room URL whenever a virtual service is booked. The URL is stored with the booking record.
**Implementation Prompt**: Integrate Daily.co to automatically generate secure, browser-based video meeting links for virtual bookings. Ensure the link is provided to both the business owner and the customer in their booking confirmations.
**Priority**: P2
**Estimated Scope**: Small
