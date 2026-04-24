# 🔍 Scout: Tool Integration Research [Q2]

This report contains integration research and issue briefs for tools across 7 key categories to expand the capabilities of OneHumanCorp (OHC) and empower non-technical small business owners.

---

## 1. Social Media Integration: ManyChat
**Title**: Integrate ManyChat for Unified Instagram & Facebook Direct Message Automation
**Problem Statement**:
Maya (The Home Baker) receives dozens of Instagram DMs asking "do you do vegan cakes?" and "how much for a custom cake?" while she sleeps. Managing these manually across multiple apps is overwhelming, leading to missed orders and slow response times. A non-technical user needs a unified inbox that leverages AI to draft responses automatically.

**Research Report**:
- **Tool**: ManyChat
- **Target Persona**: Maya (The Home Baker), Priya (The Boutique Owner)
- **Ease of Use**: Very high. ManyChat already abstracts complex Meta Graph APIs. For OHC users, it will be a 1-click OAuth connection.
- **Pricing**: Free tier available; Pro starts at $15/mo.
- **Reputation**: Industry leader in IG/FB messaging automation.
- **Analysis**: Connecting ManyChat allows OHC's "Customer Success" AI agent to intercept incoming social media messages, draft replies, and push them to the user's unified OHC inbox.
- **Cloud vs Standalone**: Works in Cloud mode (OAuth flow). In Standalone mode, requires the user to provide their own ManyChat API key.
- **Advantages**: Solves the biggest pain point for social-first sellers. Huge time saver.
- **Risks**: Meta's strict 24-hour messaging window rule for automated replies requires careful compliance handling.

**Design Doc**:
- **Triggers**: User connects their Instagram/Facebook account via OHC Settings. An incoming DM triggers a webhook to OHC.
- **Actions**: The OHC Customer Success Agent reads the message, generates a suggested reply based on the business's data (e.g., cake catalog), and places it in the unified OHC Mobile Inbox.
- **User Interface**: A simple "Connect Instagram" button in the Marketing department. The Inbox screen shows a unified feed of messages with "AI Draft" bubbles ready for the user to approve and send.

**Implementation Prompt**:
Implement a 1-click ManyChat OAuth integration that syncs Instagram and Facebook DMs into the OHC unified inbox. When a message is received, the Customer Success AI agent must generate a draft response visible to the business owner in the OHC mobile app. The owner can tap "Approve" to send the reply back to Instagram. Ensure errors regarding Meta's 24-hour reply window are caught and displayed clearly in simple language.
**Priority**: P1
**Estimated Scope**: Medium

---

## 2. Calendar & Scheduling: Calendly
**Title**: Integrate Calendly for Automated Service Bookings & Meeting Generation
**Problem Statement**:
Leo (The Music Tutor) and Carlos (The Freelance Handyman) struggle with back-and-forth emails to find a time that works for their clients. They need a simple booking page that syncs with their personal Google Calendar to prevent double-booking and automatically generates meeting links for online sessions.

**Research Report**:
- **Tool**: Calendly
- **Target Persona**: Leo (The Music Tutor), Carlos (The Freelance Handyman)
- **Ease of Use**: Excellent. Non-technical users understand "share my link to book."
- **Pricing**: Free basic tier; Premium starts at $10/mo.
- **Reputation**: Gold standard for scheduling.
- **Analysis**: Calendly provides robust APIs for retrieving available slots, booking events, and generating dynamic meeting links (Zoom/Meet). It perfectly abstracts timezone math and calendar conflict resolution.
- **Cloud vs Standalone**: Works perfectly in Cloud mode via OAuth. Standalone requires Personal Access Tokens.
- **Advantages**: Instant reduction in admin work. Built-in timezone handling prevents missed appointments.
- **Risks**: Free tier limitations might restrict some advanced webhook features needed for OHC's real-time sync.

**Design Doc**:
- **Triggers**: User clicks "Set up Bookings" in the Operations department. A client books a slot on the OHC public storefront.
- **Actions**: OHC creates a Calendly event behind the scenes. If it's a virtual service (like Leo's lessons), an online meeting link is generated and sent via email.
- **User Interface**: The OHC app shows a daily agenda view. The public storefront displays a clean calendar widget where clients can pick an available date and time.

**Implementation Prompt**:
Integrate Calendly to power the OHC booking system. The business owner should be able to connect their Google Calendar, set working hours, and publish a booking widget on their OHC storefront. When a client books, the slot must immediately reflect as "Busy" in the OHC Operations dashboard, and an automated email confirmation with calendar invites must be sent to both parties.
**Priority**: P0
**Estimated Scope**: Large

---

## 3. Email Marketing: MailerLite
**Title**: Integrate MailerLite for AI-Driven Customer Retention Campaigns
**Problem Statement**:
Priya (The Boutique Owner) wants to notify past customers when new seasonal clothing arrives, but she finds tools like Mailchimp too complex and full of marketing jargon. She needs an invisible tool that takes her new product photos and auto-generates a beautiful email newsletter.

