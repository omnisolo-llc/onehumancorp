# OHC Scout: Core Tool Integrations Research

## 1. Social Media Integration

### Title: Unified Social Media Inbox & Auto-Responder (Instagram/Facebook/WhatsApp/TikTok)

**Problem Statement:**
Business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Jumping between apps to answer questions like "Do you do vegan cakes?" or "Is this in stock?" is overwhelming and leads to lost sales when they sleep or are busy working.

**Research Report:**
- **Market Analysis:** Social commerce is the primary driver of top-of-funnel leads for modern SMBs.
- **Candidates:**
  - *Meta Graph API (Instagram/Messenger/WhatsApp):* The undeniable leader. Complex OAuth and webhook setup, but mandatory for any real social integration. Free to use (API calls), though WhatsApp has per-conversation pricing after a free tier.
  - *TikTok Shop/API:* Rapidly growing for product discovery.
  - *ManyChat / Chatfuel (Competitors/Alternatives):* Good for technical users, but too complex for our personas to configure conversational flows.
- **Evaluation for OHC:** We must integrate directly with Meta Graph API and TikTok API to pull messages into a unified OHC Inbox. The complexity of OAuth, token refreshing, and webhook signature verification must be completely abstracted. For the user, it should just be "Connect Instagram" -> Login -> Done.
- **Pricing:** API access is free. WhatsApp Business API has conversation-based pricing.
- **Cloud/Standalone:** Webhooks require public endpoints. Cloud mode can receive webhooks easily. Standalone mode might need a polling fallback or a local tunnel (ngrok equivalent) for webhooks, or rely solely on polling if the API permits, which Meta graph API often restricts.

**Design Doc:**
- **Trigger:** User clicks "Connect Instagram/Facebook" in the Marketing/Customer Success settings.
- **Action:** Standard OAuth flow opens. Once connected, OHC subscribes to webhooks for DMs and comments.
- **User Interface:** A new "Unified Inbox" appears in the Customer Success tab. The "Customer Success - Ambassador" AI agent automatically drafts responses based on the business's data (inventory, FAQs, calendar) and can optionally auto-reply while the user sleeps.

**Implementation Prompt:**
Implement a Unified Inbox feature that allows a user to securely authenticate with their Instagram and Facebook accounts. Once connected, incoming DMs and comments should appear in a single chronological feed within the OHC app. The Customer Success AI agent should generate suggested replies for every incoming message, and the user should be able to configure an "Auto-reply when I'm sleeping" toggle.
- **Priority:** `P0`
- **Estimated Scope:** Large


## 2. Calendar & Scheduling

### Title: Smart Calendar Sync & Booking Automation

**Problem Statement:**
Service providers like Carlos (Handyman) and Leo (Music Tutor) live and die by their schedules. Double-booking a client or forgetting to send a meeting link damages their reputation. They need a simple way to show their availability and let customers book without endless back-and-forth texts.

**Research Report:**
- **Market Analysis:** Appointment scheduling is a saturated market (Calendly, Acuity, Cal.com).
- **Candidates:**
  - *Cal.com (Open Source):* Excellent API, modern architecture, very customizable. Can be self-hosted (great for Standalone).
  - *Google Calendar API:* The source of truth for most personal calendars. Essential for two-way sync to prevent double booking.
  - *Calendly API:* Popular, but expensive and restrictive for deep white-label integration.
