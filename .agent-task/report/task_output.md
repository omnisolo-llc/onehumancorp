# Scout: Tool Integration Research Report (Q4)

This report outlines 7 actionable integration proposals to significantly enhance OneHumanCorp's (OHC) capabilities for non-technical small business owners. These tool integrations are designed to expand OHC's product offering while maintaining extreme ease-of-use.

Below are the detailed issue briefs for each integration domain.

---

# [Social Media Integration] Social Media Unified Inbox via MessageBird

**Title**: Implement Omnichannel Unified Inbox using MessageBird

**Problem Statement**:
Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive customer inquiries, custom orders, and support questions scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. It is overwhelming to constantly switch between 4-5 apps on their phones, and missed messages directly lead to lost sales. They need a single, simple inbox inside the OneHumanCorp (OHC) app where every message appears, allowing them—or their AI "Ambassador"—to read and reply instantly.

**Research Report**:
I evaluated direct Meta Graph API integration, Twilio Conversations, and MessageBird (now Bird) for consolidating social media messages.
- **MessageBird**: Excellent omnichannel API. Standardizes payloads across WhatsApp, Instagram, Facebook, and TikTok. Handles the complex Meta API changes under the hood. High webhook reliability. Pricing: flexible pay-as-you-go or $50/mo base, which is manageable. Best fit for OHC to abstract complexity.
- **Twilio**: Strong for SMS and WhatsApp, but historically slower to fully support native features of Instagram DMs and TikTok compared to MessageBird.
- **Direct Meta API**: Lowest cost (free API), but extremely high OAuth and compliance complexity. It would require OHC to constantly maintain API version bumps for Facebook and Instagram independently, and it doesn't solve TikTok.
- **Conclusion**: MessageBird is the optimal provider. It simplifies the OAuth flow for the end-user (business owner) and standardizes message parsing for our AI agents. It works flawlessly in a Cloud (multi-tenant) environment, while Standalone mode users could provide their own MessageBird API key or rely on a relayed OHC connection.

**Design Doc**:
- **Integration Point**: Resides within the "Customer Success" (The Ambassador) department.
- **Triggers & Flow**:
  1. The user navigates to "Channels" and clicks "Connect Instagram/WhatsApp".
  2. The user completes an OAuth flow (managed by the provider).
  3. Once linked, incoming messages on those platforms trigger a webhook to OHC.
  4. The "Ambassador" AI agent intercepts the webhook, parses the context (and past memory of the customer), and drafts a reply.
  5. The message appears in the user's OHC Mobile Inbox.
- **User View**: A unified, WhatsApp-style chat interface on their 375px mobile screen. Badges indicate the source (e.g., a small Instagram icon next to the message).

**Implementation Prompt**:
Build a unified inbox interface within the OHC Flutter app and integrate it with the selected Omnichannel messaging provider. The user must be able to securely connect their social media accounts via a simple UI flow. Once connected, incoming messages from Instagram, Facebook, and WhatsApp must populate a single chat view in real-time. The "Ambassador" AI agent should automatically draft suggested responses for unread messages, which the user can approve or edit with one tap. Ensure the design relies on OHC Premium Tokens and works flawlessly on a 375px mobile screen.

**Priority**: P1
**Estimated Scope**: Large

---

# [Calendar & Scheduling] Automated Meeting Sync via Cronofy

**Title**: Implement Universal Calendar Sync & Auto-Booking with Cronofy

**Problem Statement**:
Service providers like Leo (The Music Tutor) and Carlos (The Freelance Handyman) lose hours every week negotiating meeting times back and forth with clients. They need a system where customers can view their real-time availability and instantly book a slot. Critically, this system must automatically block out times when they have personal appointments in their existing Google or Apple Calendars, so they never get double-booked, and it must auto-generate a video link for online sessions.

**Research Report**:
I evaluated direct Google/Microsoft API integration, Cal.com API, and Cronofy API.
- **Cronofy**: A unified API that connects to Google, Apple, Microsoft Exchange, and Office 365. Handles all the complex timezone logic and conflicting event resolution. Has deep features for generating smart booking links. Excellent for SaaS platforms wanting to embed calendar features. Pricing is structured for platforms.
- **Cal.com API**: Open-source, very strong feature set, but their enterprise API pricing can be steep for early-stage platforms.
- **Direct Google/Apple APIs**: Zero cost but massive engineering overhead to maintain OAuth scopes, refresh tokens, and distinct API quirks across 3-4 different calendar ecosystems.
- **Conclusion**: Cronofy is the best infrastructure choice. It allows OHC to offer a flawless booking experience without the heavy engineering burden of maintaining individual calendar API connections. It supports both Cloud mode securely and Standalone mode (via local OAuth token persistence).

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department.
- **Triggers & Flow**:
  1. User goes to settings and connects their personal calendar (Google, Apple, etc.) via a simple OAuth popup.
  2. The user sets their "Working Hours" in OHC.
  3. OHC generates a public booking page link.
  4. When a customer visits the link, OHC queries the provider for real-time free/busy slots (ignoring private event details).
  5. The customer selects a slot and books.
  6. The provider pushes the new event back to the user's personal calendar and OHC's DB, triggering an auto-generated Zoom/Meet link if required.
