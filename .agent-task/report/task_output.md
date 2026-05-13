# Research Report: OHC Tools for Small Business Owners

## 1. Social Media Integration
**Title:** Unified Inbox for Social Media Channels
**Problem Statement:** Small business owners (e.g., boutique owners, consultants) often miss customer inquiries because messages are scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Constantly checking multiple apps is time-consuming and leads to dropped leads.
**Research Report:** A unified inbox consolidates messages from various platforms into a single stream.
- *Ease of Use:* Very high. Business owners interact with a single interface.
- *Pricing:* Typically $15-$50/month depending on volume and features. Many have a free tier for basic usage.
- *Reputation:* Tools like Buffer, Hootsuite, and Sprout Social are well-regarded, but can be complex. Simpler tools tailored for SMBs are emerging.
- *Cloud/Standalone:* Cloud is straightforward (webhooks/APIs). Standalone requires careful handling of API credentials and local polling mechanisms.
**Design Doc:**
- *Trigger:* Incoming message from any connected social platform.
- *Action:* OHC receives the message, standardizes its format, and displays it in a unified UI.
- *User Views:* A single dashboard where they can see and reply to all messages, regardless of the source platform.
**Implementation Prompt:** Create a unified inbox view in the OHC dashboard. Users should be able to connect at least two social platforms (e.g., Instagram, WhatsApp). Incoming messages should appear in a single list, and replies should be routed back to the correct platform.
**Priority:** P1
**Estimated Scope:** Large

## 2. Calendar & Scheduling
**Title:** Automated Scheduling and Calendar Sync
**Problem Statement:** Back-and-forth emails to find a meeting time are inefficient. Small business owners (e.g., therapists, tutors, consultants) need an easy way for clients to book available slots without double-booking.
**Research Report:** Calendar scheduling tools sync with existing calendars (Google, Outlook) and provide a booking page.
- *Ease of Use:* High. Users set availability, and clients pick a slot.
- *Pricing:* $10-$30/month (e.g., Calendly, Acuity). Free tiers exist but often lack advanced features like custom branding.
- *Reputation:* Very positive. They significantly reduce administrative overhead.
- *Cloud/Standalone:* Cloud integrates seamlessly with OAuth. Standalone might require more manual setup (e.g., iCal feeds or local calendar app integration).
**Design Doc:**
- *Trigger:* User connects their calendar and defines working hours.
- *Action:* OHC generates a shareable booking link. When a client books, an event is created in the user's calendar.
- *User Views:* A settings page to connect calendars and define availability. A public-facing booking page for clients.
**Implementation Prompt:** Implement a feature that allows users to connect a Google or Outlook calendar, set their available hours, and generate a booking link. The booking page should display available slots, and booking a slot should create an event in the connected calendar.
**Priority:** P0
**Estimated Scope:** Medium

## 3. Email Marketing
**Title:** Integrated Email Campaigns for Customer Retention
**Problem Statement:** Small business owners need to keep their customers engaged with updates, promotions, and newsletters, but standalone email marketing tools (like Mailchimp) are often too complex and require managing a separate contact list.
**Research Report:** Integrated email marketing leverages the existing customer database to send targeted campaigns.
- *Ease of Use:* High, if the builder is simple (drag-and-drop) and templates are provided.
- *Pricing:* Free up to ~1,000 contacts, then $15-$50/month based on list size.
- *Reputation:* Essential for growth, but deliverability and spam compliance are common challenges.
- *Cloud/Standalone:* Cloud relies on external SMTP providers (SendGrid, Mailgun). Standalone might require configuring local SMTP or using external services via API.
**Design Doc:**
- *Trigger:* User selects a group of customers and creates an email campaign.
- *Action:* OHC formats the email and sends it via an SMTP provider, tracking opens and clicks.
- *User Views:* An email builder interface, a campaign management dashboard, and analytics (open/click rates).
**Implementation Prompt:** Build a simple email campaign tool within OHC. Users should be able to select a segment of their customer list, compose an email using a basic text editor or simple templates, and schedule or send the campaign. Include basic tracking (sent, opened).
**Priority:** P2
**Estimated Scope:** Large