**Research Report**:
- **Tool**: MailerLite
- **Target Persona**: Priya (The Boutique Owner)
- **Ease of Use**: Extremely high. Cleaner and simpler API than Mailchimp for basic list management and campaign sending.
- **Pricing**: Free up to 1,000 subscribers; very affordable paid tiers.
- **Reputation**: Loved by small businesses for simplicity and great deliverability.
- **Analysis**: Integrating MailerLite allows the "Marketing & Advertising" AI agent to sync the OHC customer list, draft visual emails based on new store inventory, and schedule them for sending without the user needing to touch an email builder.
- **Cloud vs Standalone**: Cloud via OAuth. Standalone via standard API keys.
- **Advantages**: Exceptional free tier value. Excellent drag-and-drop HTML API generation.
- **Risks**: Strict anti-spam approval process for new accounts may cause friction during the initial user onboarding.

**Design Doc**:
- **Triggers**: A new product is added to the catalog, or a customer makes a purchase (adding them to the list).
- **Actions**: OHC syncs the new customer email to a MailerLite list. The AI Marketing Agent drafts an email campaign announcing the new product.
- **User Interface**: In the Marketing tab, the user sees a card: "Draft Email: New Arrivals." They tap it, see a preview of the email with their product photos, and tap "Send to 150 customers."

**Implementation Prompt**:
Build an integration with MailerLite that automatically syncs OHC customer emails to a master subscriber list. Enable the AI Marketing Agent to generate and stage HTML email campaigns using new inventory data. The business owner must be able to review the AI-generated email in the OHC app and send it with a single tap. Ensure simple error handling for bounced emails or unsubscribes.
**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing: Mercado Pago
**Title**: Integrate Mercado Pago for LATAM Localized Payments
**Problem Statement**:
Small business owners in Latin America cannot rely solely on Stripe due to limited availability or high cross-border fees. They need a localized payment gateway that supports local cards, installments (cuotas), and bank transfers (like PIX in Brazil).

**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Any persona operating in LATAM (e.g., Carlos the Handyman in Mexico, Priya the Boutique Owner in Brazil).
- **Ease of Use**: Standard for LATAM users; widely trusted by local consumers.
- **Pricing**: Transaction fees vary by country (e.g., ~3-4% + fixed fee).
- **Reputation**: The dominant fintech and payment processor in Latin America.
- **Analysis**: Adding Mercado Pago unlocks the massive LATAM small business market for OHC. It supports local payment methods that Stripe lacks in certain regions, crucial for conversion.
- **Cloud vs Standalone**: Works in both via standard API key configuration or OAuth marketplace connect.
- **Advantages**: Massive market expansion. Critical for local trust and conversion rates in LATAM.
- **Risks**: Highly fragmented API capabilities depending on the specific LATAM country.

**Design Doc**:
- **Triggers**: User sets their business country to a LATAM region during onboarding.
- **Actions**: OHC prompts the user to connect Mercado Pago instead of Stripe. Checkout flows dynamically route to Mercado Pago's checkout Pro or transparent checkout.
- **User Interface**: In the Finance & Payments department, users see their Mercado Pago balance and a toggle to enable "Installments" (cuotas) on their public storefront.

**Implementation Prompt**:
Implement Mercado Pago as an alternative payment gateway to Stripe for OHC users in Latin America. The checkout experience on the public storefront must support local payment methods (e.g., PIX, local credit cards with installments). Ensure the Finance Agent can read transaction statuses (pending, paid, failed) from Mercado Pago webhooks and update the OHC order status accordingly.
**Priority**: P2
**Estimated Scope**: Large

---

## 5. Shipping & Logistics: Shippo
**Title**: Integrate Shippo for 1-Click Shipping Label Generation and Tracking
**Problem Statement**:
Priya (The Boutique Owner) wastes hours copying addresses from her sales dashboard into carrier websites to buy shipping labels. She needs to tap a button, get a printable label, and automatically send a tracking number to the customer.

**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (The Boutique Owner), Maya (The Home Baker - if shipping nationwide).
- **Ease of Use**: High. Shippo aggregates dozens of carriers (USPS, UPS, FedEx) into one simple interface.
- **Pricing**: Pay-as-you-go (5¢ per label) or free tier with default carrier accounts.
- **Reputation**: Highly developer-friendly and reliable for SMBs.
- **Analysis**: Shippo allows OHC to instantly calculate shipping rates at checkout and generate PDF labels from the Operations dashboard. It also provides standardized webhook tracking updates.
- **Cloud vs Standalone**: Works well in both via API keys.
- **Advantages**: Instantly solves the physical product fulfillment nightmare. Drastically reduces customer "where is my order" inquiries via auto-tracking.
- **Risks**: Accurate package weight/dimension data is required from the user, which non-technical users often skip or guess wrong.

