# Tool Integration Research Report Q4

## [Social Media] Unified Inbox Integration: ManyChat / Meta Webhooks

- **Title**: Integrate ManyChat / Meta Webhooks for Unified Social Media Inbox
- **Problem Statement**: Small business owners (like local bakers or salons) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Constantly switching apps to reply causes them to miss messages, lose track of customer context, and ultimately lose sales. They need a single, simple inbox to see and reply to all social media messages.
- **Research Report**:
  - **Findings**: ManyChat provides a robust API for Instagram, FB Messenger, and WhatsApp, which handles the complex Meta Meta Graph API approvals. Alternatively, native Meta Webhooks can be used but require a steep learning curve and business verification.
  - **Ease of Use**: Once connected via a standard OAuth flow, the business owner simply sees messages appear in the OHC app and replies natively. Very easy for non-technical users.
  - **Pricing**: ManyChat is ~$15/mo for Pro (required for API access). Native Meta API is technically free but costs messaging fees for WhatsApp (approx. $0.015 per conversation).
  - **Cloud vs Standalone**: Works well in Cloud. Standalone might require a proxy server or ngrok-like tunnel for webhooks to be delivered locally.
- **Design Doc**: OHC will feature an "Inbox" tab. Users click "Connect Instagram/Facebook". This triggers an OAuth popup. Once connected, incoming messages are routed via webhook to OHC and stored in the database. When the user replies in OHC, the backend calls the respective API to send the message back to the customer on the original platform.
- **Implementation Prompt**: Create a user-facing Unified Inbox view where business owners can connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages should appear in a single conversation thread per customer. Replies sent from the OHC Inbox must be delivered back to the customer's social media app. The integration flow must be plain-language and take less than 3 clicks.
- **Priority**: P0
- **Estimated Scope**: Large


## [Calendar & Scheduling] Appointment Sync: Calendly / Google Calendar API

- **Title**: Integrate Google Calendar / Calendly for Automated Scheduling
- **Problem Statement**: Service-based businesses (like consultants or tutors) spend hours going back and forth over email or text to find a meeting time. Double bookings are common, leading to frustrated customers and lost revenue. They need a simple way to let customers pick an available time that automatically syncs with their personal calendar.
- **Research Report**:
  - **Findings**: Google Calendar API is the standard for direct calendar sync. Calendly provides a higher-level embeddable booking page that handles timezone conversion and conflict resolution automatically.
  - **Ease of Use**: Calendly is extremely user-friendly for both the owner and the customer. Connecting Google Calendar directly requires a simple OAuth flow but OHC would have to build the booking UI.
  - **Pricing**: Google Calendar API is free (usage limits apply). Calendly has a free tier, but the API requires a paid plan ($12/mo).
  - **Cloud vs Standalone**: Both work in Cloud and Standalone (OAuth tokens can be stored locally).
- **Design Doc**: Provide a "Scheduling" settings page. The user connects their Google Calendar via OAuth. OHC generates a public booking link (e.g., `ohc.com/book/mybusiness`). When a customer visits the link, they see available slots based on the user's free/busy status. Booking an appointment creates an event in the user's Google Calendar.
- **Implementation Prompt**: Build a feature allowing business owners to connect their Google Calendar and generate a public booking page for their clients. The booking page must automatically read the owner's free/busy times, adjust for the customer's timezone, and prevent double-booking. Once a client books, the appointment should instantly appear on the owner's Google Calendar.
- **Priority**: P1
- **Estimated Scope**: Medium


## [Email Marketing] Campaign Management: MailerLite / SendGrid

- **Title**: Integrate MailerLite for Simple Customer Newsletters
- **Problem Statement**: Small businesses want to send promotional emails (like holiday sales or new product announcements) to their customer list, but find tools like Mailchimp too complex and expensive. They need a simple way to blast plain-text or basic graphical emails to their past customers directly from their customer list.
- **Research Report**:
  - **Findings**: MailerLite offers a very generous free tier (up to 1,000 subscribers) and a simple API. SendGrid is more developer-focused and harder for non-technical users to manage templates.
  - **Ease of Use**: MailerLite is known for its clean, beginner-friendly interface. Integrating it so OHC automatically syncs the customer list to a MailerLite group would save users from manual CSV exports/imports.
  - **Pricing**: MailerLite is Free up to 1k subscribers, then $9/mo. SendGrid has a free tier for 100 emails/day.
  - **Cloud vs Standalone**: Works in both environments via standard API requests.
- **Design Doc**: A "Marketing" tab allows the user to paste a MailerLite API key (with instructions on where to find it). Whenever a new customer is added to OHC (via booking or sale), they are automatically synced to the MailerLite subscriber list. Users can trigger basic pre-written templates from OHC or write custom newsletters that are sent via the API.
- **Implementation Prompt**: Implement an email marketing sync feature. Business owners should be able to connect an email provider (like MailerLite) so that any new customer added to OHC is automatically subscribed to their mailing list. Include a simple interface in OHC to send a plain-text announcement to all customers without leaving the app.
- **Priority**: P2
- **Estimated Scope**: Medium


## [Payment Processing] Localized Payments: Mercado Pago / Razorpay

- **Title**: Integrate Regional Payment Gateways (Mercado Pago / Razorpay)
- **Problem Statement**: Stripe is not available or is too expensive in many emerging markets. Small businesses in LATAM or India need to accept payments using popular local methods (like PIX in Brazil or UPI in India) to actually close sales online.
- **Research Report**:
  - **Findings**: Mercado Pago dominates LATAM with support for PIX, Boleto, and local credit cards. Razorpay is the standard for India, supporting UPI and local wallets. Both have modern REST APIs.
  - **Ease of Use**: Business owners in these regions are already familiar with these platforms. Connecting them usually involves copy-pasting API keys or a standard OAuth flow.
  - **Pricing**: Transaction-based (e.g., Mercado Pago charges around 3-5% per transaction depending on the country and settlement time).
  - **Cloud vs Standalone**: Works in both environments, though webhook delivery for payment success requires a public endpoint (Cloud) or a tunneling solution (Standalone).