- **User View**: A clean, mobile-optimized list of upcoming appointments. A simple toggle to "Sync my personal calendar".

**Implementation Prompt**:
Develop a universal calendar syncing and booking module. Integrate the selected calendar provider to allow the business owner to connect their Google, Apple, or Outlook calendar with a single click. The system must generate a public-facing booking UI for customers that accurately reflects real-time availability by cross-referencing OHC's working hours with the owner's personal calendar events. When a booking is made, the system must create an event in both OHC and the external calendar, and notify the "Manager" AI agent to send confirmation emails and reminders. The booking UI must be flawless on a 375px mobile screen.

**Priority**: P0
**Estimated Scope**: Large

---

# [Email Marketing] AI-Driven Email Campaigns via Resend

**Title**: Implement AI-Generated Email Marketing Campaigns via Resend

**Problem Statement**:
Business owners like Priya (The Boutique Owner) want to notify their customer base when new stock arrives, or send a discount code to VIP customers. However, traditional tools like Mailchimp are too complex and expensive, requiring HTML knowledge or frustrating drag-and-drop editors. They need a system where they can simply tell their AI "Promoter", "Send an email to my top 20 customers giving them 15% off the new summer collection," and the email is beautifully designed, targeted, and sent automatically.

**Research Report**:
I evaluated Mailchimp API, SendGrid, and Resend.
- **Resend**: Developer-first, extremely fast, modern API. Focuses on delivering beautiful emails (integrates perfectly with React Email, which conceptually aligns with our modern stack). High deliverability and simpler webhook management. Transparent, accessible pricing.
- **SendGrid**: The industry legacy standard. Highly reliable but has an antiquated dashboard and complex sub-user management that would be difficult to abstract cleanly for non-technical users in OHC.
- **Mailchimp API**: Very expensive at scale, and pushes users toward their own heavy UI rather than allowing seamless integration into OHC's white-labeled experience.
- **Conclusion**: Resend is the modern choice. Its API allows us to programmatically generate beautiful emails (via the AI Promoter) and send them reliably. It handles bounce and spam complaint webhooks gracefully, which is essential for protecting OHC's domain reputation.

