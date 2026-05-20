# Tool Integration Research Report Q4

## Overview
This report evaluates tools across 7 categories to empower small business owners using OHC. For each category, we have selected a top recommendation and provided a detailed issue brief.

## Findings


**Title:** Social Media Integration: Unified Inbox with ManyChat

**Problem Statement:** Small business owners struggle to manage customer inquiries across Instagram, Facebook, WhatsApp, and SMS, leading to missed messages and lost sales. A unified inbox would save time and improve response rates.

**Research Report:** ManyChat offers strong cross-platform messaging capabilities (IG, FB, WhatsApp, SMS). It is relatively easy to use for non-technical users and has a good reputation.
*   **Key Advantages:** Unified cross-platform inbox, easy-to-use visual flow builder, official Meta API partner.
*   **Key Risks:** Dependency on Meta's API policies; if Meta changes rules, the integration breaks. Account bans can happen if spam rules are violated.
*   **Rough Pricing Estimate:** Free tier available; Pro tier starts at $15/mo scaling with contacts.
*   **Environment Compatibility:** Works fully in Cloud mode. Standalone mode will require setting up public webhooks via a service like ngrok or Cloudflare Tunnels to receive messages.

**Design Doc:** The tool will integrate as an optional "Inbox Connect" feature. The user authorizes their social media accounts via an OAuth flow. OHC will listen for incoming messages via webhooks and display them in a unified "Customer Messages" tab.

**Implementation Prompt:** Create a unified "Customer Messages" interface where business owners can view and reply to cross-platform messages without leaving the app. The setup process should be a simple "Connect Account" button with clear permissions.

**Priority:** P1

**Estimated Scope:** Medium


---


**Title:** Calendar Scheduling Integration: Automated Booking with Cal.com

**Problem Statement:** Scheduling appointments and consultations often involves back-and-forth emails, leading to double-booking and lost time for business owners.

**Research Report:** Cal.com is an open-source, flexible scheduling tool. It allows users to set availability and share a booking link. It integrates well with Google Calendar and Outlook.
*   **Key Advantages:** Open-source, highly customizable, supports self-hosting, developer-friendly API, white-label options.
*   **Key Risks:** The UI is slightly more complex than Calendly for completely non-technical users. Managing OAuth tokens for calendar sync requires careful security handling.
*   **Rough Pricing Estimate:** Free tier for individuals; $12/user/mo for teams.
*   **Environment Compatibility:** Ideal for both Cloud and Standalone modes, as it can be entirely self-hosted alongside the Standalone OHC instance.

**Design Doc:** Integrate a "Booking Page" section in OHC. The user connects their primary calendar (Google/Outlook). A unique booking link is generated, which they can share with clients. Appointments booked via this link will automatically appear in their OHC calendar and external calendar.

**Implementation Prompt:** Build a simple scheduling flow where the user connects their calendar and sets their availability. Generate a shareable booking link. Ensure conflicts are automatically resolved.

**Priority:** P0

**Estimated Scope:** Large


---


**Title:** Email Marketing Integration: Customer Campaigns with MailerLite

**Problem Statement:** Business owners want to keep their customers engaged with promotions and updates, but find complex email marketing tools overwhelming and expensive.

**Research Report:** MailerLite is known for its user-friendly interface and affordability, making it great for small businesses.
*   **Key Advantages:** Extremely simple drag-and-drop editor, generous free tier, good deliverability rates, plain language analytics.
*   **Key Risks:** Strict approval process for new accounts to prevent spam; some users might get rejected. Feature set is less advanced than Mailchimp for complex automation.
*   **Rough Pricing Estimate:** Free up to 1,000 subscribers and 12,000 emails/mo. Paid plans start at $9/mo.
*   **Environment Compatibility:** Works well via API in Cloud mode. In Standalone mode, it works via outbound API calls, though rate limits must be respected by the local instance.

**Design Doc:** Add a "Campaigns" tab. Users can select a list of their OHC customers and compose an email using a simple rich-text editor or basic templates. Sending the campaign will push the data to MailerLite via API to handle delivery and tracking.

**Implementation Prompt:** Provide a simple email composition interface where owners can draft a message and select customer segments to send it to. Track open rates in a simple dashboard.

**Priority:** P2

**Estimated Scope:** Medium


---


**Title:** Payment Processing: LATAM Support with Mercado Pago

**Problem Statement:** Many business owners in Latin America cannot use Stripe and need a reliable, widely accepted local payment provider to accept online payments.

