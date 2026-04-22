# OHC Tool Integration Research Report

## Executive Summary
This report evaluates seven tool categories vital for empowering non-technical small business owners on the OneHumanCorp (OHC) platform. Our goal is to select integrations that provide seamless, "invisible" functionality, requiring zero technical configuration from the user. We prioritized tools with generous free tiers, robust APIs, and the ability to operate across both Cloud and Standalone environments.

## 1. Social Media Integration (Unified Inbox)
**Evaluated Tool:** ManyChat (API) / Meta Graph API
- **Target Persona:** Maya the Home Baker, Carlos the Handyman.
- **Problem:** Constantly switching apps to answer customer inquiries leads to missed sales and burnout.
- **Solution:** A unified inbox within OHC aggregating Instagram DMs, Facebook Messenger, and WhatsApp.
- **Integration Approach:** Utilize Meta's APIs to route messages into a single OHC chat interface. Integrate the Gemini Pro agent to draft suggested responses based on FAQs.
- **Recommendation:** Proceed with Meta Graph API directly or a simplified ManyChat integration. High risk due to Meta API volatility.

---
# Social Media Integration: ManyChat Unified Inbox

## Problem Statement
Small business owners (like Maya the Baker or Carlos the Handyman) receive customer inquiries across multiple platforms: Instagram DMs, Facebook comments, WhatsApp, and SMS. Constantly switching between apps is overwhelming and leads to missed messages and lost sales. They need a single, unified inbox to view and respond to all customer messages in one place, without needing to learn complex software.

## Research Report
ManyChat is a leading platform for chat marketing and automation.
- **Ease of Use:** ManyChat is designed for non-technical users, offering a visual flow builder. However, its full feature set can be complex. Integrating it into OHC would require simplifying the interface to just a unified inbox and basic auto-replies.
- **Pricing:** Offers a free tier for up to 1,000 contacts, which is ideal for many small businesses. Pro plans start at $15/month, making it affordable.
- **Reputation:** Highly regarded in the e-commerce and marketing space, particularly for Instagram and Facebook Messenger automation.
- **Integration Risk:** High reliance on Meta's APIs (Facebook, Instagram, WhatsApp), which frequently change and have strict opt-in/opt-out rules.

## Design Doc
- **Integration Point:** OHC's "Customer Success" department ("The Ambassador").
- **Trigger:** Business owner connects their Facebook/Instagram/WhatsApp accounts via an OAuth flow in the OHC dashboard.
- **Action:** Incoming messages from these platforms are routed to a new "Unified Inbox" screen in the OHC mobile and web apps. The AI agent can draft suggested replies based on the business's FAQs and past interactions.
- **User Experience:** A familiar, chat-like interface (similar to iMessage or WhatsApp) where the owner sees all messages regardless of the source. The platform icon (e.g., Instagram logo) is displayed next to the message.

## Implementation Prompt
Implement a "Unified Inbox" feature using the ManyChat API (or similar Meta Graph APIs directly if more feasible).
1.  **OAuth Flow:** Create a simple flow for users to connect their social accounts.
2.  **Inbox UI:** Build a mobile-first chat interface displaying messages from all connected sources.
3.  **Replying:** Allow users to type replies in the OHC app, which are then sent back to the respective platform.
4.  **AI Drafting:** Integrate the existing Gemini Pro agent to suggest replies based on context.

## Priority
P1 (High)

## Estimated Scope
Large

---
## 2. Calendar & Scheduling
**Evaluated Tool:** Cal.com
- **Target Persona:** Leo the Music Tutor, Carlos the Handyman.
- **Problem:** Back-and-forth emails to schedule appointments are inefficient.
- **Solution:** Embeddable booking widget syncing with the owner's Google/Outlook calendar.
- **Integration Approach:** Leverage Cal.com's open-source infrastructure via API. Auto-generate booking pages and dispatch confirmation emails with meeting links.
- **Recommendation:** Highly recommended (P0). Crucial for service businesses. Open-source nature aligns well with OHC Standalone mode.

---
# Calendar & Scheduling: Cal.com Integration

## Problem Statement
Service-based businesses (like Leo the Music Tutor or Carlos the Handyman) need a way for customers to easily book appointments or lessons online. Manual back-and-forth emails to find a time are inefficient. They need a simple booking page that syncs with their personal calendar (Google/Outlook) and automatically generates meeting links or location details.

## Research Report
Cal.com is an open-source scheduling infrastructure platform.
- **Ease of Use:** Very user-friendly for both the business owner setting up availability and the customer booking the slot.
- **Pricing:** Core scheduling is free for individuals. Teams and advanced features have paid tiers, but the free tier covers most single-person SMB needs. Being open-source, OHC could potentially self-host the core infrastructure for Standalone mode.
- **Reputation:** Rapidly growing alternative to Calendly, praised for its developer-friendly API, open-source nature, and clean design.
- **Features:** Strong support for Google Calendar, Outlook, Zoom, Google Meet, and payment collection (Stripe) during booking.

