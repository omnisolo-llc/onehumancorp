# OHC Tool Integration Research Report

## [Social Media Integration] Unified Inbox Sync

**Title**: Implement Unified Social Media Inbox Sync (Instagram, Facebook, WhatsApp)

**Problem Statement**:
Small business owners like Maya (the home baker) get inquiries via Instagram DMs, WhatsApp, and Facebook comments. Keeping track of all these messages across multiple apps is overwhelming, leading to missed orders and slow response times. They need a single, simple inbox inside the OHC app where all customer messages appear in one place, allowing their AI "Ambassador" to help draft replies.

**Research Report**:
- **Target Tools**: Meta Graph API (Instagram Messaging, Messenger, WhatsApp Business API).
- **Competitive Analysis**: Tools like ManyChat and Shopify Inbox offer similar integrations, but they often require complex initial setups or separate apps.
- **Ease of Use**: By utilizing Meta's official APIs, we can create a streamlined OAuth flow. The business owner just clicks "Connect Instagram" and logs in. No technical setup is required.
- **Pricing**: Meta Graph APIs are generally free for standard messaging. WhatsApp Business has volume-based pricing, but a free tier for initial conversations exists which fits our target personas.
- **Reputation**: Meta APIs are the industry standard for these integrations, despite occasional strict approval processes for API access.
- **Advantages and Risks**: Advantage is native reach on the platforms users already use; risk is Meta's strict API approval and account suspension policies.
- **Cloud vs Standalone**: Works in Cloud mode (central webhooks). Standalone mode would require the user to configure their own Meta App or routing incoming events through the OHC Cloud proxy (which introduces complexity).

**Design Doc**:
- **Integration Flow**: The user accesses the "Customer Success" department in the OHC app and clicks to connect their social accounts via a standard Meta OAuth popup.
- **Actions**: Once connected, incoming DMs and comments are fetched and displayed in a unified OHC inbox. The AI Ambassador can read these messages to suggest draft replies. When the user approves a reply, it is sent back through the Meta API to the respective platform.
- **User Experience**: A seamless, mobile-optimized chat interface inside OHC where messages show a small icon indicating their source (e.g., an Instagram logo).

**Implementation Prompt**:
Create a unified inbox feature that allows users to authenticate with Meta and connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages from these platforms should populate a single chat interface within the OHC app. The feature must include the ability to read messages, see which platform they came from, and reply directly from the OHC app, with replies routing back to the correct original platform. Ensure the authentication flow is simple enough for a non-technical user on a mobile device.

**Priority**: P0
**Estimated Scope**: Large

---

## [Calendar & Scheduling] Seamless Booking Sync

**Title**: Integrate Google Calendar and Outlook Sync for Bookings

**Problem Statement**:
Service providers like Leo (the music tutor) and Carlos (the handyman) need to manage their time efficiently. If they get a booking through OHC but forget to add it to their personal calendar, they double-book. They need a simple way to connect their existing Google or Outlook calendars so OHC knows when they are busy and automatically adds new bookings.

**Research Report**:
- **Target Tools**: Google Calendar API, Microsoft Graph API.
- **Competitive Analysis**: Calendly and Squarespace Scheduling handle this well, but require managing a separate tool. OHC brings this natively into the business owner's stack.
- **Ease of Use**: Standard OAuth flows allow one-click connection. Non-technical users are very familiar with "Sign in with Google."
- **Pricing**: Both Google and Microsoft provide these APIs for free within generous rate limits suitable for small businesses.
- **Reputation**: Highly reliable, industry-standard APIs.
- **Advantages and Risks**: Advantage is seamless double-booking prevention; risk involves syncing delays or timezone mismatches.
- **Cloud vs Standalone**: Works well in Cloud. Standalone mode might have issues with OAuth redirect URIs and will likely require an OHC proxy or local OAuth credentials.

**Design Doc**:
- **Integration Flow**: In the "Operations" department, users click to sync their Google or Outlook calendar.
- **Actions**: The system reads the external calendar to block out unavailable times on the OHC booking page. When a customer books a slot, the system creates an event on the connected external calendar.
- **User Experience**: The user sees a straightforward "Connect Calendar" button. Once connected, their OHC booking availability automatically reflects their personal calendar's busy times.

**Implementation Prompt**:
Implement calendar synchronization allowing users to link their Google Calendar and Outlook accounts. The system must use these linked calendars to automatically block out unavailable time slots on the user's public OHC booking page. Furthermore, when a new booking is made through OHC, it must automatically create a corresponding event in the user's linked calendar. The connection process should be a simple OAuth flow.

**Priority**: P0
**Estimated Scope**: Medium

---

## [Email Marketing] Automated Campaign Campaigns

**Title**: Implement Native Email Marketing and Newsletter Engine

