# Scout: Tool Integration Research Q3

## Social Media Integration: ManyChat
**Problem Statement:** Small business owners struggle to keep up with DMs and comments across Instagram, Facebook, and WhatsApp. Missing a message often means losing a sale.
**Research Report:** ManyChat is a leading chat automation tool. It connects directly with Meta's APIs. For non-technical users, it offers a visual drag-and-drop builder to create automated replies and FAQs. Pricing starts free, with a $15/mo Pro tier. It is well-regarded but requires a Facebook page connection, which can be tricky for some users.
**Design Doc:** OHC integrates by capturing webhook events from ManyChat or linking directly to a ManyChat account. A unified "Inbox" view in OHC aggregates these conversations, allowing the business owner to reply directly from OHC or let the automation handle it.
**Implementation Prompt:** Provide a UI in the "Integrations" tab to "Connect Social Accounts." Once connected, show a unified inbox with a "reply" box that successfully sends messages back through the connected platform.
**Priority:** P0
**Estimated Scope:** Large
**Cloud vs Standalone:** Works well in Cloud via webhooks. For Standalone, it requires a secure tunnel (like ngrok) or polling to receive webhooks from ManyChat.

## Calendar & Scheduling: Calendly
**Problem Statement:** Booking appointments (consultations, lessons, services) involves endless back-and-forth emails to find a time that works, frustrating both the business owner and the client.
**Research Report:** Calendly is the industry standard for scheduling. It syncs with Google, Outlook, and iCloud calendars. It handles timezones automatically and offers a simple, shareable booking link. The free tier is sufficient for single-event types; paid plans start around $10/mo. Extremely high ease-of-use for both owners and their clients.
**Design Doc:** OHC allows users to embed their Calendly booking page directly into their OHC-hosted storefront or client portal. The integration uses Calendly's webhooks to notify OHC when a new booking is made, automatically updating the client's CRM record.
**Implementation Prompt:** Add a "Scheduling" settings page where users can paste their Calendly personal link. Display the Calendly embed widget on the user's public-facing OHC page. Display upcoming bookings on the OHC dashboard.
**Priority:** P0
**Estimated Scope:** Medium
**Cloud vs Standalone:** Fully supported in both.

## Email Marketing: Mailchimp
**Problem Statement:** Sending newsletters or promotions manually via standard email clients is tedious, looks unprofessional, and risks being flagged as spam. Business owners need an easy way to email their customer list.
**Research Report:** Mailchimp is highly recognizable. It offers a generous free tier (though shrinking) and an intuitive drag-and-drop email builder. It handles unsubscribe compliance automatically. Paid plans start at roughly $13/mo. It can be overwhelming with advanced features, but the core campaign flow is straightforward.
**Design Doc:** OHC acts as the source of truth for customer data. OHC syncs the "Contacts" list one-way to a Mailchimp Audience. The business owner creates and sends emails within Mailchimp, but OHC displays high-level stats (open rate, clicks) on the dashboard.
**Implementation Prompt:** Implement an "Export/Sync to Mailchimp" button in the Contacts view. Use OAuth to connect the account. Show a summary widget on the dashboard displaying the latest campaign's performance metrics.
**Priority:** P1
**Estimated Scope:** Medium
**Cloud vs Standalone:** Fully supported in both.

## Payment Processing: Mercado Pago
**Problem Statement:** Stripe is not available or preferred everywhere. In LATAM, business owners need a reliable, widely accepted local payment method to reduce cart abandonment and accept local payment types (like PIX in Brazil).
**Research Report:** Mercado Pago is ubiquitous in Latin America. It supports local cards, bank transfers, and cash payments (e.g., OXXO, Boleto). It provides a checkout pro (hosted by them) or an API for custom checkouts. Settlement is reliable. Fees vary by country but are competitive locally.
**Design Doc:** OHC checkout offers "Mercado Pago" alongside other methods. When selected, the user is redirected to the Mercado Pago hosted checkout (simplest for compliance), and upon success, redirected back to OHC with the payment status.
**Implementation Prompt:** Add a "Mercado Pago" toggle in Payment Settings requiring their public/access keys. During checkout, if selected, initiate a preference and redirect the buyer to the Mercado Pago payment URL. Handle the success/failure return URLs to update the order status.
**Priority:** P1
**Estimated Scope:** Large
**Cloud vs Standalone:** Fully supported in both.

## Shipping & Logistics: Shippo
**Problem Statement:** Calculating accurate shipping rates and buying/printing labels manually is slow and error-prone, eating into profit margins and time.
**Research Report:** Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL, etc.) and provides discounted rates. The UI is clean, and the API is robust. It offers a pay-as-you-go model (cents per label) which is perfect for small businesses without volume to justify a monthly subscription.
**Design Doc:** OHC integrates with Shippo to fetch live rates during checkout based on cart weight/dimensions. In the OHC order management view, the owner clicks "Buy Label," which uses Shippo to generate the PDF and automatically updates the order with the tracking number.
**Implementation Prompt:** Integrate Shippo to provide live shipping rates on the checkout page. Add a "Generate Label" button in the Order Details page that fetches a printable PDF label and displays the tracking link.
**Priority:** P1
**Estimated Scope:** Large
**Cloud vs Standalone:** Fully supported in both.

## SMS & Notifications: Twilio
**Problem Statement:** Emails get missed. For urgent updates (like appointment reminders or order pickups), small business owners need a way to reach customers directly on their phones.
**Research Report:** Twilio is the foundational API for SMS. While it requires some technical setup to buy a number and configure messaging services (A2P 10DLC compliance is a hurdle), it offers global reach and high reliability. Pricing is per-message (fractions of a cent).
**Design Doc:** OHC abstracts Twilio's complexity. The business owner simply toggles "Enable SMS Reminders" in OHC. OHC handles the API calls to Twilio to send automated transactional messages (e.g., "Your order is ready!").
**Implementation Prompt:** Add a "Notifications" setting where users can toggle SMS alerts for specific events (e.g., order confirmed, appointment reminder). When triggered, send the SMS via Twilio to the customer's registered phone number.
**Priority:** P2
**Estimated Scope:** Medium
**Cloud vs Standalone:** Fully supported in both.

## Video Conferencing: Zoom
**Problem Statement:** Tutors, consultants, and coaches need to manually create and email meeting links for every booked session, causing confusion if the link gets lost or forgotten.
**Research Report:** Zoom is universally recognized. It offers a robust API. The free tier allows 40-minute meetings, while Pro ($15/mo) removes the limit. Integration requires an OAuth app approval process which can be stringent.
**Design Doc:** Tied closely to the Scheduling integration. When an online appointment is booked, OHC calls the Zoom API to generate a unique meeting link. This link is automatically included in the calendar invite and the OHC dashboard for both the owner and the client.
**Implementation Prompt:** Allow users to "Connect Zoom." When creating a service type, allow selecting "Zoom" as the location. Upon booking, automatically generate a Zoom meeting and display the join link on the appointment details page.
**Priority:** P2
**Estimated Scope:** Medium
**Cloud vs Standalone:** Fully supported in both.
