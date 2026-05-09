# OHC Tool Integration Research Report Q4

**Executive Summary**

This report evaluates key third-party integrations across seven crucial operational categories for small business owners. Our primary focus is identifying tools that provide immediate value to non-technical users while fitting seamlessly into the One Human Corp (OHC) ecosystem in both cloud and standalone environments.

The evaluation prioritizes ease of use (the "Grandmother Test"), transparent pricing, and robust APIs. Technical implementation specifics (SQL DDL, function signatures) are excluded, allowing the Implementer swarm flexibility in execution.

---

## 1. Social Media Integration: Unified Inbox

**Problem Statement:**
Small business owners miss potential sales because customer inquiries are scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments. Managing these multiple channels is overwhelming.

**Tool Evaluated:** Meta Graph API & Webhooks (Instagram, FB, WhatsApp)
* **What it solves:** Consolidates messaging into a single OHC inbox.
* **User Benefit:** Owners can reply to an Instagram DM or WhatsApp message directly from the OHC dashboard without juggling apps.
* **Risks:** Complex OAuth approval process; strict Meta policies on automated messaging.
* **Pricing Estimate:** Free for basic Graph API usage; WhatsApp Business API has per-conversation pricing (approx. $0.01 - $0.08 depending on region).
* **Hybrid Support:** Yes (Cloud uses standard webhooks; Standalone requires a local webhook relayer or polling mechanism).

---

## 2. Calendar & Scheduling

**Problem Statement:**
Booking consultations or services often involves back-and-forth emails ("What time works for you?"). This creates friction and lost revenue.

**Tool Evaluated:** Cal.com (Open Source Scheduling API)
* **What it solves:** Generates a professional booking page that syncs with the owner's Google/Outlook calendar.
* **User Benefit:** The owner sends a link, the client books a time, and it magically appears on their calendar without double-booking.
* **Risks:** Calendar sync can sometimes experience minor delays. Timezone edge cases.
* **Pricing Estimate:** Open source/free tier available; API usage scales with volume ($15/user/mo for premium features).
* **Hybrid Support:** Yes. Ideal for Standalone as Cal.com can be self-hosted alongside OHC or accessed via API.

---

## 3. Email Marketing

**Problem Statement:**
Business owners have customer emails but no easy way to send professional newsletters, promotions, or updates without learning complex tools like Mailchimp.

**Tool Evaluated:** Resend
* **What it solves:** Developer-friendly email API that allows OHC to build a simplified email campaign sender directly into the OHC UI.
* **User Benefit:** Owners can draft a plain-text or simple template email in OHC, click "Send to all customers," and trust it won't go to spam.
* **Risks:** Requires domain verification (DNS records) which is difficult for non-technical users.
* **Pricing Estimate:** 3,000 emails/month free; $20/mo for 50,000 emails.
* **Hybrid Support:** Yes. Cloud can use standard API keys; Standalone can require the user to input their own Resend key.

---

## 4. Payment Processing (LATAM Focus)

**Problem Statement:**
While Stripe is the global standard, many business owners in Latin America need localized payment methods (Pix, Boleto, local credit cards) that Stripe doesn't support well.

**Tool Evaluated:** Mercado Pago API
* **What it solves:** Provides a reliable, localized payment gateway for the LATAM market.
* **User Benefit:** Owners can generate payment links or invoices that their local customers can actually pay using familiar methods.
* **Risks:** Settlement times vary by country; API documentation is sometimes fragmented.
* **Pricing Estimate:** No monthly fee; per-transaction fee varies heavily by country and payment method (typically 3-5% + flat fee).
* **Hybrid Support:** Yes. Standard REST API works identically in Cloud and Standalone.

---

## 5. Shipping & Logistics

**Problem Statement:**
E-commerce SMBs struggle to calculate accurate shipping rates, print labels, and provide tracking numbers to customers without logging into multiple carrier websites.

**Tool Evaluated:** Shippo API
* **What it solves:** Connects to dozens of carriers (USPS, UPS, FedEx, DHL) through a single interface.
* **User Benefit:** When an order comes in through OHC, the owner clicks "Generate Label," prints it, and tracking is automatically sent to the customer.
* **Risks:** International shipping requires complex customs declarations that are hard to simplify in the UI.
* **Pricing Estimate:** Pay-as-you-go: $0.05 per label + postage costs.
* **Hybrid Support:** Yes. API relies on external webhooks for tracking updates.

---

## 6. SMS & Notifications

**Problem Statement:**
For customers or business owners who are rarely at a computer or don't regularly check email, important updates (appointment reminders, order ready) are missed.

**Tool Evaluated:** Twilio Programmable SMS
* **What it solves:** Reliable global SMS delivery.
* **User Benefit:** The owner can set up automated text reminders ("Your appointment is tomorrow") or manually text a client from the OHC dashboard.
* **Risks:** A2P 10DLC compliance in the US requires business registration, which is a major hurdle for very small businesses.
* **Pricing Estimate:** Pay-as-you-go: roughly $0.0079 per text in the US.
* **Hybrid Support:** Yes. Standard REST API.

---

## 7. Video Conferencing

**Problem Statement:**
Consultants, tutors, and remote service providers need an easy way to generate secure video links for meetings without manually creating them in Zoom and pasting them into emails.

**Tool Evaluated:** Google Meet (via Google Workspace API)
* **What it solves:** Automatically generates a Meet link when a calendar event is created.
* **User Benefit:** When a client books a meeting, a "Join Video Call" button automatically appears in both the owner's and client's OHC dashboard. No manual link sharing required.
* **Risks:** Requires the business owner to use Google Workspace; OAuth flow can be intimidating.
* **Pricing Estimate:** Included in Google Workspace subscriptions ($6/user/mo).
* **Hybrid Support:** Yes. Relies on Google's cloud infrastructure.