- **Evaluation for OHC:** We need our own native booking UI that syncs deeply with Google Calendar (the user's source of truth). Cal.com could be used under the hood as an infrastructure provider, or we build a lightweight scheduling engine syncing via Google Calendar API. To keep things radical simple, users just authorize Google Calendar, and OHC handles availability.
- **Pricing:** Google Calendar API is free (within generous limits). Cal.com has commercial licensing or self-hosted options.
- **Cloud/Standalone:** Works perfectly in both.

**Design Doc:**
- **Trigger:** User enables "Services & Bookings" on a product.
- **Action:** User is prompted to connect Google Calendar.
- **User Interface:** The storefront displays a clean, mobile-optimized date/time picker (only showing available slots). When a customer books, OHC writes the event to the user's Google Calendar and creates a booking record in OHC. The Operations AI agent sends booking confirmations and reminders.

**Implementation Prompt:**
Build a seamless scheduling flow where a user can connect their Google Calendar to define their true availability. When creating a Service product, the user specifies duration and buffer times. The storefront should render a native, mobile-friendly date/time picker. Upon booking, the event must instantly sync to the user's Google Calendar, and automated confirmation emails should be dispatched.
- **Priority:** `P0`
- **Estimated Scope:** Medium


## 3. Email Marketing

### Title: AI-Driven Customer Lifecycle Emails

**Problem Statement:**
Business owners know they should email their customers ("New Arrivals!", "Book again!"), but writing newsletters is time-consuming, and setting up Mailchimp is too complicated. Priya needs a way to automatically email her best customers when new stock arrives, without writing a single line of copy.

**Research Report:**
- **Market Analysis:** Mailchimp and Klaviyo dominate but are feature-bloated.
- **Candidates:**
  - *Resend:* Developer-friendly, exceptional deliverability, modern API, built for React/modern stacks but works well everywhere.
  - *SendGrid / AWS SES:* Robust, but very legacy APIs.
  - *Postmark:* Great for transactional, less focused on marketing blasts.
- **Evaluation for OHC:** Resend is the best fit for our modern stack. We abstract the email provider entirely. The business owner never sees "Resend". They just see "Send an update to my customers." The Marketing AI agent generates the email copy and design.
- **Pricing:** Resend has a generous free tier (3,000 emails/mo), then scales linearly.
- **Cloud/Standalone:** Cloud can use OHC's pooled Resend account. Standalone users will need to provide their own Resend/SendGrid API key in settings.

**Design Doc:**
- **Trigger:** The Marketing AI agent suggests: "You have 10 new products. Want me to email your 50 past customers?" User clicks "Yes, preview."
- **Action:** AI drafts the email with product images. User hits "Send."
- **User Interface:** A "Campaigns" tab in the Marketing department showing open rates and revenue generated from the email.

**Implementation Prompt:**
Create an automated email marketing engine powered by the Marketing AI agent. The system should identify opportunities (e.g., new products added, abandoned carts) and proactively present the user with fully drafted, beautiful email campaigns. The user only needs to click 'Approve and Send'. Track and display simple metrics: Sent, Opened, and Sales from Email.
- **Priority:** `P1`
- **Estimated Scope:** Medium


## 4. Payment Processing

### Title: Localized Global Payments Integration

**Problem Statement:**
While Stripe is fantastic, it doesn't cover every country or local payment preference. A seller in Brazil needs Mercado Pago; a seller in India needs UPI/Razorpay. If OHC is "for everyone," we must support local payment methods transparently.

**Research Report:**
- **Market Analysis:** Stripe is the baseline, but local wallets often have higher conversion rates in specific regions.
- **Candidates:**
  - *Mercado Pago:* Dominant in LATAM. Pix support is critical in Brazil.
  - *Razorpay:* Dominant in India (UPI support).
  - *PayPal:* Global fallback, though often disliked for high fees.
- **Evaluation for OHC:** We need a unified "Payment Gateway" interface in the backend. When a user creates their account and sets their country, OHC automatically provisions the correct localized gateway (e.g., Stripe for US/EU, Mercado Pago for LATAM) without the user needing to compare providers.
- **Pricing:** Standard processing fees (usually 2.9% + 30c or local equivalent).
- **Cloud/Standalone:** Cloud handles webhook routing. Standalone users connect their own gateway accounts via API keys.

**Design Doc:**
- **Trigger:** User onboarding / setting up the store.
- **Action:** Based on the user's region, the best payment provider is selected.
- **User Interface:** A simple "Get Paid" settings page where the user verifies their identity and connects their bank account. The complexity of the specific gateway is hidden.

**Implementation Prompt:**
Abstract the payment processing layer to seamlessly support regional payment gateways beyond Stripe, starting with Mercado Pago for Latin America. The onboarding flow should automatically recommend and configure the appropriate gateway based on the user's business address. The storefront checkout must natively render the appropriate local payment methods (e.g., Pix, UPI) flawlessly on mobile devices.
- **Priority:** `P1`
- **Estimated Scope:** Large


## 5. Shipping & Logistics

### Title: One-Click Shipping Labels & Real-Time Rates

**Problem Statement:**
For physical product sellers, shipping is a nightmare. Figuring out box sizes, comparing USPS vs UPS rates, and manually typing addresses into a separate label printer website takes hours. Maya needs to print a label and get a tracking number with one tap.

**Research Report:**
- **Market Analysis:** Shippo and EasyPost are the main API providers bridging the gap to carriers.
- **Candidates:**
  - *Shippo API:* Very startup-friendly, clean API, great international support.
  - *EasyPost API:* Robust, highly reliable.
- **Evaluation for OHC:** Shippo is ideal for our use case. We can integrate real-time rates at checkout (so the customer pays the exact shipping cost) and provide a one-click "Buy Label & Print" button in the OHC Operations dashboard.
- **Pricing:** Shippo charges ~5 cents per label plus the actual postage cost.
- **Cloud/Standalone:** Works identically via API keys.

**Design Doc:**
- **Trigger:** A customer places an order requiring shipping.
- **Action:** Operations AI verifies the address and calculates the cheapest label.
- **User Interface:** In the Order detail view, a prominent "Buy Shipping Label ($4.50)" button. Clicking it generates a PDF optimized for 4x6 thermal printers and standard A4 printers. Tracking info is automatically emailed to the customer.

**Implementation Prompt:**
Integrate a shipping API (like Shippo) to automate logistics for physical products. At checkout, dynamically calculate shipping costs based on the customer's address and the product's weight. In the order management dashboard, allow the business owner to purchase and download a printable shipping label with a single click. Automate the tracking number notification to the customer.
- **Priority:** `P1`
- **Estimated Scope:** Medium


## 6. SMS & Notifications

### Title: Global SMS Order Alerts & Customer Updates

**Problem Statement:**
Fatima (Food Cart Operator) is busy cooking; she doesn't have time to refresh an app or check email. She needs a loud, immediate SMS ping when a pre-order comes in. Her customers also appreciate an SMS saying "Your food is ready for pickup!"

**Research Report:**
- **Market Analysis:** Twilio is the industry standard, but MessageBird and Plivo offer aggressive pricing globally.
- **Candidates:**
  - *Twilio:* Most reliable, easiest to integrate, massive global coverage.
  - *MessageBird / Plivo:* Good alternatives for cost-sensitive international routes.
- **Evaluation for OHC:** Twilio is the safest bet for global reliability. We need SMS for two primary flows: 1) Urgent alerts to the business owner (New Order, Cancelled Order), 2) Status updates to the customer (Shipped, Ready for Pickup).
- **Pricing:** Varies by country. US is ~$0.0079/msg.
- **Cloud/Standalone:** Cloud uses OHC Twilio pool. Standalone needs a custom Twilio SID/Auth Token.

