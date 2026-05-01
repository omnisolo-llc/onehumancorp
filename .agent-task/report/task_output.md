# Scout: Tool Integration Research Q3

## Overview
This report contains evaluations and structured issue briefs for integrating third-party tools into the OneHumanCorp (OHC) platform. The focus is on solving real problems for our non-technical small business owner personas across various categories, ensuring seamless functionality in both Cloud and Standalone environments.

---

## 1. Social Media Integration: Meta Graph API (Unified Inbox)

### Title
Integrate Meta Graph API for Unified Customer Inbox (Instagram, FB Messenger, WhatsApp)

### Problem Statement
**Persona:** Maya (The Home Baker)
Maya receives orders and inquiries through Instagram DMs, Facebook comments, and WhatsApp. She frequently misses messages or forgets to reply because she has to constantly switch between apps. She needs a single, simple inbox where her AI "Ambassador" agent can draft replies, and where she can view and manage all customer communications in one place.

### Research Report
- **Evaluated Tools:** Meta Graph API (direct) vs. Twilio / MessageBird.
- **Ease of Use:** Direct Meta integration allows users to connect their accounts via standard OAuth ("Log in with Facebook/Instagram"), which is familiar and requires zero technical knowledge. Aggregators like Twilio require setting up complex phone numbers or specialized API keys, which fails our non-technical requirement.
- **Pricing:** Meta Graph API is free for standard messaging. WhatsApp Business API has per-conversation pricing, but the first 1,000 service conversations per month are free, covering most small businesses.
- **Reputation:** Meta APIs are standard and reliable.

### Design Doc
- **User Experience:** The user navigates to "Settings > Connect Accounts" and clicks "Connect Instagram/Facebook". A standard Meta login popup appears. Once authorized, all incoming messages appear in the OHC "Customer Inbox" tab.
- **AI Integration:** The Customer Success agent ("The Ambassador") monitors the inbox. When a message arrives, it drafts a suggested reply based on business context (e.g., pricing, hours) and presents it to the user with a "Send" button.
- **Data Flow:** Webhooks receive incoming messages. The payload is parsed and stored in the unified inbox. The AI is triggered to generate a draft.

### Implementation Prompt
Implement a unified inbox feature that allows users to connect their Meta accounts (Instagram, Facebook Messenger, WhatsApp) via OAuth. Ensure incoming messages appear in a single, mobile-optimized view. The system should support incoming text and images, and allow the user to reply directly from the OHC app. Include a feature where the AI automatically drafts suggested replies for incoming messages.
- **Acceptance Criteria:** User can connect an account via OAuth. Messages from IG/FB appear in the app within 5 seconds. User can reply successfully. AI drafts are visible for incoming messages.

### Priority
P0 (Critical)

### Estimated Scope
Large

### Cloud vs. Standalone
- **Cloud:** Webhooks configured to point to OHC cloud endpoints.
- **Standalone:** Requires a tunneling service (like ngrok or a dedicated proxy) or polling mechanism to receive webhooks locally, or an OHC cloud relay.

---

## 2. Calendar & Scheduling: Cal.com Integration

### Title
Integrate Cal.com API for Automated Booking & Scheduling

### Problem Statement
**Persona:** Leo (The Music Tutor) & Carlos (The Freelance Handyman)
Leo and Carlos spend hours going back and forth with clients trying to find a time to meet or provide a service. They need a simple link they can share on TikTok or their website where customers can view their availability, pick a time, and pay a deposit instantly without manual intervention.

### Research Report
- **Evaluated Tools:** Cal.com vs. Calendly vs. Nylas.
- **Ease of Use:** Cal.com provides an open API and white-label options. The user doesn't even need to know Cal.com is powering it; they just set their availability in OHC.
- **Pricing:** Cal.com has an excellent free tier for individuals and scalable pricing for platforms. Open-source nature allows for self-hosting.
- **Reputation:** Highly respected in the developer community, fast, and reliable.

### Design Doc
- **User Experience:** Under the "Bookings" tab, the user sets their working hours (e.g., 9 AM - 5 PM) and connects their personal Google or Outlook calendar to prevent double-booking. A public booking page is automatically generated.
- **Integration:** OHC uses Cal.com's platform API to provision scheduling endpoints for the user transparently.
- **AI Integration:** The Operations agent ("The Manager") sends booking reminders and manages rescheduling requests.

### Implementation Prompt
Implement a scheduling system powered by Cal.com under the hood. Create an interface where users can define their availability (working hours, days off). Generate a mobile-friendly public booking flow for their storefront. Ensure that when a booking is made, it syncs with the user's connected personal calendar (Google/Outlook) and automatically triggers a confirmation notification.
- **Acceptance Criteria:** User can set availability. Public users can book a slot. Confirmation is received. Syncs with external calendar.

