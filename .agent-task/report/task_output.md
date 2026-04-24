# 🔍 Scout: Tool Integration Research [Q3]

## 1. Social Media Integration
### [Social] Integrate ManyChat for Unified Inbox & Automated Replies

**Problem Statement:**
Business owners like Maya receive dozens of DMs on Instagram, Facebook, and WhatsApp daily. Managing these across different apps is overwhelming, and she often misses messages or responds late, losing potential sales. She needs a single place to see all messages and have her AI assistant draft or send replies.

**Research Report:**
- **Evaluated Tools:** ManyChat, Chatfuel, Twilio (WhatsApp only).
- **Findings:** ManyChat is the industry standard for omni-channel messaging automation (Instagram, Messenger, WhatsApp). It has native meta integrations and robust webhook support for external systems like OHC.
- **Ease of Use:** Extremely simple OAuth flow for non-technical users to connect their social accounts.
- **Pricing:** Starts at $15/month for basic features, which is affordable for SMBs.
- **OAuth & Webhooks:** Very reliable webhook infrastructure and clear Meta-approved APIs. Message parsing quality is high.

**Design Doc:**
- **Trigger:** User connects their Instagram/Facebook/WhatsApp in the "Customer Success" settings.
- **Actions:** When a customer sends a DM, the message routes to OHC's unified inbox. The AI "Customer Success" agent reads the message and either drafts a reply or auto-replies based on the user's settings.
- **User View:** A clean "Inbox" tab in the OHC mobile app where all DMs appear in one threaded conversation per customer.

**Implementation Prompt:**
Integrate ManyChat (or Meta direct APIs if preferred) so that users can click "Connect Instagram/Facebook", authorize OHC, and start receiving their social DMs in the OHC unified inbox. Ensure the AI agent can read and draft replies to these messages seamlessly. The inbox must be mobile-friendly and support real-time updates.

**Priority:** P1
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling
### [Scheduling] Integrate Cronofy for Unified Calendar Sync

**Problem Statement:**
Service providers like Leo (the music tutor) have a personal Google Calendar and need to ensure customers can't book over their existing personal appointments. They also need automatic meeting links generated without copy-pasting.

**Research Report:**
- **Evaluated Tools:** Cronofy, Nylas, Cal.com API.
- **Findings:** Cronofy offers a highly reliable, developer-friendly unified calendar API that supports Google, Outlook, and Apple calendars without needing to maintain separate OAuth apps for each.
- **Ease of Use:** Non-technical users just click "Connect Calendar", select their provider, and sign in.
- **Pricing:** Roughly $1/user/month depending on volume, which fits well into OHC's unit economics.
- **Timezone/Conflicts:** Cronofy handles all timezone logic and conflict resolution natively, making the OHC booking system much simpler to build.

**Design Doc:**
- **Trigger:** User sets up a "Service/Booking" product and is prompted to connect their personal calendar.
- **Actions:** OHC syncs free/busy times. When a customer books, OHC creates an event on the user's calendar and sends a calendar invite to the customer.
- **User View:** A "Availability" section where the user sees their synced calendars and sets working hours.

**Implementation Prompt:**
Integrate a unified calendar API (like Cronofy) to allow business owners to connect their Google/Outlook calendars. Update the booking system to check real-time availability against the synced calendar. When a booking occurs, push an event back to the owner's calendar. All calendar connections must be a simple 1-click OAuth flow.

**Priority:** P0
**Estimated Scope:** Medium

---

## 3. Email Marketing
### [Marketing] Integrate Resend for AI-Driven Email Campaigns

**Problem Statement:**
Priya wants to email her customer list when new boutique inventory arrives, but she finds Mailchimp too complex and expensive. She needs a simple way to tell her AI to "email my VIP customers about the new summer collection."

**Research Report:**
- **Evaluated Tools:** Resend, SendGrid, Mailgun.
- **Findings:** Resend is incredibly developer-friendly and offers excellent React Email templates which can be adapted for OHC. Deliverability is high, and spam compliance tracking is solid.
- **Ease of Use:** Completely invisible to the end user. They just type what they want to say, and OHC generates the email.
- **Pricing:** Generous free tier (3,000 emails/month), then $20/mo for 50k emails. Perfect for OHC's free-tier offering.
- **Spam Compliance:** Automatic handling of unsubscribes and bounce tracking.

**Design Doc:**
- **Trigger:** User asks the "Marketing" agent to send an announcement, or an automated trigger (e.g., "cart abandonment") fires.
- **Actions:** OHC formats the AI-drafted content into a beautiful HTML template and dispatches it via Resend to the filtered customer list.
- **User View:** A simple interface to approve AI-drafted emails, select an audience segment, and view basic stats (open rate, clicks).

**Implementation Prompt:**
Add email campaign capabilities to the Marketing agent. Users should be able to prompt the agent to draft a promotional email, review the drafted content in a clean mobile UI, and click "Send". Use a transactional email provider (like Resend) behind the scenes. Track and display basic open/click metrics for the campaign.

**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing
### [Payments] Integrate Mercado Pago for LATAM Expansion

**Problem Statement:**
Small businesses in Latin America cannot rely solely on Stripe due to high unbanked populations and regional payment preferences (like PIX in Brazil, or OXXO in Mexico). Without local payment options, they lose massive sales volume.