**Design Doc:**
- **Trigger:** Order state changes (Created, Ready).
- **Action:** Dispatch SMS via Twilio API.
- **User Interface:** In notification settings, a simple toggle: "Text me when I get a new order." For the customer, a checkbox at checkout: "Send me order updates via text."

**Implementation Prompt:**
Implement a reliable SMS notification engine using Twilio. Provide business owners with a toggle to receive instant text message alerts for new orders and cancellations. Additionally, add an opt-in at checkout allowing customers to receive critical order status updates (e.g., 'Ready for Pickup', 'Shipped') via SMS.
- **Priority:** `P2`
- **Estimated Scope:** Small


## 7. Video Conferencing

### Title: Auto-Generated Meeting Links for Services

**Problem Statement:**
Leo (Music Tutor) sells online guitar lessons. When a student books a time, Leo currently has to manually create a Zoom link, email it to the student, and add it to his calendar. It's error-prone and looks unprofessional if he forgets.

**Research Report:**
- **Market Analysis:** Zoom and Google Meet are the primary tools.
- **Candidates:**
  - *Google Meet API:* Included free if the user connects their Google Calendar. The simplest, zero-friction option.
  - *Zoom API:* Widespread, but requires a separate OAuth connection.
- **Evaluation for OHC:** Since we are already integrating Google Calendar (see Category 2), we can just append `conferenceData` to the Google Calendar API insert request to auto-generate a Google Meet link for free, with zero extra configuration. Zoom can be added later if requested, but Meet solves 90% of the problem instantly.
- **Pricing:** Google Meet generation via Calendar API is free.
- **Cloud/Standalone:** Perfectly identical.

**Design Doc:**
- **Trigger:** A customer books an online service.
- **Action:** Google Calendar API is called with `conferenceDataVersion=1` to generate a Meet link.
- **User Interface:** The booking confirmation screen and email prominently display the "Join Video Call" link. Leo's internal schedule also shows the link next to the appointment.

**Implementation Prompt:**
Enhance the scheduling system to automatically generate video conferencing links for 'Online' service bookings. Leverage the Google Calendar integration to instantly create a Google Meet link upon successful booking. Distribute this link automatically in the customer's confirmation email, calendar invite, and the business owner's internal appointment dashboard.
- **Priority:** `P2`
- **Estimated Scope:** Small
