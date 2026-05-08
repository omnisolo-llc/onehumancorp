# Research Report: Core Tool Integrations for OneHumanCorp

This report contains structured issue briefs for 7 core tool integration categories critical for OneHumanCorp's mission to empower non-technical small business owners.

---

## 1. Social Media Integration

### Title
Unified Social Inbox Integration (Instagram, Facebook, WhatsApp)

### Problem Statement
Business owners like Maya (the home baker) receive customer inquiries scattered across Instagram DMs, Facebook Messenger, and WhatsApp. Missing a message means losing a sale. They need a single, unified place to see and reply to all customer messages, and they want their AI Assistant to handle common questions automatically while they sleep.

### Research Report
- **Tool Evaluated**: Meta Graph API (Instagram Messaging, Messenger, WhatsApp Business API).
- **Ease of Use for Non-Technical Users**: Meta's standard OAuth flows can be daunting. We need a seamless 1-click connect within OHC that hides Meta App configuration from the user.
- **Pricing**: Free for standard messaging (Instagram/Messenger). WhatsApp Business charges per conversation, which requires OHC to either absorb costs, pass them on, or restrict to a premium tier.
- **Reputation**: Industry standard, absolutely required for any commerce platform today.
- **Cloud/Standalone**: Cloud mode works flawlessly via OHC webhook relays. Standalone might require a local proxy or simplified webhook forwarding mechanism.

### Design Doc
- **User Experience**: A "Connect Socials" button in the OHC dashboard. Once clicked, an OAuth popup guides them through logging in to Facebook/Instagram.
- **Integration Flow**: Incoming messages from connected platforms trigger webhooks to OHC. These messages populate the unified "Customer Inbox" in the app.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".

### Implementation Prompt
Implement a unified inbox feature that allows users to connect their Instagram and Facebook accounts via OAuth. Messages from these platforms should appear in a single unified view in the OHC mobile and web apps. Users should be able to reply directly from the OHC app, and the response should be routed back to the appropriate social platform. Ensure the UI feels instant and natively mobile.
- **Acceptance Criteria**: User can connect Instagram/Facebook. Incoming messages appear in OHC unified inbox. User can reply from OHC, and it shows up on the customer's social app.
- **Priority**: P0
- **Estimated Scope**: Large

---

## 2. Calendar & Scheduling

### Title
Google Calendar 2-Way Sync & Booking Integration

### Problem Statement
Service providers like Leo (the music tutor) and Carlos (the handyman) rely on calendars to manage their time. If their booking system isn't synced with their personal Google Calendar, they risk double-booking or missing appointments. They need a system that checks their availability in real-time and automatically blocks off time when a customer books.

### Research Report
- **Tool Evaluated**: Google Calendar API / Google Workspace Integration.
- **Ease of Use for Non-Technical Users**: Very high, familiar OAuth flow ("Sign in with Google").
- **Pricing**: Free API usage within standard rate limits.
- **Reputation**: Gold standard for scheduling. Highly reliable.
- **Cloud/Standalone**: Works seamlessly in both, though Standalone requires users to provision their own Google OAuth credentials unless OHC provides a brokered proxy.

### Design Doc
- **User Experience**: A "Sync Calendar" settings page where the user clicks "Connect Google Calendar". They select which calendar dictates availability and which calendar receives new bookings.
- **Integration Flow**: OHC periodically syncs (or uses Google Push Notifications) to update local availability slots. When a customer books a time, OHC writes the event to the user's Google Calendar.
- **AI Integration**: The Operations Agent monitors the calendar and alerts the business owner if they have back-to-back appointments without buffer times, suggesting schedule optimizations.

### Implementation Prompt
Create a 2-way Google Calendar sync feature for business owners using the booking flow. Users must be able to authenticate with Google, select a calendar to check for availability conflicts, and have new customer bookings automatically created as events on their Google Calendar. The public booking page must never offer a time slot that is blocked on the user's Google Calendar.
- **Acceptance Criteria**: User connects Google Calendar. Blocked times on Google Calendar hide available slots on the OHC booking page. New bookings create Google Calendar events.
- **Priority**: P0
- **Estimated Scope**: Medium

