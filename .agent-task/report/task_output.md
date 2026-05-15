# 🔎 Scout: Tool Integration Research [quarter]

This document contains research and evaluation of various tools for integrating into OHC to empower small business owners in both Cloud and Standalone modes. The focus is on tools that provide immediate, tangible value to non-technical users.

## Category: Social Media Integration

### Tool: ManyChat Integration

#### Title: Integrate ManyChat for Social Media Integration

#### Problem Statement
Manual tracking of Instagram DMs and Facebook comments leads to delayed responses and lost sales.

**What problem it solves for which persona:**
For Carlos (Plumbing Services), this automates initial inquiries and routes critical leads directly to his phone, so he never misses a job request while on site.

#### Research Report
- **Overview:** ManyChat is a leading chat marketing platform focused on Instagram, Facebook Messenger, WhatsApp, and SMS.
- **Ease of Use:** Very user-friendly flow-builder interface. OAuth integration is standard.
- **Competitive Analysis:** Compared to Chatfuel, ManyChat offers superior Instagram automation and better visual flow building.
- **Key Advantages:** Excellent visual builder, official Meta partner, robust WhatsApp support.
- **Risks:** Highly dependent on Meta's API policies. A policy change can break core functionality.
- **Rough Pricing Estimate:** Free tier available. Pro starts at $15/month and scales with contact volume.
- **Cloud vs Standalone Mode:** Cloud: Webhooks for real-time messaging. Standalone: Local client pulling via long-polling or connecting to local Meta Graph API bridges.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Social Media Integration'.
- **Action:** Once authorized via an OAuth popup, OHC begins syncing messages from Instagram and Messenger into the unified inbox.
- **User Visibility:** The user will see a new 'ManyChat' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect ManyChat' button. After clicking, their social DMs start appearing alongside emails in the OHC Inbox.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to ManyChat', authorize the app, and immediately see social messages flow into OHC.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for ManyChat.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

### Tool: Sprout Social Integration

#### Title: Integrate Sprout Social for Social Media Integration

#### Problem Statement
Managing multiple social feeds across various platforms requires constantly switching apps and tabs.

**What problem it solves for which persona:**
For Sarah (Freelance Consultant), this provides a unified inbox, allowing her to schedule posts and respond to clients across LinkedIn and Twitter from one place.

#### Research Report
- **Overview:** Sprout Social is an enterprise-grade social media management and analytics platform.
- **Ease of Use:** High-end UI, very intuitive but feature-dense. Integration is straightforward via standard OAuth.
- **Competitive Analysis:** More expensive and robust than Hootsuite or Buffer, prioritizing deep analytics and enterprise workflows over simple scheduling.
- **Key Advantages:** Deep analytics, unified smart inbox, excellent listening tools.
- **Risks:** Cost is prohibitive for micro-businesses. API rate limits on lower tiers.
- **Rough Pricing Estimate:** Starts at $249/month. No meaningful free tier for long-term use.
- **Cloud vs Standalone Mode:** Cloud: Standard webhook and API integration. Standalone: Polling required, as they prefer delivering data to public webhooks.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Social Media Integration'.
- **Action:** Once authorized, OHC begins pulling unified social feed data and analytics summaries.
- **User Visibility:** The user will see a new 'Sprout Social' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Sprout Social' button. After clicking, a summary of social engagements appears on the OHC Dashboard.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Sprout Social', authorize the app, and see social engagement metrics in OHC.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Sprout Social.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: Calendar & Scheduling

### Tool: Calendly Integration

#### Title: Integrate Calendly for Calendar & Scheduling

#### Problem Statement
Back-and-forth emails to schedule meetings waste time and cause booking friction.

**What problem it solves for which persona:**
For Sarah (Freelance Consultant), this lets clients book consultations directly on her website, automatically syncing with her Google Calendar to prevent double-booking.

#### Research Report
- **Overview:** Calendly is the ubiquitous scheduling automation platform used globally.
- **Ease of Use:** Extremely simple for both the host and the invitee. Integrates easily with Google and Outlook.
- **Competitive Analysis:** Easier to set up than Acuity, but slightly less customizable for complex service-based businesses.
- **Key Advantages:** Widespread brand recognition, reliable timezone handling, seamless Zoom/Meet integration.
- **Risks:** Users might feel it's too 'impersonal'. Changes to Google/Microsoft calendar APIs can cause sync issues.
- **Rough Pricing Estimate:** Basic is free. Essentials starts at $8/month/seat.
- **Cloud vs Standalone Mode:** Cloud: Immediate webhook notifications on bookings. Standalone: Local app can periodically sync via REST API if webhooks are impossible.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Calendar & Scheduling'.
- **Action:** Once authorized, OHC starts syncing upcoming appointments and checking calendar availability.
- **User Visibility:** The user will see a new 'Calendly' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Calendly' button. After clicking, their upcoming meetings are visible in the OHC Calendar view.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Calendly', authorize the app, and manage their bookings directly inside OHC.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Calendly.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

