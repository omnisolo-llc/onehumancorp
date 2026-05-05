# 🔍 Scout: Tool Integration Research Q2

## [Social Media] Meta Graph API Integration
**Title**: Integrate Meta Graph API for Unified Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Tool**: Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Direct integration with Facebook, Instagram, and WhatsApp without third-party middleware. Better data privacy, control over user experience, and no extra SaaS subscription fees for the user. Aligns perfectly with OHC's "Radical Simplicity" by hiding the complexity from the user.
- **Risks**: Meta's API limits and strict app review processes. Managing API version deprecations internally. Rate limits on message sending.
- **Pricing**: Free to use the API (though WhatsApp Business API may have conversation-based pricing which OHC would need to pass through or absorb). No third-party SaaS fees.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires user to create their own Meta App or OHC provides a passthrough service).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth directly to OHC.
- OHC registers webhooks to receive new DMs using the Meta Graph API.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via the Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement an OAuth flow to connect a user's Meta account directly. Create a webhook endpoint that receives incoming messages via the Meta Graph API, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar] Google Calendar API Integration
**Title**: Integrate Google Calendar API for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly on their calendar.
**Research Report**:
- **Tool**: Google Calendar API
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Ubiquitous usage, deeply integrated with most users' existing workflows. Eliminates the need for users to learn or pay for a third-party booking SaaS like Calendly. Native OHC UI for booking provides a more cohesive experience.
- **Risks**: Handling complex timezone logic, recurring events, and edge cases natively. Google App Verification is required for production OAuth access.
- **Pricing**: Free for standard usage quotas. No extra monthly SaaS cost for the business owner.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth or Service Account).
**Design Doc**:
- User goes to Sales dashboard and connects their Google Calendar via OAuth.
- OHC allows the user to define available time slots directly in the OHC app.
- OHC checks Google Calendar API for conflicts before showing available times on the public storefront.
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC creates the event in the user's Google Calendar and records the appointment in the Operations dashboard.
**Implementation Prompt**: Create an OAuth integration with Google Calendar. Build a native booking widget that checks for conflicts via the Google Calendar API and inserts new bookings directly into the user's calendar.
**Priority**: P1
**Estimated Scope**: Medium

## [Email Marketing] SendGrid Integration
**Title**: Integrate SendGrid/SES for Native Email Marketing
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns. She needs an automated way to email customers without leaving the OHC app.
**Research Report**:
- **Tool**: SendGrid/SES
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Abstracted completely from the user. OHC handles the delivery infrastructure, providing a frictionless, zero-setup email marketing experience. Business owners don't need to learn a separate tool like Mailchimp. Lower per-email cost at scale.
- **Risks**: Managing IP reputation and deliverability for multiple tenants. Handling bounces and spam complaints internally. Potential for abuse by bad actors.
- **Pricing**: Very low per-email cost paid by OHC. No separate SaaS subscription for the business owner.
- **Compatibility**: Cloud (Server-side API calls). Standalone (requires user to provide SMTP credentials or API key).
**Design Doc**:
- When a customer buys something, they are automatically added to the OHC internal customer directory with tags (e.g., "Bought: Cake").
- The Marketing agent suggests campaigns ("Send an email to past customers about your new holiday cakes").
- The user approves the AI-generated email in the OHC UI.
- OHC dispatches the emails directly using SendGrid/SES via backend API.
- The user sees open rates and clicks (tracked natively by OHC) in the Marketing dashboard.
**Implementation Prompt**: Build an internal email delivery pipeline using SendGrid/SES. Sync customers to OHC's native directory after purchase. Allow the AI Marketing agent to create campaigns, and send them directly via the backend integration, tracking opens and clicks natively.
**Priority**: P1
**Estimated Scope**: Medium

## [Payment] Stripe Local Payment Methods Integration
**Title**: Integrate Stripe Local Payment Methods (Pix, OXXO) for LATAM
**Problem Statement**: Small business owners in Latin America cannot easily use standard credit cards and need trusted local payment processors (like Pix or Pago Fácil) to accept payments.
**Research Report**:
- **Tool**: Stripe Local Payment Methods API
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Maintains a single payment gateway (Stripe) instead of integrating multiple third-party processors like Mercado Pago. Stripe natively supports Pix, OXXO, and other local LATAM methods. Keeps the codebase simpler and aligns with the Radical Simplicity core value by avoiding fragmented payment architectures.
- **Risks**: Stripe's availability in certain LATAM countries might be more restricted than a hyper-local provider.
- **Pricing**: Standard Stripe transaction fees for local payment methods.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- User selects their country during onboarding.
- If in LATAM, OHC automatically configures the connected Stripe account to accept local payment methods (e.g., Pix).
- Customers see a "Pay with Pix" or "Pay with OXXO" button at checkout, powered natively by Stripe.
- Webhooks update the order status in OHC when the asynchronous local payment succeeds.
**Implementation Prompt**: Extend the existing Stripe integration to support Local Payment Methods. Modify the checkout flow to dynamically display supported local payment methods based on the customer's region.
**Priority**: P2
**Estimated Scope**: Medium