**Research Report:**
- **Evaluated Tools:** Mercado Pago, dLocal, Ebanx.
- **Findings:** Mercado Pago is the dominant player in LATAM. It offers a comprehensive suite of local payment methods and has a trusted consumer brand.
- **Ease of Use:** SMBs in LATAM already use Mercado Pago; linking their existing account is standard practice.
- **Pricing:** Varies by country, typically around 3-4% + fixed fee, which is standard for the region.
- **Failure Rate & Currency:** Supports local currencies and dramatically reduces failure rates compared to cross-border Stripe processing.

**Design Doc:**
- **Trigger:** A business owner based in a supported LATAM country goes to "Finance & Payments" and selects Mercado Pago as their provider.
- **Actions:** Checkout flows dynamically swap Stripe for Mercado Pago's checkout SDK/redirect based on the tenant's configuration.
- **User View:** Customers see familiar local payment options (PIX, Boleto) at checkout. The business owner sees their balance in local currency within OHC.

**Implementation Prompt:**
Implement Mercado Pago as an alternative payment provider to Stripe for merchants in LATAM. Create an onboarding flow for merchants to connect their Mercado Pago account. Update the storefront checkout experience to securely process payments using the local provider when configured, supporting native methods like PIX.

**Priority:** P2
**Estimated Scope:** Large

---

## 5. Shipping & Logistics
### [Shipping] Integrate Shippo for Real-Time Rates & Labels

**Problem Statement:**
Business owners shipping physical products struggle with calculating accurate shipping costs at checkout and manually copying addresses to buy postage. They need automated shipping rates and 1-click label generation.

**Research Report:**
- **Evaluated Tools:** Shippo, EasyPost, ShipStation.
- **Findings:** Shippo offers an excellent API for multi-carrier rates and label generation. It abstracts away carrier-specific complexities.
- **Ease of Use:** Users just pack their box, enter dimensions, and click "Buy Label".
- **Pricing:** Pay-as-you-go ($0.05/label) with discounted USPS/UPS rates, saving the merchant money immediately.
- **Carrier Coverage:** Excellent US and international carrier support.

**Design Doc:**
- **Trigger:** Customer reaches the checkout page (rates) -> Owner views a paid order in the Operations tab (labels).
- **Actions:** OHC calls Shippo to fetch live rates at checkout. Later, OHC requests a PDF label from Shippo when the owner purchases postage.
- **User View:** A "Fulfill Order" screen where the owner confirms package size, buys the label, and can print it directly from their phone or desktop.

**Implementation Prompt:**
Integrate a shipping API (like Shippo) to support real-time shipping rate calculation at checkout and 1-click shipping label generation from the order management screen. Provide a simple mobile UI for business owners to purchase postage and print labels directly.

**Priority:** P1
**Estimated Scope:** Medium

---

## 6. SMS & Notifications
### [Notifications] Integrate Twilio for Global SMS Alerts

**Problem Statement:**
Users like Fatima (Food Cart Operator) need immediate, loud notifications when an order arrives, especially if they have poor internet access or don't keep the app open. SMS is the most reliable fallback.

**Research Report:**
- **Evaluated Tools:** Twilio, MessageBird, Plivo.
- **Findings:** Twilio is the gold standard for global SMS delivery and reliability.
- **Ease of Use:** Fully invisible to the merchant. They just enter their phone number in OHC to receive alerts.
- **Pricing:** $0.0079/message in the US. Extremely cheap for the value it provides.
- **Compliance:** Built-in opt-out management (STOP handling) for customer-facing texts.

**Design Doc:**
- **Trigger:** A new high-priority event occurs (e.g., new food pre-order, urgent booking cancellation).
- **Actions:** The Operations agent dispatches an SMS payload via Twilio.
- **User View:** The owner receives a standard text message on their phone: "New OHC Order: $15 from John. Reply with ETA."

**Implementation Prompt:**
Implement Twilio SMS to send critical transactional alerts (like new orders or booking cancellations) to business owners. Allow owners to configure which events trigger an SMS in their notification settings. Ensure the integration supports global phone numbers and handles opt-outs automatically.

**Priority:** P1
**Estimated Scope:** Small

---

## 7. Video Conferencing
### [Conferencing] Integrate Zoom API for Auto-Generated Meeting Links

**Problem Statement:**
Online service providers (like Leo the tutor) waste time manually creating Zoom links for every booked lesson and emailing them to clients.

**Research Report:**
- **Evaluated Tools:** Zoom API, Google Meet API, Daily.co.
- **Findings:** Zoom is universally recognized by consumers. The API allows server-to-server OAuth to create meetings on behalf of connected users.
- **Ease of Use:** Merchant clicks "Connect Zoom". OHC handles the rest invisibly.
- **Pricing:** Free API usage for standard meeting creation on behalf of licensed Zoom users.
- **Join Experience:** World-class consumer familiarity.

**Design Doc:**
- **Trigger:** A customer books a service marked as "Online Meeting".
- **Actions:** OHC calls the Zoom API to generate a unique meeting room link and includes it in the calendar invite and confirmation email.
- **User View:** The booking details page shows a "Join Meeting" button for both the owner and the customer.

**Implementation Prompt:**
Add a Zoom integration allowing merchants to authenticate their Zoom accounts. Update the booking system to automatically generate unique Zoom meeting links for any service configured as an online meeting. Ensure the link is securely sent to the customer and visible on the merchant's dashboard.

**Priority:** P2
**Estimated Scope:** Medium
