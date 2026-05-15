# 🔎 Scout: Tool Integration Research Q4

## [Social Media Integration] Unified Inbox Sync

**Problem Statement:**
As a small business owner, I get DMs, comments, and messages across Instagram, Facebook, TikTok, and WhatsApp. Jumping between apps to answer customer questions or take orders is exhausting, and I often miss messages. I need one place to see and reply to everything.

**Research Report:**
* **ManyChat:** Best in class for Instagram/Facebook automation, but complex to set up. Reliable APIs. Pricing starts around $15/mo. Great for automation, but maybe too complex for simple unified inbox. (Cloud only)
* **Respond.io:** Excellent multi-channel inbox (WhatsApp, IG, FB, Telegram). Highly reliable webhook parsing. Good pricing for small teams (~$79/mo). Very user-friendly interface. (Cloud only)
* **Meta Business Suite API:** Free, direct integration for FB/IG. Can be finicky. Requires OAuth setup which can be daunting for non-technical users. (Works in Cloud, Standalone might be hard due to OAuth redirect URI limits)
* **Conclusion:** For a unified inbox, building a direct integration using Meta Graph API (for FB/IG) and WhatsApp Business API is the most cost-effective, though building the OAuth flow requires careful UX design to not overwhelm the user. It works natively in Cloud; Standalone users may need an OAuth proxy service hosted by OHC.

**Design Doc:**
* **Trigger:** User connects their social accounts via an OAuth popup in OHC Settings.
* **Action:** Background job periodically pulls new messages/comments or listens for webhooks from connected platforms.
* **User View:** A new "Inbox" tab in OHC that consolidates all conversations into a single chronological feed. The user can type a reply in OHC, and it sends the message back to the originating platform natively.

**Implementation Prompt:**
Create a unified Inbox UI where users can read and reply to messages from multiple social platforms. Ensure a seamless "Connect Account" flow that guides them through the necessary authorizations without using technical jargon.

**Priority:** P0
**Estimated Scope:** Large

---

## [Calendar & Scheduling] Automated Meeting Links

**Problem Statement:**
I offer online consultations, but organizing a time that works for everyone, creating a Zoom link, and sending it to the client takes too many emails. I need a way for clients to just pick a time on my calendar and automatically get a meeting link.

**Research Report:**
* **Calendly API:** Industry standard. Extremely reliable timezone handling and Google/Outlook sync. Generates Meet/Zoom links natively. Starts at $10/mo for premium features. (Cloud only)
* **Cal.com (formerly Calendso):** Open source alternative. Has a robust API and webhooks. Great for self-hosting (Standalone) and cloud.
* **Acuity Scheduling:** Good for service-based businesses, but API can be restrictive on lower tiers. (Cloud only)
* **Conclusion:** Cal.com offers the best flexibility for both our Cloud and Standalone environments.

**Design Doc:**
* **Trigger:** User creates a new "Service" in OHC and enables "Online Booking".
* **Action:** OHC integrates with the chosen scheduling provider (e.g., Cal.com) to generate a unique booking page URL and synchronizes availability with the user's primary calendar.
* **User View:** A simple toggle on their service offering: "Allow online booking". They see a shareable link they can put in their Instagram bio. When a client books, the owner gets an instant notification and it appears on their OHC calendar.

**Implementation Prompt:**
Add a "Scheduling" settings panel where a user can connect their primary calendar. When creating a service, add a toggle to generate a public booking page. When a client uses this page, they should receive an email with the auto-generated meeting link, and the owner should see the event on their internal calendar.

**Priority:** P1
**Estimated Scope:** Medium

---

## [Payment Processing] Global Localized Payments

**Problem Statement:**
Stripe is great, but many of my local customers prefer to pay with Mercado Pago, UPI, or local bank transfers. If I only offer credit cards, I lose sales. I need to accept the payment methods my customers actually use.

**Research Report:**
* **Mercado Pago (LATAM):** Essential for Latin America. High conversion rates for local methods (Pix in Brazil, Oxxo in Mexico). API is decent. (Works in Cloud and Standalone via generic webhooks/redirects)
* **Razorpay / Paytm (India):** Crucial for UPI payments. (Works in Cloud and Standalone via generic webhooks/redirects)
* **Adyen:** Great global coverage, but enterprise-focused and complex to implement. (Cloud preferred)
* **Conclusion:** Integrating regional heavyweights (Mercado Pago, Razorpay) directly alongside Stripe offers the best value proposition for our global user base. It works in both Cloud and Standalone modes as payment gateways support generic redirect flows and webhook endpoints that can be configured per instance.

**Design Doc:**
* **Trigger:** User sets up their store location/currency in OHC.
* **Action:** OHC exposes the relevant regional payment providers based on the user's location setting.
* **User View:** In the "Payments" setup, instead of just "Connect Stripe", the user sees options like "Connect Mercado Pago" if they are in LATAM. The checkout page automatically adapts to show these local payment options to their customers.

**Implementation Prompt:**
Expand the Payments settings to dynamically offer regional payment gateways based on the business's location. The checkout flow should seamlessly support these new methods without requiring the business owner to write any code or manage complex API keys beyond a simple "Connect" button.

**Priority:** P1
**Estimated Scope:** Large

---

## [Email Marketing] Integrated Customer Broadcasts

**Problem Statement:**
I have a list of customers who bought from me, but sending them an update about a new product means exporting the list from my store and importing it into Mailchimp. It's tedious and I forget to do it. I want to email them directly from my store dashboard.