**Design Doc**:
- **Integration Point**: Resides within the "Marketing & Advertising" (The Promoter) department.
- **Triggers & Flow**:
  1. The user asks the AI Promoter (via chat or a simple form) to send an announcement.
  2. The AI uses the context of the user's products and past successful emails to draft the content.
  3. The AI selects the appropriate customer segment from the OHC database.
  4. The AI renders a beautiful email template (using OHC's standard design tokens).
  5. The draft is presented to the user for 1-tap approval.
  6. Upon approval, OHC dispatches the emails via the Resend API in batches.
- **User View**: A simple "Campaigns" tab showing sent emails, open rates (plain language: "30 people read your email"), and a prominent "Create New Campaign" button that invokes the AI.

**Implementation Prompt**:
Build an AI-driven email campaign manager powered by the selected email provider. The system must allow the "Promoter" AI agent to query the customer database, segment users, and draft beautifully formatted promotional emails based on simple natural language prompts from the business owner. The UI must include a mobile-friendly preview of the email draft, a one-tap approval workflow, and a simple analytics dashboard showing open and click rates in plain language. Ensure strict handling of unsubscribe links and bounce webhooks to maintain domain reputation.

**Priority**: P1
**Estimated Scope**: Medium

---

# [Payment Processing] Global Payment Alternatives via Mercado Pago & Razorpay

**Title**: Implement Localized Payment Gateways (Mercado Pago / Razorpay)

**Problem Statement**:
While OHC relies heavily on Stripe, Stripe is not supported in all global markets, or it lacks dominance in local payment methods. For example, a business owner in Brazil relies heavily on Pix (often via Mercado Pago), and a merchant in India relies on UPI (often via Razorpay). To fulfill OHC's mission of empowering "anyone", the platform must support the payment gateways that small business owners actually use in their local regions, without confusing them with technical setup.

**Research Report**:
I evaluated adding Mercado Pago (LATAM) and Razorpay (India) as Stripe alternatives within OHC.
- **Mercado Pago**: Absolute dominance in Latin America. Supports Pix (Brazil), local credit cards, and cash payments (e.g., Boleto). The API is mature and supports webhook notifications for asynchronous payments (like Boleto).
- **Razorpay**: The dominant player in India. Flawless UPI integration, which is critical as UPI handles the vast majority of small transactions in India.
- **Stripe Local Methods**: Stripe supports some local methods (like Pix and UPI), but merchant account creation in these specific countries via Stripe Connect can be highly restrictive or currently unsupported compared to the local giants.
- **Conclusion**: We need an abstracted "Payment Provider" interface in the OHC backend. When a user in Brazil sets up their store, the AI "Accountant" should seamlessly configure Mercado Pago. When in India, it configures Razorpay.

**Design Doc**:
- **Integration Point**: Resides within the "Finance & Payments" (The Accountant) department.
- **Triggers & Flow**:
  1. During onboarding, OHC detects the user's country.
  2. If the country is best served by Mercado Pago or Razorpay, the "Accountant" guides them through an OAuth/API key setup tailored to that provider.
  3. The OHC checkout UI dynamically swaps its backend payment intent generator to point to the correct provider.
  4. Webhooks from the localized provider update the order status in OHC.
- **User View**: A simple "Payments" settings page that automatically recommends the best gateway for their region. A seamless checkout experience for their customers supporting Pix/UPI.

**Implementation Prompt**:
Refactor the checkout and payment intent architecture to support multiple payment gateways behind a unified interface. Implement integrations for Mercado Pago (targeting LATAM/Pix) and Razorpay (targeting India/UPI). The onboarding flow must automatically detect the user's region and recommend the appropriate gateway. Ensure the checkout UI gracefully handles asynchronous payment methods (like waiting for a Pix scan or UPI approval) with real-time UI updates powered by provider webhooks.

**Priority**: P2
**Estimated Scope**: Large

---

# [Shipping & Logistics] Automated Shipping Operations via Shippo

**Title**: Implement Automated Shipping Rates and Label Generation with Shippo

**Problem Statement**:
Business owners selling physical goods, like Priya (The Boutique Owner) or Maya (The Home Baker shipping cookies), struggle with calculating accurate shipping costs at checkout. Overcharging loses the customer; undercharging costs the business money. Furthermore, copying and pasting addresses into USPS/FedEx websites to print labels is tedious and error-prone. They need accurate live rates at checkout and a one-click way to print labels from their phone.

**Research Report**:
I evaluated EasyPost, ShipStation API, and Shippo.
- **Shippo**: Extremely developer-friendly API. Excellent pre-negotiated rates for USPS, UPS, and FedEx right out of the box (critical for new small businesses without their own carrier accounts). Great support for international shipping and customs forms. Pricing is highly favorable for platforms.
- **EasyPost**: Also an excellent API, very similar to Shippo, but Shippo's dashboard and pre-negotiated rate structures are slightly more geared toward enabling small merchants quickly.
- **ShipStation**: Powerful, but their API is designed more for integrating into their heavy dashboard rather than letting OHC completely white-label the experience.
- **Conclusion**: Shippo is the best fit. It allows OHC to provide live rates at checkout and lets the AI "Manager" instantly generate a printable PDF label as soon as an order is paid, without the user ever leaving the OHC app.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department.
- **Triggers & Flow**:
  1. User adds package dimensions/weight to their products.
  2. At checkout, OHC pings Shippo to get live carrier rates based on the customer's address.
  3. The customer pays for the order + shipping.
  4. The AI "Manager" immediately calls Shippo to purchase the label and generates a tracking number.
  5. The Tracking number is sent to the customer via the "Ambassador".
  6. The business owner opens the OHC app, taps "Print Label", and the PDF is sent to their AirPrint/Google Print printer.
- **User View**: A clean "Orders to Ship" list. A prominent "Buy & Print Label" button that handles the transaction invisibly.

**Implementation Prompt**:
Integrate the Shippo API to provide end-to-end shipping logistics. Implement a mechanism to fetch and display live, accurate shipping rates during the customer checkout flow based on product weights and destination. Create an "Order Fulfillment" UI in the backend where the business owner can purchase and generate a shipping label with one tap. Ensure the resulting PDF label is easily viewable and printable directly from a mobile device. The system must automatically update the order status to "Shipped" and notify the customer with the tracking link.

**Priority**: P1
**Estimated Scope**: Large

---

# [SMS & Notifications] Global Transactional SMS via Twilio

**Title**: Implement Global Transactional SMS Notifications via Twilio

**Problem Statement**:
Business owners like Fatima (The Food Cart Operator) and her customers are not always looking at their email. When a customer places a pre-order for pickup, Fatima needs an immediate, loud notification on her phone (which may not always have a strong data connection for push notifications). Similarly, her customers expect a text message saying "Your food is ready for pickup!". SMS remains the most reliable, universally understood communication method for urgent transactional updates.

**Research Report**:
I evaluated Twilio, MessageBird, and AWS SNS for SMS delivery.
- **Twilio**: The undisputed industry leader in global SMS delivery. Extremely high reliability, massive global carrier network, and robust handling of international number formatting (E.164) and opt-out compliance (STOP messages). Pricing is straightforward.
- **MessageBird**: Also excellent (and chosen for our Omnichannel Inbox), but Twilio's raw SMS deliverability in certain emerging markets is often cited as slightly superior, and their API specifically for pure transactional SMS is deeply battle-tested. (Note: Using MessageBird for *both* Inbox and SMS is also a valid architectural choice to reduce vendor sprawl, but Twilio is the gold standard for SMS).
- **AWS SNS**: Difficult to configure for two-way communication or handling opt-outs gracefully. Not developer-friendly for this specific use case.
- **Conclusion**: Twilio is the safest choice for guaranteed delivery of critical alerts (like "New Order"), especially in low-data environments.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) and "Customer Success" (The Ambassador) departments.
- **Triggers & Flow**:
  1. A customer places an order on Fatima's OHC site.
  2. The OHC backend immediately dispatches an SMS via Twilio to Fatima's phone: "New order! 2x Falafel. Reply 1 to Accept."
  3. Fatima replies "1" (or clicks a link).
  4. Twilio webhook hits OHC, updating the order status to "Preparing".
  5. The "Ambassador" dispatches an SMS to the customer: "Fatima is preparing your order! We'll text you when it's ready."
