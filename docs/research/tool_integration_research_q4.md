# 🔍 Tool Integration Research Q4

## [Social Media Integration] Unified Inbox via Meta Graph API
**Problem Statement:** Maya (The Home Baker) gets orders via Instagram DMs, Facebook Comments, and WhatsApp. Managing multiple apps is overwhelming and she misses orders while sleeping.
**Research Report:** Meta Graph API is the official way to access messages from Meta's platforms. It requires OAuth approval and business verification. It supports webhooks for real-time messages.
- **Ease of Use:** Non-technical users just click "Connect with Facebook/Instagram" and grant permissions.
- **Pricing:** Free for basic Graph API usage. WhatsApp Business API has per-conversation pricing (varies by region, roughly $0.01 - $0.08/conversation).
- **Advantages:** Official API, highest reliability, covers 3 major platforms.
- **Risks:** Meta's review process for app approval can be tedious. Token expiration requires handling.
- **Modes:** Works well in Cloud mode. Standalone mode might require a proxy or users providing their own Meta App credentials, which is highly technical. Might need OHC Cloud as a relay for Standalone users.
**Design Doc:**
- **Trigger:** User navigates to Settings -> Integrations -> "Connect Social Accounts".
- **Action:** OAuth flow initiates. Once connected, OHC subscribes to webhooks for DMs/Comments. Incoming messages are routed to the user's unified inbox in OHC. The Customer Success AI agent drafts replies based on context.
- **User Experience:** A simple unified chat interface in the OHC app where Maya can see an Instagram DM, reply, and the response is sent back to Instagram natively.
**Implementation Prompt:** Implement the "Connect Meta" button in the Settings view. After connection, display a unified inbox screen that shows incoming Instagram DMs and Facebook Comments in a single feed. Ensure the Customer Success AI automatically drafts a reply for unread messages, which the user can approve and send.
**Priority:** P0
**Estimated Scope:** Large

## [Calendar & Scheduling] Seamless Booking via Cal.com
**Problem Statement:** Leo (The Music Tutor) needs students to book lessons online based on his real-time availability, avoiding double bookings.
**Research Report:** Cal.com is an open-source scheduling infrastructure.
- **Ease of Use:** Users just connect their Google/Outlook calendar and set working hours.
- **Pricing:** Free for basic individual use. API usage might have platform fees depending on volume (Platform tier).
- **Advantages:** Open-source, highly customizable, handles complex timezone logic and conflicts automatically.
- **Risks:** Overkill for simple bookings, API rate limits.
- **Modes:** Works perfectly in both Cloud and Standalone modes (as Standalone can communicate directly with Cal.com API).
**Design Doc:**
- **Trigger:** User creates a "Service" product type and toggles "Enable Booking".
- **Action:** User connects their external calendar. OHC syncs availability using Cal.com infrastructure.
- **User Experience:** Customer sees a beautiful date/time picker on Leo's public page. Once booked, Leo gets a calendar invite, and OHC blocks out that time.
**Implementation Prompt:** Add a "Enable Booking" toggle for Service products. When enabled, show a calendar connection flow. On the public storefront, render a Cal.com-powered date/time picker for that service. Ensure a successful booking blocks the time on the user's connected calendar.
**Priority:** P1
**Estimated Scope:** Medium

## [Email Marketing] Automated Campaigns via Resend
**Problem Statement:** Priya (The Boutique Owner) wants to email her customer list when new stock arrives, but finds Mailchimp too complex to design templates.
**Research Report:** Resend is a developer-first email API that is incredibly fast and supports React Email for beautiful templates.
- **Ease of Use:** For the business owner, they just type text or ask the AI to "draft an email about the new summer collection". The complex HTML template generation is hidden.
- **Pricing:** Free up to 3,000 emails/month. $20/mo for 50,000 emails. Very affordable.
- **Advantages:** Extremely fast delivery, great deliverability rates, modern API.
- **Risks:** Requires domain verification (DNS records) which is hard for non-technical users. OHC might need to send from a shared OHC domain (e.g., `priya@stores.onehumancorp.com`) by default.
- **Modes:** Works well in Cloud. Standalone users would need their own Resend API key.
**Design Doc:**
- **Trigger:** User goes to "Marketing" tab and clicks "Send Email Broadcast".
- **Action:** Marketing AI drafts the email content. Upon approval, Resend API dispatches the emails to the customer list.
- **User Experience:** Priya sees a simple text editor and an AI prompt box. She reviews the generated preview, clicks "Send", and sees a progress bar.
**Implementation Prompt:** Create an "Email Broadcast" screen in the Marketing tab. Allow the user to input a prompt for the AI to generate an email. Show a preview. On "Send", use the Resend API to dispatch the email to all registered customers of that tenant. Show delivery metrics (sent, opened) on the dashboard later.
**Priority:** P1
**Estimated Scope:** Medium