**Problem Statement**:
Boutique owners like Priya want to notify their existing customers when new stock arrives or a sale starts. Using a separate tool like Mailchimp is complicated, expensive, and requires manually exporting/importing customer lists. They need a built-in way to send beautiful emails to their OHC customer list.

**Research Report**:
- **Target Tools**: Amazon SES or SendGrid (under the hood for OHC).
- **Competitive Analysis**: Shopify Email provides basic functionality. Dedicated tools (Mailchimp, Klaviyo) are too complex for our personas.
- **Ease of Use**: OHC abstracts the complexity. The "Promoter" AI helps draft the email based on a prompt ("Tell my customers about the summer sale").
- **Pricing**: SES is extremely cost-effective ($0.10 per 1000 emails). We can offer a generous free tier for OHC users.
- **Reputation**: High deliverability when properly configured.
- **Advantages and Risks**: High ROI and keeps users inside the platform. Main risk is users sending spam and ruining the OHC shared domain reputation.
- **Cloud vs Standalone**: Cloud implementation is straightforward (shared SES). Standalone would require users to provide their own SMTP credentials, which is too technical.

**Design Doc**:
- **Integration Flow**: In the "Marketing & Advertising" department, users can select "Send an Email Announcement."
- **Actions**: The system uses the existing customer list. The AI can draft the subject and body. The system handles unsubscribe links and bounce tracking automatically.
- **User Experience**: A simple interface to draft a message (or have AI draft it), pick an audience (e.g., "All past customers"), and click send. No managing of API keys or domain verification for the user; OHC handles sending from a verified shared domain or the user's connected domain.

**Implementation Prompt**:
Build a native email marketing feature that allows users to send broadcast emails to their customer list directly from the OHC app. The feature should integrate with the existing customer database, allow for AI-assisted drafting of email content, and automatically handle unsubscribe requests and deliverability tracking. Ensure the user interface is completely free of technical email jargon.

**Priority**: P1
**Estimated Scope**: Medium

---

## [Payment Processing] Localized Alternative Payments

**Title**: Integrate Alternative Payment Methods (Mercado Pago, Alipay, Paytm)

**Problem Statement**:
While Stripe is excellent, users in certain regions (LATAM, India, Asia) rely on localized payment methods that Stripe might not fully support or where direct integrations are preferred by local consumers. A business owner needs to accept the payment methods their local customers actually use.

**Research Report**:
- **Target Tools**: Mercado Pago API (LATAM), Paytm API (India), Alipay API.
- **Competitive Analysis**: Global platforms often struggle here. Shopify supports many gateways but setup is complex.
- **Ease of Use**: Requires the user to have an existing account with the provider and link it via OAuth or a simple API key exchange.
- **Pricing**: Transaction fees vary by provider but are standard for the respective regions.
- **Reputation**: These are the dominant payment processors in their respective local markets.
- **Advantages and Risks**: Critical for international adoption; risk is the engineering overhead of supporting and testing 5+ distinct payment APIs.
- **Cloud vs Standalone**: Cloud works well via webhooks. Standalone might struggle to securely receive webhooks if the local instance isn't exposed to the public internet securely.

**Design Doc**:
- **Integration Flow**: In the "Finance & Payments" department, under regional settings, users can enable local payment providers.
- **Actions**: Replaces or supplements the standard Stripe checkout with the localized checkout flow when a customer initiates a purchase.
- **User Experience**: The business owner toggles the local provider on and logs in. For the buyer, they see their familiar local payment option at checkout.

**Implementation Prompt**:
Extend the payment processing system to support alternative, region-specific payment gateways alongside the existing Stripe integration. Implement the connection flow for at least one major regional provider (e.g., Mercado Pago). The checkout experience for the end-customer must seamlessly present these new payment options, and the business owner's dashboard must accurately reflect transactions processed through these alternative gateways.

**Priority**: P2
**Estimated Scope**: Large

---

## [Shipping & Logistics] Automated Shipping Rates and Labels

**Title**: Integrate Real-Time Shipping Rates and Label Generation

**Problem Statement**:
Sellers of physical products need to charge customers the correct amount for shipping and easily print shipping labels. Calculating this manually is error-prone, and going to the post office to buy labels is time-consuming. They need shipping costs calculated automatically at checkout and one-click label printing.

**Research Report**:
- **Target Tools**: Shippo API or EasyPost API.
- **Competitive Analysis**: Shopify has robust native shipping. OHC needs a simple, comparable alternative without the complexity of carrier negotiation.
- **Ease of Use**: Shippo/EasyPost abstract multiple carriers (USPS, FedEx, UPS) into a single API. OHC users don't need their own carrier accounts.
- **Pricing**: Pay-as-you-go per label (cents per label + postage). Can be passed to the user or absorbed in OHC premium tiers.
- **Reputation**: Highly reliable APIs used by many e-commerce platforms.
- **Advantages and Risks**: Massive time-saver for physical product sellers. Risk involves miscalculating package weights resulting in undercharging for shipping.
- **Cloud vs Standalone**: Cloud integrates directly. Standalone could work if the API calls are client-side, but might still rely on a Cloud proxy to handle billing.

