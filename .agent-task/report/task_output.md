# Consolidated Tool Integration Research Report

## 1. Social Media Integration

### [Social Media] Issue Brief: Automated Direct Message Integration

**Title**: Scout 🔍: Integrate Meta API for Automated Instagram & Messenger DMs
**Problem Statement**:
Small business owners like Maya (Home Baker) and Priya (Boutique) are overwhelmed by repetitive direct messages on Instagram and Facebook (e.g., "Do you do vegan?", "Is this in stock?"). Replying manually takes away from their actual work, and missing DMs means losing sales. They need an automated way to handle these inquiries without touching any code or configuring complex webhook flows.
**Research Report**:
- **Tool**: Meta Graph API (Instagram Direct & Messenger) or a managed wrapper like ManyChat.
- **Evaluation**: The Meta API allows full programmatic access to read and reply to DMs. By integrating this, OHC's "Customer Success" AI agent can draft and send replies based on the business's existing catalog, FAQs, and business hours.
- **Advantages**: Direct integration with the source platforms means no intermediate failure points. Provides the raw data needed for AI-driven automated responses.
- **Risks**: Meta API changes frequently and requires maintaining complex OAuth flows.
- **Ease of Use**: Very easy for the user. They simply click "Log in with Facebook/Instagram" to grant permissions. No API keys to manage.
- **Pricing Estimate**: Free to use the Meta API for standard text/media. WhatsApp integration (if added later) has per-conversation pricing.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode (OHC manages the Meta App and Webhooks). In Standalone mode, it would be complex as the user would need to create their own Meta App.
**Design Doc**:
- The user navigates to a "Social Inbox" tab and clicks "Connect Instagram".
- Uses OAuth to grant OHC permission to read/write messages.
- OHC registers a centralized webhook for the tenant.
- Incoming messages are routed to the AI Agent (Customer Success).
- The agent formulates a response based on the tenant's context (products, availability) and sends it back via the Meta API.
**Implementation Prompt**:
Implement the Instagram/Messenger integration. Provide a UI for the user to connect their Meta account. Set up a secure webhook endpoint to receive incoming DMs, route them to the LLM with the user's business context, and send the generated reply back to the customer. Ensure the user can toggle the AI on/off or set it to "draft only" mode.
**Priority**: P1
**Estimated Scope**: Medium

### [Social Media] Manychat Integration
**Title**: Integrate Manychat for Unified Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Tool**: Manychat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Excellent Instagram and WhatsApp API integrations. Robust webhook support for routing messages to OHC's backend. Extremely popular among SMBs for basic automation.
- **Risks**: Pricing scales with contacts, which may be expensive for high-volume, low-margin businesses. Requires Meta business verification for some features.
- **Pricing Estimate**: Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
- **Compatibility (Cloud vs. Standalone)**: Cloud (via webhooks/OAuth). Standalone (would require local reverse proxy for webhooks, possible but complex).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Manychat's API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement an OAuth flow to connect a user's Instagram/Facebook account via Manychat. Create a webhook endpoint that receives incoming messages, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large

### [Social Media] Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations.
**Research Report**:
- **Tool**: Ayrshare
- **Target Persona**: Maya (Home Baker), Carlos (Handyman)
- **Advantages**: Provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn). Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively. Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies. Non-technical users benefit by never leaving the OHC interface.
- **Risks**: Reliance on a single aggregator for all social platforms.
- **Pricing Estimate**: Free tier available, then scales per user.
- **Compatibility (Cloud vs. Standalone)**: Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.
**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
**Priority**: P1
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

### [Calendar] Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs with their personal Google Calendar seamlessly.
**Research Report**:
- **Tool**: Cal.com
- **Target Persona**: Leo (Music Tutor), Carlos (Handyman)
- **Advantages**: Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box. It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes. Alternative is building from scratch, which is error-prone.
- **Risks**: Maintaining updates to Cal.com infrastructure.
- **Pricing Estimate**: Free tier available for individuals; great for our free tier users.
- **Compatibility (Cloud vs. Standalone)**: Compatible with both Cloud (SaaS) and Standalone OHC modes.
**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect their Google/Outlook calendar via a one-click OAuth button in the "Operations" tab.
- When a customer books a slot on the OHC public page, Cal.com manages the calendar event and conflict resolution transparently.
**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
**Priority**: P0
**Estimated Scope**: Medium