**Design Doc**:
- **Triggers**: A physical product order is marked "Paid". Carrier tracking status changes to "Delivered".
- **Actions**: OHC Operations Agent requests a shipping label from Shippo. When the package is shipped, the Customer Success Agent emails the tracking link.
- **User Interface**: On the Order Details screen, a "Buy Shipping Label" button appears. Tapping it shows the cheapest USPS/UPS rate. Confirming it generates a PDF that can be printed directly from the phone.

**Implementation Prompt**:
Integrate Shippo to enable real-time shipping rate calculation at checkout and 1-click label purchasing in the OHC Operations dashboard. When an owner buys a label, automatically attach the tracking number to the order and trigger the Customer Success Agent to notify the buyer. Handle cases where the user's provided box dimensions are missing by prompting for default box sizes.
**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications: Twilio
**Title**: Integrate Twilio for Critical SMS Order Alerts and Pre-order Notifications
**Problem Statement**:
Fatima (The Food Cart Operator) works in a noisy environment with a slow data connection. She might miss an app push notification for an incoming pre-order. She needs a reliable SMS text message the second a customer pays for a pickup order.

**Research Report**:
- **Tool**: Twilio (Programmable SMS)
- **Target Persona**: Fatima (The Food Cart Operator)
- **Ease of Use**: Invisible to the user. OHC handles the backend integration.
- **Pricing**: Very cheap per message (~$0.0079 per SMS in the US).
- **Reputation**: The undisputed industry standard for programmable communications.
- **Analysis**: Twilio will be used by the Operations Agent to dispatch high-priority alerts to the business owner, and by the Customer Success Agent to send pickup-ready texts to buyers.
- **Cloud vs Standalone**: Cloud uses OHC's master Twilio account (billed via OHC metrics). Standalone requires the user's own Twilio SID/Token.
- **Advantages**: Near 100% deliverability. Does not rely on the user having a good 4G/5G data connection (crucial for food carts).
- **Risks**: Strict A2P 10DLC compliance rules in the US require business registration to avoid message filtering.

**Design Doc**:
- **Triggers**: A new rush order is placed. An order is marked "Ready for Pickup".
- **Actions**: OHC sends an SMS to the business owner (New Order alert) and later sends an SMS to the customer ("Your food is ready!").
- **User Interface**: In Settings -> Notifications, Fatima can toggle "Send me a text message for new orders." The customer checkout screen includes a field for "Phone number for SMS updates."

**Implementation Prompt**:
Integrate Twilio SMS to provide critical, offline-resilient notifications. Implement a toggle for business owners to receive SMS alerts for new orders. Add functionality for the Operations Agent to send automated "Order Ready for Pickup" text messages to customers. Ensure all phone numbers are validated and formatted to E.164 standard before sending.
**Priority**: P0
**Estimated Scope**: Medium

---

## 7. Video Conferencing: Zoom
**Title**: Integrate Zoom for Auto-Generated Online Consultation Links
**Problem Statement**:
Leo (The Music Tutor) offers virtual guitar lessons. Currently, he manually creates a Zoom meeting and emails the link to the student after they book, which is error-prone and unprofessional. He needs the link generated instantly upon booking.

**Research Report**:
- **Tool**: Zoom (Server-to-Server OAuth / App Marketplace)
- **Target Persona**: Leo (The Music Tutor), Carlos (The Freelance Handyman - for virtual quotes).
- **Ease of Use**: High. "Connect Zoom" is a familiar pattern.
- **Pricing**: Free tier allows 40-minute meetings; Pro is $15/mo.
- **Reputation**: The default video conferencing tool globally.
- **Analysis**: Works in tandem with the scheduling integration. When a virtual service is booked, OHC calls the Zoom API to create a meeting and injects the `join_url` into the calendar invite and confirmation email.
- **Cloud vs Standalone**: Cloud uses an OHC Zoom Marketplace App. Standalone requires developer credentials.
- **Advantages**: Seamless professional experience for service businesses.
- **Risks**: Zoom's API rate limits and strict token expiration policies require robust background job handling and token refreshes.

**Design Doc**:
- **Triggers**: A service marked as "Virtual/Online" is successfully booked by a client.
- **Actions**: OHC requests a new meeting via the Zoom API, retrieves the join link and passcode, and stores it in the booking record.
- **User Interface**: In the service creation form, Leo selects "Location: Online (Zoom)". In the agenda view, the booking shows a large "Start Zoom Meeting" button for the owner to tap when it's time.

**Implementation Prompt**:
Build a Zoom OAuth integration that allows users to link their Zoom accounts. When a client books a service designated as "Virtual", automatically generate a unique Zoom meeting link and password. Display a "Start Meeting" button in the OHC daily agenda for the business owner, and ensure the student receives the join link in their automated confirmation email. Handle automatic token refreshing securely in the background.
**Priority**: P1
**Estimated Scope**: Medium
