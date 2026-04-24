# 🔍 Scout: Tool Integration Research Q4

## 1. Social Media Integration: WhatsApp Cloud API
**Title:** Integrate WhatsApp Cloud API for Unified Inbox
**Problem Statement:** Business owners like Maya (The Home Baker) and Fatima (The Food Cart Operator) receive many inquiries via WhatsApp. Managing multiple apps is overwhelming, and they miss messages while busy. They need a single inbox inside OHC to see and reply to WhatsApp messages, and allow the AI agent to draft responses.
**Research Report:**
- **Tool:** WhatsApp Cloud API (Meta Graph API).
- **Pros:** Native Meta support, reliable, supports rich media, massive global user base. Essential for LATAM, India, and European markets.
- **Cons:** Strict opt-in rules and template messaging requirements for business-initiated conversations. Requires a Facebook Business account.
- **Pricing:** Conversation-based pricing. First 1,000 service conversations per month are free.
- **Ease of Use:** For the end-user, once connected via an OAuth-like flow (Embedded Signup), messages appear magically in the OHC inbox.
- **Cloud/Standalone:** Works in both, but Standalone requires local webhook tunneling or polling if possible, or OHC cloud acting as a webhook relay.
**Design Doc:**
- User connects their WhatsApp Business account via Meta Embedded Signup in the "Marketing & Advertising" or "Customer Success" settings.
- Incoming WhatsApp messages trigger webhooks to OHC, which are routed to the unified customer inbox.
- The "Customer Success" AI agent drafts replies based on memory and inventory.
- User can review and send replies from the OHC mobile app, pushing them back to WhatsApp via the API.
**Implementation Prompt:** Provide a UI in the settings for users to connect their WhatsApp account. Once connected, incoming messages must appear in the OHC unified inbox, and outgoing messages typed in the inbox must be delivered to the customer's WhatsApp. The AI should generate draft replies for incoming messages.
**Priority:** P0
**Estimated Scope:** Large

## 2. Calendar & Scheduling: Cal.com
**Title:** Integrate Cal.com API for Booking & Scheduling
**Problem Statement:** Service providers like Carlos (Freelance Handyman) and Leo (Music Tutor) need clients to book available time slots without back-and-forth messaging. They need a system that prevents double-booking and automatically generates meeting links or blocks travel time.
**Research Report:**
- **Tool:** Cal.com API / Platform
- **Pros:** Open source, API-first, supports white-labeling, handles timezone math perfectly, integrates with multiple calendars (Google, Outlook, Apple) out-of-the-box.
- **Cons:** Custom white-labeling can be complex.
- **Pricing:** Very generous free tier for individuals. API pricing is scalable.
- **Ease of Use:** Users just connect their existing calendar. OHC handles the booking page UI.
- **Cloud/Standalone:** Cal.com can be self-hosted, making it an excellent fit for OHC's Standalone mode, while using the managed service for Cloud mode.
**Design Doc:**
- User links their external calendar via Cal.com OAuth in OHC settings.
- The "Operations" agent reads availability to show available slots on the user's public OHC storefront.
- When a customer selects a slot, OHC books it via Cal.com API, which syncs to the user's personal calendar.
**Implementation Prompt:** Implement a booking widget on the public storefront that pulls availability from a connected Cal.com account. When a user books a slot, record the appointment in OHC and sync it to the owner's external calendar.
**Priority:** P1
**Estimated Scope:** Medium

