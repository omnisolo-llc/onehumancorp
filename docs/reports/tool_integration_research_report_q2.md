# OHC Tool Integration Research Report Q2

## 1. Social Media Integration: Meta Graph API (Instagram/Facebook)
**Title:** Meta Graph API Integration for Unified Social Inbox
**Problem Statement:** Small business owners struggle to keep up with customer messages scattered across Instagram, Facebook, and WhatsApp. It's easy to miss inquiries or sales opportunities because they have to constantly switch between apps.
**Research Report:** Meta provides the Graph API, which allows accessing Instagram Direct Messages, Facebook Messenger, and WhatsApp Business API. Integrating this directly provides a reliable, official path for two-way communication. Competitors like ManyChat or Ayrshare exist, but direct Meta integration saves the SMB owner third-party subscription fees and gives us more control over the user experience.
- **Pricing Estimate:** Free for basic Instagram/Messenger API limits. WhatsApp Business API is roughly $0.01 to $0.08 per conversation depending on region and message type.
- **Cloud vs Standalone:** Fully compatible with both Cloud (via centralized OAuth application) and Standalone modes (via direct API keys or a bridge relay).
**Design Doc:** OHC will provide a "Connect Socials" button in the dashboard. Upon clicking, the user goes through the standard Meta OAuth flow to grant OHC permission to read and reply to messages. Incoming messages from any connected Meta platform will appear in the OHC unified inbox, and replies from the OHC dashboard will be sent back to the respective platform.
**Implementation Prompt:** Implement a unified inbox feature that allows users to connect their Instagram and Facebook accounts. Users should be able to view incoming messages from these platforms in a single feed within the OHC dashboard and reply directly. Ensure a smooth onboarding flow for connecting accounts.
**Priority:** P0
**Estimated Scope:** Large

## 2. Calendar & Scheduling: Google Calendar
**Title:** Google Calendar Sync for Seamless Booking
**Problem Statement:** Business owners waste time playing phone tag or exchanging emails just to find a time to meet with clients. They also risk double-booking themselves if their personal calendar isn't synced with their business availability.
**Research Report:** Google Calendar is ubiquitous. A direct integration using the Google Calendar API enables two-way sync. Other options like Calendly or Cal.com are great but require users to set up and manage another account. Direct Google Calendar integration keeps the user in the OHC ecosystem and reduces friction.
- **Pricing Estimate:** Free (API usage falls well within generous free tier limits for individual users).
- **Cloud vs Standalone:** Fully compatible with both Cloud (central OAuth) and Standalone modes (local OAuth credentials).
**Design Doc:** Users can authenticate their Google account from the OHC settings. Once connected, OHC will read the user's availability from their Google Calendar to present open slots to clients on the OHC booking page. When a client books a slot, OHC will automatically create an event on the user's Google Calendar.
**Implementation Prompt:** Build a feature allowing users to connect their Google Calendar. The system should read free/busy times to prevent double-booking and automatically add new appointments scheduled through OHC directly to the user's Google Calendar.
**Priority:** P1
**Estimated Scope:** Medium

## 3. Email Marketing: Resend
**Title:** Integrated Email Campaigns via Resend
**Problem Statement:** Sending newsletters or promotional emails to customers is complicated for small business owners. They often have to export their customer list from their main system and import it into a complex tool like Mailchimp, which is confusing and expensive.
**Research Report:** Resend is a developer-first email API that is incredibly reliable, fast, and easy to integrate. While Mailchimp or Listmonk are powerful, Resend allows us to build a simplified, embedded email campaign tool directly within OHC. This means the business owner doesn't need to leave OHC or manage a separate subscription just to send a basic newsletter to their existing customer list.
- **Pricing Estimate:** Free tier up to 3,000 emails/month. Then $20/month for 50,000 emails.
- **Cloud vs Standalone:** Fully compatible with both Cloud and Standalone modes (API key based).
**Design Doc:** A new "Campaigns" tab in the OHC dashboard will allow users to compose simple, text or image-based emails. The user selects their target audience (e.g., "All Customers", "Recent Customers") directly from their OHC contacts. Upon sending, OHC will dispatch the emails using the Resend API, handling unsubscribes and bounce tracking automatically.
**Implementation Prompt:** Create an embedded email marketing tool where users can draft and send emails to their customer contacts. The tool should handle sending via a reliable API, provide basic open-rate tracking, and automatically manage unsubscribe requests.
**Priority:** P1
**Estimated Scope:** Medium

