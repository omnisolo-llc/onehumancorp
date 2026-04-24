# OHC Tool Integration Research Report

This report evaluates and proposes integration strategies for third-party tools to expand One Human Corp (OHC)'s capabilities, focusing on the needs of non-technical small business owners (like Maya the baker, Carlos the handyman, Fatima the food cart operator).

---

## 1. Social Media Integration

### Title: Integrate Ayrshare for Unified Social Media Inbox and Auto-Posting
**Problem Statement:**
Business owners like Maya the baker spend hours jumping between Instagram DMs, Facebook comments, and TikTok to reply to customers and post updates. They need a single, unified inbox within the OHC app where the AI agent (Customer Success) can draft replies, and a simple way to schedule posts across all platforms.

**Research Report:**
Ayrshare is a unified API that allows posting and message retrieval across Instagram, Facebook, TikTok, X (Twitter), and LinkedIn.
- **Ease of Use:** As an API integration, OHC abstracts the complexity. The user simply connects their accounts via OAuth in the OHC app once.
- **Pricing:** Ayrshare offers a competitive platform API pricing starting around $100-$300/month for platforms, which is cost-effective for OHC's multi-tenant cloud model. It also supports standalone users bringing their own keys if needed.
- **Cloud vs Standalone:** Excellent for Cloud (Webhook delivery to OHC). Standalone would require polling or a relay server if public webhooks aren't feasible.

**Design Doc:**
- **Trigger:** User connects social accounts via "Marketing & Advertising" tab. New DMs/comments trigger an event to the "Customer Success" agent.
- **Action:** The agent drafts a reply and shows it in the OHC Unified Inbox for user approval (or auto-sends if trusted). The "Marketing" agent can auto-publish new products to Instagram.
- **User Interface:** A simple "Social Accounts" connection page, a unified chat interface combining SMS, Email, and Social DMs, and a "Post Update" button on products.

**Implementation Prompt:**
Implement a social media integration that allows a user to authenticate their Instagram and Facebook accounts. Create a unified inbox view where incoming DMs and comments appear alongside emails. Provide a one-click "Post to Social" button on product pages that uses the AI agent to generate a caption and publish the product image. Ensure all API keys and tokens are securely managed per tenant.

**Priority:** P1
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling

### Title: Integrate Cal.com for Seamless Booking and Scheduling
**Problem Statement:**
Service providers like Carlos (handyman) and Leo (tutor) need a way to let clients book available time slots without back-and-forth texts. They need this synced with their personal Google/Apple calendars so they don't get double-booked.

**Research Report:**
Cal.com offers a robust, open-source scheduling infrastructure API (Platform API).
- **Ease of Use:** Users connect their Google Calendar, and OHC generates a beautiful, branded booking page.
- **Pricing:** Cal.com has a generous free tier for individuals and a white-label Platform plan for SaaS (pricing scales by active users). Being open-source, it's highly flexible.
- **Cloud vs Standalone:** Perfect for both. Standalone users can use the free public Cal.com API, while Cloud uses the Platform API.

