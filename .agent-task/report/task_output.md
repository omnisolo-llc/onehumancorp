# Integrations Research Report: Q2

**Executive Summary:**
As the Principal Integrations Engineer, I have evaluated seven key third-party integrations to enhance the OneHumanCorp (OHC) platform. These integrations focus on solving critical pain points for our core SMB personas (e.g., Maya the Baker, Leo the Music Tutor) while adhering to our "Radical Simplicity" core value.

All evaluations consider the user experience first (requiring zero technical knowledge) and are designed to seamlessly empower our internal AI departments.

---

# Meta Graph API Integration

**Problem Statement:**
SMBs like Maya the Baker receive orders and inquiries across multiple platforms (Instagram DMs, WhatsApp, and Facebook Messenger). Managing multiple apps is overwhelming, leading to missed messages, slow response times, and lost revenue.

**Research Report:**
*   **Tool:** Meta Graph API (Instagram, Messenger, WhatsApp Business)
*   **Ease of Use (for SMB):** Excellent. Once connected, they never leave the OHC app. The "Ambassador" AI handles initial replies.
*   **Pricing:** WhatsApp has a cost-per-conversation model (first 1,000 free per month). Messenger and Instagram are largely free.
*   **Reputation/Reliability:** Industry standard, highly reliable webhook infrastructure.
*   **Cloud/Standalone:** Cloud mode is straightforward (OHC acts as the centralized webhook receiver). Standalone mode might be challenging due to Meta's app verification requirements for custom webhook endpoints, but OHC could act as a proxy.

**Design Doc:**
*   **Integration Point:** The user connects their Facebook/Instagram account via an OAuth popup in the OHC settings ("Connect Social Media").
*   **Trigger:** Incoming message webhook from Meta.
*   **Action:** OHC normalizes the message into a generic `InboundMessage` format. If it matches a known customer profile, it updates the conversation history. The "Ambassador" AI is triggered to evaluate if an auto-reply or action (like generating a quote) is needed.
*   **User View:** The business owner sees a unified "Inbox" screen showing the message, the platform icon (Instagram/WhatsApp), and a draft AI response ready to approve or edit.

**Implementation Prompt:**
Implement the Meta Graph API webhook listener and unified inbox UI. The user must be able to connect their Meta account, receive messages from Instagram/WhatsApp in a single OHC inbox screen, and see AI-drafted replies. Ensure the inbox updates in real-time.

**Priority:** P0
**Estimated Scope:** Large

---

# Nylas Calendar Sync Integration

**Problem Statement:**
Service-based SMBs like Leo the Music Tutor need to sync their OHC booking page with their personal Google or Outlook calendars. Without this, they risk double-booking if a client schedules a session during a personal appointment.

**Research Report:**
*   **Tool:** Nylas API
*   **Ease of Use (for SMB):** Very high. The user simply clicks "Connect Google Calendar" or "Connect Outlook" and authenticates.
*   **Pricing:** Around $1-2 per connected account per month. Needs to be factored into OHC's premium tiers or subsidized for free tier.
*   **Reputation/Reliability:** Strong reputation for normalizing complex calendar APIs (Google, Microsoft, Apple) into a single interface.
*   **Cloud/Standalone:** Fully supported in Cloud mode. In Standalone, Nylas acts as an intermediary SaaS.

**Design Doc:**
*   **Integration Point:** User settings -> "Calendars & Sync".
*   **Trigger:** A customer views the OHC booking page, or the business owner adds a manual block.
*   **Action:** OHC queries Nylas to get unified free/busy times. When a booking occurs in OHC, OHC tells Nylas to create a matching event in the user's primary personal calendar.
*   **User View:** The OHC calendar view shows both OHC bookings and personal calendar blocks (grayed out for privacy). Customers only see available slots.

**Implementation Prompt:**
Integrate the Nylas API for unified calendar synchronization. The user must be able to link an external calendar (Google/Outlook). The OHC booking system must then respect free/busy times from that external calendar, preventing double bookings.

**Priority:** P1
**Estimated Scope:** Medium

---

# Resend Email Campaign Integration

**Problem Statement:**
SMBs like Priya the Boutique Owner need a simple way to send marketing campaigns and transactional emails without managing a complex, separate platform like Mailchimp.

**Research Report:**
*   **Tool:** Resend API
*   **Ease of Use (for SMB):** High. The SMB doesn't interact with Resend directly. The OHC "Promoter" AI drafts the email campaign.
*   **Pricing:** Very developer-friendly, robust free tier for small volumes.
*   **Reputation/Reliability:** Excellent deliverability, modern API, built on AWS SES.
*   **Cloud/Standalone:** Cloud mode uses the primary OHC Resend account. Standalone mode can support custom Resend API keys or fallback to SMTP.

**Design Doc:**
*   **Integration Point:** The "Marketing" tab in the OHC app.
*   **Trigger:** Business owner approves an AI-drafted email campaign, or an automated trigger occurs (e.g., "Abandoned Cart" or "New Inventory").
*   **Action:** OHC compiles the email using React Email (or similar) into HTML and sends via Resend API.
*   **User View:** User sees an "Email Campaigns" dashboard with open rates and click rates synced back via Resend webhooks.