## Design Doc
- **Integration Point:** OHC's "Operations" department ("The Manager") and "Sales & Acquisition" department.
- **Trigger:** Business owner sets their working hours and service duration in the OHC dashboard. They connect their Google/Outlook calendar.
- **Action:** OHC generates a "Booking Page" widget that can be embedded on the business's OHC website or shared via a link-in-bio.
- **User Experience:** Customers see a calendar view, pick an available slot, enter their details, and optionally pay a deposit. Both parties receive confirmation emails and calendar invites with automatically generated Zoom/Meet links.

## Implementation Prompt
Integrate Cal.com's API (or self-hosted instance) to provide scheduling functionality.
1.  **Availability Setup:** UI for the business owner to define working hours, service types, and durations.
2.  **Calendar Sync:** OAuth flow to connect external calendars to prevent double-booking.
3.  **Booking Widget:** A mobile-first UI component for customers to select times.
4.  **Confirmation:** Automated email/SMS dispatch upon successful booking, including meeting links.

## Priority
P0 (Critical - required for the Service persona)

## Estimated Scope
Medium

---
## 3. Email Marketing
**Evaluated Tool:** Resend
- **Target Persona:** Priya the Boutique Owner.
- **Problem:** Traditional tools (Mailchimp) are too complex for simple announcements.
- **Solution:** Send beautiful, branded emails directly from the OHC CRM.
- **Integration Approach:** Provide a simple text editor in OHC. Use the Gemini AI to format the text into an HTML template, then dispatch via the Resend API.
- **Recommendation:** Recommended (P2). Excellent developer experience and deliverability.

---
# Email Marketing: Resend Integration

## Problem Statement
Small business owners (like Priya the Boutique Owner) need to announce new products, sales, or updates to their customer base. Traditional email marketing tools (Mailchimp, Klaviyo) are overly complex and expensive for simple announcements. They need a straightforward way to send beautiful, branded emails to their collected contacts directly from their management app.

## Research Report
Resend is a modern email API designed for developers, but powerful enough to back consumer-facing email marketing features.
- **Ease of Use:** As an API, it's invisible to the end-user. The ease of use depends entirely on the UI OHC builds on top of it.
- **Pricing:** Very developer-friendly. Often features a generous free tier (e.g., 3,000 emails/month free), which covers the needs of many early-stage SMBs. Paid tiers are volume-based and affordable.
- **Reputation:** Known for high deliverability, developer experience (React Email), and modern architecture compared to legacy providers like SendGrid.
- **Capabilities:** Excellent for both transactional emails (receipts) and broadcast campaigns.

## Design Doc
- **Integration Point:** OHC's "Marketing & Advertising" department ("The Promoter").
- **Trigger:** Business owner selects "Send Announcement" in the OHC dashboard, chooses a recipient list (e.g., "All Customers", "Recent Buyers"), and writes the content.
- **Action:** The AI agent formats the content into a beautiful, mobile-responsive HTML template (using React Email concepts). OHC dispatches the emails via the Resend API.
- **User Experience:** A simple text editor interface. The AI handles the design and layout invisibly. The owner sees basic stats later: "Sent to 50 people, 30 opened."

## Implementation Prompt
Integrate the Resend API to handle outbound email campaigns.
1.  **Audience Selection:** UI to select customer segments from the OHC CRM database.
2.  **Drafting:** A simple WYSIWYG editor for the email content.
3.  **AI Formatting:** Use Gemini to take the draft text and wrap it in an aesthetically pleasing, brand-consistent HTML template.
4.  **Dispatch & Analytics:** Send via Resend API and display basic open/click tracking data back to the user.

## Priority
P2 (Medium)

## Estimated Scope
Small

---
## 4. Payment Processing (LATAM Focus)
**Evaluated Tool:** Mercado Pago
- **Target Persona:** Any business operating in Latin America.
- **Problem:** Stripe lacks sufficient penetration and local payment method support in LATAM.
- **Solution:** Offer Mercado Pago as an alternative gateway for LATAM merchants.
- **Integration Approach:** Integrate Mercado Pago Checkout API into the OHC storefront flow, supporting local methods like PIX and cash vouchers.
- **Recommendation:** Recommended (P1) for international expansion.

---
# Payment Processing: Mercado Pago for LATAM

## Problem Statement
While Stripe is excellent globally, it has varying levels of adoption, feature support, and settlement speeds in Latin America. SMBs in countries like Brazil, Argentina, and Mexico heavily rely on local payment methods (like PIX in Brazil or OXXO cash payments in Mexico). A global platform must offer localized payment options to be viable in these regions.

