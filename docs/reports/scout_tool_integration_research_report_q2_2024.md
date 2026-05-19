# Scout: Tool Integration Research [Q2 2024]

This report evaluates key third-party tool integrations to expand OHC's capabilities for small business owners. The focus is on tools that directly alleviate pain points like operational fatigue, complex setup, and fragmented communication.

---

## [Social Media Integration] Unified Inbox with Meta API (Instagram DM & WhatsApp)

### Problem Statement
Small business owners like Maya (The Home Baker) and Carlos (The Freelance Handyman) struggle with "operational fatigue"—the burden of answering the same customer inquiries across multiple platforms (Instagram, WhatsApp, website). Losing track of DMs leads to lost sales and poor customer experience.

### Research Report
- **Tools Evaluated:** Meta Graph API (Instagram DM), WhatsApp Business API.
- **Findings:** Both APIs are essential for reaching customers where they are. Meta's unified infrastructure allows for centralized message ingestion.
- **Ease of Use:** High for the end-user (unified view in OHC), but setup requires simplified OAuth flows to shield users from Meta's complex developer portal.
- **Pricing:** WhatsApp Business API charges per conversation (free tier available); Instagram DMs are generally free but subject to rate limits.
- **Support:** Fully compatible with Cloud (webhooks) and Standalone (local polling/sync via Cloud bridge).

### Design Doc
- **Integration Flow:** User authenticates via a simplified OHC OAuth flow to connect their Instagram/WhatsApp accounts.
- **Agent Action:** The "Ambassador" agent monitors incoming messages, drafts responses (e.g., pricing, availability), and queues them for user approval in the OHC activity feed.
- **Data Model:** Messages are normalized into a unified `ohc_message` format, tagged with the source platform.

### Implementation Prompt
Implement the Meta API integration to ingest Instagram DMs and WhatsApp messages into OHC's unified inbox. The feature should allow the "Ambassador" AI agent to draft replies to these messages autonomously. The mobile UI must display these drafts in the "Agent Activity Feed" for easy approval by the business owner. Ensure the OAuth setup is seamless and requires zero technical knowledge.

### Priority
P0

### Estimated Scope
Large

---

## [Calendar & Scheduling] Automated Booking with Google Calendar & Zoom

### Problem Statement
Service providers like Leo (The Music Tutor) and Carlos (The Handyman) lose time coordinating schedules and manually generating video meeting links. They need a system that prevents double-booking and automatically handles meeting logistics without requiring them to juggle multiple apps.

### Research Report
- **Tools Evaluated:** Google Calendar API, Zoom API.
- **Findings:** Google Calendar is the most ubiquitous personal calendar. Integrating it prevents double-booking. Zoom remains the standard for online lessons/consultations.
- **Ease of Use:** Connecting Google/Zoom via standard OAuth is familiar to most users.
- **Pricing:** Google Calendar API is free within standard limits. Zoom requires a Pro account for advanced API features, though basic scheduling is accessible.
- **Support:** Cloud and Standalone (requires online connection for sync).

### Design Doc
- **Integration Flow:** User connects Google Calendar and Zoom via OAuth in the OHC Settings.
- **Agent Action:** The "Operations" agent automatically syncs OHC bookings with Google Calendar, checks for conflicts, and generates Zoom links for virtual services.
- **UI:** The booking flow displays available slots based on the user's real-time Google Calendar availability.

### Implementation Prompt
Develop the Google Calendar and Zoom API integrations. When a customer books a service, the system should automatically check the owner's Google Calendar for conflicts, block off the booked time, and generate a Zoom meeting link if the service is virtual. The integration must be completely invisible to the customer and seamlessly manageable from the owner's 375px mobile dashboard.

### Priority
P1

### Estimated Scope
Medium

---

## [Email Marketing] Automated Campaigns with Klaviyo/Mailchimp

### Problem Statement
Business owners need to re-engage past customers (e.g., Priya the Boutique Owner announcing new stock) but find creating email campaigns manually to be intimidating and time-consuming. "Marketing Dread" leads to missed revenue opportunities.

### Research Report
- **Tools Evaluated:** Klaviyo, Mailchimp API.
- **Findings:** Klaviyo excels in e-commerce specific triggers, while Mailchimp is more general-purpose.
- **Ease of Use:** Both offer robust APIs, but exposing their full complexity to OHC users would violate the "Radical Simplicity" value. OHC must abstract the campaign creation.
- **Pricing:** Both offer free tiers based on subscriber count, scaling with list size.
- **Support:** Cloud and Standalone (requires online sync).

