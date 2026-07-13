# Tool Integration Research Report (Q3)
> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.

## 1. Social Media Integration: Chatwoot & Meta Graph API

### Title: Integrate Unified Social Media Inbox via Chatwoot
**Problem Statement:** Business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook comments, and WhatsApp. Checking multiple apps is overwhelming, and she often misses messages or forgets to reply, losing potential sales. She needs one simple inbox to see and reply to everything.

**Research Report:**
- **Tool:** Chatwoot (Open Source / Cloud) + Meta Graph API.
- **Findings:** Chatwoot is already supported in the OHC architecture but needs deep integration into the OHC Unified Inbox. It provides a reliable way to aggregate WhatsApp, Instagram, Facebook, and Twitter.
- **OAuth Complexity:** Meta requires business verification and complex OAuth scopes (`instagram_manage_messages`, `pages_messaging`), which we can simplify via an embedded signup flow.
- **Message Parsing Quality:** High. Native support for images, videos, and quick replies.
- **Webhook Reliability:** Meta webhooks are robust but require strict response times (<20s). Chatwoot handles this well.
- **Pricing:** Chatwoot Cloud is ~$19/mo per agent. OHC could self-host for Standalone mode, or use Cloud for multi-tenant SaaS. Meta APIs are mostly free for inbound service messages.
- **Cloud vs Standalone:** Works in both. Self-hosted Chatwoot can run locally in Standalone; SaaS can connect via API.

**Design Doc:**
- **Trigger:** A customer sends a DM on Instagram.
- **Action:** Chatwoot receives the webhook and routes the message to the "Customer Success" AI agent. If the AI cannot handle it, it alerts the business owner via the OHC mobile app push notification.
- **User Interface:** A "Unified Inbox" tab in the OHC app. The business owner sees a chat interface with a small Instagram or WhatsApp icon indicating the source. They type a reply normally, and it sends back to the original platform.

**Implementation Prompt:**
Create a Unified Inbox UI in the OHC app and connect it to the Chatwoot API. A business owner should be able to click "Connect Instagram", authenticate with Meta, and immediately see new DMs appear in the OHC inbox. Replies sent from OHC must successfully appear in the customer's Instagram app.
**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling: Cal.com API

### Title: Enable Automated Booking with Cal.com
**Problem Statement:** Service providers like Carlos (Handyman) and Leo (Music Tutor) waste time going back and forth with clients to find a time that works. They need a simple link where clients can see availability, book a slot, and get a calendar invite automatically.

**Research Report:**
- **Tool:** Cal.com (Open Source scheduling infrastructure).
- **Findings:** Cal.com offers a robust API for white-labeled scheduling. It handles timezone math, calendar conflict resolution (syncing with Google/Outlook), and automated reminders.
- **Ease of Use for Non-Technical Users:** Excellent, as we will white-label it. The user just connects their Google Calendar once.
- **Calendar Conflict Resolution:** Best in class. It checks connected calendars in real-time before offering slots.
- **Pricing:** Cal.com API has volume-based pricing or can be self-hosted. Free tier is generous.
- **Cloud vs Standalone:** Works in both. Can be embedded via iframe or API in the cloud, and self-hosted via Docker for standalone.

**Design Doc:**
- **Trigger:** A customer visits Carlos's OHC website and clicks "Book a Repair."
- **Action:** Cal.com API serves available time slots. When booked, an event is added to Carlos's Google Calendar and the customer receives an email.
- **User Interface:** Carlos sees a "Booking Link" in his dashboard that he can copy or add to his site. He can set his working hours (e.g., 9 AM - 5 PM).

**Implementation Prompt:**
Integrate the Cal.com API to provide a seamless scheduling experience. Add a "Working Hours" setting for business owners. Generate a public booking widget on the business owner's OHC storefront. Ensure double-bookings are prevented by syncing with the owner's connected external calendar.
**Priority:** P0
**Estimated Scope:** Medium

---

## 3. Email Marketing: Resend

### Title: Simple AI-Driven Email Campaigns with Resend
**Problem Statement:** Boutique owners like Priya want to notify their customers when new stock arrives, but tools like Mailchimp are too complex and expensive. She needs a way to send a beautiful email to her past customers with one click.

**Research Report:**
- **Tool:** Resend.
- **Findings:** Resend is a developer-first email API that focuses on deliverability and clean templates (using React Email).
- **Ease of List Management:** Handled by OHC's internal PostgreSQL database; Resend simply sends the batches.
- **Template Quality:** Extremely high. We can pre-build beautiful templates using React Email.
- **Spam Compliance:** Handles unsubscribe links and suppression lists automatically.
- **Pricing:** Very affordable. First 3,000 emails/month free, then $20 for 50,000.
- **Cloud vs Standalone:** Cloud-only (requires an API key and verified domain). Standalone users would need their own API key.

**Design Doc:**
- **Trigger:** Priya asks the "Marketing Agent" to announce the new summer dress collection.
- **Action:** The AI drafts the email and shows Priya a preview. Upon approval, the backend iterates through Priya's customer list and sends the emails via the Resend API.
- **User Interface:** A simple "Broadcast" button in the Marketing tab. The user sees a visual preview of the email and a "Send to X customers" button.

**Implementation Prompt:**
Implement a broadcast email feature using the Resend API. Allow the AI "Marketing Agent" to generate an email draft with an image and text. Provide a mobile-friendly preview for the business owner, and a "Send" button that dispatches the email to all opted-in customers.
**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing: Mercado Pago & Razorpay

