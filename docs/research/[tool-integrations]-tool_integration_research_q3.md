<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Tool Integration Research [Q3]

## [Social Media Integration] Unified Inbox for Instagram & WhatsApp

**Title**: Implement Unified Unified Inbox for Instagram DMs and WhatsApp Messages

**Problem Statement**: Small business owners like Maya the baker receive orders and questions scattered across Instagram DMs and WhatsApp. Managing multiple apps is overwhelming, and missed messages mean lost revenue. They need a single, simple inbox where they can see and reply to all customer messages, and have the AI agent auto-draft responses.

**Research Report**:
- **Tool Evaluated**: Meta Graph API (Instagram Messaging, WhatsApp Business API).
- **Evaluation**: The Meta API allows centralized messaging. Setup requires Facebook Business login and OAuth, which can be slightly complex but we can streamline it into a "Connect Facebook" button. It's the industry standard for these channels. Pricing is generally per conversation (WhatsApp) or free (Instagram DMs). It's robust and essential. Works well in both Cloud and Standalone (with proper OAuth app credentials).

**Design Doc**:
- Users navigate to the "Customer Success" section of OHC.
- They click "Connect Instagram/WhatsApp".
- After standard Meta OAuth, incoming DMs and WhatsApp messages appear in the OHC Customer Inbox.
- The AI "Ambassador" drafts suggested replies based on past messages and FAQs.
- Users can click to send the drafted reply or edit it.

**Implementation Prompt**:
- Create a unified inbox view on both mobile and desktop.
- Add an integration flow to connect an Instagram Professional account and WhatsApp Business account.
- Ensure incoming messages appear in real-time in the inbox.
- Provide UI for AI-suggested replies.
- Acceptance criteria: A user can connect their account, receive a message from Instagram, and reply from within the OHC app.

**Priority**: P0
**Estimated Scope**: Large


## [Calendar & Scheduling] Google Calendar Two-Way Sync

**Title**: Add Google Calendar Two-Way Sync for Seamless Bookings

**Problem Statement**: Service providers like Carlos the handyman and Leo the music tutor rely on Google Calendar to manage their lives. When customers book appointments on OHC, they need to avoid double-booking with personal events, and new bookings must show up on their phone's native calendar immediately.

**Research Report**:
- **Tool Evaluated**: Google Calendar API.
- **Evaluation**: Industry standard for personal and small business scheduling. Free tier is generous. OAuth flow is straightforward. Handling timezones and recurring events requires care. Completely necessary for any booking-based business. Works in both Cloud and Standalone (with standard OAuth).

**Design Doc**:
- The "Operations" department handles the calendar sync.
- Users click "Sync Google Calendar" in their profile.
- Once connected, OHC reads free/busy times from Google Calendar and removes those slots from the public booking page.
- When a new booking is made via OHC, an event is automatically pushed to the user's Google Calendar.

**Implementation Prompt**:
- Build a connection flow for Google Calendar via OAuth.
- Update the booking widget to check availability against the connected calendar.
- Create calendar events when a service is booked.
- Acceptance criteria: Connecting a calendar prevents booking over existing events, and new OHC bookings appear on the synced Google Calendar.

**Priority**: P0
**Estimated Scope**: Medium


## [Email Marketing] Automated Email Campaigns with Mailgun

**Title**: Integrated Email Campaigns for Customer Re-engagement

**Problem Statement**: Boutique owners like Priya want to tell their customers about a new clothing line, but standalone tools like Mailchimp are too complex and expensive. She just wants to click "Send email to everyone who bought a dress last month" without leaving the app.

**Research Report**:
- **Tool Evaluated**: Mailgun (or SendGrid/Postmark via unified API).
- **Evaluation**: Mailgun offers reliable transactional and bulk email sending. Pricing is very affordable for small volumes (often free tier covers early SMBs). The challenge is managing spam reputation (SPF/DKIM), which we must abstract away or handle automatically on custom domains. For Standalone, users might plug in their own SMTP or Mailgun API keys.

**Design Doc**:
- The "Marketing & Advertising" department provides a simple "Campaigns" tab.
- Users select an audience (e.g., "All Customers", "Recent Buyers").
- AI drafts the email content based on a simple prompt ("Tell them about the summer sale").
- OHC handles the sending via the integrated email provider.

**Implementation Prompt**:
- Create a simple UI to draft and send bulk emails to customer segments.
- Integrate an email sending service (like Mailgun) in the backend.
- Abstract away the complexity of email list management.
- Acceptance criteria: User can select a segment, use AI to draft a message, and send it. Recipients receive the email formatted cleanly.

**Priority**: P1
**Estimated Scope**: Medium


