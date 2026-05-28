# Tool Integration Research Report (Q1 2025)

## 1. Social Media Integration: Meta Business Suite (Instagram, Facebook, WhatsApp)
**Problem Statement:**
Small business owners (like Maya the Baker) suffer from "Operational Fatigue" (Pain Point #2) trying to manage inquiries across Instagram DMs, Facebook comments, and WhatsApp. They lose sales when they miss messages while sleeping or working ("Communication Lag" - Pain Point #8). They need a single, unified inbox where their AI Ambassador can seamlessly read and reply to cross-platform messages without them having to switch apps constantly.

**Research Report:**
*   **Tool:** Meta Business Suite / Messenger API for Instagram and WhatsApp Business API.
*   **Persona Value:** Extremely high. Instagram and WhatsApp are the primary sales channels for home businesses (Maya), food carts (Fatima), and local services (Carlos) worldwide.
*   **Ease of Use:** For the end-user, the initial OAuth connection via Facebook Login is a standard flow. Once connected, the complexity is entirely hidden.
*   **Pricing:** Receiving messages is generally free; sending messages via WhatsApp Business API has per-conversation pricing depending on the region (first 1000 service conversations are usually free monthly). IG/FB messaging is free.
*   **Cloud vs Standalone:**
    *   **Cloud:** Excellent fit. OHC can register a centralized Meta App and handle webhooks for all tenants, routing messages via the event mesh.
    *   **Standalone:** Challenging. Meta requires public webhook URLs for real-time delivery. Standalone users would either need a relay service provided by OHC, or they would have to rely on polling (if supported) which adds latency.

**Design Doc:**
*   **Integration Trigger:** User visits "Customer Success" department settings and clicks "Connect Instagram / WhatsApp".
*   **User Flow:** User completes Meta's OAuth consent screen. OHC links the connected Facebook Page/Instagram Professional account to the tenant ID.
*   **Action Flow:** Meta sends webhooks to OHC when a new DM arrives. OHC routes this to the unified Customer Inbox. The Ambassador agent analyzes the message and drafts a reply. The user approves it (or it auto-replies based on settings), and OHC calls the Meta Send API to reply to the customer on their native app.

**Implementation Prompt:**
Implement the integration with Meta Business Suite to aggregate Instagram DMs, Facebook messages, and WhatsApp chats into a single OHC inbox. The user should be able to authenticate their social accounts with one click. The AI Ambassador must be able to read incoming messages from these channels, draft context-aware responses, and send replies back to the original platform (e.g., replying to an Instagram DM directly from the OHC dashboard).
*   **Priority:** P0 (Critical for addressing Communication Lag)
*   **Estimated Scope:** Large

---

## 2. Calendar & Scheduling: Google Workspace (Calendar + Meet)
**Problem Statement:**
Service providers (like Carlos the Handyman and Leo the Music Tutor) struggle to coordinate bookings and generate meeting links manually. They need an automated system where customers can pick an available slot, and the system instantly books the calendar and provides a video link without double-booking or manual email back-and-forth.

**Research Report:**
*   **Tool:** Google Calendar API and Google Meet (via Google Workspace).
*   **Persona Value:** High. Leo needs this for online lessons; Carlos needs this to avoid double-booking in-person repair visits.
*   **Ease of Use:** Google OAuth is universally understood. Non-technical users are very familiar with Google Calendar.
*   **Pricing:** Free for basic calendar syncing. Generating Google Meet links programmatically requires the user to have a Google Workspace account.
*   **Cloud vs Standalone:** Works seamlessly in both environments as it relies on standard OAuth 2.0 and REST API calls. Standalone requires secure local storage of OAuth refresh tokens.

**Design Doc:**
*   **Integration Trigger:** User enables "Online Bookings" in the Operations department and clicks "Sync Google Calendar".
*   **User Flow:** User completes Google OAuth, granting calendar read/write permissions.
*   **Action Flow:** OHC reads the user's free/busy schedule to display available slots on their public storefront. When a customer books a slot, OHC creates a Calendar Event, optionally attaches a generated Google Meet link (if it's an online service), and adds the customer as an attendee so they receive the invite automatically.

**Implementation Prompt:**
Build a two-way sync integration with Google Calendar. The system must fetch the business owner's free/busy times to dynamically update availability on their OHC booking page. When a new booking is made, it must create an event in the owner's Google Calendar and automatically generate and attach a Google Meet link for virtual services (like tutoring). Ensure calendar conflicts are strictly prevented.
*   **Priority:** P1
*   **Estimated Scope:** Medium