### [Calendar] Calendly Integration
**Title**: Integrate Calendly for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly on their calendar.
**Research Report**:
- **Tool**: Calendly
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Industry standard, highly recognizable to customers. Excellent conflict resolution and timezone handling. Easy API integration.
- **Risks**: If a user cancels via Calendly directly instead of OHC, state might go out of sync without robust webhook handling.
- **Pricing Estimate**: Free tier available. Premium starts at $10/mo.
- **Compatibility (Cloud vs. Standalone)**: Cloud (OAuth). Standalone (requires API key).
**Design Doc**:
- User goes to Sales dashboard and connects Calendly.
- OHC pulls available event types (e.g., "30-min Consultation") and displays them on the user's public storefront.
- When a customer clicks to book, they are shown the Calendly widget.
- Upon successful booking, a webhook notifies OHC to record the appointment in the Operations dashboard.
**Implementation Prompt**: Create an integration that allows a user to connect their Calendly account. Fetch their existing event types and display a booking widget on their public profile page. Ensure booked events sync back to the OHC dashboard.
**Priority**: P1
**Estimated Scope**: Medium

### [Calendar] Google Calendar API Integration
**Title**: Native Calendar Sync for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.
**Research Report**:
- **Tool**: Direct Google Calendar API / CalDAV integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
- **Risks**: Handling complex timezone logic internally.
- **Pricing Estimate**: Free API usage.
- **Compatibility (Cloud vs. Standalone)**: Cloud (OAuth). Standalone (OAuth).
**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
- **AI Integration**: The Operations Agent monitors the calendar and alerts the business owner if they have back-to-back appointments without buffer times, suggesting schedule optimizations.
**Implementation Prompt**: Create a native booking widget and Google Calendar OAuth integration. Calculate availability based on existing calendar events and sync new bookings directly to Google Calendar.
**Priority**: P1
**Estimated Scope**: Medium

---

## 3. Email Marketing

### [Email Marketing] Issue Brief: AI-Generated Customer Broadcasts
**Title**: Scout 🔍: Integrate Resend for AI-Powered Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling.
**Research Report**:
- **Tool**: Resend.
- **Target Persona**: Priya (Boutique Owner)
- **Evaluation**: Resend provides a developer-friendly, reliable email API. Instead of giving users a complex drag-and-drop builder, OHC can use the "Marketing" AI agent to generate beautiful HTML emails based on a simple text prompt from the user.
- **Advantages**: Simplicity and robust API.
- **Risks**: Relies on OHC's sender reputation for all merchants.
- **Ease of Use**: Zero-friction. The user types "Tell my customers about the new summer dress collection," and the AI generates the subject line, body, and inserts product photos automatically.
- **Pricing Estimate**: Resend charges around $20/mo for up to 50k emails, very economical to bundle into an OHC premium tier.
- **Compatibility (Cloud vs. Standalone)**: Cloud mode uses OHC's centralized Resend account. Standalone mode requires the user to input their own SMTP credentials.
**Design Doc**:
- "Marketing" tab -> "Send a Broadcast".
- User provides a 1-sentence prompt.
- The AI Agent generates a responsive HTML email preview.
- User clicks "Send to all customers".
- The system chunks the customer list and sends via the Resend API.
**Implementation Prompt**:
Create a feature where the user can prompt the AI to draft an email blast. Use the business's product catalog to enrich the email. Provide a preview UI. Once approved, queue the emails to be sent out via the Resend API to all opted-in customers, handling rate limits and basic bounce tracking.
**Priority**: P2
**Estimated Scope**: Medium

### [Email Marketing] Native Email Campaign Manager
**Title**: Native Email Campaign Manager (SendGrid/SES)
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.
**Research Report**:
- **Tool**: Native email campaign manager utilizing a transactional email API (SendGrid or AWS SES)
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool. No additional SaaS subscriptions required for the user.
- **Risks**: Requires building list management and unsubscribe logic internally.
- **Pricing Estimate**: Included in OHC platform costs (transactional API costs scale predictably).
- **Compatibility (Cloud vs. Standalone)**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).
**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
- **AI Integration**: The Marketing & Advertising Agent writes the subject lines, generates the copy, and tracks open/click rates to suggest the best times to send future emails.
**Implementation Prompt**: Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.
**Priority**: P1
**Estimated Scope**: Large

