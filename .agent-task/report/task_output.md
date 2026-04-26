# Scout Research Report: Tool Integration Evaluation

This report evaluates tools across 7 core categories to expand OneHumanCorp's (OHC) capabilities for small business owners. Each brief focuses on the user-facing outcome and integration considerations.

---

## 1. Social Media Integration: Unified Inbox

### Title: Integrate Unified Inbox for Instagram, Facebook, and WhatsApp
**Problem Statement:** Business owners like Maya receive orders and inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Managing these separately causes missed opportunities and slow response times. A non-technical user needs one central inbox where their AI agent can draft replies.
**Research Report:** Meta's Graph API provides the necessary endpoints to aggregate messages from Instagram, Facebook, and WhatsApp. It is the industry standard for this use case. However, the OAuth process can be intimidating for non-technical users, requiring clear, guided onboarding. Pricing is generally free for the API itself, but WhatsApp Business API has per-conversation pricing that needs to be abstracted or passed through transparently. It supports both Cloud and Standalone modes, provided the OAuth application is configured correctly.
**Design Doc:**
- **Trigger:** User connects their Meta account via a guided "Connect Social Media" wizard in the Operations department.
- **Action:** Webhooks subscribe to incoming messages. The "Customer Success" AI agent reads incoming messages, drafts a reply, and places it in the unified OHC inbox.
- **User View:** A single "Inbox" tab showing all conversations, with the source icon (Instagram, WhatsApp) and suggested AI replies ready to send.
**Implementation Prompt:** Implement a unified inbox feature allowing users to connect their Meta accounts. Incoming messages from Instagram, Facebook, and WhatsApp must appear in a single view within the OHC app. The Customer Success agent must be able to draft replies to these messages.
**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling: Booking Engine

### Title: Integrate Cal.com for Service Booking and Calendar Sync
**Problem Statement:** Service providers like Carlos and Leo need a way for customers to book time slots without back-and-forth emails. The system must prevent double-booking with their personal calendars.
**Research Report:** Cal.com is an open-source scheduling tool with a robust API and embeddable booking pages. It is significantly more developer-friendly than building a scheduling engine from scratch and offers a white-label API. It handles timezone complexity and calendar sync (Google, Outlook, Apple). It offers a free tier for individuals and a scalable API pricing model. Given its open-source nature, it is highly suitable for both Cloud and Standalone deployments.
**Design Doc:**
- **Trigger:** User sets up a "Service" in the Sales & Acquisition department.
- **Action:** OHC provisions a Cal.com event type via API behind the scenes.
- **User View:** The business owner sets their availability in a simple weekly grid in OHC. Customers see a beautiful, embeddable booking widget on the OHC-generated website.
**Implementation Prompt:** Integrate Cal.com via API to power OHC's booking engine. Users must be able to define their availability and connect their personal calendar (Google/Outlook) to prevent conflicts. The booking widget must be embedded seamlessly into their OHC storefront.
**Priority:** P0
**Estimated Scope:** Large

---

## 3. Email Marketing: Campaign Manager

### Title: Integrate Resend for Transactional and Marketing Emails
**Problem Statement:** Business owners like Priya need to send beautiful, professional emails (order confirmations, newsletter blasts) without managing complex tools like Mailchimp. They need high deliverability with zero setup.
**Research Report:** Resend offers a developer-first API for sending emails with excellent deliverability and a React Email templating engine. It abstracts away the complexity of SMTP and domain warming. Pricing is very startup-friendly with a generous free tier (3,000 emails/month). It focuses purely on sending, meaning OHC must build the campaign management UI, which fits perfectly with our "invisible tools" philosophy. It works well in Cloud mode; Standalone users may need to provide their own API key if exceeding generous limits.
**Design Doc:**
- **Trigger:** The Marketing & Advertising agent decides to send a promotional email, or the Operations agent sends a receipt.
- **Action:** OHC compiles the email using React Email templates and sends via Resend API.
- **User View:** The user sees a "Campaigns" tab where they can review AI-drafted emails and click "Send". They see open/click rates inline.
**Implementation Prompt:** Replace any basic SMTP implementations with Resend for all outbound emails. Build a simple UI for users to approve and send AI-drafted email campaigns to their customer list. Ensure tracking for open and click rates is surfaced in the Business Advisory reports.
**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing: Global Expansion