## [Payment Processing] LATAM Payments via Mercado Pago
**Problem Statement:** Carlos (The Freelance Handyman) operates in Mexico where credit card penetration is lower, and many customers prefer paying via OXXO cash deposits or local bank transfers. Stripe doesn't support all local methods efficiently.
**Research Report:** Mercado Pago is the leading payment gateway in Latin America.
- **Ease of Use:** Business owners create a Mercado Pago account and link it to OHC.
- **Pricing:** Varies by country, typically around 3.49% + fixed fee per transaction.
- **Advantages:** Dominant in LATAM, supports cash payments (OXXO, Boleto), QR code payments, and local credit cards.
- **Risks:** API documentation is sometimes fragmented. Webhook reliability can vary.
- **Modes:** Works in both Cloud and Standalone modes.
**Design Doc:**
- **Trigger:** User in LATAM selects "Mercado Pago" as their payment provider during setup.
- **Action:** User authenticates via OAuth. Checkout flows redirect to Mercado Pago or use their transparent checkout API.
- **User Experience:** Customers see local, trusted payment options at checkout. Carlos sees the funds in his Mercado Pago dashboard and OHC marks the invoice as "Paid" via webhook.
**Implementation Prompt:** Add Mercado Pago to the Payment Providers settings. Implement the OAuth connection flow. In the checkout UI, if Mercado Pago is the active provider, render their payment widget supporting local payment methods. Handle the `payment.created` webhook to update the order status in OHC.
**Priority:** P2
**Estimated Scope:** Large

## [Shipping & Logistics] Real-time Rates & Labels via Shippo
**Problem Statement:** Maya (The Home Baker) wants to ship non-perishable cookies nationwide. She doesn't know how much to charge for shipping or how to print USPS labels.
**Research Report:** Shippo provides a unified API for 85+ carriers (USPS, UPS, FedEx, DHL, etc.).
- **Ease of Use:** Owner just enters package weight/dimensions. Shippo handles the rest.
- **Pricing:** Free basic tier (just pay for postage). $0.05 per label if using own carrier accounts. Very cheap for small businesses.
- **Advantages:** Abstract away carrier complexities. Prints standard 4x6 labels easily.
- **Risks:** Accurate weights/dimensions are required from the user, which they often get wrong leading to undercharged shipping.
- **Modes:** Works well in Cloud. Standalone mode users would need to provide their own API key.
**Design Doc:**
- **Trigger:** Customer reaches checkout for a physical product. Or, owner clicks "Fulfill Order".
- **Action:** OHC fetches real-time rates based on cart contents via Shippo. When fulfilled, OHC generates a printable PDF label via Shippo API.
- **User Experience:** At checkout, the customer sees accurate shipping costs. When Maya is ready to ship, she clicks "Print Label", and a PDF downloads immediately.
**Implementation Prompt:** Integrate Shippo API to fetch real-time shipping rates during the checkout flow based on product weights and the destination address. Add a "Print Shipping Label" button to the Order details screen that generates and downloads a USPS label PDF.
**Priority:** P1
**Estimated Scope:** Medium

## [SMS & Notifications] Global Messaging via Twilio
**Problem Statement:** Fatima (The Food Cart Operator) needs to receive a text message immediately when a pre-order is placed, because she doesn't always have the OHC app open or reliable internet for push notifications.
**Research Report:** Twilio is the industry standard for SMS and voice APIs.
- **Ease of Use:** Completely invisible to the business owner. They just input their phone number.
- **Pricing:** ~$0.0079 per SMS in the US, varies globally.
- **Advantages:** Extremely reliable, global reach.
- **Risks:** Spam regulations (A2P 10DLC in the US) make automated SMS complex and require business registration, which is a massive hurdle for micro-businesses.
- **Modes:** Cloud only (OHC manages the Twilio account and bills the user). Standalone would require the user to navigate Twilio's complex setup.
**Design Doc:**
- **Trigger:** A customer places an order.
- **Action:** The Operations AI agent uses Twilio API to send an SMS to Fatima's verified phone number.
- **User Experience:** Fatima receives a standard text message: "New Order #123: 2x Chicken Halal Plate. Total: $24. Pickup at 12:30 PM."
**Implementation Prompt:** Add a "Receive SMS Notifications" toggle in the Notifications settings. When an order is placed, check this setting and use the Twilio API to send a formatted summary SMS to the business owner's registered phone number.
**Priority:** P2
**Estimated Scope:** Small

## [Video Conferencing] Auto-Meeting Links via Zoom API
**Problem Statement:** Leo (The Music Tutor) teaches guitar online. Manually creating a Zoom link for every booking and emailing it to the student is tedious and error-prone.
**Research Report:** Zoom API allows programmatic creation of meetings.
- **Ease of Use:** Owner connects their Zoom account once via OAuth.
- **Pricing:** Free API access, but features depend on the user's Zoom plan.
- **Advantages:** Zoom is universally recognized by customers.
- **Risks:** OAuth token lifecycle management. Zoom API has rate limits.
- **Modes:** Works in Cloud and Standalone modes.
**Design Doc:**
- **Trigger:** A student completes a booking for an "Online Service".
- **Action:** OHC calls Zoom API to create a scheduled meeting and attaches the `join_url` to the calendar invite and confirmation email.
- **User Experience:** Leo sees the booking in his calendar with a Zoom link. The student receives an email with the exact Zoom link. Neither had to manually copy-paste anything.
**Implementation Prompt:** Add a Zoom integration option in Settings. For services marked "Online Video", automatically generate a Zoom meeting link via the Zoom API upon successful booking. Display this link in the confirmation UI and include it in the automated confirmation email.
**Priority:** P2
**Estimated Scope:** Medium