---

## 3. Email Marketing

### Title
Native Email Campaign Manager

### Problem Statement
Store owners like Priya (the boutique owner) want to email their past customers when new stock arrives or to offer a holiday discount. External tools like Mailchimp are too complex, expensive, and require exporting/importing CSV lists. They need a simple way to say "Email all my past customers a 10% off coupon for Black Friday" directly from their phone.

### Research Report
- **Tool Evaluated**: SendGrid API or AWS SES (Backend delivery) with OHC native UI.
- **Ease of Use for Non-Technical Users**: By abstracting the complex ESP (Email Service Provider) backend and providing a native drag-and-drop or AI-generated template builder, the user experience becomes completely frictionless.
- **Pricing**: SendGrid/SES are very cheap at scale. OHC can bundle this into the monthly subscription or offer a generous free tier.
- **Reputation**: Essential for customer retention.
- **Cloud/Standalone**: Cloud mode uses OHC's centralized ESP accounts. Standalone mode will require the user to input their own SMTP credentials.

### Design Doc
- **User Experience**: A "Campaigns" tab where the user can click "New Email". They write a simple prompt ("Tell my customers about the summer sale") and the AI generates a beautifully formatted email with their brand colors and products.
- **Integration Flow**: OHC generates the HTML email, segments the user's customer list, and queues the emails for delivery via the configured ESP backend.
- **AI Integration**: The Marketing & Advertising Agent writes the subject lines, generates the copy, and tracks open/click rates to suggest the best times to send future emails.

### Implementation Prompt
Develop a native email campaign tool within the OHC dashboard. Users should be able to draft emails (or use AI to generate them), select an audience (e.g., "All Customers" or "Past 30 Days"), and send the campaign. The system must handle unsubscribes automatically and provide a simple report on open rates. Do not require the user to manage SMTP settings in the Cloud version.
- **Acceptance Criteria**: User can create an email campaign. AI can generate content. Emails are delivered. Unsubscribe links work. Open rates are displayed.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 4. Payment Processing

### Title
Alternative Payment Gateways for Global Markets (Mercado Pago & Razorpay)

### Problem Statement
While Stripe is fantastic, it is not supported everywhere. A merchant in Brazil or India cannot use Stripe easily. To be truly global, OHC needs to support regional heavyweights so a business owner in LATAM or India can accept local payment methods (like PIX or UPI) without friction.

### Research Report
- **Tools Evaluated**: Mercado Pago (LATAM) and Razorpay (India).
- **Ease of Use for Non-Technical Users**: Both offer standard OAuth or API key integrations. OHC must simplify the onboarding to match Stripe's ease of use.
- **Pricing**: Standard transaction fees apply; merchants expect these.
- **Reputation**: Mercado Pago is dominant in LATAM; Razorpay is dominant in India.
- **Cloud/Standalone**: Both work in Cloud and Standalone modes via API integrations and webhook callbacks.

### Design Doc
- **User Experience**: During checkout setup, if the user's business country is Brazil, prompt them to connect Mercado Pago. If India, prompt Razorpay.
- **Integration Flow**: OHC's payment gateway interface must be abstracted to route checkout sessions and webhook processing through the respective provider based on the tenant's configuration.
- **AI Integration**: Finance & Payments Agent seamlessly aggregates revenue across providers into a unified dashboard, regardless of whether the money came from Stripe, Mercado Pago, or Razorpay.

### Implementation Prompt
Integrate Mercado Pago and Razorpay as alternative payment providers alongside the existing Stripe integration. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings. Webhooks from all providers must normalize into standard OHC order fulfillment events.
- **Acceptance Criteria**: Merchant in a supported region can connect Mercado Pago or Razorpay. Customers can checkout using local methods. Orders are marked paid upon successful webhook receipt.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 5. Shipping & Logistics

### Title
Real-Time Shipping Rates & Label Generation (EasyPost)

### Problem Statement
Sellers of physical products need to charge customers the correct amount for shipping and easily print shipping labels. Calculating USPS or FedEx rates manually is impossible. They need the checkout page to automatically calculate shipping costs, and they need a 1-click button to print the shipping label from their phone or computer.

