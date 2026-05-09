# Tool Integration Research Report Q2

## 1. Social Media Integration
**Title**: Integrate Meta Graph API for Unified Native Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.
**Research Report**:
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".
**Implementation Prompt**: Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large

## 2. Calendar & Scheduling
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Small business owners like Leo the Music Tutor and Carlos the Handyman lose potential customers due to back-and-forth scheduling via text and email. They need a public booking link that seamlessly syncs with their personal Google or Outlook calendars without requiring complex technical setup.
**Research Report**:
- **Tool**: Cal.com
- **Target Persona**: Leo (Music Tutor), Carlos (Handyman)
- **Advantages**: Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and custom booking pages out-of-the-box. It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- **Risks**: Ensuring the self-hosted Standalone mode remains lightweight enough not to overwhelm local resources.
- **Pricing**: Free tier available for individuals; highly cost-effective for our free tier users.
- **Compatibility**: Cloud and Standalone (can run self-hosted or via Cal.com's hosted API).
**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours and availability preferences.
- Users connect their Google/Outlook calendar via a one-click OAuth button located in the "Operations" dashboard.
- A public booking widget is embedded on the user's storefront.
- When a customer selects a slot on the OHC public page, Cal.com transparently manages the calendar event creation and conflict resolution.
- The business owner receives a simple notification of the new booking in their unified inbox.
**Implementation Prompt**: Embed Cal.com's scheduling infrastructure to allow users to sync their personal calendars. Provide a public-facing booking widget on their storefront that prevents double-booking and automatically updates their synced calendar upon successful booking.
**Priority**: P0
**Estimated Scope**: Medium

## 3. Email Marketing
**Title**: Integrate Resend for Modern Transactional & Marketing Email
**Problem Statement**: Storefront owners need to send order confirmations and occasional promotional emails to their customer list. Setting up Mailchimp or SendGrid is too complex for non-technical users.
**Research Report**:
- **Tool**: Resend
- **Target Persona**: Any business owner needing email communication.
- **Advantages**: Excellent developer experience, very modern API, fast delivery. Focuses heavily on deliverability. Easy to integrate.
- **Risks**: Newer company compared to SendGrid, though rapidly growing.
- **Pricing**: Generous free tier (up to 3,000 emails/month). Affordable paid tiers.
- **Compatibility**: Cloud. Standalone (would require user to bring their own API key, but feasible).
**Design Doc**:
- OHC handles email sending transparently in the background.
- Users can view a simplified "Email Campaigns" dashboard to draft and schedule emails.
- "The Ambassador" AI can help draft engaging promotional emails.
- Under the hood, OHC uses Resend's API to dispatch emails and track opens/clicks.
**Implementation Prompt**: Integrate Resend API for sending transactional and marketing emails. Build a simple UI for users to view email campaign performance.
**Priority**: P1
**Estimated Scope**: Medium

## 4. Payment Processing
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil.
**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico). Good developer docs.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a secondary payment provider. Implement the checkout flow to redirect to Mercado Pago and handle the success/failure webhooks to update order status.
**Priority**: P2
**Estimated Scope**: Large

## 5. Shipping & Logistics
**Title**: Integrate EasyPost for Streamlined Label Generation & Tracking
**Problem Statement**: Retailers selling physical goods struggle with calculating shipping rates and generating labels. They need an automated way to get shipping rates and buy labels directly from their OHC dashboard.
**Research Report**:
- **Tool**: EasyPost
- **Target Persona**: Retail business owners shipping physical goods.
- **Advantages**: Unified API for 100+ carriers (USPS, FedEx, UPS, international). High reliability.
- **Risks**: Requires handling complex shipping logic (box sizes, weights).
- **Pricing**: Free tier up to 120,000 shipments/year.
- **Compatibility**: Cloud. Standalone.
**Design Doc**:
- Users enter product weights and dimensions in their inventory.
- During checkout, EasyPost API is called to present real-time shipping rates.
- Upon order fulfillment, the user clicks "Generate Label", which uses EasyPost to purchase and print the label.
- Tracking numbers are automatically synced and emailed to the customer.
**Implementation Prompt**: Integrate EasyPost API to fetch real-time shipping rates during checkout and allow business owners to generate and print shipping labels from the order management dashboard.
**Priority**: P1
**Estimated Scope**: Large

## 6. SMS & Notifications
**Title**: Integrate Twilio for Reliable Global SMS Notifications
**Problem Statement**: Business owners like Fatima need to send critical appointment reminders or order updates via SMS, as their customers may not check email frequently.
**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Service providers (appointments) and local delivery businesses.
- **Advantages**: The industry standard for SMS. Global coverage, highly reliable.
- **Risks**: SMS pricing can add up quickly. Requires handling opt-outs (STOP messages) carefully for compliance.
- **Pricing**: Pay-as-you-go per message (varies by country).
- **Compatibility**: Cloud. Standalone (user brings API key).
**Design Doc**:
- OHC manages a central Twilio account for Cloud users.
- Users can enable SMS notifications for specific events (e.g., "Appointment Tomorrow").
- OHC handles sending the SMS via Twilio API and automatically processes STOP replies to ensure compliance.
**Implementation Prompt**: Integrate Twilio API to send SMS notifications for critical business events. Implement automated handling of opt-out requests to ensure regulatory compliance.
**Priority**: P1
**Estimated Scope**: Medium

## 7. Video Conferencing
**Title**: Integrate Google Meet for Automated Consultation Links
**Problem Statement**: Tutors and consultants waste time manually generating video links for their sessions and emailing them to clients.
**Research Report**:
- **Tool**: Google Meet (via Google Workspace API)
- **Target Persona**: Tutors, Consultants, Coaches.
- **Advantages**: Extremely ubiquitous, most users already have a Google account. Free to use for basic meetings.
- **Risks**: Requires robust Google OAuth integration and managing token refresh cycles.
- **Pricing**: Free for standard Google accounts.
- **Compatibility**: Cloud. Standalone.
**Design Doc**:
- As part of the Calendar integration, users authorize OHC to manage their Google Calendar.
- When a service requiring a video call is booked, OHC automatically attaches a Google Meet link to the calendar event.
- The link is sent to both the business owner and the customer in their confirmation messages.
**Implementation Prompt**: Enhance the Calendar integration to automatically generate and attach Google Meet links to scheduled appointments that require video conferencing.
**Priority**: P1
**Estimated Scope**: Small
