# OHC Tool Integration Research Report

## 1. Social Media Integration: Unified Inbox for Meta (Instagram/Facebook) & WhatsApp

**Title:** Integrate Meta Graph APIs for Unified Social Media Inbox
**Problem Statement:** Small business owners waste hours switching between the Instagram app, Facebook page manager, and WhatsApp to reply to customer questions and comments. It's easy to miss a message and lose a sale. They need a single place inside OHC to see and reply to all social media messages.
**Research Report:**
- **Overview:** Meta offers APIs to manage messages across Facebook, Instagram, and WhatsApp. Tools like ManyChat or direct Meta APIs can be used.
- **Key advantages and risks:** Advantages include capturing leads directly where customers are and improving response times. Risks include Meta's complex OAuth approval processes and frequent API changes.
- **Rough pricing estimate:** Meta APIs are generally free, but WhatsApp Business API charges per conversation (approx $0.01 - $0.08 depending on country and type).
- **Whether it works in both Cloud and Standalone modes:** Yes. In Cloud mode, we can use a centralized webhook. In Standalone mode, users may need to provide their own Meta Developer credentials or we can route through an OHC proxy.
**Design Doc:** OHC will add a "Social Inbox" tab. Users will click a "Connect Meta" button to authenticate. Once connected, new DMs and comments will appear as unread messages in the OHC inbox. Replying in OHC will send the message back to the customer on their original platform.
**Implementation Prompt:** Implement a unified inbox view that supports reading and replying to Instagram DMs, Facebook messages, and WhatsApp messages. Acceptance criteria: A user can authenticate with Meta, receive real-time messages in OHC, and reply successfully without leaving the OHC interface.
**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling: Google Calendar Two-Way Sync

**Title:** Google Calendar Integration for Automated Scheduling
**Problem Statement:** Service-based business owners currently use a paper calendar or manually copy appointments from OHC into their personal Google Calendar. Double bookings happen often when they forget to block out time for personal events. They need OHC to know when they are busy and automatically add new bookings to their phone's calendar.
**Research Report:**
- **Overview:** Google Calendar API allows reading busy slots and writing new events.
- **Key advantages and risks:** Advantages include eliminating double-bookings and keeping the business owner organized automatically. Risks include timezone mismatch issues and handling recurring events correctly.
- **Rough pricing estimate:** Free for standard usage limits.
- **Whether it works in both Cloud and Standalone modes:** Yes. Works natively in both if the OAuth flow is configured to allow local redirect URIs for Standalone.
**Design Doc:** A "Calendar Sync" settings page where the user logs into Google. OHC will read the user's "busy" blocks from Google to prevent customers from booking during those times. When a customer books a service via OHC, an event is immediately created on the user's Google Calendar.
**Implementation Prompt:** Build a two-way Google Calendar synchronization feature. Acceptance criteria: A user can connect their Google account; OHC availability reflects Google Calendar busy times; new OHC bookings appear on Google Calendar; cancelling a booking in OHC removes it from Google Calendar.
**Priority:** P1
**Estimated Scope:** Medium

---

## 3. Email Marketing: Mailchimp Integration for Customer Campaigns

**Title:** Connect OHC Customer List to Mailchimp
**Problem Statement:** Business owners want to send newsletters or promotional discounts to their customers, but currently have to manually export their OHC customer list and import it into an email tool every time. They need their OHC customer list to automatically sync with an email marketing tool.
**Research Report:**
- **Overview:** Mailchimp is the industry standard for small business email marketing, offering robust list management APIs.
- **Key advantages and risks:** Advantages include automated audience building and professional email templates. Risks include managing subscribe/unsubscribe states cleanly between OHC and Mailchimp to comply with spam laws.
- **Rough pricing estimate:** Free up to 500 contacts, then starts at ~$13/month.
- **Whether it works in both Cloud and Standalone modes:** Yes. Uses standard REST API keys which the user can easily input in Standalone mode.
**Design Doc:** A "Marketing" tab where the user inputs their Mailchimp API key. OHC will automatically push any new customer who checks out (and opts in) to a designated Mailchimp audience. It will also tag them with "OHC Customer".
**Implementation Prompt:** Create an integration that syncs OHC contacts to Mailchimp. Acceptance criteria: Users can input a Mailchimp API key; new customers created in OHC are automatically added to the Mailchimp audience; OHC respects email marketing opt-in preferences.
**Priority:** P2
**Estimated Scope:** Medium

---

## 4. Payment Processing: Mercado Pago Integration for LATAM

