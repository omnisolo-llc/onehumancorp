# Scout: Tool Integration Research

## [Social Media Integration] Unified Inbox for Instagram & WhatsApp

**Title:** Integrate Meta API for Unified Instagram & WhatsApp Inbox

**Problem Statement:**
As a small business owner, I am overwhelmed by checking messages across Instagram DMs, Facebook, and WhatsApp. I often miss customer inquiries or take too long to reply, losing potential sales. I need one single place inside my dashboard where I can see and reply to all customer messages from these platforms without switching apps constantly.

**Research Report:**
- **Findings:** Meta offers Graph API for Instagram Messaging and WhatsApp Business API. Competitors like Hootsuite and Sprout Social charge high premiums ($99+/mo) for this feature.
- **Ease of Use:** Non-technical users struggle with Meta's complex OAuth and Business Manager setup. A streamlined "Log in with Meta" flow is required to make this accessible.
- **Pricing:** WhatsApp Business charges per conversation (varies by country, usually a few cents). Instagram messaging is currently free within standard rate limits.
- **Reputation:** Meta API is the industry standard but is known for strict policy enforcement and occasional platform instability.
- **Environment:** Works in both Cloud and Standalone modes via OAuth and webhooks (Cloud) or direct polling/local webhooks if tunneled (Standalone).

**Design Doc:**
- **Trigger:** A user clicks "Connect Meta" in the OHC Integration settings.
- **Action:** They are redirected to a Meta login screen to grant permissions. Once connected, incoming messages from connected Instagram and WhatsApp accounts are routed to the OHC internal messaging inbox.
- **User Experience:** The business owner sees a unified "Inbox" tab. When a new message arrives on WhatsApp or Instagram, a notification appears. The business owner can type a reply in OHC, which is sent back to the customer's original platform.

**Implementation Prompt:**
Create a unified inbox feature that allows users to connect their Instagram and WhatsApp accounts. The outcome should be a seamless authentication flow and a unified chat interface where business owners can read and reply to messages from both platforms. Acceptance criteria: user can connect their Meta account in 3 clicks, incoming messages appear in OHC within 5 seconds, and replies from OHC are successfully delivered to the customer on their chosen platform.

**Priority:** P0
**Estimated Scope:** Large

---

## [Calendar & Scheduling] Seamless Booking Page Sync

**Title:** Integrate Cal.com / Google Calendar for Automated Scheduling

**Problem Statement:**
I spend too much time going back and forth over email to find a time to meet with clients. I need a simple booking link I can send them, which automatically syncs with my existing Google Calendar so I never get double-booked, and automatically creates a meeting event.

**Research Report:**
- **Findings:** Cal.com provides an open-source, API-first scheduling infrastructure. Google Calendar API is ubiquitous but complex to manage for timezone and conflict resolution.
- **Ease of Use:** Cal.com abstractions make it easier to embed booking pages directly into the OHC dashboard.
- **Pricing:** Cal.com has generous free tiers and developer API pricing. Google Calendar API is free for standard usage.
- **Reputation:** Cal.com is highly regarded in the developer community for its flexibility and reliability.
- **Environment:** Fully compatible with both Cloud and Standalone modes.

**Design Doc:**
- **Trigger:** The business owner sets their availability hours in OHC and connects their Google Calendar.
- **Action:** OHC generates a unique booking link. When a customer books a time, it blocks that time on the owner's Google Calendar and sends a confirmation email to both parties.
- **User Experience:** The owner sees a "Meetings" tab with their upcoming schedule. They can share a simple link (`ohc.com/book/mybusiness`) with clients.

**Implementation Prompt:**
Implement a scheduling system where users can define their availability and connect their external calendar (e.g., Google Calendar) to prevent double-booking. Create a public-facing booking page for customers. Acceptance criteria: users can generate a booking link, customers can book available slots, the booked slot immediately reflects on the owner's connected calendar, and both parties receive a confirmation notification.

**Priority:** P1
**Estimated Scope:** Medium

---

## [Email Marketing] Automated Customer Retention Campaigns

**Title:** Integrate Brevo for Simplified Email Marketing