**Research Report:**
* **Mailchimp API:** Very popular, but getting expensive. Good deliverability. (Cloud only)
* **SendGrid / Resend:** Developer-focused transactional email, but can be adapted for marketing broadcasts. High deliverability. Resend is very modern. (Cloud only)
* **Brevo (formerly Sendinblue):** Good pricing, built-in CRM features, decent API. (Cloud only)
* **Conclusion:** For maximum simplicity for the user, OHC should abstract the provider (using something like Resend under the hood for Cloud, or allowing custom SMTP for Standalone) and provide a native broadcast tool.

**Design Doc:**
* **Trigger:** User goes to their "Customers" list and clicks "Send Broadcast".
* **Action:** OHC uses an internal email delivery service (or configured SMTP) to send a templated HTML email to the selected customer segment.
* **User View:** A simple WYSIWYG editor right inside OHC where they can type a message, add a picture of their new product, select "All past buyers", and hit send. They see open rates next to the sent message.

**Implementation Prompt:**
Build a native "Email Broadcast" feature. Users should be able to select segments of their customer list and send them a rich-text email directly from the OHC dashboard. The focus must be on simplicity—no complex drag-and-drop builders, just a clean text and image editor.

**Priority:** P2
**Estimated Scope:** Medium

---

## [Shipping & Logistics] Automated Shipping Labels

**Problem Statement:**
When I get an order, figuring out how much shipping will cost, going to the post office website, typing in the address manually, and printing a label is incredibly slow and error-prone. I want shipping labels generated automatically when someone buys a physical product.

**Research Report:**
* **EasyPost:** One API for dozens of carriers (USPS, FedEx, UPS). Very developer friendly. Reliable. Excellent for US-based businesses. (Works in Cloud and Standalone)
* **Shippo:** Very user-friendly, great API. Offers deep discounts on USPS. Good international support. (Works in Cloud and Standalone)
* **ShipStation:** Industry leader for larger sellers, but complex and pricey for beginners. (Cloud only)
* **Conclusion:** Integrating Shippo or EasyPost directly into OHC provides the most value for sellers needing automated shipping calculation and label generation. It works seamlessly in Cloud and Standalone through API integration.

**Design Doc:**
* **Trigger:** User creates a physical product and enters its weight/dimensions. A customer places an order.
* **Action:** OHC requests shipping rates via the integration provider, and upon purchase, creates a fulfillment order to generate a printable label.
* **User View:** When viewing a new order in OHC, there is a "Print Shipping Label" button. Clicking it instantly downloads a PDF label they can print and stick on the box. Tracking numbers are auto-sent to the customer.

**Implementation Prompt:**
Build a shipping workflow for physical products. Users should be able to configure box sizes and weights, view real-time shipping costs at checkout, and print shipping labels directly from the order details screen without leaving OHC.

**Priority:** P1
**Estimated Scope:** Large

---

## [SMS & Notifications] Reliable Text Alerts

**Problem Statement:**
Many of my customers don't check email regularly, especially my older customers or those who prefer texting. I need to send them appointment reminders and order updates via SMS so they don't miss important information.

**Research Report:**
* **Twilio:** Industry standard. Extremely reliable globally. Flexible API. Requires phone number registration (A2P 10DLC in the US), which is complex for small businesses. (Works in Cloud and Standalone)
* **Plivo / Vonage:** Good alternatives to Twilio, often slightly cheaper. Still requires complex registration. (Works in Cloud and Standalone)
* **Conclusion:** Twilio is best for global reach. OHC needs to abstract the complexity of phone number registration and A2P compliance. Works well in Cloud via OHC's master account, and in Standalone via users providing their own Twilio credentials.

**Design Doc:**
* **Trigger:** An important event occurs (e.g., an upcoming appointment tomorrow, or an order is shipped).
* **Action:** OHC sends an automated text message to the customer's phone number using the configured SMS provider.
* **User View:** A simple toggle in "Notifications" settings: "Enable SMS reminders". The user enters their business name as the sender ID. They do not have to mess with API keys or compliance forms.

**Implementation Prompt:**
Implement SMS notifications for critical alerts. The interface should allow business owners to enable SMS updates for their customers with a single click, handling all provider abstraction and compliance logic behind the scenes.

**Priority:** P1
**Estimated Scope:** Medium

---

## [Video Conferencing] Auto-generated Online Meeting Links

**Problem Statement:**
When I schedule an online class or meeting with a client, I have to manually go into Zoom or Google Meet, create a meeting, copy the link, and email it to them. It's too many steps. I want a meeting link generated automatically for every booking.

**Research Report:**
* **Zoom API:** Highly reliable, widely used. Requires OAuth for each user to connect their Zoom account. (Works in Cloud, Standalone needs OAuth proxy)
* **Google Meet API:** Native integration if they use Google Workspace. Extremely reliable. (Works in Cloud, Standalone needs OAuth proxy)
* **Jitsi Meet:** Open source. Easy to embed and generate links without accounts. (Works natively in Cloud and Standalone without complex OAuth)
* **Conclusion:** While Zoom and Google Meet are standard, Jitsi offers the simplest zero-setup experience. OHC could auto-generate unique Jitsi links instantly without requiring the user to connect anything.

**Design Doc:**
* **Trigger:** A new booking is created for an "Online" service type.
* **Action:** OHC automatically generates a unique video conferencing link (e.g., via Jitsi) and embeds it in the calendar invite and confirmation email.
* **User View:** When setting up a service, the location can be set to "Online Meeting". OHC handles the rest. The business owner and the client simply click the "Join Meeting" button in their email at the scheduled time.

**Implementation Prompt:**
Integrate an automatic video conferencing link generator for online services. Business owners should simply select "Online Video Call" as the location, and the system should handle generating and distributing the unique meeting link to both parties automatically.

**Priority:** P2
**Estimated Scope:** Small
