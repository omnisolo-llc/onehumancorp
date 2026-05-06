# Tool Integration Research Q3

## [Social Media Integration] Unified Social Inbox
**Title**: Integrate ManyChat / Chatwoot for Unified Social Media Inbox

**Problem Statement**: Small business owners are overwhelmed by managing customer messages across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Constantly switching apps leads to missed messages, slow response times, and lost sales. They need a single place to see and reply to all customer inquiries.

**Research Report**:
- **Persona Context**: Small retailers and service providers who rely heavily on social media for lead generation but lack dedicated customer support staff.
- **Solution Evaluated**: ManyChat and Chatwoot. Chatwoot provides a unified inbox that aggregates conversations from major platforms and has strong WhatsApp/Facebook integration. ManyChat is better for automated flows but more complex.
- **Ease of Use**: Chatwoot is very straightforward for a non-technical user. It looks like a standard email or chat inbox.
- **Advantages**: Solves the fragmentation problem perfectly. Can be self-hosted (Standalone) or consumed as SaaS (Cloud).
- **Risks**: Relying on Meta's official API limits (WhatsApp requires business accounts). Rate limits and account blocks for small users.
- **Pricing Estimate**: Chatwoot ranges from free (self-hosted or basic cloud) to $19/user/month.
- **Cloud/Standalone Support**: Works in both Cloud (via SaaS APIs) and Standalone (can self-host or integrate via API tokens locally).

**Design Doc**:
- **Triggers**: Incoming messages on connected social platforms trigger an event in OHC.
- **Actions**: OHC displays the incoming message in a unified inbox tab within the Slint UI.
- **User Interface**: The user sees a "Social Messages" section in OHC where they can link their social accounts via a simple OAuth flow (or API key for Standalone). Once linked, messages appear in a chat-like interface. Replying in OHC sends the message back to the native platform.

**Implementation Prompt**:
Build a unified inbox interface in OHC where users can read and reply to messages from Instagram, Facebook, and WhatsApp. The user must be able to authenticate their social accounts easily from the settings page. Ensure replies sent from OHC appear natively on the customer's social media app.

**Priority**: P0
**Estimated Scope**: Large


## [Calendar & Scheduling] Seamless Booking & Calendar Sync
**Title**: Integrate Cal.com for Zero-Friction Booking

**Problem Statement**: Service-based business owners (consultants, tutors, handymen) waste hours going back and forth over email or text to find a time to meet with clients. They need a simple link to send clients that automatically syncs with their personal calendar.

**Research Report**:
- **Persona Context**: Solo entrepreneurs and service businesses whose core product is their time.
- **Solution Evaluated**: Calendly vs. Cal.com. Both offer excellent user experiences. Cal.com is open-source, highly embeddable, and developer-friendly.
- **Ease of Use**: Very high. Owners just share a link.
- **Advantages**: Cal.com supports round-robin scheduling, Zoom/Meet link generation, and handles timezones effortlessly. Open-source nature aligns with OHC's hybrid model.
- **Risks**: Syncing personal calendars (Google/Outlook) requires sensitive OAuth scopes, which might concern some privacy-focused standalone users.
- **Pricing Estimate**: Cal.com is free for individuals, $15/user/month for teams.
- **Cloud/Standalone Support**: Works in Cloud (via Cal.com API) and Standalone (self-hosting or local API integrations).

**Design Doc**:
- **Triggers**: A customer books a slot via the generated booking link.
- **Actions**: OHC receives a notification, blocks out the time on the owner's synced calendar, and generates a notification.
- **User Interface**: A "Scheduling" tab where the owner clicks "Connect Google/Outlook Calendar" and sets their available hours. OHC provides a shareable booking link. Upcoming appointments appear on the OHC dashboard.

**Implementation Prompt**:
Create a scheduling feature that allows users to connect their Google or Outlook calendar. Generate a public booking page for their clients. When a client books, the appointment should appear on the OHC dashboard and block the corresponding time on the user's connected calendar.

**Priority**: P1
**Estimated Scope**: Medium


## [Email Marketing] Automated Customer Outreach
**Title**: Integrate Resend / MailerLite for Simple Email Campaigns

**Problem Statement**: Small businesses have a list of past customers but no easy way to send them updates, promotions, or newsletters. Enterprise tools like Mailchimp have become too bloated, expensive, and intimidating for simple use cases.