### Tool: Acuity Scheduling Integration

#### Title: Integrate Acuity Scheduling for Calendar & Scheduling

#### Problem Statement
Service businesses need to manage complex appointment types, add-ons, and upfront payments.

**What problem it solves for which persona:**
For Fatima (Local Bakery Owner), this handles custom cake consultation bookings, allowing her to collect intake forms and deposits simultaneously.

#### Research Report
- **Overview:** Acuity (now part of Squarespace) is a highly customizable scheduling tool geared towards service professionals.
- **Ease of Use:** Steeper learning curve than Calendly, but offers deep customization for service menus.
- **Competitive Analysis:** More powerful for specific service businesses than Calendly; integrates tightly with Square and Stripe for deposits.
- **Key Advantages:** Deep customization, native payment collection, robust intake forms.
- **Risks:** UI can be overwhelming. Acquired by Squarespace, leading to potential platform lock-in concerns.
- **Rough Pricing Estimate:** Starts at $16/month. No free tier.
- **Cloud vs Standalone Mode:** Cloud: Robust webhooks. Standalone: API polling or required intermediate relay server.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Calendar & Scheduling'.
- **Action:** Once authorized, OHC imports service menus and syncs appointment intake data.
- **User Visibility:** The user will see a new 'Acuity Scheduling' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Acuity' button. After clicking, detailed appointment schedules and paid deposits appear in OHC.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Acuity', authorize the app, and see complex booking data flow into OHC.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Acuity Scheduling.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: Email Marketing

### Tool: Mailchimp Integration

#### Title: Integrate Mailchimp for Email Marketing

#### Problem Statement
Small businesses struggle to keep in touch with customers to drive repeat business without spending hours writing individual emails.

**What problem it solves for which persona:**
For Fatima (Local Bakery Owner), Mailchimp allows her to easily send out a monthly newsletter with photos of new cakes and a discount code, driving repeat orders.

#### Research Report
- **Overview:** Mailchimp is one of the most popular email marketing platforms globally, aimed specifically at small to medium businesses.
- **Ease of Use:** Very easy to use drag-and-drop template builder. Seamless integrations with most platforms.
- **Competitive Analysis:** More expensive at higher tiers than Sendinblue (Brevo) or MailerLite, but has the most user-friendly interface.
- **Key Advantages:** Great templates, huge ecosystem of integrations, intuitive UI.
- **Risks:** Pricing scales aggressively as the contact list grows. Strict compliance rules can lead to account suspension if spam complaints are high.
- **Rough Pricing Estimate:** Free tier up to 500 contacts. Essentials starts at $13/month.
- **Cloud vs Standalone Mode:** Cloud: Standard API for list sync. Standalone: Local system can push list updates via API.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Email Marketing'.
- **Action:** Once authorized via an API key, OHC begins pushing customer contact details to designated Mailchimp audiences.
- **User Visibility:** The user will see a new 'Mailchimp' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Mailchimp' button. After clicking, the customer list stays in sync automatically without manual CSV exports.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Mailchimp', authorize the app, and ensure their customer list is always up-to-date for newsletters.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Mailchimp.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

### Tool: Sendinblue (Brevo) Integration

#### Title: Integrate Sendinblue (Brevo) for Email Marketing

#### Problem Statement
Paying for email marketing based on contact list size punishes businesses that have large lists but send emails infrequently.

**What problem it solves for which persona:**
For Raj (E-commerce Seller), Brevo allows him to maintain a massive customer list without paying exorbitant fees, as he only pays for the actual emails sent during big sales.