### Research Report
- **Tool Evaluated**: EasyPost API (aggregates USPS, UPS, FedEx, etc.).
- **Ease of Use for Non-Technical Users**: Very high once configured. User just clicks "Buy Label & Print".
- **Pricing**: Free tier available, nominal fee per label thereafter.
- **Reputation**: Highly reliable, developer-friendly aggregator.
- **Cloud/Standalone**: Fully supported in both environments.

### Design Doc
- **User Experience**: When viewing a paid order for physical goods, the user sees a "Fulfill Order" button. Clicking it generates a shipping label PDF that they can print. The checkout automatically calculates the rate based on the buyer's address and the product's weight.
- **Integration Flow**: OHC sends package weight and destination to EasyPost to get live rates during checkout. Upon fulfillment, OHC purchases the label via EasyPost and saves the tracking number, auto-emailing the customer.
- **AI Integration**: The Customer Success Agent monitors tracking numbers and proactively notifies the customer if a delivery is delayed.

### Implementation Prompt
Implement a shipping and fulfillment module powered by EasyPost. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
- **Acceptance Criteria**: Live shipping rates appear at checkout. Merchant can click "Print Label" to generate a PDF label. Tracking number is automatically sent to the customer.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 6. SMS & Notifications

### Title
Global SMS Notifications & Reminders (Twilio)

### Problem Statement
Many customers ignore emails but read every text message. For food cart operators like Fatima or tutors like Leo, an SMS reminder ("Your order is ready!" or "Lesson in 1 hour") drastically reduces no-shows and improves customer satisfaction. It needs to work globally and reliably.

### Research Report
- **Tool Evaluated**: Twilio Programmable SMS.
- **Ease of Use for Non-Technical Users**: Invisible to the user. They just toggle "Send SMS reminders" in their settings.
- **Pricing**: Pay-per-message. OHC will need to manage quotas or require merchants to buy "SMS Credits".
- **Reputation**: The industry standard for reliable SMS delivery globally.
- **Cloud/Standalone**: Cloud mode utilizes OHC's Twilio account. Standalone mode requires the user to input their Twilio Account SID and Auth Token.

### Design Doc
- **User Experience**: A toggle in the booking or order settings: "Send SMS confirmation to customers". For merchants, an option to "Receive SMS when I get a new order".
- **Integration Flow**: OHC dispatches async jobs to send SMS messages via Twilio API when specific events trigger (Order Ready, Appointment Reminder).
- **AI Integration**: The Operations Agent decides the optimal time to send the reminder (e.g., 2 hours before a booking, or immediately when food is ready).

### Implementation Prompt
Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message. Include a settings panel for merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164).
- **Acceptance Criteria**: Customer receives an SMS when their order is marked "Ready for Pickup". Customer receives a reminder SMS before a booked appointment.
- **Priority**: P2
- **Estimated Scope**: Medium

---

## 7. Video Conferencing

### Title
Automated Zoom Link Generation for Bookings

### Problem Statement
Online service providers (like Leo teaching guitar) shouldn't have to manually create a Zoom meeting and email the link to every student. The system should automatically generate a unique Zoom link the moment a customer books an online session.

### Research Report
- **Tool Evaluated**: Zoom API (Server-to-Server OAuth).
- **Ease of Use for Non-Technical Users**: Standard OAuth connection process. Highly intuitive.
- **Pricing**: API is free for Zoom users, but requires the merchant to have a Zoom account.
- **Reputation**: Ubiquitous for video calls.
- **Cloud/Standalone**: Works identically in both environments.

### Design Doc
- **User Experience**: In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- **Integration Flow**: Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
- **AI Integration**: The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.

### Implementation Prompt
Build a Zoom integration that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account. When a customer books a service marked as "Online Meeting", the system must dynamically generate a Zoom link, store it with the booking, and share it with both the merchant and the customer.
- **Acceptance Criteria**: Merchant connects Zoom. Customer books online service. Unique Zoom link is generated and sent to both parties.
- **Priority**: P2
- **Estimated Scope**: Medium