**Research Report**:
- **Persona Context**: Local shops, bakeries, and creators wanting to announce new products or seasonal sales to their existing customer base.
- **Solution Evaluated**: MailerLite for visual builders, Resend for developer-friendly transactional + marketing emails. MailerLite is vastly superior for non-technical users due to its intuitive drag-and-drop builder.
- **Ease of Use**: MailerLite is very accessible for non-designers. Resend requires more custom UI work from our side to make it usable for the end-user.
- **Advantages**: High deliverability, simple list management, robust analytics (open rates, clicks).
- **Risks**: Spam compliance (CAN-SPAM/GDPR). Users might accidentally send spam and get their domains blacklisted.
- **Pricing Estimate**: MailerLite is free up to 1,000 subscribers, then starts at $9/month.
- **Cloud/Standalone Support**: Cloud-native SaaS integration. Standalone mode can bridge to these services using API keys provided by the user.

**Design Doc**:
- **Triggers**: User initiates a "New Campaign" or an automated flow (e.g., "Welcome Email" when a new customer is added).
- **Actions**: OHC syncs the local customer list to the email provider and triggers the send.
- **User Interface**: A "Marketing" tab where users can select segments of their customer list, write a plain-text or simple rich-text email, and click "Send". Analytics (opens/clicks) are displayed next to past campaigns.

**Implementation Prompt**:
Implement a simple email campaign tool. Users should be able to select contacts from their OHC customer list, compose an email with a subject and body, and send it. Show a basic summary of sent campaigns with open and click rates. Ensure users can easily provide their own API keys for the email service in Standalone mode.

**Priority**: P2
**Estimated Scope**: Medium


## [Payment Processing] Global Localized Payments
**Title**: Integrate Mercado Pago & Alipay for Local Market Penetration

**Problem Statement**: While Stripe is excellent in the US and Europe, business owners in LATAM, India, or Asia struggle with high fees, lack of local payment methods (like PIX or Alipay), or lack of support altogether. They need payment processors that their local customers actually use.

**Research Report**:
- **Persona Context**: Merchants in emerging markets (e.g., Brazil, Mexico, China) who sell online or via chat and need to collect payments seamlessly.
- **Solution Evaluated**: Mercado Pago for LATAM, Alipay for China. Both dominate their respective markets.
- **Ease of Use**: For the business owner, it's a standard OAuth or API key setup. For their customers, it's the familiar, trusted local checkout experience.
- **Advantages**: Drastically increases conversion rates in these regions. Faster local settlement.
- **Risks**: Fragmented API designs. Handling currency conversions and localized dispute/refund processes can be complex.
- **Pricing Estimate**: Standard local transaction fees (varies by region, typically 1.5% to 3.5%).
- **Cloud/Standalone Support**: Fully supported in both modes. Standalone users manage their own API credentials.

**Design Doc**:
- **Triggers**: A user generates an invoice or payment link in OHC.
- **Actions**: OHC calls the respective payment gateway to generate a localized checkout URL or QR code.
- **User Interface**: In settings, users can enable local payment methods alongside or instead of Stripe. When creating a bill, they can generate a Mercado Pago payment link or an Alipay QR code to send to the customer.

**Implementation Prompt**:
Add support for generating payment links via Mercado Pago and Alipay. Users must be able to configure these providers in their billing settings. When they create an invoice, allow them to choose the provider and generate a shareable checkout link or QR code for their customers.

**Priority**: P1
**Estimated Scope**: Large


## [Shipping & Logistics] One-Click Label Generation
**Title**: Integrate Shippo for Multi-Carrier Shipping Labels

**Problem Statement**: E-commerce sellers waste hours copying addresses from orders into carrier websites (USPS, UPS, FedEx, local post) to buy and print shipping labels. They need a way to generate labels instantly when an order comes in.

**Research Report**:
- **Persona Context**: Boutique owners, crafters, and independent e-commerce sellers shipping physical goods.
- **Solution Evaluated**: Shippo and EasyPost. Both aggregate carriers. Shippo has a slightly more forgiving onboarding for small businesses and excellent international coverage.
- **Ease of Use**: Drastically simplifies the shipping process. Instead of 10 clicks across different tabs, it's 2 clicks in one place.
- **Advantages**: Real-time rate comparisons across carriers, discounted rates, automatic tracking updates.
- **Risks**: Label printing requires exact dimensions and weights; user error here can lead to under-postage penalties.
- **Pricing Estimate**: Shippo is free to install, $0.05 per label + carrier postage costs.
- **Cloud/Standalone Support**: Cloud API integration; perfectly suitable for Standalone via user-provided API tokens.