## Research Report
Mercado Pago is the dominant digital payment platform in Latin America (part of Mercado Libre).
- **Ease of Use:** Familiar and trusted by consumers in LATAM. Easy checkout flows.
- **Pricing:** Competitive local rates, though currency conversion and cross-border fees need consideration if OHC operates globally.
- **Reputation:** The undisputed leader in LATAM e-commerce payments.
- **Features:** Supports all critical local payment methods: credit/debit cards, bank transfers, PIX (Brazil instantly), and cash payment vouchers (OXXO, PagoFácil).

## Design Doc
- **Integration Point:** OHC's "Finance & Payments" department ("The Accountant").
- **Trigger:** During the OHC onboarding flow, if the business's country is set to a supported LATAM country, Mercado Pago is offered as the primary or alternative payment gateway alongside Stripe.
- **Action:** OHC integrates with the Mercado Pago Checkout API.
- **User Experience:** The business owner connects their Mercado Pago account. When their customers check out, they see familiar, localized payment options instead of just a generic credit card form.

## Implementation Prompt
Integrate the Mercado Pago Checkout API as a regional payment provider.
1.  **Gateway Routing:** Logic to offer Mercado Pago based on the tenant's registered country.
2.  **Checkout UI:** Integrate the Mercado Pago web checkout or native SDKs into the OHC storefront flow.
3.  **Webhook Handling:** Implement secure webhook endpoints to listen for payment success, failure, and pending (cash voucher) states.
4.  **Financial Dashboard:** Normalize Mercado Pago transaction data to display in the unified OHC financial reports.

## Priority
P1 (High - essential for international expansion)

## Estimated Scope
Medium

---
## 5. Shipping & Logistics
**Evaluated Tool:** Shippo
- **Target Persona:** Priya the Boutique Owner.
- **Problem:** Manual shipping rate calculation and label generation are tedious.
- **Solution:** Auto-calculate rates at checkout and generate printable labels from the phone.
- **Integration Approach:** Integrate Shippo API for real-time rates and label purchase. Abstract carrier complexity away from the user.
- **Recommendation:** Highly recommended (P1) for physical product sellers. Pay-as-you-go pricing is ideal for SMBs.

---
# Shipping & Logistics: Shippo Integration

## Problem Statement
Businesses selling physical products (like Priya the Boutique Owner) need to ship orders to customers. Manually going to the post office or typing addresses into carrier websites is tedious and error-prone. They need a way to automatically calculate shipping rates at checkout and generate printable shipping labels directly from their phone.

## Research Report
Shippo is a multi-carrier shipping API and web application.
- **Ease of Use:** Provides a unified API for dozens of carriers (USPS, FedEx, UPS, DHL, etc.), abstracting away the complexity of dealing with each carrier individually.
- **Pricing:** Pay-as-you-go model (e.g., 5 cents per label) plus postage costs. No monthly fees required for basic API access, making it very SMB friendly.
- **Reputation:** Highly regarded API, widely used by e-commerce platforms and independent developers.
- **Features:** Real-time rate calculation, label generation, address validation, and tracking webhooks.

## Design Doc
- **Integration Point:** OHC's "Operations" department ("The Manager").
- **Trigger:** A customer places an order for a physical product.
- **Action:** The AI Manager uses the Shippo API to fetch the best shipping rate and present it to the business owner. The owner taps "Buy Label".
- **User Experience:** The business owner receives a notification of a new order. They tap "Fulfill", confirm the package weight/dimensions (or use defaults), and a shipping label is generated as a PDF that they can print directly from their phone. Tracking info is automatically emailed to the customer.

## Implementation Prompt
Integrate the Shippo API for shipping rate calculation and label generation.
1.  **Rate Calculation:** Fetch real-time shipping rates during the customer checkout process based on cart weight and destination.
2.  **Label Purchase:** UI for the business owner to review the order, select a shipping service, and purchase the label via API.
3.  **Artifact Generation:** Retrieve the label PDF and display it for printing.
4.  **Tracking:** Listen to Shippo webhooks to update the order status and notify the customer of shipping progress.

## Priority
P1 (High - required for physical product personas)

## Estimated Scope
Large

---
## 6. SMS & Notifications
**Evaluated Tool:** Twilio
- **Target Persona:** Fatima the Food Cart Operator.
- **Problem:** Email is not reliable for urgent alerts (e.g., "Food is ready").
- **Solution:** Automated SMS notifications for critical events.
- **Integration Approach:** Integrate Twilio Programmable SMS. Create a routing engine to choose between email and SMS based on urgency and user preference.
- **Recommendation:** Recommended (P1). High reliability, but requires careful navigation of A2P 10DLC compliance.

---
# SMS & Notifications: Twilio Integration

