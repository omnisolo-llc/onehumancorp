# OHC Tool Integration Research Report

## 1. Social Media Integration

**Title:** Integrate Chatwoot for Unified Social Inbox
**Problem Statement:** Business owners like Maya (The Home Baker) receive inquiries across Instagram DMs, WhatsApp, and Facebook. Managing multiple apps is overwhelming and leads to missed sales. They need a single, simple inbox within OHC to see and reply to all customer messages.
**Research Report:** Chatwoot is an open-source omnichannel customer support platform. It natively supports WhatsApp, Instagram, Facebook, Line, Twitter, and email. It is highly rated for its simplicity and API-first design. Since it's open-source, we can self-host it for Cloud mode (multi-tenant) and potentially integrate its headless API for Standalone mode. The cost is negligible if self-hosted, compared to Zendesk or Intercom which are prohibitively expensive for small businesses. Non-technical users will just see a "Connect Instagram" button and an "Inbox" tab.
*Key Advantages:* Open-source, supports many channels out-of-the-box, cheap.
*Key Risks:* Self-hosting requires maintenance. Sync issues if webhooks fail.
*Modes Supported:* Cloud (self-hosted backend), Standalone (via cloud APIs or embedded local instance).
**Design Doc:**
- User clicks "Connect Instagram" in OHC settings.
- OHC initiates OAuth flow via Chatwoot APIs.
- Webhooks from Chatwoot are routed to the OHC agent "Customer Success - The Ambassador".
- The Ambassador agent reads messages, auto-drafts replies, and displays them in the unified OHC inbox.
- Business owner reviews and clicks "Send", which routes back through Chatwoot to the native social platform.
**Implementation Prompt:** Implement a unified inbox UI in the OHC Flutter app. Add "Connect" buttons for Instagram and WhatsApp. When a customer sends a message on those platforms, it must appear in the OHC inbox. The user should be able to type a reply in OHC and have it sent back to the customer on the original platform.
**Priority:** P0
**Estimated Scope:** Large

## 2. Calendar & Scheduling

**Title:** Integrate Cal.com for Seamless Booking & Calendar Sync
**Problem Statement:** Service providers like Leo (The Music Tutor) and Carlos (The Freelance Handyman) need customers to book specific time slots. Back-and-forth messaging to find a time causes drop-offs. They need a way to show availability and let customers book directly.
**Research Report:** Cal.com is an open-source alternative to Calendly. It supports Google Calendar, Outlook, and Apple Calendar sync to prevent double-booking. It can auto-generate Google Meet or Zoom links. It offers a generous free tier for individuals and a white-label API for platforms. For OHC users, the experience will be seamless: they connect their Google calendar, set their working hours, and OHC generates a booking page.
*Key Advantages:* Developer friendly white-labeling, handles timezones perfectly, free tier.
*Key Risks:* Syncing delays with Google calendar.
*Modes Supported:* Both Cloud (OAuth via Cal.com hosted) and Standalone (direct API token).
**Design Doc:**
- User connects Google/Outlook calendar via Cal.com OAuth integration.
- User configures "Service Types" (e.g., 1-hour plumbing repair, 30-min guitar lesson) in OHC.
- OHC creates corresponding event types in Cal.com via API.
- The OHC public storefront displays an embedded Cal.com booking widget.
- Upon booking, Cal.com triggers a webhook to OHC, which then notifies the "Operations" agent to update the dashboard.
**Implementation Prompt:** Add a "Bookings" tab in OHC. Allow the user to connect their personal calendar. Create a booking widget on their public storefront that shows available time slots. When a customer books a slot, it should appear in the OHC dashboard and automatically add an event to the business owner's personal calendar.
**Priority:** P1
**Estimated Scope:** Medium

## 3. Email Marketing

**Title:** Integrate Resend for Transactional and Marketing Emails
**Problem Statement:** Business owners need to send order confirmations, appointment reminders, and promotional newsletters (e.g., "New Spring Collection"). Setting up Mailchimp is too complex and disconnected from their core business data.
**Research Report:** Resend is a developer-first email API built for modern apps. It offers excellent deliverability, a simple API, and beautiful email templates (via React Email, though we can generate HTML). It handles unsubscribe links and spam compliance automatically. The free tier allows 3,000 emails/month, which covers most micro-businesses. For users, they simply type a message in OHC, select an audience (e.g., "Past Customers"), and hit send.
*Key Advantages:* High deliverability, simple API, free tier up to 3000/month.
*Key Risks:* Users getting marked as spam if they abuse marketing emails. Domain verification is tricky for non-technical users.
*Modes Supported:* Both Cloud (Platform API keys) and Standalone (User provides their own key or uses OHC shared relay).
**Design Doc:**
- OHC maintains the central customer directory.
- For transactional emails (receipts), the backend directly calls the Resend API.
- For marketing emails, the "Marketing & Advertising" agent drafts the email content.
- OHC renders the email using predefined, beautiful OHC-branded templates.
- OHC sends the batch via Resend API and listens to webhooks for open/click metrics.
**Implementation Prompt:** Implement an email capability within OHC. Create a "Broadcast" feature where the user can type a message, and it will be sent to all their past customers via email. Also, ensure automatic email receipts are sent when a customer pays for an order.
**Priority:** P1
**Estimated Scope:** Medium

## 4. Payment Processing