### Title: Expand Global Payments with Mercado Pago & Razorpay
**Problem Statement:** Stripe is not dominant in all regions. A seller in Latin America needs Mercado Pago, and a seller in India needs Razorpay (UPI). Without these, OHC cannot serve global users effectively.

**Research Report:**
- **Tool:** Mercado Pago (LATAM) and Razorpay (India).
- **Findings:** Both offer robust APIs similar to Stripe. Razorpay is essential for UPI in India. Mercado Pago handles local card networks and Pix (Brazil).
- **Settlement Speed:** 1-3 days typically.
- **Currency Support:** Excellent local currency support (BRL, ARS, INR).
- **Pricing:** Standard payment gateway fees (2-3% per transaction).
- **Cloud vs Standalone:** Both. Business owners bring their own API keys via an OAuth connect flow.

**Design Doc:**
- **Trigger:** A customer in Brazil checks out on a storefront.
- **Action:** OHC detects the region and offers Mercado Pago (Pix/Card) instead of Stripe.
- **User Interface:** A "Payment Providers" settings page where the owner can toggle and connect Stripe, Mercado Pago, or Razorpay based on their country.

**Implementation Prompt:**
Add a plugin system for payment providers. Implement Mercado Pago and Razorpay as alternatives to Stripe for the checkout flow. The business owner should be able to authenticate with their preferred provider, and the storefront checkout should dynamically display the correct payment elements.
**Priority:** P1
**Estimated Scope:** Large

---

## 5. Shipping & Logistics: EasyPost

### Title: Automated Shipping Labels and Tracking via EasyPost
**Problem Statement:** Sellers of physical goods waste time manually copying addresses into the post office website to buy shipping labels. They need to print a label directly from the OHC app with one tap.

**Research Report:**
- **Tool:** EasyPost.
- **Findings:** EasyPost aggregates 100+ carriers (USPS, UPS, FedEx, DHL, Royal Mail).
- **Carrier Coverage:** Global.
- **API Reliability:** Excellent, 99.99% uptime.
- **Pricing:** 120,000 shipments free per year, then 1¢ per label.
- **Cloud vs Standalone:** Both. OHC can use a master account for Cloud billing, or let Standalone users enter their own EasyPost key.

**Design Doc:**
- **Trigger:** A customer places an order for a physical product.
- **Action:** EasyPost API calculates the shipping rate during checkout. When the owner fulfills the order, EasyPost generates a printable PDF label and a tracking number.
- **User Interface:** In the "Orders" tab, the owner taps an order and sees a "Buy Shipping Label" button. After confirming the box size, a PDF pops up ready to print. The tracking number is automatically emailed to the customer.

**Implementation Prompt:**
Integrate the EasyPost API for real-time shipping rates and label generation. Add a flow to the Orders UI where the business owner can input package dimensions, purchase a label, and generate a printable PDF. Automatically update the order status to "Shipped" and notify the customer with the tracking link.
**Priority:** P1
**Estimated Scope:** Large

---

## 6. SMS & Notifications: Twilio

### Title: Reliable SMS Notifications for Customers & Owners
**Problem Statement:** Fatima (Food Cart Operator) needs a loud SMS ping on her phone when an order arrives because she's busy cooking and might miss an app notification. Customers also expect SMS updates when their food is ready.

**Research Report:**
- **Tool:** Twilio.
- **Findings:** The industry standard for programmatic SMS.
- **Global Carrier Coverage:** Near 100%.
- **Delivery Reliability:** Very high.
- **Opt-out Compliance:** Handles STOP replies automatically.
- **Pricing:** ~$0.0079 per message in the US.
- **Cloud vs Standalone:** Both.

**Design Doc:**
- **Trigger:** A food order is marked "Ready for Pickup" by Fatima.
- **Action:** Twilio API sends an SMS to the customer: "Your order from Fatima's Cart is ready for pickup!"
- **User Interface:** Fatima has a toggle in Settings: "Notify me via SMS for new orders." Customers have a checkbox at checkout: "Text me order updates."

**Implementation Prompt:**
Implement an SMS notification service using Twilio. Provide settings for the business owner to receive SMS alerts for new orders. Allow customers to opt-in to SMS updates at checkout. Send automated SMS messages when order status changes to "Ready".
**Priority:** P2
**Estimated Scope:** Small

---

## 7. Video Conferencing: Daily.co

### Title: Frictionless Video Lessons with Daily.co
**Problem Statement:** Leo (Music Tutor) currently has to manually create a Zoom link, email it to the student, and remind them. He wants the link to just "be there" when the student books a lesson.

**Research Report:**
- **Tool:** Daily.co.
- **Findings:** A WebRTC video API that allows embedding video calls directly into the browser or app, without requiring the student to download Zoom.
- **Link Generation Speed:** Instant via API.
- **Join Experience:** Frictionless. The student clicks a link and joins directly in their browser.
- **Pricing:** 10,000 audio/video minutes free per month, then $0.004/min.
- **Cloud vs Standalone:** Both.

**Design Doc:**
- **Trigger:** A student books a 1-hour guitar lesson.
- **Action:** Daily.co API creates a unique video room URL. This URL is saved to the booking record and sent in the calendar invite.
- **User Interface:** In the OHC app, Leo sees his upcoming schedule. At the time of the lesson, he taps "Join Call," which opens the Daily.co room directly inside the OHC interface or browser.

**Implementation Prompt:**
Integrate the Daily.co API to auto-generate video meeting links for bookings. Embed the Daily.co pre-built UI into the OHC web and mobile client so the business owner and customer can join the video call without leaving the OHC platform.
**Priority:** P2
**Estimated Scope:** Medium
