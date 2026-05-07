<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Scout: Tool Integration Research Q2

## 1. Social Media Integration

**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting

**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.
**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn). Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).

**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
- **Priority**: P0
- **Estimated Scope**: Large

---

## 2. Calendar & Scheduling

**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync

**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.
**Research Report**:
- Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box.

**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
- **Priority**: P1
- **Estimated Scope**: Medium

---

## 3. Email Marketing

**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns

**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.

**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 4. Payment Processing

**Title**: Expand Payments with Mercado Pago for LATAM Users

**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.
**Research Report**:
- Direct API integration with Mercado Pago for seamless LATAM coverage.

**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 5. Shipping & Logistics

**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking

**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.
**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).

**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo and saves the tracking number.
- OHC automatically emails the customer the tracking number.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 6. SMS & Notifications

**Title**: Integrate Twilio for Global SMS Alerts & Customer Notifications

**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.
**Research Report**:
- Direct integration with the Twilio SDK for native outbound SMS.

**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- The Operations Agent decides the optimal time to send the reminder.
**Implementation Prompt**: Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.
- **Priority**: P2
- **Estimated Scope**: Medium

---

## 7. Video Conferencing

**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons

**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.
**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.

**Design Doc**:
- In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
- The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
- **Priority**: P2
- **Estimated Scope**: Medium


</div>