**Title:** Integrate Mercado Pago for LATAM Payments
**Problem Statement:** Stripe is excellent but not dominant in Latin America. Business owners in LATAM need to accept local payment methods like PIX (Brazil), OXXO (Mexico), and local credit cards with installments. Without this, OHC cannot effectively serve the LATAM market.
**Research Report:** Mercado Pago is the leading payment processor in Latin America. It offers a robust API, supports local payment methods seamlessly, and handles the complex regulatory environment of the region. The integration will allow users in supported countries to select Mercado Pago instead of Stripe during onboarding.
*Key Advantages:* Unlocks massive LATAM market, supports PIX and local installments.
*Key Risks:* Dispute resolution and fraud handling is different from Stripe.
*Modes Supported:* Cloud (Platform integration) and Standalone (User provides API keys).
**Design Doc:**
- Add region detection during user onboarding.
- If the user is in LATAM, offer Mercado Pago connection alongside or instead of Stripe.
- Implement Mercado Pago Checkout Pro (redirect) or Checkout API for custom UI.
- Handle Mercado Pago IPN (Instant Payment Notification) webhooks to update order status in OHC.
**Implementation Prompt:** Add Mercado Pago as a payment provider option. For users in Latin America, allow them to connect their Mercado Pago account. When their customers check out, they should be able to pay using Mercado Pago (including PIX and local cards).
**Priority:** P2
**Estimated Scope:** Large

## 5. Shipping & Logistics

**Title:** Integrate Shippo for Automated Label Generation and Tracking
**Problem Statement:** Sellers of physical products like Priya (The Boutique Owner) struggle with manually creating shipping labels and tracking packages for customers. They need a system that calculates shipping rates at checkout and auto-generates labels upon payment.
**Research Report:** Shippo is a multi-carrier shipping API that integrates with USPS, UPS, FedEx, DHL, and local carriers globally. It provides discounted shipping rates without volume minimums, making it perfect for small sellers.
*Key Advantages:* Immediate discounted rates, simple rate calculation at checkout, easy tracking webhook integration.
*Key Risks:* Customs forms for international shipping can be complex.
*Modes Supported:* Both Cloud (Multi-tenant Platform Account) and Standalone (Local user API token).
**Design Doc:**
- During checkout, frontend calls OHC backend to fetch shipping rates via Shippo API based on product dimensions and destination.
- When an order is marked "Ready to Ship", the OHC "Operations" agent requests a label from Shippo.
- OHC stores the tracking number and notifies the "Customer Success" agent to email the tracking link.
**Implementation Prompt:** Add shipping configuration to OHC. At checkout, display live shipping rates. In the order management view, add a "Generate Label" button that downloads a printable shipping label and automatically sends the tracking number to the customer.
**Priority:** P1
**Estimated Scope:** Medium

## 6. SMS & Notifications

**Title:** Integrate Twilio for SMS Order Alerts and Reminders
**Problem Statement:** Business owners like Fatima (The Food Cart Operator) work in fast-paced environments where they might not see a push notification, but they always check SMS. Similarly, their customers prefer SMS for "food is ready" alerts or appointment reminders.
**Research Report:** Twilio is the industry standard for SMS delivery globally. It offers highly reliable delivery and phone number provisioning. For users, they don't need to know it's Twilio; they just toggle "Send SMS alerts" in OHC.
*Key Advantages:* Reliable global delivery, programmable numbers.
*Key Risks:* High cost compared to email. Strict regulatory compliance (A2P 10DLC in the US).
*Modes Supported:* Cloud (Centrally managed Twilio account, usage billed to user) and Standalone (User brings their own Twilio SID/Token).
**Design Doc:**
- OHC provisions a Twilio subaccount and local phone number for the user's business.
- The "Customer Success" agent is configured to send SMS via the Twilio API for critical alerts (e.g., appointment tomorrow).
- Twilio webhooks feed incoming SMS replies back into the OHC unified inbox.
**Implementation Prompt:** Add SMS support to the OHC notification engine. Allow business owners to opt-in to receiving SMS alerts when a new order is placed. Allow the business to send automatic SMS appointment reminders 24 hours before a booking.
**Priority:** P1
**Estimated Scope:** Medium

## 7. Video Conferencing

**Title:** Integrate Zoom API for Auto-Generated Online Meeting Links
**Problem Statement:** Service providers offering online lessons like Leo (The Music Tutor) have to manually create Zoom meetings and email the links to clients after they book. This manual step often leads to errors or forgotten links.
**Research Report:** Zoom's API allows automatic meeting creation. When a booking is confirmed, a unique meeting link can be generated and embedded directly into the calendar invite and email.
*Key Advantages:* Industry standard, everyone knows how to use it.
*Key Risks:* OAuth token expiration issues, free tier 40-minute limit for group meetings.
*Modes Supported:* Both Cloud (Platform OAuth App) and Standalone (Server-to-Server OAuth or individual OAuth).
**Design Doc:**
- User connects Zoom via OAuth in OHC settings.
- For online service types, the "Operations" agent hooks into the booking flow.
- OHC calls Zoom API `POST /users/me/meetings` upon successful booking.
- OHC saves the `join_url` and sends it to the customer via the calendar invite and email.
**Implementation Prompt:** Add a "Connect Zoom" button in the integrations tab. When a customer books a service marked as "Online Meeting", automatically generate a unique Zoom link and include it in the confirmation email and calendar event for both the customer and the business owner.
**Priority:** P2
**Estimated Scope:** Medium