**Implementation Prompt:**
Integrate the Resend API to handle outbound email campaigns. Build a simple UI where the user can approve AI-generated newsletters and view basic metrics (sent, opened, clicked) retrieved from Resend webhooks.

**Priority:** P1
**Estimated Scope:** Medium

---

# Mercado Pago LATAM Support

**Problem Statement:**
Stripe is not universally available. LATAM users need a robust local payment processor that supports regional payment methods like Pix (Brazil), Boleto, and Oxxo to successfully run their online businesses.

**Research Report:**
*   **Tool:** Mercado Pago API
*   **Ease of Use (for SMB):** Familiar and trusted in LATAM. The onboarding is standard for regional businesses.
*   **Pricing:** Standard processing fees.
*   **Reputation/Reliability:** The dominant processor in South America.
*   **Cloud/Standalone:** Fully supported in both modes.

**Design Doc:**
*   **Integration Point:** User settings -> "Payments & Checkout".
*   **Trigger:** Customer attempts to check out.
*   **Action:** OHC routes the payment intent to Mercado Pago if the user configured it, handling specific flows (like showing a Pix QR code).
*   **User View:** The SMB sees Mercado Pago as an option alongside Stripe. The customer sees local payment methods at checkout.

**Implementation Prompt:**
Add Mercado Pago as a supported payment provider. Users should be able to connect their Mercado Pago account. The checkout flow must support generating a Pix QR code or Boleto and verifying payment success via webhooks.

**Priority:** P2
**Estimated Scope:** Large

---

# EasyPost Logistics API

**Problem Statement:**
SMBs shipping physical goods need real-time shipping rates during checkout and the ability to easily print shipping labels without integrating multiple carrier APIs (USPS, FedEx, UPS) manually.

**Research Report:**
*   **Tool:** EasyPost API
*   **Ease of Use (for SMB):** Very high. They connect their carrier accounts or use EasyPost default rates. OHC handles the complexity.
*   **Pricing:** A few cents per label.
*   **Reputation/Reliability:** Industry standard for unified logistics API.
*   **Cloud/Standalone:** Fully supported.

**Design Doc:**
*   **Integration Point:** "Shipping Settings" and the order fulfillment flow.
*   **Trigger:** Customer enters address at checkout (fetches rates); business owner clicks "Fulfill Order" (buys label).
*   **Action:** OHC queries EasyPost for rates, presents them to the customer, and later requests a printable PDF label from EasyPost.
*   **User View:** Business owner sees a "Print Label" button on the order detail page.

**Implementation Prompt:**
Integrate the EasyPost API to fetch live shipping rates at checkout and generate printable shipping labels. The business owner must be able to click "Fulfill" on an order and download a PDF shipping label.

**Priority:** P1
**Estimated Scope:** Medium

---

# Twilio SMS Notifications

**Problem Statement:**
SMBs like Fatima the Food Cart Operator may have poor mobile data connectivity or limited English, making push notifications or emails unreliable. They need robust, immediate SMS alerts for new orders.

**Research Report:**
*   **Tool:** Twilio API
*   **Ease of Use (for SMB):** Completely transparent. They just provide their phone number in OHC settings.
*   **Pricing:** Pay-per-message. OHC needs to manage A2P 10DLC compliance in the US.
*   **Reputation/Reliability:** The industry leader in programmable SMS.
*   **Cloud/Standalone:** Cloud mode uses OHC Twilio pool. Standalone can accept custom credentials.

**Design Doc:**
*   **Integration Point:** "Notifications" settings.
*   **Trigger:** A critical event occurs (e.g., "New Order Received").
*   **Action:** OHC calls Twilio API to dispatch an SMS to the business owner.
*   **User View:** Business owner receives a standard SMS: "New Order: 2x Falafel Wrap from John ($18). Reply ACCEPT to confirm."

**Implementation Prompt:**
Integrate the Twilio API to send critical SMS alerts (e.g., new orders) to the business owner. Include an option in the user settings to enable/disable SMS notifications and specify the destination phone number.

**Priority:** P2
**Estimated Scope:** Small

---

# Zoom API Video Conferencing

**Problem Statement:**
SMBs like Leo the Music Tutor need automated video link generation for their online lessons. Manually creating and emailing Zoom links for every booking is tedious and prone to error.

**Research Report:**
*   **Tool:** Zoom API (Server-to-Server or OAuth)
*   **Ease of Use (for SMB):** High. The user connects their Zoom account once.
*   **Pricing:** Requires the user to have a suitable Zoom plan, but the API integration itself is generally free.
*   **Reputation/Reliability:** Ubiquitous in online meetings.
*   **Cloud/Standalone:** Supported in both.

**Design Doc:**
*   **Integration Point:** "Integrations" -> "Zoom".
*   **Trigger:** A customer books an "Online Video" service type.
*   **Action:** OHC uses the Zoom API to create a scheduled meeting and attaches the join URL to the calendar invite and confirmation email.
*   **User View:** The business owner sees the Zoom link automatically populated in their booking dashboard.

**Implementation Prompt:**
Integrate the Zoom API to automatically generate meeting links when a customer books a virtual service. The generated link must be included in the booking confirmation UI and any related notification emails.

**Priority:** P2
**Estimated Scope:** Medium