#### Research Report
- **Overview:** Brevo (formerly Sendinblue) is a robust digital marketing platform offering email, SMS, and chat.
- **Ease of Use:** Slightly more complex than Mailchimp but very powerful. API is developer-friendly.
- **Competitive Analysis:** Pricing model (pay per email sent, not per contact) is a massive advantage over Mailchimp for certain businesses.
- **Key Advantages:** Cost-effective for large lists, includes SMS marketing natively.
- **Risks:** Template builder is not as slick as Mailchimp's. Deliverability can sometimes require more domain warmup.
- **Rough Pricing Estimate:** Free tier (300 emails/day). Starter is $25/month for 20k emails.
- **Cloud vs Standalone Mode:** Cloud: Webhooks for bounces/opens. Standalone: Reliable API for pushing list data.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Email Marketing'.
- **Action:** Once authorized, OHC syncs contacts and pulls basic campaign performance metrics.
- **User Visibility:** The user will see a new 'Sendinblue (Brevo)' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Brevo' button. After clicking, contact lists sync automatically.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Brevo', authorize the app, and manage their marketing contacts seamlessly.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Sendinblue (Brevo).
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: Payment Processing

### Tool: Mercado Pago Integration

#### Title: Integrate Mercado Pago for Payment Processing

#### Problem Statement
LATAM businesses need to accept local payment methods (Pix, Boletos) that standard international gateways ignore.

**What problem it solves for which persona:**
For Raj (E-commerce Seller) expanding to Brazil, this enables him to accept Pix payments instantly, drastically reducing cart abandonment in the region.

#### Research Report
- **Overview:** Mercado Pago is the dominant payment gateway in Latin America, born from Mercado Libre.
- **Ease of Use:** API is comprehensive but localized. Developer docs are mostly in Spanish/Portuguese.
- **Competitive Analysis:** Outperforms Stripe in LATAM due to deep integration with local banking systems and cash payment networks.
- **Key Advantages:** Unmatched local payment method support in LATAM, high consumer trust, fast settlement speeds locally.
- **Risks:** Customer support can be slow. API changes are sometimes poorly communicated.
- **Rough Pricing Estimate:** Varies by country, typically 3-5% + flat fee per transaction.
- **Cloud vs Standalone Mode:** Cloud: IPN (Instant Payment Notification) webhooks. Standalone: Requires a local server to expose an endpoint via ngrok/Cloudflare Tunnels or aggressive polling.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Payment Processing'.
- **Action:** Once authorized via API credentials, OHC enables Mercado Pago as a checkout option for invoices.
- **User Visibility:** The user will see a new 'Mercado Pago' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Mercado Pago' button. After clicking, clients see Pix and Boletos as payment options.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Mercado Pago', enter credentials, and immediately start accepting LATAM payments.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Mercado Pago.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

### Tool: Alipay Integration

#### Title: Integrate Alipay for Payment Processing

#### Problem Statement
Capturing the Chinese market is impossible without supporting domestic digital wallets.

**What problem it solves for which persona:**
For Raj (E-commerce Seller), integrating Alipay opens his store to millions of Chinese consumers who do not use Western credit cards.

#### Research Report
- **Overview:** Alipay is one of the two dominant mobile payment platforms in China (alongside WeChat Pay).
- **Ease of Use:** Integration requires navigating strict Chinese regulatory compliance and cross-border payment rules.
- **Competitive Analysis:** Essential for China; not directly comparable to Western gateways. Competes primarily with WeChat Pay.
- **Key Advantages:** Access to massive user base, seamless mobile checkout experience.
- **Risks:** High regulatory risk, complex onboarding process for non-Chinese entities.
- **Rough Pricing Estimate:** Typically around 2.9% for cross-border transactions.
- **Cloud vs Standalone Mode:** Cloud: Standard asynchronous notifications. Standalone: Highly complex due to strict IP whitelisting and security requirements by Alipay.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Payment Processing'.
- **Action:** Once authorized, OHC enables Alipay QR codes and direct mobile payment links.
- **User Visibility:** The user will see a new 'Alipay' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Alipay' button. After clicking, Chinese customers can pay seamlessly via QR code.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Alipay', complete onboarding, and start accepting payments from Chinese users.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Alipay.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: Shipping & Logistics

### Tool: ShipStation Integration

#### Title: Integrate ShipStation for Shipping & Logistics

#### Problem Statement
Manually copying orders into carrier websites to print labels is error-prone and time-consuming.

**What problem it solves for which persona:**
For Raj (E-commerce Seller), ShipStation automatically pulls orders from Shopify and prints FedEx/USPS labels in bulk, saving him hours daily.