### Priority
P0 (Critical)

### Estimated Scope
Medium

### Cloud vs. Standalone
- **Cloud:** Uses Cal.com SaaS or self-hosted enterprise cluster.
- **Standalone:** Cal.com is open-source and can be bundled or run alongside the standalone OHC instance.

---

## 3. Email Marketing: Resend Integration

### Title
Integrate Resend for Automated Customer Email Campaigns

### Problem Statement
**Persona:** Priya (The Boutique Owner)
Priya wants to send beautiful email announcements when she gets new seasonal clothing in stock. She currently uses a separate tool (Mailchimp), which requires her to manually export and import her customer lists. She needs a way to send promotional emails directly to her OHC customer list with zero configuration.

### Research Report
- **Evaluated Tools:** Resend vs. Mailchimp vs. SendGrid.
- **Ease of Use:** Resend is exceptionally developer-friendly and handles domain authentication elegantly. For the end-user, the complexity of SMTP and DNS records must be hidden or heavily abstracted by OHC.
- **Pricing:** Generous free tier (3,000 emails/month), then extremely affordable per-email pricing.
- **Reputation:** Modern, fast, and known for great developer experience and deliverability.

### Design Doc
- **User Experience:** The user navigates to "Marketing > Email Campaigns", types a simple message, and attaches a photo. The AI ("The Promoter") formats it into a beautiful, mobile-responsive email template. The user selects "Send to all past customers" and clicks Send. No DNS configuration required for default OHC subdomain sending (e.g., priya.shops.onehumancorp.com).
- **AI Integration:** "The Promoter" drafts subject lines, optimizes email body copy, and suggests the best time to send.

### Implementation Prompt
Build a simple email campaign tool that integrates with the existing customer directory. The user should be able to draft a plain-text/image message, which the AI automatically transforms into a styled HTML email using OHC's design system. Integrate with Resend API to handle the actual delivery. Provide basic analytics (open rates, click rates) in plain language.
- **Acceptance Criteria:** User can draft and send a broadcast email to their customer list. Emails are styled beautifully. Delivery is successful. Analytics are captured.

### Priority
P1 (High)

### Estimated Scope
Medium

### Cloud vs. Standalone
- **Cloud:** Handled via OHC's central Resend account.
- **Standalone:** Users will need to provide their own Resend API key or configure a custom SMTP server.

---

## 4. Payment Processing: Mercado Pago & Regional Providers

### Title
Expand Payment Support with Regional Gateways (e.g., Mercado Pago for LATAM)

### Problem Statement
**Persona:** Global Users
While Stripe is fantastic, it is not available in every country or is not the preferred local payment method (e.g., PIX in Brazil via Mercado Pago). Users in unsupported regions cannot accept online payments, entirely blocking them from using OHC for e-commerce.

### Research Report
- **Evaluated Tools:** Mercado Pago (LATAM), Razorpay (India), Paystack (Africa).
- **Ease of Use:** These providers offer standard OAuth or simple API key integrations tailored to their specific markets.
- **Pricing:** Standard payment gateway fees (typically 2-3% + fixed fee per transaction).
- **Reputation:** Dominant and highly trusted in their respective regional markets.

### Design Doc
- **User Experience:** During onboarding, if a user is in a non-Stripe region (e.g., Brazil, India), OHC suggests the leading local provider. The user clicks "Connect Mercado Pago", completes the provider's standard auth flow, and can immediately start accepting local payment methods like PIX.
- **Integration:** Implement a unified `PaymentProvider` interface in the backend so the core checkout flow remains unchanged regardless of the underlying gateway.

### Implementation Prompt
Implement a modular payment provider system and integrate Mercado Pago as the first regional alternative to Stripe. Ensure the checkout experience supports local payment methods (e.g., PIX, Boleto) seamlessly. The business owner should simply connect their account via OAuth and immediately be able to accept payments.
- **Acceptance Criteria:** User in LATAM can connect Mercado Pago. Customer can complete checkout using a local payment method. Order is marked as paid in OHC.

### Priority
P1 (High)

### Estimated Scope
Large

### Cloud vs. Standalone
- **Cloud:** Webhooks received centrally and routed to tenants.
- **Standalone:** Direct API key configuration and local webhook endpoints (requiring tunneling).

---

## 5. Shipping & Logistics: Shippo Integration

### Title
Integrate Shippo for Automated Label Generation and Tracking

### Problem Statement
**Persona:** Priya (The Boutique Owner)
Priya ships clothing across the country. Currently, she has to manually copy customer addresses from OHC into a shipping provider's website to print labels, which is error-prone and slow. She needs to print a shipping label with one click directly from the OHC order screen.