**Problem Statement:**
I have a list of customer emails, but sending newsletters or promotional offers manually is tedious and gets my email marked as spam. I need an easy way to send professional-looking updates and promotions to my customer base directly from my dashboard.

**Research Report:**
- **Findings:** Brevo (formerly Sendinblue) and Mailchimp are top contenders. Brevo offers better transactional and marketing email APIs with generous free tiers (300 emails/day).
- **Ease of Use:** Both require some domain authentication (DKIM/SPF) which is hard for non-technical users. OHC should handle this or provide very simple, copy-paste DNS instructions.
- **Pricing:** Brevo is free up to 300 emails/day, then scalable monthly plans.
- **Reputation:** Brevo is known for good deliverability and developer-friendly APIs.
- **Environment:** Cloud (API based). Standalone can also make outbound API calls to Brevo.

**Design Doc:**
- **Trigger:** The business owner creates a new "Campaign" in OHC, selecting a segment of their customer list.
- **Action:** OHC packages the email content and recipient list, sending it to the Brevo API for delivery.
- **User Experience:** The owner uses a simple text editor to write a newsletter. They hit "Send to all past customers." They can later see how many people opened the email in the OHC dashboard.

**Implementation Prompt:**
Build an email campaign feature that integrates with a provider like Brevo. The user should be able to write an email, select a group of existing contacts, and send a bulk email campaign. Acceptance criteria: users can compose an email, send it to a selected list of saved contacts, and view basic analytics (sent, opened, bounced) within the OHC platform.

**Priority:** P2
**Estimated Scope:** Medium

---

## [Payment Processing] Global Alternative Payment Integration

**Title:** Integrate Mercado Pago / Stripe for Flexible Invoicing

**Problem Statement:**
Getting paid is difficult because not all my customers use credit cards. Depending on where I am, they might want to pay with local methods like Mercado Pago or bank transfers. I need an easy way to send an invoice that lets my customers pay however is easiest for them, and instantly notifies me when the money arrives.

**Research Report:**
- **Findings:** Stripe covers credit cards globally, but Mercado Pago dominates LATAM, and Razorpay dominates India.
- **Ease of Use:** Invoicing must be as simple as generating a PDF with a "Pay Now" button or a shareable link.
- **Pricing:** Standard payment gateway fees apply (e.g., 2.9% + 30¢ per transaction). No fixed monthly cost.
- **Reputation:** Stripe is the gold standard; Mercado Pago is trusted across LATAM.
- **Environment:** Cloud webhooks receive payment confirmations. Standalone mode requires a polling mechanism or tunnel to receive payment success events.

**Design Doc:**
- **Trigger:** Business owner generates an "Invoice" in OHC for a specific amount and customer.
- **Action:** OHC generates a secure payment link using the integrated payment provider and sends it to the customer.
- **User Experience:** The owner enters "Consultation - $100", clicks "Create Link", and texts it to the client. When the client pays, the owner's dashboard shows a green "Paid" badge and sends a push notification.

**Implementation Prompt:**
Add a feature allowing users to connect a payment gateway and generate shareable payment links or invoices. Acceptance criteria: users can connect their payment account, generate a payment link for a specific amount, and the OHC system automatically marks the invoice as "Paid" when the transaction is completed by the customer.

**Priority:** P0
**Estimated Scope:** Large

---

## [Shipping & Logistics] Automated Shipping Label Generation

**Title:** Integrate EasyPost for Multi-Carrier Shipping Labels

**Problem Statement:**
When I sell a physical product, figuring out shipping costs and buying labels at the post office takes hours. I need my system to automatically calculate the cheapest shipping rate and print a label the moment a customer buys something.

**Research Report:**
- **Findings:** EasyPost and Shippo aggregate multiple carriers (USPS, UPS, FedEx, DHL, local carriers).
- **Ease of Use:** Extremely high value for e-commerce. Removes the need for business owners to negotiate carrier rates themselves.
- **Pricing:** EasyPost charges ~1¢ per label after the free tier (120k shipments/year).
- **Reputation:** EasyPost is reliable with excellent uptime and developer documentation.
- **Environment:** API calls work seamlessly in both Cloud and Standalone modes.