---

## 4. Payment Processing

### [Payment] Mercado Pago Integration
**Title**: Expand Payments with Mercado Pago for LATAM Users
**Problem Statement**: Non-US users in Latin America cannot rely solely on Stripe due to high fees, lack of local currency support, and specific local payment methods (like Pix in Brazil or OXXO in Mexico). Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods natively within the OHC platform, avoiding complex third-party payment routing.
**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU (LATAM).
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico) which are critical for conversion (often >50% of transactions). Good developer docs. Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools. API is well-documented. Settlement times are faster locally compared to cross-border Stripe.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing Estimate**: Variable by country (e.g., ~4-5% per transaction). Standard transaction fees apply; merchants expect these.
- **Compatibility (Cloud vs. Standalone)**: Works for both Cloud (via OHC platform account / OAuth) and Standalone (user supplies API keys).
**Design Doc**:
- In the "Finance & Payments" settings, users select their region/country during onboarding. If LATAM, Mercado Pago is highlighted as the recommended provider alongside Stripe.
- Setup involves standard OAuth flow or API key drop-in to connect their Mercado Pago account.
- Supports one-off payments and split payments for the eventual marketplace feature. Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.
- **AI Integration**: Finance & Payments Agent seamlessly aggregates revenue across providers into a unified native dashboard.
**Implementation Prompt**: Add Mercado Pago as a secondary payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow. Implement the checkout flow to dynamically switch to the appropriate provider based on the merchant's settings, redirect to Mercado Pago, and handle the success/failure webhooks to update order status. Webhooks must normalize into standard OHC order fulfillment events.
**Priority**: P1
**Estimated Scope**: Large

---

## 5. Shipping & Logistics

### [Shipping] Integrate EasyPost for Painless Shipping Labels & Tracking
**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**: Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number.
**Research Report**:
- **Tool**: EasyPost
- **Target Persona**: Priya (Boutique Owner)
- **Advantages**: EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL). Abstracts away complex carrier-specific APIs and handles tracking webhooks. Great fit for OHC physical product merchants.
- **Risks**: Dependence on a third-party aggregator.
- **Pricing Estimate**: Competitive pricing (free tier for low volume, pennies per label after).
- **Compatibility (Cloud vs. Standalone)**: Cloud (API). Standalone (API Key).
**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
**Priority**: P1
**Estimated Scope**: Medium

### [Shipping] Shippo Integration
**Title**: Native Shipping Rate Calculation and Label Generation (Shippo)
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.
**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC. Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing Estimate**: Free tier available (pay per label + postage), nominal fee per label thereafter.
- **Compatibility (Cloud vs. Standalone)**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button / "Buy Label", and OHC purchases and downloads the PDF label for printing via Shippo and saves the tracking number.
- OHC automatically emails the customer the tracking number.
- **AI Integration**: The Customer Success Agent monitors tracking numbers natively and proactively notifies the customer if a delivery is delayed.
**Implementation Prompt**: Implement a native shipping and fulfillment module powered by Shippo. Connect the Shippo API to fetch real-time shipping rates based on order weight/dimensions at checkout. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications

### [SMS] Twilio Integration
**Title**: Native SMS Order Notifications (Twilio)
**Problem Statement**: Fatima (Food Cart Operator) doesn't have a reliable internet connection at her cart, relies on her phone for everything, and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.
**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Twilio is the industry standard for SMS and WhatsApp messaging globally. Reliable delivery, deep global coverage. Supports WhatsApp, which is critical for markets outside the US. Simple API, integrates well with Go backend. Invisible to the user; they just toggle "Send SMS reminders" in their settings. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing Estimate**: Pay-per-message / Pay-as-you-go (~$0.0079 per SMS in US). Can be passed to the tenant, subsidized in premium tiers, or require merchants to buy "SMS Credits".
- **Compatibility (Cloud vs. Standalone)**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).
**Design Doc**:
- Users can enable "SMS Notifications" / toggle "Send me SMS for new orders" in the "Operations" settings.
- When an order is placed/paid, the OHC backend / Operations agent triggers a Twilio API call to dispatch async jobs to text the business owner (e.g., "New order! 2x Falafel for John. Pickup in 15m.").
- The Operations Agent decides the optimal time to send the reminder.
- Additionally, "The Ambassador" / customers can also receive SMS receipts/order confirmation texts / appointment reminders if they prefer SMS over email.
**Implementation Prompt**: Integrate the Twilio SDK to add Twilio integration for sending outbound SMS notifications to dispatch SMS order notifications to the business owner, and provide SMS-based order updates (confirmations, pickup notifications, and appointment reminders via text message) to end customers. Add a settings panel for the business owner/merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164) and compliance with local messaging regulations.
**Priority**: P0
**Estimated Scope**: Medium