#### Research Report
- **Overview:** ShipStation is a leading web-based order management and shipping software.
- **Ease of Use:** Very user-friendly dashboard. Connecting carrier accounts is wizard-driven.
- **Competitive Analysis:** More feature-rich than Shippo for complex workflows, but slightly more expensive base cost.
- **Key Advantages:** Massive list of supported carriers and selling channels, excellent automation rules.
- **Risks:** Occasional sync delays during peak holiday seasons. Pricing can escalate with high volume.
- **Rough Pricing Estimate:** Starts at $9.99/month for 50 shipments.
- **Cloud vs Standalone Mode:** Cloud: Seamless API integration. Standalone: Local printing requires ShipStation Connect installed on the local machine.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Shipping & Logistics'.
- **Action:** Once authorized, OHC begins pushing new orders to ShipStation and pulling back tracking numbers.
- **User Visibility:** The user will see a new 'ShipStation' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect ShipStation' button. After clicking, new orders are automatically queued for label printing.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to ShipStation', authorize the app, and automate label generation for all new orders.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for ShipStation.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: SMS & Notifications

### Tool: Twilio Integration

#### Title: Integrate Twilio for SMS & Notifications

#### Problem Statement
Important updates (like appointment reminders or order deliveries) are ignored if sent only via email.

**What problem it solves for which persona:**
For Fatima (Local Bakery Owner), sending a quick SMS when a custom cake is ready for pickup ensures the customer sees it immediately.

#### Research Report
- **Overview:** Twilio is the industry standard API for SMS, voice, and video communications.
- **Ease of Use:** Developer-first API. Not meant for non-technical users directly; OHC must abstract it completely.
- **Competitive Analysis:** More robust and globally reliable than smaller players like TextMagic, though slightly more complex to configure A2P 10DLC compliance.
- **Key Advantages:** Unmatched global reach, high deliverability, massive scalability.
- **Risks:** Strict regulatory compliance (A2P 10DLC in the US) can cause message filtering if not set up correctly by the business owner.
- **Rough Pricing Estimate:** Pay-as-you-go, roughly $0.0079 per SMS in the US.
- **Cloud vs Standalone Mode:** Cloud: Direct API calls. Standalone: Local system makes outbound HTTP requests to Twilio API; works perfectly without inbound ports.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'SMS & Notifications'.
- **Action:** Once authorized via API credentials, OHC enables automated SMS triggers for critical events.
- **User Visibility:** The user will see a new 'Twilio' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Twilio' button. After clicking, automated SMS reminders are instantly enabled for customer appointments.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Twilio', input their credentials, and automatically send SMS notifications.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Twilio.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium

## Category: Video Conferencing

### Tool: Zoom Integration

#### Title: Integrate Zoom for Video Conferencing

#### Problem Statement
Manually creating meeting links and emailing them for every online consultation leads to missing links and late starts.

**What problem it solves for which persona:**
For Sarah (Freelance Consultant), integrating Zoom automatically generates a unique meeting link the moment a client books a slot, attaching it to the calendar invite.

#### Research Report
- **Overview:** Zoom is the dominant video communications platform.
- **Ease of Use:** OAuth integration is standard. End-user experience is ubiquitous.
- **Competitive Analysis:** More reliable video quality than standard Google Meet, but requires a separate app download for clients.
- **Key Advantages:** High reliability, widespread familiarity, robust API for creating/managing meetings.
- **Risks:** Security/privacy concerns (though improved). API rate limits for free tier accounts.
- **Rough Pricing Estimate:** Basic is free (40 min limit). Pro is $15.99/month.
- **Cloud vs Standalone Mode:** Cloud: Standard Server-to-Server OAuth. Standalone: Local OHC instance can make outbound API calls to generate links on demand.

#### Design Doc
**High-Level Integration:**
- **Trigger:** The integration is activated from the OHC Settings panel under 'Integrations' -> 'Video Conferencing'.
- **Action:** Once authorized via OAuth, OHC can automatically provision meeting links for new calendar events.
- **User Visibility:** The user will see a new 'Zoom' widget or indicator on their dashboard. There will be no complex technical configuration exposed to the user; it should be a 'one-click' feel wherever possible.
- **How it appears to the business owner:** A simple 'Connect Zoom' button. After clicking, Zoom links are automatically added to all new client bookings.

#### Implementation Prompt
**Outcome:** The business owner can click 'Connect to Zoom', authorize the app, and automate meeting link generation.
**Acceptance Criteria:**
1. A 'Connect' button is available in the Integrations UI specifically for Zoom.
2. The connection flow securely handles authentication (OAuth or token) without exposing secrets to the frontend.
3. Core functionality (as defined by the category) operates seamlessly. Errors are caught and displayed as friendly UI toasts, not technical stack traces.
4. Disconnecting the app securely removes integration tokens and halts background syncs.
5. The architecture supports both Cloud and Standalone environments seamlessly.

**Priority:** P1
**Estimated Scope:** Medium
