# OHC Tool Integration Research Report

## Category: Social Media Integration

### Title
**Implement Chatwoot Integration for Unified Social Media Inbox**

### Problem Statement
Small business owners, like Maya the baker, receive orders, questions, and feedback across multiple social media platforms (Instagram DMs, Facebook comments, WhatsApp, TikTok). Managing these separately is overwhelming, prone to missed messages, and requires switching contexts frequently, leading to lost sales and delayed responses.

### Research Report
**Tool Analyzed:** Chatwoot
- **Ease of Use for Non-Technical Users:** High. Once connected, it provides a unified inbox that looks like any standard email or chat app, making it intuitive for users like Maya.
- **Pricing:** Free tier available for small teams (perfect for OHC's target market); paid plans are affordable (starting around $19/month).
- **Reputation:** Open-source, strong community, and well-regarded for its unified inbox capabilities.
- **Integration Profile:** Supports both Cloud and Standalone modes. Offers webhooks and direct integrations with Facebook, Instagram, Twitter, and WhatsApp.
- **Key Advantages:** Unified messaging reduces context switching; open-source nature allows deep integration.
- **Risks:** Handling token expiration and rate limits for multiple social platforms simultaneously can cause reliability issues if not managed properly.

### Design Doc
- **Trigger:** A new message or comment is received on any connected social media platform.
- **Action:** Chatwoot ingests the message and routes it to the OHC unified inbox. OHC's "Customer Success" AI agent can draft a reply based on the business's context.
- **User View:** The business owner sees a single "Messages" tab in their OHC dashboard, consolidating all communications regardless of the source platform.

### Implementation Prompt
Implement an integration with Chatwoot to provide a unified social media inbox. The integration must allow users to connect their Facebook, Instagram, and WhatsApp accounts via a simple OAuth flow in the OHC dashboard. Messages from these platforms should appear in a single, unified view, and replies sent from OHC must be routed back to the original platform.
- **Priority:** P1
- **Estimated Scope:** Medium

---

## Category: Calendar & Scheduling

### Title
**Integrate Cal.com for Automated Booking and Calendar Sync**

### Problem Statement
Service providers like Leo the music tutor or Carlos the handyman rely on scheduling to run their business. Managing availability, double-booking, sending reminders, and generating meeting links manually takes time away from their actual work and leads to a poor customer experience.

### Research Report
**Tool Analyzed:** Cal.com
- **Ease of Use for Non-Technical Users:** High. Clean interface, straightforward setup for availability and booking types.
- **Pricing:** Free tier is robust for individuals.
- **Reputation:** Strong open-source alternative to Calendly, highly customizable.
- **Integration Profile:** Supports both Cloud and Standalone modes via API and webhooks. Excellent handling of timezone differences and calendar conflicts.
- **Key Advantages:** Open-source architecture aligns with OHC's local-first ethos; excellent UI for users.
- **Risks:** Two-way calendar syncs (Google/Outlook) are notoriously brittle regarding token expiration and sudden permission changes.

### Design Doc
- **Trigger:** Customer visits the OHC storefront and clicks "Book a Service".
- **Action:** Cal.com integration presents available slots, handles the booking, syncs with the owner's Google/Outlook calendar, and generates a video link (if applicable).
- **User View:** The business owner sees upcoming appointments in their OHC dashboard calendar and receives notifications for new bookings or cancellations.

### Implementation Prompt
Integrate Cal.com to provide robust scheduling capabilities. The feature must allow business owners to set their availability, define service durations, and connect their personal calendars (Google/Outlook) to prevent double-booking. Customers should be able to select an available time slot directly from the business's public OHC page, resulting in an automatic calendar invite being sent to both parties.
- **Priority:** P0
- **Estimated Scope:** Large

---

## Category: Email Marketing

### Title
**Implement MailerLite Integration for Automated Email Marketing**

### Problem Statement
Business owners like Priya the boutique owner need a way to announce new products, sales, or events to their existing customer base to drive repeat business. Writing HTML emails, managing subscriber lists, and tracking open rates is too complex and time-consuming.

### Research Report
**Tool Analyzed:** MailerLite
- **Ease of Use for Non-Technical Users:** Very High. Drag-and-drop builder is intuitive, and list management is straightforward.
- **Pricing:** Generous free tier (up to 1,000 subscribers and 12,000 emails/month).
- **Reputation:** Known for excellent deliverability, simplicity, and good customer support.
- **Integration Profile:** Cloud-focused. Will require proxying for Standalone mode users to protect API secrets.
- **Key Advantages:** Very simple interface for non-technical users; high deliverability rates.
- **Risks:** Accounts can be suspended quickly if spam complaints rise; OHC must handle list management carefully.

### Design Doc
- **Trigger:** A customer makes a purchase, signs up via a storefront form, or the business owner decides to send a broadcast.
- **Action:** OHC syncs the customer contact to MailerLite. The "Marketing & Advertising" AI agent can draft email content which is pushed to MailerLite for sending.
- **User View:** The business owner sees a simplified "Campaigns" interface in OHC where they can select an AI-drafted template and click "Send to All Customers".

### Implementation Prompt
Integrate MailerLite to enable seamless email marketing. The integration must automatically sync customer emails from OHC orders and sign-ups to a MailerLite list. It should also provide a simplified interface within OHC for business owners to review, edit, and send AI-generated email campaigns to their customer base, displaying basic analytics (sent, opened, clicked) post-campaign.
- **Priority:** P1
- **Estimated Scope:** Medium

---

## Category: Payment Processing

### Title
**Integrate Mercado Pago for LATAM Localized Payments**

### Problem Statement
While Stripe is excellent, it is not supported globally. Business owners in regions like LATAM (Mercado Pago), India (Paytm), or those preferring alternative methods need localized payment options. Missing localized payments means lost sales.

### Research Report
**Tool Analyzed:** Mercado Pago (for LATAM focus)
- **Ease of Use for Non-Technical Users:** High in supported regions; familiar to local customers.
- **Pricing:** Standard transaction fees (varies by country, typically ~3-4% + fixed fee).
- **Reputation:** The dominant and trusted payment processor in Latin America.
- **Integration Profile:** Cloud-focused API. For Standalone mode, requires local callback handling.
- **Key Advantages:** Massive market penetration in Latin America; supports local payment methods (e.g., PIX in Brazil, OXXO in Mexico).
- **Risks:** The API and testing environments can be inconsistent across different countries; webhook reliability varies.

### Design Doc
- **Trigger:** A customer reaches the checkout stage on an OHC storefront.
- **Action:** OHC presents Mercado Pago as a payment option. If selected, the transaction is processed via Mercado Pago's API.
- **User View:** The business owner sees the transaction recorded in their OHC "Finance & Payments" dashboard just like a Stripe payment, with funds settling to their connected Mercado Pago account.

### Implementation Prompt
Integrate Mercado Pago as an alternative payment gateway for LATAM users. The integration must allow business owners to authenticate their Mercado Pago account via OAuth. At checkout, customers should see Mercado Pago as a seamless payment option. Successful transactions must trigger webhooks to update the order status in OHC to "Paid".
- **Priority:** P2
- **Estimated Scope:** Large

---

## Category: Shipping & Logistics

### Title
**Implement Shippo Integration for Real-time Rates and Label Generation**

### Problem Statement
For sellers of physical goods, calculating accurate shipping rates, generating labels, and providing tracking information is a major pain point. Estimating costs manually leads to undercharging (eating into profits) or overcharging (abandoned carts).

### Research Report
**Tool Analyzed:** Shippo
- **Ease of Use for Non-Technical Users:** High. Simplifies the complexities of carrier rates and label generation.
- **Pricing:** Free tier available (pay only for postage); very accessible for small businesses.
- **Reputation:** Highly regarded API, wide carrier network.
- **Integration Profile:** Cloud-focused API. Cannot run entirely offline in Standalone mode as rates change dynamically.
- **Key Advantages:** Aggregates many carriers into one simple API; offers discounted shipping rates.
- **Risks:** Handling complex package dimension routing for multiple items can be difficult to model perfectly.

### Design Doc
- **Trigger:** A customer adds physical items to their cart and enters their address.
- **Action:** OHC calls Shippo to get real-time shipping rates based on item weight/dimensions. Post-purchase, Shippo is used to generate a printable label.
- **User View:** The business owner clicks "Fulfill Order" in OHC, reviews the shipping option, and clicks "Print Label", which downloads a PDF.

### Implementation Prompt
Integrate Shippo to automate shipping label generation and tracking. The integration must fetch real-time shipping rates during the customer checkout process. In the OHC dashboard, business owners must be able to generate and print shipping labels for paid orders with a single click, and tracking numbers should be automatically emailed to the customer.
- **Priority:** P1
- **Estimated Scope:** Large

---

## Category: SMS & Notifications

### Title
**Integrate Twilio for Automated SMS Order Alerts and Reminders**

### Problem Statement
For users like Fatima the food cart operator, email is too slow. They need immediate notifications when a new order arrives. Similarly, customers often prefer SMS updates for order readiness or appointment reminders.

### Research Report
**Tool Analyzed:** Twilio
- **Ease of Use for Non-Technical Users:** Low (for setup), but OHC will abstract this completely. The end-user experience is very high.
- **Pricing:** Pay-as-you-go (fractions of a cent per message). Affordable, but requires OHC to manage billing or limits.
- **Reputation:** The industry standard for programmable SMS.
- **Integration Profile:** Cloud-focused API. Needs an active internet connection for Standalone mode users.
- **Key Advantages:** Highly reliable, global delivery; exceptional API documentation.
- **Risks:** High regulatory burden (10DLC registration, opt-out management) could overwhelm small business owners if not perfectly abstracted by OHC.

### Design Doc
- **Trigger:** An order is placed, or an appointment is upcoming.
- **Action:** OHC triggers an API call to Twilio to send a templated SMS message.
- **User View:** The business owner receives an SMS: "New order: 2x Chicken Over Rice. Pickup in 15m." The customer receives: "Your order is ready!"

### Implementation Prompt
Integrate Twilio to provide critical SMS notifications. The system must support sending instant SMS alerts to business owners for new orders and sending order status updates or appointment reminders to customers. The integration must be completely abstracted from the business owner, requiring no Twilio account setup on their part.
- **Priority:** P1
- **Estimated Scope:** Medium

---

## Category: Video Conferencing

### Title
**Implement Google Meet Auto-Generation for Virtual Bookings**

### Problem Statement
Service providers like Leo the online tutor need a seamless way to generate meeting links for their sessions. Manually creating and emailing Zoom links for every booking is tedious and looks unprofessional.

### Research Report
**Tool Analyzed:** Google Meet (via Google Workspace integration)
- **Ease of Use for Non-Technical Users:** Very High. Most users already have a Google account.
- **Pricing:** Free for basic use; included in Workspace.
- **Reputation:** Ubiquitous, reliable, requires no software installation for the client.
- **Integration Profile:** Available via Google Calendar API. Functions mostly in Cloud mode, though Standalone modes can utilize OAuth flows if correctly configured.
- **Key Advantages:** Frictionless for guests (no app required); already integrated with the user's primary calendar.
- **Risks:** The Google API authorization flow requires strict app verification procedures, which can be a bottleneck for OHC's platform deployment.

### Design Doc
- **Trigger:** A booking is finalized (via the Cal.com integration or native OHC scheduling).
- **Action:** OHC requests a Google Meet link via the Google Calendar API when creating the calendar event.
- **User View:** Both the business owner and the customer receive a calendar invite and confirmation email containing the auto-generated Google Meet link.

### Implementation Prompt
Integrate Google Meet link generation into the booking flow. When a customer books a virtual service, the system must automatically create a Google Calendar event on the business owner's connected account, generating a unique Google Meet link. This link must be displayed on the booking confirmation page and included in the automated confirmation emails sent to both parties.
- **Priority:** P2
- **Estimated Scope:** Medium