- **Design Doc**: In the "Payments" settings, detect the user's country and offer the appropriate regional gateway. The user inputs their API credentials. Checkout links generated by OHC will route to the selected provider's hosted checkout page, allowing the customer to pay via local methods.
- **Implementation Prompt**: Add support for regional payment processors (specifically Mercado Pago for LATAM and Razorpay for India) alongside Stripe. Business owners should be able to select their provider, connect their account, and generate payment links that allow their customers to pay using local methods like PIX or UPI. Ensure the payment status updates automatically in OHC when the customer pays.
- **Priority**: P1
- **Estimated Scope**: Large


## [Shipping & Logistics] Automated Shipping: Shippo / EasyPost

- **Title**: Integrate Shippo for Automated Label Generation and Tracking
- **Problem Statement**: E-commerce businesses waste hours manually copying customer addresses into carrier websites to buy shipping labels. They need a way to instantly calculate shipping costs at checkout and print labels directly from their orders list.
- **Research Report**:
  - **Findings**: Shippo and EasyPost aggregate dozens of carriers (USPS, UPS, FedEx, DHL, local carriers) behind a single API. Shippo is generally more small-business friendly with a better dashboard if the user needs to log in directly.
  - **Ease of Use**: Highly automated. The business owner clicks "Buy Label" on an order, confirms package weight, and a PDF is generated. Very low friction.
  - **Pricing**: Shippo has no monthly fee, charges $0.05 per label plus the actual postage cost.
  - **Cloud vs Standalone**: Works natively in both environments.
- **Design Doc**: When an order is placed, the user views it in OHC and clicks "Fulfill". A modal asks for package dimensions/weight (or uses defaults). OHC calls the Shippo API to get rates, the user selects a rate, and the label PDF is returned and stored. Tracking numbers are automatically attached to the order and emailed to the customer.
- **Implementation Prompt**: Create a seamless shipping fulfillment flow. For any physical order, the business owner must be able to click a single button to generate a printable shipping label (PDF) using a service like Shippo. The system should automatically calculate the cheapest rate based on standard package sizes, buy the label, and provide the tracking number to the customer.
- **Priority**: P1
- **Estimated Scope**: Large


## [SMS & Notifications] Global SMS: Twilio / MessageBird

- **Title**: Integrate Twilio for Automated SMS Reminders and Alerts
- **Problem Statement**: Many customers (and business owners with low English proficiency) prefer text messages over email. Missed appointments or unread emails hurt the business. They need automated SMS reminders for bookings, order updates, and simple communications.
- **Research Report**:
  - **Findings**: Twilio is the industry standard with massive global reach. MessageBird offers competitive pricing in Europe and Asia. Both offer reliable REST APIs. A2P 10DLC compliance in the US is a major hurdle for small businesses.
  - **Ease of Use**: For the business owner, this should be completely invisible. They just turn on a toggle: "Send SMS reminders to customers". OHC handles the API calls.
  - **Pricing**: Twilio charges ~$0.0079 per message in the US, but international rates vary wildly (up to $0.10+ in some countries).
  - **Cloud vs Standalone**: Works in both. In Cloud, OHC could pool a single Twilio account and bill users. In Standalone, users must provide their own Twilio API keys.
- **Design Doc**: A "Notifications" settings tab allows users to configure automated SMS for key events (e.g., 24h before appointment, order shipped). The system uses Twilio's API to dispatch these messages using a generic sender ID or the user's provisioned number.
- **Implementation Prompt**: Implement automated SMS notifications for customers. Business owners should be able to toggle on SMS reminders for upcoming appointments and order updates. The feature must be simple enough that the owner doesn't need to understand carrier compliance—they just provide an API key or pay a small add-on fee, and the system automatically texts their customers at the right times.
- **Priority**: P0
- **Estimated Scope**: Medium


## [Video Conferencing] Auto-Meeting Links: Zoom / Google Meet

- **Title**: Integrate Zoom / Google Meet for Auto-Generated Online Consultations
- **Problem Statement**: Online tutors, therapists, and consultants struggle with manually creating Zoom links and emailing them to clients before every meeting. They need a unique, secure meeting link automatically generated and sent as soon as a client books a slot.
- **Research Report**:
  - **Findings**: Zoom has a robust Server-to-Server OAuth API. Google Meet links can be automatically generated if Google Calendar is already integrated.
  - **Ease of Use**: If Google Calendar is used, Meet links are zero-effort. For Zoom, the user must connect their Zoom account via OAuth.
  - **Pricing**: Zoom API requires a Pro account ($15/mo). Google Meet is included with free Google accounts.
  - **Cloud vs Standalone**: Both work in Cloud and Standalone environments.
- **Design Doc**: When creating a "Service" type in OHC, the user can set the location to "Online Meeting". If connected to Zoom/Google, the backend automatically calls the respective API to generate a meeting URL upon a successful booking. This URL is saved to the appointment record and included in the confirmation emails/SMS.
- **Implementation Prompt**: Add an automated video conferencing integration. When a business owner offers online services (like remote tutoring), the system should automatically generate a unique Zoom or Google Meet link for every new booking. This link must be instantly provided to the customer in their confirmation email and visible to the owner in their daily schedule view.
- **Priority**: P2
- **Estimated Scope**: Small