### Title: Integrate Mercado Pago for LATAM Payments
**Problem Statement:** While Stripe covers many regions, users in LATAM (e.g., Brazil, Argentina, Mexico) require local payment methods (like PIX in Brazil) and local currency settlement that Stripe may not fully optimize or support cheaply.
**Research Report:** Mercado Pago is the dominant payment processor in Latin America, offering extensive support for local payment methods, installments, and local regulatory compliance. The API is modern and well-documented. Pricing is competitive for the region. Integration is crucial for capturing the massive SMB market in LATAM. It supports both Cloud and Standalone modes.
**Design Doc:**
- **Trigger:** User selects LATAM country during onboarding.
- **Action:** OHC configures the checkout flow to use Mercado Pago instead of Stripe.
- **User View:** The checkout page natively displays local payment options (e.g., PIX, local credit cards with installments) seamlessly.
**Implementation Prompt:** Implement Mercado Pago as an alternative payment gateway to Stripe. The system must route checkout sessions to Mercado Pago based on the user's business location and customer currency. Ensure webhooks are handled securely to update order status.
**Priority:** P1
**Estimated Scope:** Medium

---

## 5. Shipping & Logistics: Automated Fulfillment

### Title: Integrate EasyPost for Multi-Carrier Shipping Integration
**Problem Statement:** Sellers of physical goods need to calculate shipping costs at checkout and print labels easily without manually re-entering addresses into carrier websites.
**Research Report:** EasyPost aggregates 100+ carriers (USPS, FedEx, UPS, international) into a single API. It handles rating, label generation, and tracking. It drastically simplifies the logistics stack compared to integrating individual carriers. Pricing is very attractive (free for USPS, pennies for others). It is highly reliable and fits both Cloud and Standalone modes well.
**Design Doc:**
- **Trigger:** Customer reaches the checkout step for a physical product; later, business owner clicks "Fulfill Order".
- **Action:** OHC calls EasyPost API to fetch live rates; later, calls it to purchase and generate a PDF label.
- **User View:** The customer sees accurate shipping rates at checkout. The business owner clicks one button to print a shipping label directly from the OHC app.
**Implementation Prompt:** Integrate EasyPost to provide real-time shipping quotes during checkout and one-click label generation in the order management UI. Ensure tracking numbers are automatically synced to the order and sent to the customer via the Customer Success agent.
**Priority:** P1
**Estimated Scope:** Medium

---

## 6. SMS & Notifications: Global Reach

### Title: Integrate Twilio for Global SMS Notifications
**Problem Statement:** Users like Fatima (food cart) may not have reliable data connections or smartphones running the OHC app constantly. They need immediate, reliable SMS notifications for new pre-orders.
**Research Report:** Twilio is the industry leader for programmable SMS. It offers global reach and high deliverability. While powerful, the regulatory requirements (A2P 10DLC in the US) add complexity for onboarding small businesses. OHC will need to handle this complexity transparently. Pricing is per-message. Works in both Cloud and Standalone.
**Design Doc:**
- **Trigger:** A new high-priority event occurs (e.g., new order placed for immediate pickup).
- **Action:** OHC calls Twilio API to send an SMS to the business owner's verified phone number.
- **User View:** The business owner receives a standard text message: "New Order #123: 2x Falafel Wrap. Pickup in 15m."
**Implementation Prompt:** Integrate Twilio to send critical operational alerts (like new orders) via SMS to the business owner. Implement a settings panel where users can opt-in to SMS alerts and verify their phone numbers.
**Priority:** P2
**Estimated Scope:** Medium

---

## 7. Video Conferencing: Virtual Services

### Title: Integrate Google Meet for Automated Virtual Appointments
**Problem Statement:** Users like Leo (music tutor) need a seamless way to generate video call links for their booked lessons without manually creating calendar events and copying links.
**Research Report:** Google Meet (via Google Workspace/Calendar API) is ubiquitous, free for basic use, and highly familiar to most users. It is simpler to integrate for basic 1:1 meetings than Zoom, which requires a separate app installation for many users. The primary challenge is the OAuth flow to access the user's calendar to generate the Meet link. Works well in both Cloud and Standalone modes.
**Design Doc:**
- **Trigger:** A customer successfully books a virtual service.
- **Action:** OHC (via Cal.com integration or directly) creates a Google Calendar event with a Meet link attached.
- **User View:** The customer and business owner both receive an email confirmation containing the "Join Video Call" button, and the link appears in the OHC dashboard's upcoming schedule.
**Implementation Prompt:** Enable automated generation of Google Meet links for any service configured as "Virtual". The generated link must be included in the calendar invites and accessible from the booking details view in the OHC app.
**Priority:** P2
**Estimated Scope:** Small