- **User View**: Business owner receives standard text messages for critical alerts. Customers receive branded text messages for order updates. A simple toggle in OHC settings: "Send me an SMS for new orders."

**Implementation Prompt**:
Integrate the Twilio SMS API for transactional notifications. Implement a robust notification system that triggers SMS alerts to the business owner for critical events (e.g., new orders, cancellations) based on their notification preferences. Implement an outward-facing SMS flow to update customers on their order status (e.g., "Order Confirmed", "Ready for Pickup"). Ensure all phone numbers are properly validated and formatted to E.164 standard before sending. The system must gracefully handle SMS delivery failures and provide fallback mechanisms (like push notifications or email).

**Priority**: P1
**Estimated Scope**: Medium

---

# [Video Conferencing] Auto-Generated Meeting Links via Zoom API

**Title**: Implement Auto-Generated Video Conferencing Links via Zoom

**Problem Statement**:
Service providers who teach or consult online, like Leo (The Music Tutor), need an effortless way to generate video meeting links. Currently, when a student books a lesson, Leo has to manually open Zoom, create a meeting, copy the link, and email it to the student. This manual work limits his ability to scale his tutoring business. He needs the video link to be generated automatically the second a student books and pays for a slot.

**Research Report**:
I evaluated Zoom API, Google Meet API, and Daily.co.
- **Zoom API**: Universal brand recognition. Almost every student already has the Zoom app installed. The API for generating Server-to-Server (or OAuth-based) meeting links is very robust.
- **Google Meet API**: Requires tight coupling with Google Workspace, which can be restrictive for users who prefer Apple or Outlook calendars but still want standard video calls.
- **Daily.co**: Excellent for embedding video directly *inside* the OHC app (White-labeled WebRTC). However, most users (and their clients) strongly prefer the familiarity of native Zoom or Meet apps for 60-minute tutoring sessions.
- **Conclusion**: Zoom is the most expected and frictionless platform for end-users. Integrating the Zoom API allows OHC to automatically attach a unique video link to calendar events.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department, acting as an extension of the Calendar & Scheduling module.
- **Triggers & Flow**:
  1. The business owner links their Zoom account via an OAuth flow in OHC Settings.
  2. The owner creates a Service (e.g., "1 Hour Guitar Lesson") and toggles "Online Meeting".
  3. A student books the service via the OHC scheduling page.
  4. The OHC backend calls the Zoom API to generate a unique meeting ID and password.
  5. The Zoom link is injected into the Calendar invite (via Cronofy) and sent in the confirmation email (via Resend).
- **User View**: A simple toggle when creating a service: "Generate Zoom link automatically". A button on their upcoming appointments list: "Join Video Call".

**Implementation Prompt**:
Integrate the Zoom API to automatically generate unique video conferencing links for scheduled online services. Build an OAuth flow allowing the business owner to connect their Zoom account. Modify the service creation UI to include a toggle for "Online Meeting". When a customer successfully books an online service, the system must call the Zoom API to create a meeting, store the join link in the database, and automatically include it in the customer's confirmation email and calendar invite. Ensure the integration handles token refreshes securely and gracefully degrades if the API is temporarily unavailable.

**Priority**: P2
**Estimated Scope**: Medium