## 4. Payment Processing: Mercado Pago (LATAM Focus)
**Title:** Mercado Pago Integration for LATAM Payments
**Problem Statement:** Stripe is excellent, but it is not available or preferred in many Latin American countries. Small business owners in these regions need a local, trusted payment processor to accept online payments easily.
**Research Report:** Mercado Pago is the dominant payment gateway in LATAM, offering high trust and widespread usage. It supports local payment methods (like Pix in Brazil or OXXO in Mexico). Integrating Mercado Pago ensures we can effectively serve SMBs in the LATAM market where Stripe falls short.
- **Pricing Estimate:** No monthly fee. Roughly 3.99% to 4.99% per transaction depending on the country and settlement speed.
- **Cloud vs Standalone:** Fully compatible with both Cloud and Standalone modes via API keys and webhooks (can use polling/pull model in Standalone if webhooks are unreachable).
**Design Doc:** In the payment settings, users in supported regions can choose Mercado Pago as their payment provider. They will complete an OAuth flow to connect their Mercado Pago account. OHC checkout pages will then use Mercado Pago's checkout module or API to process payments, handling webhooks for payment success/failure status updates.
**Implementation Prompt:** Integrate Mercado Pago as an alternative payment provider. Allow users to connect their account and enable customers to pay using Mercado Pago on the checkout page. The system must accurately track payment status and update order records accordingly.
**Priority:** P2
**Estimated Scope:** Large

## 5. Shipping & Logistics: Shippo
**Title:** Shippo Integration for Automated Shipping Labels
**Problem Statement:** Fulfilling physical orders is a headache. Business owners manually copy customer addresses into carrier websites to buy shipping labels, which is slow and error-prone. They also struggle to provide accurate, real-time shipping costs at checkout.
**Research Report:** Shippo provides a unified API for dozens of carriers (USPS, UPS, FedEx, DHL, etc.). It abstracts the complexity of dealing with individual carriers. EasyPost is a strong competitor, but Shippo's focus on SMBs and user-friendly dashboard (if they need to access it) gives it a slight edge.
- **Pricing Estimate:** Free tier with standard USPS rates (pay $0.05 per label). Pro tier starts at $19/month for volume discounts.
- **Cloud vs Standalone:** Fully compatible with both Cloud and Standalone modes.
**Design Doc:** Upon receiving an order, the business owner can click a "Generate Shipping Label" button in the OHC order details page. OHC will call the Shippo API with the package details to get rates, allow the user to select a rate, and generate a printable PDF label. Tracking numbers will automatically be attached to the order and emailed to the customer.
**Implementation Prompt:** Implement a shipping fulfillment flow using a unified shipping API. Users should be able to view real-time shipping rates, purchase labels directly from the order page, and have tracking information automatically sent to the customer.
**Priority:** P2
**Estimated Scope:** Large

## 6. SMS & Notifications: Twilio
**Title:** Twilio SMS Notifications for Reliable Alerts
**Problem Statement:** Many customers, especially in certain demographics or regions, do not check email frequently. Small business owners need a reliable way to send urgent updates, appointment reminders, or order confirmations directly to their customers' phones.
**Research Report:** Twilio is the industry standard for programmatic SMS. It offers global reach and high deliverability. It handles complex telecom regulations better than most alternatives.
- **Pricing Estimate:** Roughly $0.0079 per message sent in the US. Number rental is $1.15/month. Costs vary heavily by international destination.
- **Cloud vs Standalone:** Fully compatible with both Cloud and Standalone modes.
**Design Doc:** OHC will use the Twilio API to trigger SMS messages based on system events. Users can configure which events (e.g., "Appointment Reminder 24h before", "Order Shipped") trigger an SMS. The business owner will need to provision a phone number through the OHC interface (backed by Twilio).
**Implementation Prompt:** Build an SMS notification system allowing business owners to send automated text messages for key events like appointment reminders and order updates. Include a straightforward way for the owner to manage their sending phone number and view delivery logs.
**Priority:** P1
**Estimated Scope:** Medium

## 7. Video Conferencing: Google Meet
**Title:** Auto-Generated Google Meet Links for Virtual Appointments
**Problem Statement:** Business owners offering virtual consultations or lessons have to manually create a video link and email it to the client for every single booking, which is tedious and easy to forget.
**Research Report:** Google Meet is free, widely accessible, and requires no software installation for the attendee. Since we are already prioritizing Google Calendar integration, adding Google Meet is a natural extension. Zoom is also popular but often requires the attendee to download an app, adding friction.
- **Pricing Estimate:** Free (included with Google Workspace / Gmail accounts).
- **Cloud vs Standalone:** Fully compatible with both Cloud and Standalone modes.
**Design Doc:** When a user configures a service in OHC as a "Virtual Appointment" and connects their Google account, OHC will request permission to create Google Meet links. When a client books this service, OHC will automatically generate a unique Google Meet link and include it in the calendar invite and confirmation emails for both parties.
**Implementation Prompt:** Enhance the booking system to support virtual appointments. When this option is selected, the system should automatically generate a unique video conferencing link and seamlessly include it in all communication and calendar events related to the booking.
**Priority:** P2
**Estimated Scope:** Small