### Research Report
- **Evaluated Tools:** Shippo vs. EasyPost.
- **Ease of Use:** Shippo is slightly more user-friendly for non-technical users if they need to log into the provider dashboard, though ideally, they never leave OHC.
- **Pricing:** Pay-as-you-go per label (cents per label) plus postage costs. Deep discounts on standard carrier rates (USPS, UPS, DHL).
- **Reputation:** Reliable, widespread carrier support, good API.

### Design Doc
- **User Experience:** When viewing a paid physical order, the user sees a "Buy Shipping Label" button. OHC automatically calculates the cheapest rate based on standard package dimensions, charges the user's card on file, and displays a printable PDF label.
- **AI Integration:** "The Ambassador" automatically detects when the package is shipped/delivered and texts or emails the customer.

### Implementation Prompt
Integrate the Shippo API to allow users to generate and purchase shipping labels directly from an order details page. Auto-populate sender and recipient addresses. Provide a one-click "Print Label" action that downloads a PDF. Ensure tracking numbers are automatically saved to the order and shared with the customer.
- **Acceptance Criteria:** User can view carrier rates for an order. User can purchase a label. PDF is generated. Tracking number is attached to the order.

### Priority
P1 (High)

### Estimated Scope
Medium

### Cloud vs. Standalone
- **Cloud:** Platform-level Shippo integration with connected user accounts.
- **Standalone:** Users provide their own Shippo API key.

---

## 6. SMS & Notifications: Twilio Integration

### Title
Integrate Twilio for Critical SMS Notifications

### Problem Statement
**Persona:** Fatima (The Food Cart Operator)
Fatima operates in a fast-paced environment and doesn't always check her app. When a customer places a pre-order for pickup, she needs an immediate, loud text message notification. Similarly, her customers want a text when their food is ready, not an email.

### Research Report
- **Evaluated Tools:** Twilio vs. MessageBird vs. Vonage.
- **Ease of Use:** Twilio has the most robust API and global coverage. For the OHC user, they do nothing—it just works.
- **Pricing:** Fractions of a cent per message. OHC can absorb this cost on paid tiers or pass it through via usage-based billing.
- **Reputation:** The industry standard for SMS.

### Design Doc
- **User Experience:** Under "Settings > Notifications", the user toggles "Send me a text when I get a new order" and enters their phone number. For customers, at checkout, they enter their phone number and receive a text when their status changes to "Ready for Pickup".
- **AI Integration:** "The Ambassador" handles conversational SMS if the customer replies (e.g., "I'm running 5 mins late").

### Implementation Prompt
Implement Twilio SMS to support critical transactional alerts. Add functionality for the business owner to receive SMS alerts for new orders. Add functionality to send customers SMS updates regarding their order status (e.g., "Your food is ready!"). Ensure strict opt-out (STOP) compliance is handled automatically.
- **Acceptance Criteria:** Owner receives SMS on new order. Customer receives SMS on status update. Opt-out logic functions correctly.

### Priority
P1 (High)

### Estimated Scope
Small

### Cloud vs. Standalone
- **Cloud:** Utilizes OHC's central Twilio account.
- **Standalone:** Requires the user to supply a Twilio Account SID and Auth Token.

---

## 7. Video Conferencing: Daily.co / Zoom API

### Title
Integrate Daily.co for Frictionless Online Consultations and Lessons

### Problem Statement
**Persona:** Leo (The Music Tutor)
Leo teaches guitar online. Sending Zoom links manually is tedious, and sometimes students lose the link. He needs the video call link to be automatically generated upon booking and embedded directly in the student's calendar invite and reminder emails.

### Research Report
- **Evaluated Tools:** Daily.co vs. Zoom API.
- **Ease of Use:** Daily.co allows instant room creation via API with zero user setup (no "Connect your Zoom account" required). The user and student just click a link and join via the browser. Zoom is more recognizable but requires OAuth friction.
- **Pricing:** Daily.co offers 10,000 free minutes per month.
- **Reputation:** Excellent WebRTC quality and developer experience.

### Design Doc
- **User Experience:** When setting up a service, Leo selects "Online Location". When a student books, OHC generates a unique Daily.co room link. Both Leo and the student receive this link. They click it at the scheduled time and the video call opens instantly in their browser—no app downloads required.
- **Integration:** Simple POST request to Daily.co API to provision a room valid only for the duration of the meeting.

### Implementation Prompt
Integrate Daily.co API to auto-generate video conferencing rooms for "Online" bookings. Ensure the generated link is automatically included in the calendar invite, confirmation email, and the business owner's upcoming schedule view.
- **Acceptance Criteria:** Booking an online service generates a unique video link. Both parties can join the call via browser at the scheduled time. Link expires after the meeting.

### Priority
P2 (Medium)

### Estimated Scope
Medium

### Cloud vs. Standalone
- **Cloud:** OHC platform account.
- **Standalone:** User provides Daily.co API key or falls back to requiring manual link entry.