---

## 7. Video Conferencing

### [Video Conferencing] Issue Brief: Auto-Generated Meeting Links
**Title**: Scout 🔍: Integrate Google Meet for Automated Online Lessons
**Problem Statement**:
For digital service providers like Leo (Music Tutor), manually creating Zoom or Google Meet links for every booked lesson and emailing them to the student is prone to human error (e.g., forgetting to send the link or sending the wrong one).
**Research Report**:
- **Tool**: Google Workspace API (Google Meet) or Zoom API.
- **Target Persona**: Leo (Music Tutor)
- **Evaluation**: Google Meet is often preferred as it can be automatically attached to any Google Calendar event created during the booking process. Zoom requires a separate OAuth flow.
- **Advantages**: Tightly coupled to Google Calendar.
- **Risks**: Relies on user having a Google Workspace/Calendar account.
- **Ease of Use**: Zero extra effort if the user has already connected Google Calendar for availability syncing. The system automatically provisions the link.
- **Pricing Estimate**: Free if using the user's existing Google Calendar/Meet integration.
- **Compatibility (Cloud vs. Standalone)**: Works natively in both.
**Design Doc**:
- When setting up a service, the user toggles "This is an online meeting".
- When a customer books the service, the OHC backend creates a Google Calendar event.
- The calendar event is configured to auto-generate a Google Meet conference link.
- The confirmation email sent to the customer includes this generated Meet link.
**Implementation Prompt**:
Extend the calendar booking flow to support online meetings. When a service is marked as "online", ensure the Google Calendar event creation request includes the conference data parameters to auto-generate a Google Meet link. Extract this link from the response and include it in the customer's confirmation email and the business owner's dashboard.
**Priority**: P1
**Estimated Scope**: Small

### [Video Conferencing] Embed Jitsi Meet for Zero-Setup Online Lessons
**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**: Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room.
**Research Report**:
- **Tool**: Jitsi Meet
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool. Requires no account for the student. Works natively in the browser and mobile. Completely seamless integration with no technical setup required by the user.
- **Risks**: Maintaining a Jitsi instance can be resource-intensive.
- **Pricing Estimate**: Free (open source), but requires hosting infrastructure. OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription.
- **Compatibility (Cloud vs. Standalone)**: Cloud (OHC hosted instance). Standalone (Public servers or self-hosted).
**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
**Priority**: P2
**Estimated Scope**: Small

### [Video] Zoom Integration
**Title**: Native Zoom Link Generation for Appointments
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.
**Research Report**:
- **Tool**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Strong API for meeting creation. Standard OAuth connection process. Highly intuitive.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing Estimate**: Free tier (40-min limit). Pro starts at $15/mo. API is free for Zoom users, but requires the merchant to have a Zoom account.
- **Compatibility (Cloud vs. Standalone)**: Cloud (OAuth). Standalone (Server-to-Server OAuth).
**Design Doc**:
- In the service creation flow, the user connects their Zoom account via the Sales dashboard / selects "Online Meeting" as the location and clicks "Connect Zoom".
- When a customer books an online service (e.g., via Calendly or native booking), OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the automated calendar invite and confirmation email sent to the customer.
- The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.
**Implementation Prompt**: Build a Zoom integration / OAuth integration with Zoom that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account. When a customer books a virtual service / service marked as "Online Meeting", the system must dynamically generate a unique Zoom link, store it with the booking, and include this link / share it in the customer's confirmation email and with both the merchant and the customer.
**Priority**: P2
**Estimated Scope**: Medium