**Design Doc**:
- **Triggers**: An order is marked as "Ready to Ship".
- **Actions**: OHC fetches shipping rates from Shippo, purchases the label upon user confirmation, and retrieves the PDF label and tracking number.
- **User Interface**: An "Orders" view. Clicking on an order shows a "Buy Shipping Label" button. The user enters package weight/dimensions, selects the cheapest carrier rate, and clicks "Print Label". The tracking number is automatically saved.

**Implementation Prompt**:
Build a shipping label generation feature. Users should see pending orders, input package dimensions, compare rates across carriers, and purchase a label. Provide a way to view and print the generated PDF label, and store the tracking number with the order details.

**Priority**: P2
**Estimated Scope**: Medium


## [SMS & Notifications] Reliable Global Texting
**Title**: Integrate Twilio / MessageBird for Critical SMS Alerts

**Problem Statement**: Some customers (and business owners) don't use email frequently or have low English proficiency, preferring simple text messages. Businesses need a reliable way to send appointment reminders, pickup notifications, or delivery updates via SMS to reduce no-shows and improve service.

**Research Report**:
- **Persona Context**: Local service providers (salons, mechanics, clinics) and their diverse customer base, including non-tech-savvy individuals.
- **Solution Evaluated**: Twilio and MessageBird. Twilio is the industry standard with massive global reach. MessageBird offers great omnichannel capabilities.
- **Ease of Use**: Invisible to the end-customer. For the business owner, it's just toggling "Send SMS reminder".
- **Advantages**: 98% open rate for SMS. Drastically reduces appointment no-shows.
- **Risks**: Telecom regulations (A2P 10DLC in the US) make onboarding complex for small businesses. High costs for international SMS.
- **Pricing Estimate**: ~$0.0079 per message in the US, higher internationally.
- **Cloud/Standalone Support**: Works in both. Standalone users input their Twilio SID and Auth Token.

**Design Doc**:
- **Triggers**: An appointment is approaching (e.g., 24 hours prior) or an order is ready for pickup.
- **Actions**: OHC sends a templated text message to the customer's phone number.
- **User Interface**: A simple toggle in the scheduling or order management settings: "Send SMS reminders to customers". In Standalone mode, a settings page to input Twilio credentials.

**Implementation Prompt**:
Implement automated SMS notifications for key events like upcoming appointments or order pickups. Provide a straightforward settings panel for users to enable SMS and configure their provider credentials. Ensure the system handles phone number formatting (E.164) gracefully.

**Priority**: P1
**Estimated Scope**: Small


## [Video Conferencing] Auto-Generated Meeting Links
**Title**: Integrate Zoom / Google Meet for Virtual Consultations

**Problem Statement**: Tutors, therapists, and consultants who work remotely struggle with manually creating Zoom links for every meeting and emailing them to clients. They need meeting links to be generated automatically when a booking is made.

**Research Report**:
- **Persona Context**: Remote service providers, educators, and telehealth professionals.
- **Solution Evaluated**: Zoom API and Google Meet (via Google Calendar API). Google Meet is often preferred for its zero-install web experience.
- **Ease of Use**: Zero effort once connected. The link just appears on the calendar invite.
- **Advantages**: Eliminates manual copy-pasting of links. Professional appearance for the client.
- **Risks**: Zoom's OAuth approval process for public apps is stringent. Google Meet requires broader Google Calendar permissions.
- **Pricing Estimate**: APIs are generally included in the user's existing Zoom Pro ($15/mo) or Google Workspace subscription.
- **Cloud/Standalone Support**: Supported in both via OAuth (Cloud) or direct API integration (Standalone).

**Design Doc**:
- **Triggers**: A new appointment is scheduled through the OHC booking system.
- **Actions**: OHC requests a meeting link from the connected video provider and attaches it to the appointment details.
- **User Interface**: A "Video Conferencing" settings page to connect Zoom or Google. When viewing an upcoming appointment, a prominent "Join Meeting" button is displayed for both the business owner and the customer.

**Implementation Prompt**:
Enable automatic video meeting link generation for appointments. Allow users to link their Zoom or Google Meet accounts. When an appointment is created, automatically generate a meeting link and display a "Join Meeting" button in the appointment details view.

**Priority**: P2
**Estimated Scope**: Medium