**Design Doc**:
- **Integration Flow**: In "Operations", the user sets up their shipping origin address and default box sizes.
- **Actions**: During customer checkout, the system calls the API to get real-time rates based on the cart contents and destination. Post-purchase, the user can click "Generate Label" to get a printable PDF and automatically email tracking info to the customer.
- **User Experience**: A "Print Shipping Label" button appears on physical product orders. The system handles the payment for the postage in the background (deducted from their payout or charged to their card on file).

**Implementation Prompt**:
Integrate a shipping aggregation API (like EasyPost or Shippo) to provide real-time shipping rate calculation at checkout for physical products. Additionally, build a feature allowing the business owner to generate and print shipping labels directly from the order details screen in the OHC app. Ensure tracking numbers are automatically generated and attached to the order.

**Priority**: P1
**Estimated Scope**: Large

---

## [SMS & Notifications] Global SMS Alerts

**Title**: Implement Global SMS Order Notifications

**Problem Statement**:
For users like Fatima (food cart operator), checking an app or email constantly isn't feasible while working. She needs an immediate, loud text message on her basic smartphone the second an order is placed so she can start preparing the food.

**Research Report**:
- **Target Tools**: Twilio API or MessageBird.
- **Competitive Analysis**: Many platforms charge extra for SMS. Offering this out-of-the-box for critical alerts is a strong differentiator for specific personas (food, urgent services).
- **Ease of Use**: Completely invisible setup. The user just enters their phone number and checks a box for "Text me when I get a new order."
- **Pricing**: ~$0.01 - $0.05 per message depending on the country. Costs need to be managed (e.g., limited free texts per month, unlimited on paid plans).
- **Reputation**: Twilio is the gold standard for global SMS delivery.
- **Advantages and Risks**: Ensures operators like Fatima don't miss orders. Risk is high cost per message and strict compliance (A2P 10DLC) rules in the US.
- **Cloud vs Standalone**: Cloud uses central Twilio account. Standalone cannot use central SMS; users would need their own Twilio credentials, rendering it unusable for non-technical users.

**Design Doc**:
- **Integration Flow**: In the "Operations" or Profile settings, users verify their mobile number and enable SMS alerts.
- **Actions**: When a specific trigger occurs (e.g., Order Paid), the system dispatches an SMS via the Twilio API to the owner's phone.
- **User Experience**: A simple toggle: "Send me a text message for new orders." The received text is concise: "OHC Alert: New order #123 for $15.00 - Chicken Over Rice."

**Implementation Prompt**:
Build an SMS notification service integrated with Twilio that allows business owners to opt-in to receive text message alerts for critical events, such as new orders or bookings. The feature must include a simple phone number verification flow and toggle switches to control which events trigger an SMS. The notification content must be concise and informative.

**Priority**: P1
**Estimated Scope**: Small

---

## [Video Conferencing] Auto-Generated Meeting Links

**Title**: Auto-Generate Zoom/Meet Links for Online Services

**Problem Statement**:
For online service providers like Leo (music tutor), manually creating a Zoom link and emailing it to a student after every booking is tedious. The system should automatically generate a unique video meeting link and include it in the calendar invite and confirmation email.

**Research Report**:
- **Target Tools**: Zoom API, Google Meet API (via Google Workspace integration).
- **Competitive Analysis**: Calendly does this perfectly. OHC needs parity to be viable for online consultants/tutors.
- **Ease of Use**: User authenticates their Zoom or Google account once.
- **Pricing**: Free APIs, though Zoom requires the user to have a licensed Zoom account for meetings over 40 minutes.
- **Reputation**: Essential tools for remote work and online services.
- **Advantages and Risks**: Creates a fully automated tutoring business. Risk is OAuth token expiration leading to failed meeting creations.
- **Cloud vs Standalone**: Same constraints as Calendars. Works perfectly in Cloud. Standalone may have trouble with OAuth redirects unless routed through OHC Cloud.

**Design Doc**:
- **Integration Flow**: When setting up a service, the user selects "Location: Online Video Call" and connects their Zoom or Google account.
- **Actions**: Upon a successful booking, the system calls the respective API to create a scheduled meeting. The returned join URL is saved to the booking record and sent to both the user and the customer.
- **User Experience**: Completely automated. The user just sees the meeting link appear in their calendar, and the customer gets it in their email. No copy-pasting required.

**Implementation Prompt**:
Integrate video conferencing capabilities allowing users to connect their Zoom or Google Meet accounts. When a customer books a service designated as "Online Video Call," the system must automatically interact with the external API to generate a unique meeting link. This link must be automatically distributed in the booking confirmation email to the customer and embedded in the event details for the business owner.

**Priority**: P1
**Estimated Scope**: Medium