## 3. Email Marketing: Resend
**Title:** Integrate Resend for Transactional & Marketing Emails
**Problem Statement:** Priya (Boutique Owner) wants to automatically email her customers when a new collection drops, and send beautiful order confirmation emails. Non-technical users struggle with complex email builders like Mailchimp and DNS records.
**Research Report:**
- **Tool:** Resend
- **Pros:** Developer-friendly API, React Email integration makes building beautiful templates easy, high deliverability, simple setup.
- **Cons:** Newer platform, fewer out-of-the-box marketing automation flows compared to legacy providers (though OHC's AI handles the flows).
- **Pricing:** Free for up to 3,000 emails/month.
- **Ease of Use:** OHC abstracts the DNS setup as much as possible or uses an OHC subdomain. AI generates the email content and Resend handles delivery.
- **Cloud/Standalone:** Works seamlessly via API in both environments.
**Design Doc:**
- "Marketing & Advertising" agent writes an email draft (e.g., "New Arrivals").
- User approves the draft in the OHC app.
- OHC compiles the email using a premium OHC template and sends it via Resend API to the filtered customer list.
**Implementation Prompt:** Build an email campaign flow where the AI drafts a marketing email. Provide a preview of the email in the mobile app, and upon user approval, dispatch it to all subscribed customers using the Resend API.
**Priority:** P1
**Estimated Scope:** Medium

## 4. Payment Processing: Mercado Pago
**Title:** Integrate Mercado Pago for LATAM Payments
**Problem Statement:** While Stripe is great globally, small businesses in Latin America (like a boutique in Brazil or Mexico) rely heavily on local payment methods (Pix, Boleto, local credit cards) that are best supported by Mercado Pago. Without this, OHC misses a massive demographic of small business owners.
**Research Report:**
- **Tool:** Mercado Pago API
- **Pros:** Dominant in LATAM. Supports Pix (instant payments in Brazil) which is critical for local commerce.
- **Cons:** API documentation and sandbox testing can be finicky. Different from Stripe's model.
- **Pricing:** Percentage per transaction, varies by country and payment method.
- **Ease of Use:** User logs into Mercado Pago via OAuth. Customers on the storefront see a familiar Mercado Pago checkout.
- **Cloud/Standalone:** API works perfectly in both environments.
**Design Doc:**
- In "Finance & Payments" settings, user can connect Mercado Pago as an alternative or primary payment gateway depending on their region.
- Storefront checkout dynamically displays Mercado Pago (including Pix QR code generation) if configured.
- Webhooks update OHC order status when a payment clears.
**Implementation Prompt:** Add Mercado Pago as a payment provider option. Allow users to connect their account. Update the storefront checkout to generate a Mercado Pago payment link or Pix QR code, and listen for payment completion webhooks to mark orders as paid.
**Priority:** P2
**Estimated Scope:** Large

## 5. Shipping & Logistics: Shippo
**Title:** Integrate Shippo for Shipping Rates & Labels
**Problem Statement:** Maya (Home Baker) and Priya (Boutique Owner) need to ship physical goods. Calculating shipping rates manually and going to the post office to buy labels is a huge time sink.
**Research Report:**
- **Tool:** Shippo API
- **Pros:** Aggregates multiple carriers (USPS, UPS, FedEx, DHL, local carriers). Returns real-time rates. Easy label generation API.
- **Cons:** Address validation can sometimes be overly strict.
- **Pricing:** Pay-as-you-go per label, no monthly fee for basic API usage.
- **Ease of Use:** Non-technical user just inputs package weight/dimensions in OHC. Shippo handles the complex carrier math. User clicks "Buy Label" and prints it directly from their phone.
- **Cloud/Standalone:** API works well in both.
**Design Doc:**
- "Operations" agent uses Shippo API during checkout to calculate shipping costs based on the customer's address and product weights.
- When fulfilling an order, OHC presents label options (cheapest, fastest).
- User purchases the label; OHC downloads the PDF and displays a print button in the app.
**Implementation Prompt:** Integrate real-time shipping rate calculation at storefront checkout. Add a "Fulfill Order" flow in the app that purchases a shipping label using Shippo and provides a printable PDF to the business owner.
**Priority:** P1
**Estimated Scope:** Large

## 6. SMS & Notifications: Twilio
**Title:** Integrate Twilio for Customer SMS Notifications
**Problem Statement:** Fatima (Food Cart Operator) needs customers to know exactly when their food is ready. Email is too slow for food pickup. SMS is the most reliable way to notify customers who ordered from their phone.
**Research Report:**
- **Tool:** Twilio Programmable SMS
- **Pros:** Global reach, highly reliable, easy to use API.
- **Cons:** A2P 10DLC compliance in the US requires business registration, which might be a hurdle for informal businesses (OHC may need to act as the primary sender or handle registration).
- **Pricing:** Pay-as-you-go per message (e.g., ~$0.0079 per SMS in US).
- **Ease of Use:** Completely invisible to the business owner. The OHC platform handles the sending.
- **Cloud/Standalone:** Perfect for Cloud. Standalone might require the user to bring their own Twilio credentials or route through OHC cloud.
**Design Doc:**
- "Customer Success" agent monitors order status.
- When Fatima taps "Order Ready" on her OHC app, the system triggers a Twilio API call to send an SMS to the customer's phone number.
**Implementation Prompt:** Add an SMS notification trigger when an order status changes to "Ready for Pickup" or "Shipped". Send a short, localized SMS to the customer using Twilio.
**Priority:** P1
**Estimated Scope:** Medium

## 7. Video Conferencing: Zoom
**Title:** Integrate Zoom API for Online Consultations
**Problem Statement:** Leo (Music Tutor) teaches classes online. Manually creating a Zoom link and emailing it to the student for every booking is tedious and error-prone.
**Research Report:**
- **Tool:** Zoom API (Server-to-Server OAuth or standard OAuth)
- **Pros:** Industry standard, universally trusted, most clients already have the app installed.
- **Cons:** App approval process in the Zoom Marketplace can be lengthy.
- **Pricing:** Free tier available for basic usage; pro accounts needed for longer meetings.
- **Ease of Use:** User connects Zoom account. Magic links appear in calendar invites.
- **Cloud/Standalone:** Requires OAuth. In Standalone, might need an OHC Cloud relay for the OAuth callback.
**Design Doc:**
- User connects Zoom in OHC settings.
- When a service is configured as "Online Meeting", the "Operations" agent automatically generates a unique Zoom meeting link via API upon booking confirmation.
- The link is injected into the calendar invite and the customer's confirmation email.
**Implementation Prompt:** Allow users to connect their Zoom account. For bookings marked as "virtual", automatically generate a Zoom meeting link and include it in the confirmation email and appointment details view.
**Priority:** P2
**Estimated Scope:** Medium