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

---

## 5. Social Media: Manychat Integration
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.

**Research Report**:
- **Tool**: Manychat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Excellent Instagram and WhatsApp API integrations. Robust webhook support for routing messages to OHC's backend. Extremely popular among SMBs for basic automation.
- **Risks**: Pricing scales with contacts, which may be expensive for high-volume, low-margin businesses. Requires Meta business verification for some features.
- **Pricing**: Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (would require local reverse proxy for webhooks, possible but complex).

**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Manychat's API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.

**Implementation Prompt**:
Implement an OAuth flow to connect a user's Instagram/Facebook account via Manychat. Create a webhook endpoint that receives incoming messages, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
*   **Priority**: P0
*   **Estimated Scope**: Large

---

## 6. Calendar: Calendly Integration
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly on their calendar.

**Research Report**:
- **Tool**: Calendly
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Industry standard, highly recognizable to customers. Excellent conflict resolution and timezone handling. Easy API integration.
- **Risks**: If a user cancels via Calendly directly instead of OHC, state might go out of sync without robust webhook handling.
- **Pricing**: Free tier available. Premium starts at $10/mo.
- **Compatibility**: Cloud (OAuth). Standalone (requires API key).

**Design Doc**:
- User goes to Sales dashboard and connects Calendly.
- OHC pulls available event types (e.g., "30-min Consultation") and displays them on the user's public storefront.
- When a customer clicks to book, they are shown the Calendly widget.
- Upon successful booking, a webhook notifies OHC to record the appointment in the Operations dashboard.

**Implementation Prompt**:
Create an integration that allows a user to connect their Calendly account. Fetch their existing event types and display a booking widget on their public profile page. Ensure booked events sync back to the OHC dashboard.
*   **Priority**: P1
*   **Estimated Scope**: Medium

---

## 7. Email Marketing: Mailchimp Integration
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns. She needs an automated way to email customers without leaving the OHC app.

**Research Report**:
- **Tool**: Mailchimp
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Market leader, great API, supports tags and segments. High deliverability.
- **Risks**: Strict anti-spam policies might suspend users if they import bad lists.
- **Pricing**: Free tier available (up to 500 contacts). Essentials starts at $13/mo.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

**Design Doc**:
- When a customer buys something, they are automatically added to the Mailchimp audience with tags (e.g., "Bought: Cake").
- The Marketing agent suggests campaigns ("Send an email to past customers about your new holiday cakes").
- The user approves the AI-generated email, and OHC triggers Mailchimp to send it.
- The user sees open rates and clicks in the OHC Marketing dashboard.

**Implementation Prompt**:
Build an integration that syncs OHC customers to a Mailchimp audience automatically after purchase. Allow the AI Marketing agent to create and send email campaigns via the Mailchimp API.
*   **Priority**: P1
*   **Estimated Scope**: Medium

---

## 8. Shipping: Shippo Integration
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label.

**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get rates.
- The Operations agent shows the cheapest shipping option.
- The user clicks "Buy Label", and OHC downloads the PDF label for printing.
- OHC automatically emails the customer the tracking number.

**Implementation Prompt**:
Connect the Shippo API to fetch shipping rates based on order weight/dimensions. Allow the user to purchase a label and automatically email the tracking link to the customer.
*   **Priority**: P1
*   **Estimated Scope**: Large

---

## 9. SMS & Notifications: Twilio Integration
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable SMS alerts when a new pre-order arrives so she can start cooking.

**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Global coverage, incredibly reliable. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

**Design Doc**:
- Users can enable "SMS Notifications" in the "Operations" settings.
- When an order is placed, the OHC backend triggers a Twilio API call to text the business owner.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.

**Implementation Prompt**:
Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure compliance with local messaging regulations.
*   **Priority**: P0
*   **Estimated Scope**: Small

---

## 10. Video Conferencing: Zoom Integration
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically when a lesson is booked.

**Research Report**:
- **Tool**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Strong API for meeting creation.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: Free tier (40-min limit). Pro starts at $15/mo.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

**Design Doc**:
- User connects their Zoom account via the Sales dashboard.
- When a customer books an online service (e.g., via Calendly or native booking), OHC calls the Zoom API to create a meeting.
- The Zoom link is embedded in the automated calendar invite and confirmation email sent to the customer.

**Implementation Prompt**:
Create an OAuth integration with Zoom. Automatically generate a unique Zoom meeting link when a customer books a virtual service, and include this link in the customer's confirmation email.
*   **Priority**: P1
*   **Estimated Scope**: Medium

---

## 11. Social Media: Ayrshare Integration
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations.
**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn).
- Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- Pricing: Free tier available, then scales per user.
- Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies.
- Non-technical users benefit by never leaving the OHC interface.
- Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.
**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
**Priority**: P1
**Estimated Scope**: Large

---

## 12. Calendar & Scheduling: Cal.com Integration
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs with their personal Google Calendar seamlessly.
**Research Report**:
- Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box.
- It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- Free tier available for individuals; great for our free tier users.
- Alternative is building from scratch, which is error-prone.
**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect their Google/Outlook calendar via a one-click OAuth button in the "Operations" tab.
- When a customer books a slot on the OHC public page, Cal.com manages the calendar event and conflict resolution transparently.
**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
**Priority**: P0
**Estimated Scope**: Medium

---

## 13. Email Marketing: Listmonk Integration
**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month."
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- It is lightweight (Go + PostgreSQL), aligning perfectly with the OHC backend stack.
- Zero extra SaaS costs for OHC Standalone users; minimal scaling costs for Cloud.
- Simplifies list management and supports template-based sending without complex drag-and-drop builders.
**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the HTML, Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium

---

## 14. Shipping & Logistics: EasyPost Integration
**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**: Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number.
**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).
- Competitive pricing (free tier for low volume, pennies per label after).
- Abstracts away complex carrier-specific APIs and handles tracking webhooks.
- Great fit for OHC physical product merchants.
**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
**Priority**: P1
**Estimated Scope**: Medium

---

## 15. Video Conferencing: Jitsi Integration
**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**: Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room.
**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.
- Requires no account for the student. Works natively in the browser and mobile.
- OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription.
- Completely seamless integration with no technical setup required by the user.
**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
**Priority**: P2
**Estimated Scope**: Small

---

## 16. Payment Processing: Mercado Pago
**Title**: Expand Payments with Mercado Pago for LATAM Users
**Problem Statement**: Non-US users in Latin America cannot rely solely on Stripe due to high fees, lack of local currency support, and specific local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- Mercado Pago is the dominant payment gateway in LATAM.
- Supports local payment methods which are critical for conversion (often >50% of transactions).
- API is well-documented. Settlement times are faster locally compared to cross-border Stripe.
- Works for both Cloud (via OHC platform account) and Standalone (user supplies API keys).
**Design Doc**:
- In the "Finance & Payments" settings, users select their region. If in LATAM, Mercado Pago is highlighted as the recommended provider.
- Setup involves standard OAuth flow or API key drop-in.
- Supports one-off payments and split payments for the eventual marketplace feature.
**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
**Priority**: P1
**Estimated Scope**: Large