**Title:** Integrate Mercado Pago for Latin American Markets
**Problem Statement:** Stripe is not accessible or affordable for many small businesses in Latin America. Business owners in countries like Brazil, Mexico, and Argentina need a local, trusted payment processor that accepts local credit cards and payment methods like PIX to get paid online.
**Research Report:**
- **Overview:** Mercado Pago is the dominant payment gateway in LATAM, supporting local payment methods.
- **Key advantages and risks:** Advantages include unlocking huge markets in LATAM and supporting local payment habits. Risks include complex webhook handling for asynchronous payments (like PIX or cash payments).
- **Rough pricing estimate:** Varies by country, typically 3-5% per transaction. No monthly fee.
- **Whether it works in both Cloud and Standalone modes:** Yes.
**Design Doc:** In the "Payments" settings, alongside Stripe, add an option for Mercado Pago. When a LATAM customer checks out, they are redirected to a secure Mercado Pago checkout screen (or modal) to pay using local methods, and redirected back to OHC upon success.
**Implementation Prompt:** Add Mercado Pago as an alternative checkout option. Acceptance criteria: A business owner can configure Mercado Pago credentials; a customer can select Mercado Pago at checkout; OHC correctly captures the successful payment status and marks the order as paid.
**Priority:** P1
**Estimated Scope:** Large

---

## 5. Shipping & Logistics: EasyPost Integration for Automated Labels

**Title:** EasyPost Integration for Automated Shipping Labels
**Problem Statement:** Business owners selling physical goods waste time standing in line at the post office to buy shipping labels, and then have to manually copy tracking numbers back to customers. They need a way to buy and print shipping labels directly inside OHC.
**Research Report:**
- **Overview:** EasyPost aggregates dozens of carriers (USPS, UPS, FedEx) behind a single API.
- **Key advantages and risks:** Advantages include massive time savings for product sellers and automatic tracking updates. Risks include complex address validation and box dimension calculations.
- **Rough pricing estimate:** Developer plan is free for up to 120,000 shipments/year, plus carrier postage costs.
- **Whether it works in both Cloud and Standalone modes:** Yes. Requires an API key which can be securely stored in either mode.
**Design Doc:** On an Order detail page, add a "Buy Shipping Label" button. OHC asks for package weight, fetches real-time rates, and lets the owner purchase the label. The label is presented as a printable PDF, and the tracking number is automatically emailed to the customer.
**Implementation Prompt:** Implement shipping label generation via EasyPost. Acceptance criteria: Business owner can enter package dimensions/weight; view carrier rates; purchase a label; download the PDF label; and the system auto-updates the order with the tracking number.
**Priority:** P1
**Estimated Scope:** Large

---

## 6. SMS & Notifications: Twilio Integration for Order Alerts

**Title:** Twilio SMS Notifications for Order Updates
**Problem Statement:** Many small business customers (especially older demographics or non-English speakers) don't check their emails often. Business owners find that SMS is the only reliable way to confirm appointments or notify customers that an order is ready for pickup.
**Research Report:**
- **Overview:** Twilio is the global standard for programmatic SMS messaging.
- **Key advantages and risks:** Advantages include near 100% open rates and immediate delivery. Risks include strict telecom regulations (A2P 10DLC in the US) and high costs per message if abused.
- **Rough pricing estimate:** ~$0.0079 per message in the US, higher internationally.
- **Whether it works in both Cloud and Standalone modes:** Yes. Can use user-provided Twilio credentials for Standalone mode to offload telecom compliance to the user.
**Design Doc:** A "Notifications" settings page where owners can toggle SMS alerts. When an order is placed, or marked "Ready for Pickup", OHC sends a brief SMS to the customer's phone number.
**Implementation Prompt:** Add SMS order notifications powered by Twilio. Acceptance criteria: Admin can configure Twilio credentials and toggle SMS on/off; customers receive an SMS when an order status changes to 'Ready' or 'Shipped'.
**Priority:** P0
**Estimated Scope:** Medium

---

## 7. Video Conferencing: Zoom Auto-Link Generation

**Title:** Zoom Integration for Auto-Generating Meeting Links
**Problem Statement:** Tutors, consultants, and therapists using OHC have to manually create a Zoom meeting and email the link to the customer every time they get a new booking. They need OHC to automatically generate a unique video link the moment a customer books a session.
**Research Report:**
- **Overview:** Zoom API allows for creating meetings and retrieving join URLs programmatically.
- **Key advantages and risks:** Advantages include a seamless, professional experience for virtual services. Risks include managing Zoom's strict OAuth app approval process and token refresh lifecycles.
- **Rough pricing estimate:** API access is included in Zoom Pro accounts ($14.99/mo).
- **Whether it works in both Cloud and Standalone modes:** Yes. Standalone mode might require Server-to-Server OAuth or standard user OAuth.
**Design Doc:** In the Service configuration, owners can mark a service as "Virtual Meeting". When this service is booked, OHC calls Zoom, creates a meeting for the scheduled time, and includes the unique Zoom Join URL directly in the customer's confirmation email and the OHC order details.
**Implementation Prompt:** Build an integration to auto-generate Zoom links for bookings. Acceptance criteria: A user can authenticate their Zoom account; booking a "Virtual" service automatically creates a Zoom meeting; the Zoom link is displayed to both the business owner and the customer.
**Priority:** P2
**Estimated Scope:** Medium