**Design Doc:**
- **Trigger:** A customer completes a purchase with a physical address, or the owner manually enters an address to ship a package.
- **Action:** OHC requests rates from EasyPost, selects the default/cheapest, and generates a printable PDF label.
- **User Experience:** The business owner sees a "New Order" notification. They click "Print Shipping Label." A PDF downloads, and the customer automatically receives an email with their tracking number.

**Implementation Prompt:**
Develop a shipping label generation tool integrated with a multi-carrier API. Users should be able to input package dimensions/weight and a destination address, compare rates, and purchase a printable shipping label. Acceptance criteria: users can generate a valid PDF shipping label, and a tracking link is automatically generated and attached to the customer's order record.

**Priority:** P2
**Estimated Scope:** Large

---

## [SMS & Notifications] Reliable Global SMS Delivery

**Title:** Integrate Twilio for Automated Customer SMS Alerts

**Problem Statement:**
Many of my customers don't check their email, especially older demographics or non-English speakers. If I need to remind them of an appointment or tell them their order is ready, a text message is the only way to ensure they see it. I need OHC to send automated text messages so I don't have to use my personal phone number.

**Research Report:**
- **Findings:** Twilio and MessageBird are market leaders.
- **Ease of Use:** The complexity of A2P 10DLC registration in the US is a massive hurdle for small businesses. OHC needs a wizard to guide users through business verification, or provide a shared OHC number for simple alerts.
- **Pricing:** ~$0.0079 per message (US), varies globally.
- **Reputation:** Twilio is extremely reliable but strict on compliance and opt-outs.
- **Environment:** Cloud and Standalone modes can easily make outbound API calls to send SMS.

**Design Doc:**
- **Trigger:** An appointment is approaching (24h before), or an order status changes to "Ready for Pickup."
- **Action:** OHC triggers an API call to Twilio to send a pre-configured SMS template to the customer's phone number.
- **User Experience:** The business owner toggles on "Send SMS reminders." They don't have to do anything else. The customer receives a text: "Hi, your appointment with [Business Name] is tomorrow at 2 PM. Reply C to cancel."

**Implementation Prompt:**
Implement automated SMS notifications for key business events (e.g., appointment reminders, order updates). Ensure compliance with opt-out mechanisms (e.g., handling "STOP" replies). Acceptance criteria: users can enable SMS notifications, configure simple templates, and customers receive the SMS at the designated trigger time.

**Priority:** P1
**Estimated Scope:** Medium

---

## [Video Conferencing] One-Click Virtual Meeting Links

**Title:** Integrate Daily.co / Zoom for Seamless Virtual Consultations

**Problem Statement:**
When I offer online consultations or lessons, I waste time creating Zoom links, copying them, and emailing them to the client. I need the system to automatically generate a video link the moment someone books an online appointment.

**Research Report:**
- **Findings:** Zoom API is common but requires the user to have a Zoom account. Daily.co allows embedding white-labeled video calls directly into the browser without the business owner or client needing any third-party accounts.
- **Ease of Use:** Daily.co is vastly superior for non-technical users because it requires zero setup—it just works in the browser.
- **Pricing:** Daily.co has a generous free tier (10k minutes/month).
- **Reputation:** Excellent WebRTC performance and mobile compatibility.
- **Environment:** WebRTC works smoothly in Cloud and Standalone (client-side browser connection).

**Design Doc:**
- **Trigger:** A customer books a service labeled as "Virtual" or "Online."
- **Action:** OHC generates a unique Daily.co room URL via API and attaches it to the meeting invite.
- **User Experience:** The business owner and the customer both get an email saying "Join Meeting Here." When it's time, they click the link and the video call opens directly in their browser—no downloads, no logins.

**Implementation Prompt:**
Integrate a video conferencing API to automatically provision meeting rooms for virtual appointments. Acceptance criteria: when a virtual appointment is booked, a unique video room link is generated and shared with both the business owner and the client. Both parties must be able to join the call via their browser without installing extra software.

**Priority:** P1
**Estimated Scope:** Small