## 4. Payment Processing
**Title:** Localized Payment Options for Global Markets
**Problem Statement:** While Stripe is popular, it's not ideal for all regions. Small businesses in LATAM (Mercado Pago), India (Paytm), or China (Alipay) need payment processors that their local customers trust and use daily.
**Research Report:** Integrating regional payment gateways reduces friction at checkout and increases conversion rates.
- *Ease of Use:* High for the customer; moderate for the business owner (requires account setup with the specific provider).
- *Pricing:* Varies widely by region and provider (typically 1-3% per transaction + fixed fee).
- *Reputation:* Highly dependent on the specific regional provider.
- *Cloud/Standalone:* Cloud involves standard API/webhook integrations. Standalone is similar but requires secure local storage of API keys.
**Design Doc:**
- *Trigger:* Customer initiates checkout.
- *Action:* OHC presents available payment options based on region. The transaction is processed via the selected gateway.
- *User Views:* A checkout page with localized payment buttons. A dashboard for the business owner to view transactions and settlements.
**Implementation Prompt:** Expand the checkout options to include at least one regional payment provider (e.g., Mercado Pago or Alipay) alongside Stripe. The integration should handle payment intent creation, customer redirection (if necessary), and webhook processing for payment confirmation.
**Priority:** P1
**Estimated Scope:** Medium

## 5. Shipping & Logistics
**Title:** Automated Shipping Rate Calculation and Label Generation
**Problem Statement:** E-commerce businesses waste hours manually calculating shipping costs, buying postage, and generating labels. They need an automated way to handle logistics from checkout to delivery.
**Research Report:** Shipping aggregators (like Shippo or EasyPost) connect to multiple carriers (USPS, FedEx, DHL) to provide real-time rates and print labels.
- *Ease of Use:* Very high. Automates complex manual tasks.
- *Pricing:* Often a small fee per label (e.g., $0.05) plus carrier costs. Subscriptions available for volume discounts.
- *Reputation:* Generally excellent; they are lifesavers for product-based businesses.
- *Cloud/Standalone:* Both rely heavily on external APIs. Cloud is easier to manage webhook callbacks for tracking updates.
**Design Doc:**
- *Trigger:* Customer enters shipping address at checkout; business owner fulfills the order.
- *Action:* OHC fetches real-time rates at checkout. During fulfillment, it generates a shipping label and tracking number.
- *User Views:* Live shipping rates at checkout. A "Fulfill Order" button in the dashboard that generates a printable label and emails the tracking link to the customer.
**Implementation Prompt:** Integrate a shipping API (like Shippo) to calculate shipping costs during checkout based on item weight/dimensions and customer address. Add a button in the order management view to generate and print a shipping label, automatically updating the order status to 'Shipped'.
**Priority:** P1
**Estimated Scope:** Large

## 6. SMS & Notifications
**Title:** Reliable SMS Alerts for Appointments and Deliveries
**Problem Statement:** Email open rates are dropping. Critical updates like appointment reminders or delivery notifications are often missed unless sent via SMS, especially for users with lower technical literacy or English proficiency.
**Research Report:** SMS APIs (like Twilio or MessageBird) allow programmatic sending of text messages.
- *Ease of Use:* High for the recipient. The business owner sets up templates.
- *Pricing:* Pay-per-message (typically $0.005 - $0.05 depending on the country).
- *Reputation:* High open and read rates. Essential for time-sensitive alerts.
- *Cloud/Standalone:* Both require integration with an external SMS gateway API.
**Design Doc:**
- *Trigger:* A specific event occurs (e.g., appointment tomorrow, order out for delivery).
- *Action:* OHC formats the message using a template and sends it via the SMS gateway.
- *User Views:* Settings to toggle SMS notifications on/off and customize message templates.
**Implementation Prompt:** Implement automated SMS notifications for key events (e.g., appointment reminders). Allow business owners to customize the message templates. Ensure the system handles opt-outs (e.g., "Reply STOP to unsubscribe") compliantly.
**Priority:** P0
**Estimated Scope:** Medium

## 7. Video Conferencing
**Title:** Auto-Generated Meeting Links for Consultations
**Problem Statement:** Service-based businesses (tutors, consultants) currently have to manually create a Zoom or Google Meet link and email it to the client for every booked session, causing friction and errors.
**Research Report:** Video conferencing integrations automatically generate a unique meeting link when an appointment is booked.
- *Ease of Use:* Very high. Completely automates a manual step.
- *Pricing:* Often requires a paid tier on the video platform (e.g., Zoom Pro) to use the API effectively.
- *Reputation:* Standard expectation for modern scheduling tools.
- *Cloud/Standalone:* Cloud is preferred due to OAuth flows. Standalone is possible but managing OAuth tokens locally can be complex.
**Design Doc:**
- *Trigger:* A new appointment is scheduled.
- *Action:* OHC calls the Zoom/Meet API to create a meeting, retrieves the link, and adds it to the calendar invite and confirmation emails.
- *User Views:* A setting to connect their Zoom/Meet account. The meeting link appears in appointment details and client communications.
**Implementation Prompt:** Integrate with Zoom or Google Meet to automatically generate a unique video conferencing link when a user books an appointment. The link should be included in the confirmation email sent to the client and visible in the appointment details on the business owner's dashboard.
**Priority:** P2
**Estimated Scope:** Medium