---

## 3. Payment Processing: Mercado Pago (Alternative for LATAM)
**Problem Statement:**
While Stripe is powerful, it is not the dominant or most accessible payment gateway in Latin America. Business owners in LATAM need a local solution that supports regional payment methods (like Pix in Brazil, or OXXO cash payments in Mexico) with faster settlement times and familiar localized checkout experiences.

**Research Report:**
*   **Tool:** Mercado Pago API.
*   **Persona Value:** Critical for LATAM expansion. A food cart operator or freelancer in Brazil/Mexico will face high friction and conversion drops if restricted to Stripe/Credit Cards only.
*   **Ease of Use:** Mercado Pago provides highly localized checkout UI components (Checkout Pro/API) that consumers trust. Setup requires linking a Mercado Pago account, which is ubiquitous in the region.
*   **Pricing:** Transaction fees vary by country (e.g., in Mexico it can range from 3.49% to higher depending on the installment plan and payment method). It is standard for the region.
*   **Cloud vs Standalone:** Works in both. Webhooks for payment status updates (IPN) require a public endpoint, meaning Standalone setups need a webhook relay or polling mechanism, similar to the Stripe integration.

**Design Doc:**
*   **Integration Trigger:** User goes to Finance & Payments settings, sets their region to a LATAM country, and selects "Mercado Pago" as their payment provider.
*   **User Flow:** User authenticates via Mercado Pago OAuth to link their seller account.
*   **Action Flow:** At checkout, LATAM customers are presented with the Mercado Pago checkout flow (supporting local methods like Pix or OXXO). Upon successful payment, Mercado Pago sends a webhook to OHC, which triggers the Operations department to mark the order as paid and notify the business owner.

**Implementation Prompt:**
Integrate Mercado Pago as an alternative payment provider to Stripe, specifically targeting LATAM users. The integration must support creating payment preferences, handling the checkout redirect/embedded UI, and processing Webhook/IPN notifications to update order statuses in OHC. Ensure support for local payment methods like Pix (Brazil) and OXXO (Mexico) within the checkout flow.
*   **Priority:** P1 (High for global reach)
*   **Estimated Scope:** Large

---

## 4. Email Marketing: Twilio SendGrid
**Problem Statement:**
Business owners like Priya (Boutique Owner) want to send automated emails when new stock arrives, but suffer from "Marketing Dread" (Pain Point #3) and "Cost Creep" (Pain Point #6) when forced to use expensive, overly complex 3rd party tools like Mailchimp. They need a simple, reliable way to send broadcast emails and automated notifications directly from their OHC dashboard.

**Research Report:**
*   **Tool:** Twilio SendGrid Email API & Marketing Campaigns.
*   **Persona Value:** High. Essential for customer retention, automated receipts, and promotional blasts.
*   **Ease of Use:** Extremely simple for the end-user, as OHC abstracts the complexity. The AI handles drafting the emails, and SendGrid handles the delivery.
*   **Pricing:** Very generous free tier (100 emails/day forever). Paid plans are competitive and scale well.
*   **Cloud vs Standalone:**
    *   **Cloud:** Perfect fit. OHC can manage a master SendGrid account and use Subusers or Sender Authentication for tenants to maintain reputation isolation.
    *   **Standalone:** Requires the user to bring their own SendGrid API key, which introduces slight technical friction but is manageable with clear instructions.

**Design Doc:**
*   **Integration Trigger:** OHC uses SendGrid under the hood for transactional emails. For marketing, the user simply asks the Promoter agent to "email all past customers about the summer sale".
*   **User Flow:** The user reviews the AI-generated email draft and audience list, then clicks "Send".
*   **Action Flow:** OHC compiles the HTML email and uses the SendGrid API to dispatch the messages. Delivery events (opens, clicks, bounces) are received via SendGrid Event Webhooks and displayed in the OHC Analytics dashboard in plain language.

**Implementation Prompt:**
Integrate Twilio SendGrid to power both transactional and marketing emails for OHC users. Implement the Email API to handle sending. The system must allow the AI Promoter agent to draft broadcast emails, present them to the user for one-click approval, and then dispatch them via SendGrid. Additionally, implement webhook handlers to track open and click rates so the Business Advisory agent can report on campaign success in plain language.
*   **Priority:** P1
*   **Estimated Scope:** Medium