## [Shipping] Native Postal API Integration (USPS/Local Carrier)
**Title**: Integrate Native Postal APIs for Automated Label Generation
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label without managing another SaaS account.
**Research Report**:
- **Tool**: Direct USPS API (or equivalent local national carrier API)
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Eliminates the need for a middleman SaaS like Shippo. Simplifies the user experience—business owners deal directly with their national carrier without a third-party layer. Better alignment with Radical Simplicity by abstracting shipping logic natively.
- **Risks**: Dealing with archaic XML/SOAP APIs from national carriers. Handling international shipping forms directly is complex. Managing rate changes natively.
- **Pricing**: Free to use carrier APIs (user pays postage). No third-party API markup or subscription fees.
- **Compatibility**: Cloud (Direct API calls). Standalone (Direct API calls).
**Design Doc**:
- When an order is placed, OHC natively calls the USPS API to get shipping rates based on dimensions/weight.
- The Operations agent shows the cheapest native shipping option.
- The user clicks "Buy Label", OHC purchases the label natively via the carrier API, and downloads the PDF label.
- OHC automatically emails the customer the native tracking number.
**Implementation Prompt**: Connect directly to the USPS Web Tools API (or equivalent). Build an internal module to fetch shipping rates and purchase labels without third-party middleware. Allow the user to click to print a label and auto-email tracking.
**Priority**: P1
**Estimated Scope**: Large

## [SMS] Native Push Notifications & WebPush
**Title**: Integrate Native Push for Order Notifications
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs a reliable way to be alerted when a new pre-order arrives without relying on complex, regulated SMS platforms.
**Research Report**:
- **Tool**: Firebase Cloud Messaging (FCM) / Apple Push Notification Service (APNs) / WebPush
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: No variable per-message costs. Avoids the complex A2P 10DLC compliance and business registration required by Twilio in the US, which blocks informal/unregistered businesses. Truly native to the OHC app experience.
- **Risks**: Push notifications can be delayed by OS battery optimizations or accidentally disabled by the user. Requires the user to have the OHC app installed or WebPush enabled.
- **Pricing**: Free (infrastructure cost absorbed by OHC).
- **Compatibility**: Cloud (FCM/APNs integration). Standalone (WebPush or local notifications).
**Design Doc**:
- User installs the OHC app or enables WebPush on their browser.
- When an order is paid, the Operations agent triggers a high-priority, persistent push notification: "New order! 2x Falafel for John. Pickup in 15m."
- The notification features a distinct, loud sound to alert users in noisy environments.
**Implementation Prompt**: Implement a robust push notification infrastructure using FCM/APNs. Create a high-priority, "noisy" alert type specifically for new orders to ensure business owners don't miss them. Build a fallback mechanism if push delivery fails (e.g., in-app persistent alerts).
**Priority**: P2
**Estimated Scope**: Medium

## [Video] Native WebRTC Video Integration
**Title**: Integrate Native WebRTC Video for In-App Lessons
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error, looks unprofessional, and forces the student out of the OHC ecosystem to install a third-party app.
**Research Report**:
- **Tool**: Native WebRTC (Peer-to-Peer or via SFU like LiveKit)
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Aligns perfectly with Radical Simplicity. Neither the business owner nor the customer needs to install a third-party app or create a Zoom account. The video call happens directly within the OHC platform, keeping the branding cohesive and the user journey unbroken.
- **Risks**: Maintaining real-time infrastructure (SFU) is complex. Handling varied network conditions on mobile devices natively requires robust engineering.
- **Pricing**: Infrastructure costs (bandwidth/compute) absorbed by OHC. No per-user Zoom Pro subscription required for the business owner.
- **Compatibility**: Cloud (Managed SFU like LiveKit or Twilio Video). Standalone (Direct P2P WebRTC or local SFU deployment).
**Design Doc**:
- Customer books an online lesson natively in OHC (via Google Calendar integration).
- OHC generates a secure, unique "Room ID" instead of a Zoom link.
- Both the business owner and the customer receive a single link: `onehuman.corp/join/{room_id}`.
- At the scheduled time, clicking the link opens a native video calling interface directly in the browser or OHC mobile app.
- No software installation or third-party account is required.
**Implementation Prompt**: Implement a WebRTC-based video calling feature. Integrate an SFU (like LiveKit) for the backend. Create a simple UI within OHC for hosting and joining video calls securely based on generated Room IDs.
**Priority**: P1
**Estimated Scope**: Large