### Design Doc
- **Integration Flow:** User connects their preferred provider. OHC syncs customer data and tags automatically.
- **Agent Action:** The "Promoter" agent analyzes inventory/sales data and auto-drafts targeted email campaigns (e.g., "New Arrivals" or "We Miss You"), presenting them for one-click approval.
- **UI:** A simple approval interface in the activity feed showing the email preview and target audience size.

### Implementation Prompt
Integrate with Mailchimp and Klaviyo APIs to sync OHC customer data and enable automated email campaigns. Create a feature where the "Promoter" agent drafts email newsletters based on recent business activities (like new product additions) and allows the owner to review, edit, and send the campaign directly from the OHC mobile app without navigating the third-party email platform.

### Priority
P2

### Estimated Scope
Medium

---

## [Payment Processing] Expanding Reach with Mercado Pago

### Problem Statement
Not all OHC users are in Stripe-supported regions. For users in Latin America, alternative payment processors are essential for business operations.

### Research Report
- **Tools Evaluated:** Mercado Pago API.
- **Findings:** Mercado Pago is dominant in LATAM, offering familiar checkout experiences and local payment methods (e.g., Pix in Brazil).
- **Ease of Use:** Integration for the end-user should be as simple as connecting their account, similar to the existing Stripe flow.
- **Pricing:** Transaction-based fees varying by country and payment method.
- **Support:** Cloud and Standalone (online required for transaction processing).

### Design Doc
- **Integration Flow:** Add Mercado Pago as a payment provider option alongside Stripe.
- **Agent Action:** "Finance & Payments" agent tracks Mercado Pago settlements and includes them in financial reports.
- **UI:** A unified payment settings screen allowing users to toggle active processors based on their region.

### Implementation Prompt
Implement Mercado Pago as an alternative payment processing integration to support LATAM business owners. The integration must support online checkouts and deposit payments, seamlessly syncing transaction data back into the OHC "Finance & Payments" tracking system. Ensure the setup flow requires only basic account connection credentials.

### Priority
P1

### Estimated Scope
Medium

---

## [Shipping & Logistics] Real-time Rates with Shippo

### Problem Statement
Physical product sellers like Priya (Boutique) struggle with calculating accurate shipping costs, leading to either abandoned carts (shipping too high) or eroded margins (shipping too low).

### Research Report
- **Tools Evaluated:** Shippo API.
- **Findings:** Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and provides real-time rate calculation and label generation.
- **Ease of Use:** High value for the user, abstracting away the need to manage individual carrier accounts.
- **Pricing:** Pay-as-you-go per label or monthly subscription for advanced features.
- **Support:** Cloud and Standalone (online required for rate calculation).

### Design Doc
- **Integration Flow:** User connects a Shippo account or uses an OHC-managed aggregated account.
- **Agent Action:** "Operations" agent calculates shipping during checkout based on product weight/dimensions and customer address.
- **UI:** Simple shipping zone configuration and a one-click label printing option from the mobile order detail screen.

### Implementation Prompt
Integrate the Shippo API to provide real-time shipping rate calculation at checkout and one-click shipping label generation for physical product orders. The system should allow the owner to print labels directly from their mobile device. The configuration must abstract complex shipping zones into simple "Local", "National", and "International" categories.

### Priority
P2

### Estimated Scope
Large

---

## [SMS & Notifications] Reliable Messaging with Twilio

### Problem Statement
Some users, like Fatima (The Food Cart Operator), operate in environments with low English proficiency or poor internet connectivity, where email or push notifications are ineffective. SMS is the most universally accessible communication channel.

### Research Report
- **Tools Evaluated:** Twilio API.
- **Findings:** Twilio is the industry standard for SMS delivery, offering high global reach, reliability, and robust compliance tools (e.g., handling opt-outs).
- **Ease of Use:** End-user setup is minimal; OHC can manage a central Twilio account and abstract the complexity of carrier registration.
- **Pricing:** Pay-as-you-go per message sent/received, varying significantly by country.
- **Support:** Cloud and Standalone (requires online connection for sending).

### Design Doc
- **Integration Flow:** SMS capability is natively available in OHC without the user needing their own Twilio account.
- **Agent Action:** The "Operations" agent triggers SMS notifications for critical events (e.g., order ready for pickup) or appointment reminders.
- **UI:** A simple toggle in communication settings to "Send SMS Notifications" for specific events.

### Implementation Prompt
Integrate the Twilio SMS API to enable automated text message notifications for critical business events, such as order confirmations or appointment reminders. This feature is particularly crucial for users in low-data environments or those relying on immediate alerts (like food pickup). The integration should abstract away Twilio account management, providing a simple toggle switch for the business owner to enable SMS for their customers.

### Priority
P1

### Estimated Scope
Medium