**Design Doc:**
- **Trigger:** A customer visits Carlos's OHC website and clicks "Book Repair".
- **Action:** They see a calendar of available slots (synced with Carlos's Google Calendar via Cal.com). Booking a slot creates an event, triggers the "Operations" agent to send a confirmation, and optionally requests a deposit via Stripe.
- **User Interface:** A "Calendar Sync" button in the OHC app. A simple UI to define working hours (e.g., 9 AM - 5 PM).

**Implementation Prompt:**
Build a scheduling feature where a user can connect their external calendar (Google/Outlook). Generate a public booking widget that can be embedded in their OHC website. When a customer books a slot, it must appear in the OHC dashboard and the user's external calendar, and send a confirmation notification.

**Priority:** P0
**Estimated Scope:** Medium

---

## 3. Email Marketing

### Title: Integrate Resend for Transactional and Marketing Emails
**Problem Statement:**
Business owners like Priya (boutique) want to notify their customer base when new stock arrives, but platforms like Mailchimp are too complex and expensive. They need a simple way to send beautiful, AI-generated emails directly from OHC.

**Research Report:**
Resend is a developer-friendly email API designed for building and sending beautiful emails using React Email (or standard HTML).
- **Ease of Use:** Extremely fast delivery, excellent deliverability rates, and simple API. The user never sees Resend; they just see "Send Newsletter" in OHC.
- **Pricing:** Free tier up to 3,000 emails/month. $20/mo for 50,000 emails. Very scalable for OHC's multi-tenant model.
- **Cloud vs Standalone:** Cloud-native API. For standalone, users can input their own Resend API key or use a local SMTP relay.

**Design Doc:**
- **Trigger:** User clicks "Announce New Product" on a product page, or "Operations" agent sends an order receipt.
- **Action:** The "Marketing" agent drafts an email featuring the product, targets past customers, and sends it via Resend API.
- **User Interface:** A "Campaigns" tab with simple text inputs. The AI handles the design and formatting.

**Implementation Prompt:**
Integrate an email sending provider to handle both transactional (order receipts) and marketing emails. Add a "Send Announcement" feature where the AI agent drafts a promotional email for a selected product and sends it to the tenant's customer list. Ensure DKIM/SPF domain verification flows are simplified for the user.

**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing (International)

### Title: Integrate Mercado Pago for LATAM Payment Processing
**Problem Statement:**
While Stripe is great, it does not dominate Latin America where local payment methods (like PIX in Brazil or OXXO in Mexico) are essential. Users in LATAM need a native, trusted payment gateway to accept online orders.

**Research Report:**
Mercado Pago is the leading payment provider in Latin America, offering extensive support for local credit cards, cash payments, and bank transfers.
- **Ease of Use:** Similar integration complexity to Stripe. For the user, it’s a simple OAuth connection or credential drop.
- **Pricing:** Standard transaction fees (approx 3-5% depending on the country and payment method), no monthly fixed fees.
- **Cloud vs Standalone:** API-based, works well in both environments.

**Design Doc:**
- **Trigger:** A customer in LATAM reaches the checkout page on a user's OHC storefront.
- **Action:** They are presented with Mercado Pago options (e.g., PIX). Upon payment, the webhook notifies the OHC backend to mark the order as paid.
- **User Interface:** A "Payments" setting page allowing users to select their region and connect either Stripe or Mercado Pago.

**Implementation Prompt:**
Implement Mercado Pago as an alternative payment gateway alongside Stripe. Allow users to connect their Mercado Pago account. Update the checkout flow to render Mercado Pago's checkout SDK if the tenant has it enabled. Ensure webhook handlers accurately update order statuses (Paid, Failed, Pending Cash Payment).

**Priority:** P2
**Estimated Scope:** Large

---

## 5. Shipping & Logistics

### Title: Integrate Shippo for Automated Shipping Rates and Labels
**Problem Statement:**
When Maya ships her baked goods, calculating shipping rates manually and going to the post office is tedious. She needs real-time shipping costs at checkout and printable shipping labels directly in the OHC app.

**Research Report:**
Shippo provides a single API to access dozens of shipping carriers (USPS, UPS, FedEx, DHL) globally.
- **Ease of Use:** Shippo handles carrier accounts. The OHC user simply prints the label from their dashboard.
- **Pricing:** Free to install, pay per label (usually just a few cents) plus postage. Highly affordable.
- **Cloud vs Standalone:** Web API, seamless for both.

**Design Doc:**
- **Trigger:** Customer enters their address at checkout. OHC requests rates from Shippo.
- **Action:** Checkout displays accurate shipping costs. After payment, the "Operations" agent generates a shipping label in Shippo and presents a "Print Label" button in the OHC order dashboard.
- **User Interface:** Order details page showing shipping status, a "Print Label" button, and auto-populated tracking numbers sent to the customer.

**Implementation Prompt:**
Integrate a shipping API to calculate real-time shipping rates during the storefront checkout process based on product weight and customer address. In the order management dashboard, add functionality to generate and download a printable shipping label, and automatically email the tracking link to the customer.

**Priority:** P1
**Estimated Scope:** Large

---

## 6. SMS & Notifications

### Title: Integrate Twilio for Reliable SMS Notifications
**Problem Statement:**
Fatima (food cart) doesn't always have a strong data connection for app push notifications and prefers getting an SMS when a new order arrives. Her customers also prefer SMS order ready alerts over email.

**Research Report:**
Twilio is the industry standard for programmable SMS and voice.
- **Ease of Use:** OHC handles the Twilio account in the cloud. The user just turns on "SMS Notifications" in settings.
- **Pricing:** ~$0.0079 per SMS. OHC can absorb this cost or bill it to the tenant's usage limits.
- **Cloud vs Standalone:** Cloud-native. Standalone users would need to provide their own Twilio Account SID and Auth Token.

**Design Doc:**
- **Trigger:** A customer places an order for Fatima's food cart.
- **Action:** Twilio sends a concise SMS to Fatima: "New Order #102: 2x Falafel. Pickup in 15m." Once Fatima taps "Ready" in the app, Twilio sends an SMS to the customer.
- **User Interface:** Simple toggle switches in the Settings -> Notifications tab for "Notify me via SMS" and "Notify customers via SMS".

**Implementation Prompt:**
Integrate an SMS provider to send critical alerts. Implement two flows: 1) Sending a brief SMS to the business owner when a new order or booking is placed. 2) Sending an SMS to the customer when their order is ready for pickup or shipped. Ensure phone numbers are properly formatted and opt-out compliance (STOP replies) is handled.

**Priority:** P1
**Estimated Scope:** Medium

---

## 7. Video Conferencing

### Title: Integrate Zoom API for Automated Virtual Lesson Links
**Problem Statement:**
Leo (music tutor) spends too much time manually creating Zoom links and emailing them to students for his online guitar lessons. He needs this completely automated.

**Research Report:**
Zoom API allows programmatic creation of meetings and retrieval of join links.
- **Ease of Use:** User authenticates Zoom via OAuth once. OHC does the rest.
- **Pricing:** Free API access for Zoom account holders.
- **Cloud vs Standalone:** Works seamlessly via OAuth in both modes.

**Design Doc:**
- **Trigger:** A student books a "Virtual Lesson" on Leo's calendar.
- **Action:** The OHC backend calls the Zoom API to create a meeting for that specific time, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
- **User Interface:** A "Connect Zoom" button in the Service setup page. When enabled, virtual services automatically generate links.

**Implementation Prompt:**
Integrate video conferencing link generation for virtual bookings. Allow the user to connect their Zoom account. When a customer books a service marked as 'Virtual', automatically generate a unique Zoom meeting link and include it in the confirmation email and calendar event.

**Priority:** P2
**Estimated Scope:** Medium
