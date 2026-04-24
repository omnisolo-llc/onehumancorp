# OHC Tool Integration Research Report

## [social] Unified Social Media Inbox via Chatwoot
**Title**: Integrate Chatwoot for Unified Social Media Inbox (Instagram, Facebook, WhatsApp)
**Problem Statement**: "I get messages from customers on Instagram, Facebook, and WhatsApp. I miss orders because I can't keep track of 3 different apps on my phone while I'm baking." (Maya - The Home Baker)
**Research Report**:
Chatwoot is an open-source, omnichannel customer support inbox. It directly connects with Instagram DMs, Facebook Messenger, WhatsApp (via Cloud API), and Web Chat.
- **Ease of use**: Very high for the end-user. Once connected, all messages appear in one single chat view.
- **Pricing**: Free tier exists, open-source version is self-hostable (which fits OHC's Standalone mode). Cloud pricing is very affordable.
- **Reputation**: High. A solid alternative to Intercom for small businesses.
- **Cloud vs Standalone**: Works seamlessly in both. Standalone can run the open-source Chatwoot docker container. Cloud can use Chatwoot Cloud or a managed instance.
**Design Doc**:
OHC's "Customer Success - The Ambassador" AI agent will monitor the unified Chatwoot inbox. When a customer messages Maya on Instagram ("do you do vegan cakes?"), Chatwoot routes it to OHC. The AI agent drafts a reply based on Maya's business context. Maya sees a simple "Inbox" in her OHC app where she can approve the AI draft or type her own reply. OHC handles the OAuth flows to connect the social accounts to Chatwoot behind the scenes.
**Implementation Prompt**:
Build a unified "Inbox" UI in the OHC mobile/web app where business owners can connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages from these platforms should appear in a single thread per customer. The AI agent should automatically draft suggested replies for new messages. The user should be able to send replies directly from the OHC app back to the original social platform.
**Priority**: P0
**Estimated Scope**: Large

---

## [calendar] Seamless Calendar & Booking via Cal.com
**Title**: Integrate Cal.com for Scheduling and Booking
**Problem Statement**: "I spend hours texting students back and forth trying to find a lesson time. Sometimes I double-book myself with my personal calendar." (Leo - The Music Tutor)
**Research Report**:
Cal.com is an open-source scheduling tool (Calendly alternative).
- **Ease of use**: High. Provides simple booking links and embeds.
- **Pricing**: Free for individuals (perfect for OHC free tier), open-source for self-hosting.
- **Reputation**: Excellent. Very developer-friendly, robust timezone handling.
- **Cloud vs Standalone**: Self-hostable for Standalone mode, SaaS API available for Cloud mode.
**Design Doc**:
Integrate Cal.com into the "Operations - The Manager" department. When Carlos or Leo sets up their service, OHC provisions a Cal.com event type behind the scenes. The public storefront displays a booking calendar. When a customer books a slot, Cal.com handles timezone conversion and calendar conflict checking against the owner's connected Google/Apple calendar. OHC listens to Cal.com webhooks to trigger Stripe deposits and automated reminders.
**Implementation Prompt**:
Create a "Booking Calendar" feature for service-based businesses. The user should be able to connect their personal Google/Apple calendar to prevent double-booking. The public storefront should show available time slots for customers to book. Once booked, both the business owner and the customer should receive confirmation emails with calendar invites.
**Priority**: P0
**Estimated Scope**: Medium

---

## [email] Automated Email Marketing via Resend
**Title**: Integrate Resend for Transactional & Marketing Emails
**Problem Statement**: "I want to email my past customers when I get new boutique stock, but Mailchimp is too complicated and expensive." (Priya - The Boutique Owner)
**Research Report**:
Resend is an API-first email sending platform designed for developers but enables building simple user-facing email tools.
- **Ease of use**: OHC will abstract the complexity. Users will just tell the AI what they want to announce.
- **Pricing**: 3,000 free emails per month (covers most small businesses).
- **Reputation**: Extremely high, known for great deliverability and modern React Email templates.
- **Cloud vs Standalone**: Cloud-native, but Standalone users can easily provide their own API key or use OHC's shared proxy limit.
**Design Doc**:
The "Marketing & Advertising - The Promoter" agent uses Resend to send beautifully formatted emails. Priya tells her OHC app, "Email my customers about the new summer dresses." The AI drafts the email, generates a preview using a standard OHC Glassmorphism template, and upon approval, uses Resend's API to broadcast it to her customer list. Resend webhooks feed open/click metrics back to the "Business Advisory" agent.
**Implementation Prompt**:
Implement an "Email Broadcast" feature where the AI agent drafts marketing emails based on a simple user prompt. The user can review the draft, select which customer segments to send to (e.g., "all past customers"), and click send. Track and display basic metrics like open rates in the business dashboard.
**Priority**: P1
**Estimated Scope**: Medium

---

## [payments] Localized Payment Processing via Mercado Pago
**Title**: Integrate Mercado Pago for LATAM Payment Processing
**Problem Statement**: "My customers in Brazil and Mexico don't use credit cards as much; they need PIX or local bank transfers, which Stripe doesn't always handle well for my region." (Standalone OHC User in LATAM)
**Research Report**:
Mercado Pago is the leading payment processor in Latin America, supporting local payment methods like PIX (Brazil), OXXO (Mexico), and local debit cards.
- **Ease of use**: Familiar to LATAM users.
- **Pricing**: Transaction-based, competitive for the region.
- **Reputation**: The standard for LATAM e-commerce.
- **Cloud vs Standalone**: APIs work well in both modes.
**Design Doc**:
Expand the "Finance & Payments - The Accountant" department to support multiple payment gateways. In the OHC settings, users in supported regions can choose Mercado Pago instead of Stripe. The checkout flow on the storefront will dynamically render the Mercado Pago Web Tokenizer or redirect to their checkout experience for PIX/OXXO payments. Webhooks will reconcile the order status in OHC.
**Implementation Prompt**:
Add Mercado Pago as an alternative payment provider to Stripe. Users should be able to connect their Mercado Pago account. The public storefront checkout must support generating PIX codes (for Brazil) and standard LATAM card payments. Orders should only be marked as 'Paid' when the Mercado Pago webhook confirms successful capture.
**Priority**: P2
**Estimated Scope**: Large

---

## [shipping] Automated Shipping Rates & Labels via Shippo
**Title**: Integrate Shippo for Shipping Labels and Real-time Rates
**Problem Statement**: "I never know how much to charge for shipping my art prints, and waiting in line at the post office to buy labels wastes my whole morning." (Creative Portfolio Persona)
**Research Report**:
Shippo is a multi-carrier shipping API that aggregates rates from USPS, UPS, FedEx, DHL, etc.
- **Ease of use**: End-user just enters package dimensions and clicks "Buy Label".
- **Pricing**: Pay-as-you-go, discounted carrier rates. Very small business friendly.
- **Reputation**: High reliability, standard in SMB e-commerce.
- **Cloud vs Standalone**: Cloud API, easily accessible in both modes.
**Design Doc**:
The "Operations - The Manager" agent connects to Shippo. When setting up a physical product, the user inputs rough weight/dimensions. At checkout, OHC calls Shippo to show the customer real-time shipping costs. When Maya is ready to fulfill an order, she clicks "Print Shipping Label" in the OHC app. OHC purchases the label via Shippo, deducts the cost, and provides a PDF for Maya to print from her phone. The AI automatically emails the tracking number to the customer.
**Implementation Prompt**:
Build a shipping management flow for physical products. The checkout page should calculate and display accurate shipping rates based on the customer's address. In the order management view, the business owner must be able to purchase and generate a printable PDF shipping label with one click. The system should automatically send the tracking link to the buyer.
**Priority**: P1
**Estimated Scope**: Medium

---

## [sms] Global SMS Notifications via Twilio
**Title**: Integrate Twilio for Critical SMS Notifications
**Problem Statement**: "I don't check email while I'm cooking. I need a text message the second someone pre-orders a meal so I can start making it." (Fatima - The Food Cart Operator)
**Research Report**:
Twilio is the industry standard for programmatic SMS.
- **Ease of use**: Transparent to the user. They just provide their phone number.
- **Pricing**: Per-message pricing (fractions of a cent).
- **Reputation**: Enterprise-grade reliability, global carrier coverage.
- **Cloud vs Standalone**: Cloud API. For Standalone, user can provide their own Twilio credentials or use an OHC proxy.
**Design Doc**:
The "Customer Success" and "Operations" agents use Twilio to send critical alerts. Fatima configures her OHC app to "Notify me by SMS for new orders." When an order is placed, the backend triggers a Twilio SMS. Additionally, for customers who prefer SMS over email, OHC can send order ready/pickup notifications via SMS.
**Implementation Prompt**:
Add an SMS notification preference for business owners to receive instant text alerts for new orders or bookings. Also, allow customers to opt-in to SMS updates at checkout (e.g., "Your food is ready for pickup!"). Ensure the integration handles international phone number formatting correctly.
**Priority**: P0
**Estimated Scope**: Small

---

## [video] Auto-Generated Video Meeting Links via Zoom API
**Title**: Integrate Zoom for Automated Video Conferencing Links
**Problem Statement**: "When a student books an online guitar lesson, I have to manually create a Zoom link and email it to them. Sometimes I forget and we miss the lesson." (Leo - The Music Tutor)
**Research Report**:
Zoom API allows programmatic meeting creation. (Alternative: Google Meet via Calendar API).
- **Ease of use**: Zero touch once authorized.
- **Pricing**: Zoom requires a Pro account for API usage, which might be a blocker for free-tier users. (Google Meet might be a better fallback as it's free with Google Calendar).
- **Reputation**: Universal familiarity.
- **Cloud vs Standalone**: Both support OAuth flows.
**Design Doc**:
Works closely with the Cal.com integration. When Leo sets up his "Online Guitar Lesson" service, he selects "Video Call" as the location. He authorizes Zoom via OAuth. When a student books, the "Operations" agent calls the Zoom API to generate a unique meeting URL, attaches it to the calendar invite, and includes it in the confirmation email.
**Implementation Prompt**:
Enhance the booking system to support "Virtual" locations. Allow the business owner to connect their Zoom account (or Google Meet). When a customer books a virtual service, automatically generate a unique video meeting link and include it in both the business owner's and the customer's calendar events and confirmation notifications.
**Priority**: P1
**Estimated Scope**: Small