**Research Report:** Mercado Pago is the dominant payment processor in LATAM, supporting local currencies, installments, and various local payment methods (e.g., PIX in Brazil).
*   **Key Advantages:** Massive market share in LATAM, supports local payment methods (PIX, OXXO, Boleto), native installment support.
*   **Key Risks:** Integration documentation can be fragmented across different LATAM countries. High dispute/chargeback rates in certain regions.
*   **Rough Pricing Estimate:** Transaction-based, varies heavily by country and payment method (typically 3-5% + fixed fee).
*   **Environment Compatibility:** Works in Cloud and Standalone modes via API integrations.

**Design Doc:** Add Mercado Pago as a payment option in the "Payments" settings. Users authorize their Mercado Pago account. Invoices and online checkout pages generated by OHC will then display a "Pay with Mercado Pago" button.

**Implementation Prompt:** Add a seamless Mercado Pago onboarding flow in the settings. Update checkout experiences to handle local payment options and redirect smoothly back to the app after payment success or failure.

**Priority:** P1

**Estimated Scope:** Medium


---


**Title:** Shipping & Logistics: Simplified Label Generation with Shippo

**Problem Statement:** Managing order fulfillment, comparing carrier rates, and generating shipping labels is a manual and error-prone process for small business owners selling physical goods.

**Research Report:** Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL, etc.) and provides discounted rates.
*   **Key Advantages:** Single API for dozens of global carriers, access to discounted USPS/UPS rates, pay-as-you-go model.
*   **Key Risks:** International customs forms integration can be complex. Carrier API downtimes will directly affect OHC users.
*   **Rough Pricing Estimate:** Free to sign up; $0.05 per label fee (often waived with default carriers) + actual postage costs.
*   **Environment Compatibility:** Fits perfectly in Cloud mode. For Standalone, outbound API calls work fine, but webhook tracking updates will require public routing.

**Design Doc:** When an order is marked as "Ready to Ship" in OHC, display a "Get Shipping Label" button. OHC will fetch rates from Shippo, allow the user to select a carrier, and generate a printable PDF label. Tracking info will be automatically added to the order.

**Implementation Prompt:** Build a shipping label generation flow directly on the order details page. Let the user compare rates, buy a label, and print it with minimal clicks.

**Priority:** P2

**Estimated Scope:** Large


---


**Title:** SMS Notifications: Reliable Delivery with Twilio

**Problem Statement:** Some business owners and their customers have low English proficiency or limited internet access, making email notifications unreliable. SMS is critical for appointment reminders and order updates.

**Research Report:** Twilio is the industry standard for SMS APIs with global coverage and high reliability.
*   **Key Advantages:** Unmatched global carrier network, high reliability, massive developer ecosystem.
*   **Key Risks:** A2P 10DLC compliance in the US is extremely strict and requires complex business registration; small businesses often fail this verification.
*   **Rough Pricing Estimate:** Varies by country; approx. $0.0079 per message in the US, plus monthly phone number fees ($1-$2).
*   **Environment Compatibility:** Fully compatible with both Cloud and Standalone environments via API.

**Design Doc:** Introduce an "SMS Reminders" toggle in the notification settings. When enabled, OHC will use the Twilio API to send automated text messages for key events (e.g., appointment confirmations, shipping updates).

**Implementation Prompt:** Add settings to enable SMS notifications for specific events. Ensure phone numbers are correctly formatted and handle delivery failures gracefully by falling back to email if necessary.

**Priority:** P0

**Estimated Scope:** Medium


---


**Title:** Video Conferencing: Auto-Generated Meetings with Zoom

**Problem Statement:** Service-based businesses (like tutors or consultants) waste time manually creating and sending video meeting links for every online appointment.

**Research Report:** Zoom is the most recognizable video conferencing tool globally.
*   **Key Advantages:** Ubiquitous brand recognition, highly reliable video quality, users already have the app installed.
*   **Key Risks:** Zoom's OAuth app approval process is notoriously slow and strict. Requires annual security reviews.
*   **Rough Pricing Estimate:** Free tier available (40-min limit); Pro tier is $15.99/mo per host. API usage is generally free for account management.
*   **Environment Compatibility:** Works smoothly in both Cloud and Standalone environments via OAuth and API calls.

**Design Doc:** When an appointment is scheduled and marked as "Online", OHC will automatically call the Zoom API to create a meeting. The unique join link will be saved to the appointment and emailed/texted to the client.

**Implementation Prompt:** Add a "Connect Zoom" option. For online appointments, automatically generate a Zoom link and display it prominently in the calendar event and customer confirmation notifications.

**Priority:** P1

**Estimated Scope:** Small