## Problem Statement
Not all customers reliably check email, and some business owners (like Fatima the Food Cart Operator) rely entirely on SMS for immediate alerts. Businesses need a reliable way to send critical, time-sensitive updates (e.g., "Your food is ready for pickup," "Appointment reminder for tomorrow") directly to mobile phones globally.

## Research Report
Twilio is the industry standard for cloud communications, specifically SMS and voice APIs.
- **Ease of Use:** Extremely developer-friendly API. For the end-user, it's invisible infrastructure.
- **Pricing:** Pay-as-you-go per message (fractions of a cent in the US, varies globally). Requires renting a virtual phone number (approx $1/month).
- **Reputation:** The gold standard for reliability and global carrier reach.
- **Risks:** High regulatory compliance overhead (A2P 10DLC registration in the US, opt-in requirements, carrier filtering for spam).

## Design Doc
- **Integration Point:** OHC's "Customer Success" department and "Operations" department.
- **Trigger:** Time-sensitive events: Order ready for pickup, upcoming appointment reminder, or critical system alerts for the business owner.
- **Action:** OHC backend triggers an SMS dispatch via the Twilio API.
- **User Experience:** Customers receive a standard text message. Business owners can configure which events trigger SMS vs. email in their notification settings.

## Implementation Prompt
Integrate the Twilio Programmable SMS API for critical alerts.
1.  **Notification Engine:** Create a routing layer that can send alerts via email (Resend) or SMS (Twilio) based on user preference and urgency.
2.  **Number Provisioning:** (Future) Allow businesses to lease their own dedicated Twilio number through OHC. Initially, use a shared short code or toll-free number.
3.  **Compliance:** Ensure opt-in checkboxes are present on all checkout and booking forms before collecting phone numbers for SMS.
4.  **Templates:** Create concise, plain-text templates for common alerts (e.g., "Order #[ID] from [Business] is ready!").

## Priority
P1 (High)

## Estimated Scope
Medium

---
## 7. Video Conferencing
**Evaluated Tool:** Daily.co
- **Target Persona:** Leo the Music Tutor (online lessons).
- **Problem:** Manually generating and sharing Zoom links is error-prone.
- **Solution:** Auto-generated, embedded video rooms for scheduled appointments.
- **Integration Approach:** Use Daily.co API to provision secure room URLs upon booking confirmation (via Cal.com integration). Embed using Daily Prebuilt.
- **Recommendation:** Recommended (P2). Generous free tier and white-labeling capabilities keep users within the OHC ecosystem.

---
# Video Conferencing: Daily.co Integration

## Problem Statement
Service providers offering virtual consultations or lessons (like Leo the Music Tutor) struggle with managing video links. Generating a Zoom link, emailing it to the client, and keeping track of which link belongs to which appointment is a manual, error-prone process. They need automatically generated, secure video rooms for every booked online appointment.

## Research Report
Daily.co is a developer-focused video and audio API platform.
- **Ease of Use:** Provides pre-built UI components (Daily Prebuilt) that can be embedded directly into apps, or raw APIs for custom UIs.
- **Pricing:** Generous free tier (10,000 participant minutes/month free), making it highly cost-effective for SMBs starting out.
- **Reputation:** Known for high-quality video, ease of integration compared to raw WebRTC, and strong developer support.
- **Advantage over Zoom:** Can be completely white-labeled and embedded within the OHC platform, keeping users in the OHC ecosystem rather than bouncing them to a third-party app.

## Design Doc
- **Integration Point:** OHC's "Operations" department ("The Manager").
- **Trigger:** A customer books an "Online Service" appointment (integrated with Cal.com logic).
- **Action:** OHC calls the Daily.co API to provision a unique, secure video room URL specifically for that appointment time.
- **User Experience:** The booking confirmation email contains a link like `mybusiness.ohc.app/meeting/123`. Clicking it opens the video call directly in the browser—no app downloads required for the customer or the business owner.

## Implementation Prompt
Integrate Daily.co API to auto-generate video meeting rooms for virtual bookings.
1.  **Room Provisioning:** Call the Daily API to create a room when an online booking is confirmed.
2.  **Access Control:** Ensure rooms are only active during the scheduled appointment window.
3.  **Embedded UI:** Use Daily Prebuilt to embed the video call experience directly into the OHC web and mobile interfaces.
4.  **Notification:** Include the unique room link in the automated confirmation and reminder emails/SMS.

## Priority
P2 (Medium)

## Estimated Scope
Medium

## Conclusion
The evaluated tools strongly align with OHC's mission of radical simplicity. By abstracting the complexity of these APIs, OHC can provide enterprise-grade capabilities to non-technical users. The highest priorities for immediate implementation are Calendar Scheduling (Cal.com) and local payment support (Mercado Pago) to unlock the Service persona and LATAM markets, respectively.