## [Payment Processing] Mercado Pago for LATAM

**Title**: Integrate Mercado Pago as an Alternative Payment Processor

**Problem Statement**: While Stripe is great, it's not available or preferred everywhere. For users in Latin America, Mercado Pago is essential. Business owners need a way to accept local payment methods seamlessly.

**Research Report**:
- **Tool Evaluated**: Mercado Pago API.
- **Evaluation**: Dominant in LATAM. Supports local payment methods like PIX in Brazil, which are critical for conversion. The API is robust and well-documented. Transaction fees are competitive for the region. Works well in Cloud and Standalone.

**Design Doc**:
- The "Finance & Payments" department adds Mercado Pago as an option alongside Stripe.
- Users in supported regions can authenticate their Mercado Pago account.
- The checkout flow dynamically offers Mercado Pago (including PIX) based on the seller's configuration.

**Implementation Prompt**:
- Add Mercado Pago to the payment provider options.
- Update the checkout UI to support Mercado Pago payment flows.
- Ensure webhooks handle payment success/failure correctly.
- Acceptance criteria: A merchant in LATAM can connect Mercado Pago and a customer can successfully checkout using it.

**Priority**: P1
**Estimated Scope**: Medium


## [Shipping & Logistics] Real-time Shipping Labels via Shippo

**Title**: Automated Shipping Label Generation with Shippo

**Problem Statement**: Sellers of physical goods struggle with calculating shipping costs and buying labels at the post office. They need a way to automatically calculate rates at checkout and click "Print Label" when an order is ready.

**Research Report**:
- **Tool Evaluated**: Shippo API.
- **Evaluation**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, international) into one API. It handles rate calculation and label generation. It's very developer-friendly. Pricing is per-label (often pennies) plus carrier costs, which is perfect for SMBs. Fits well with OHC.

**Design Doc**:
- The "Operations" department handles order fulfillment.
- When an order is placed, Shippo calculates the rate.
- In the order details view, the owner clicks "Create Shipping Label".
- OHC fetches a printable PDF label from Shippo.

**Implementation Prompt**:
- Integrate Shippo for real-time rates during checkout.
- Add a "Buy Label" button in the order management UI.
- Provide a simple view to download/print the generated label.
- Acceptance criteria: Checkout shows accurate shipping rates, and the merchant can generate and print a valid shipping label for an order.

**Priority**: P1
**Estimated Scope**: Large


## [SMS & Notifications] Twilio SMS Alerts

**Title**: SMS Order Notifications via Twilio

**Problem Statement**: Food cart operators like Fatima don't always look at their email or app. When an order comes in, they need an immediate text message on their phone so they can start cooking.

**Research Report**:
- **Tool Evaluated**: Twilio Programmable SMS.
- **Evaluation**: Twilio is the gold standard for SMS delivery globally. Very reliable. Cost is fractions of a cent per message. Crucial for real-time, low-tech notifications. In Standalone, users could supply their own Twilio credentials.

**Design Doc**:
- Users can enable "SMS Notifications" in their profile settings.
- The system asks for their mobile number and verifies it.
- When a new order or booking occurs, the "Customer Success" department triggers an SMS via Twilio directly to the business owner.

**Implementation Prompt**:
- Add a phone number field and verification flow in user settings.
- Integrate Twilio to send short, plain-text alerts for new orders.
- Acceptance criteria: When a customer places an order, the business owner receives a text message with the order summary within seconds.

**Priority**: P0
**Estimated Scope**: Small


## [Video Conferencing] Auto-generated Google Meet Links

**Title**: Auto-generate Google Meet Links for Bookings

**Problem Statement**: Online tutors like Leo need a unique video link for every lesson. Manually creating and emailing Zoom or Meet links for every booking is tedious and error-prone.

**Research Report**:
- **Tool Evaluated**: Google Calendar/Meet API.
- **Evaluation**: Since we are already building Google Calendar sync (see above), adding Google Meet links is almost free via the same API by simply requesting conference data when creating the event. It is easier and cheaper than implementing a separate Zoom OAuth flow.

**Design Doc**:
- When a user configures a "Service" (e.g., "1hr Guitar Lesson"), they can select "Location: Online (Google Meet)".
- Upon booking, the created Google Calendar event automatically includes a Meet link.
- Both the owner and the customer receive the link in their confirmation emails.

**Implementation Prompt**:
- Extend the Google Calendar integration to request conference data (Meet links).
- Update the booking confirmation UI and emails to prominently display the video link.
- Acceptance criteria: A customer booking an online service receives an email with a valid Google Meet link, and the owner sees the same link in their calendar.

**Priority**: P1
**Estimated Scope**: Small

</div>
