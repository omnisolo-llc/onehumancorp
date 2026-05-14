# OHC Core Tool Integrations Research Report

This report outlines the recommended tools to integrate with OHC to solve critical pain points for small business owners in both Cloud and Standalone environments.

## 1. Social Media Integration
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya (Home Baker) loses track of messages across Instagram DMs, Facebook comments, and WhatsApp. She misses custom cake orders because she forgets to check a specific app. She needs a single, unified inbox that just works, without dealing with complex Meta developer portal setups.
**Research Report**:
- **Tool evaluated**: Ayrshare
- **Ease of Use**: Highly abstract API that removes the complexity of direct Meta Graph API OAuth. Business owners can connect their accounts with a simple 1-click OAuth flow within OHC.
- **Pricing**: ~$5/month per connected profile bundle (cost-effective for OHC free tier or basic paid tier).
- **Reputation**: Strongly favored by SaaS platforms for white-labeled social media integration.
- **Environment**: Works seamlessly in Cloud mode via webhook aggregation; can be configured for Standalone mode via local webhook forwarding or polling.
**Design Doc**:
- **Trigger**: User connects their social accounts in the "Integrations" tab.
- **Action**: Inbound messages from Instagram, FB, and WhatsApp trigger webhooks to OHC. The "Salesperson" agent can optionally draft replies.
- **User Interface**: A new "Unified Inbox" view in the OHC dashboard where all messages appear in a single conversational thread per customer.
**Implementation Prompt**: Implement an Ayrshare integration that provides an OAuth flow for tenants to connect their social accounts. Create webhook handlers to ingest inbound messages into a unified OHC inbox table, and an outgoing queue to send replies back through Ayrshare.
**Priority**: P0 (critical)
**Estimated Scope**: Large

## 2. Calendar & Scheduling
**Title**: Self-Hosted Scheduling with Cal.com Integration
**Problem Statement**: Leo (Music Tutor) currently uses a messy combination of Google Calendar links and SMS to schedule piano lessons. Double bookings happen constantly, and students forget to show up. He needs an automated scheduling page that generates meeting links instantly.
**Research Report**:
- **Tool evaluated**: Cal.com
- **Ease of Use**: Very simple for end-users; provides a clean booking page.
- **Pricing**: Open-source core is free. Hosted version is $12/user/mo.
- **Reputation**: High developer trust, actively maintained, strong alternative to Calendly.
- **Environment**: Perfect for OHC. The open-source version can be embedded directly into Standalone mode deployments with no recurring SaaS fees.
**Design Doc**:
- **Trigger**: A customer clicks "Book Now" on a tenant's OHC storefront.
- **Action**: OHC renders an embedded Cal.com widget. Upon booking completion, Cal.com dispatches a webhook to OHC to log the appointment and trigger the billing engine (if applicable).
- **User Interface**: The tenant sees a unified calendar view in OHC. The customer sees a clean, frictionless booking page.
**Implementation Prompt**: Embed Cal.com's open-source scheduling engine into the OHC storefront builder. Configure the system so tenants can define their working hours within OHC, which syncs to Cal.com. Handle the booking webhook to create calendar events in the OHC dashboard and auto-generate meeting links (e.g., Zoom).
**Priority**: P1 (high)
**Estimated Scope**: Medium

## 3. Email Marketing
**Title**: White-labeled Email Campaigns with Resend
**Problem Statement**: Fatima (Cleaning Service) wants to email her customer list a 10% discount for the holidays. Currently, she has to manually export her OHC customer list to Mailchimp, which is confusing and expensive. She needs to send professional emails directly from her OHC dashboard.
**Research Report**:
- **Tool evaluated**: Resend
- **Ease of Use**: For the business owner, it's invisible. They just type an email in OHC and click "Send".
- **Pricing**: Excellent free tier (3,000 emails/mo) and very cheap thereafter ($20 for 50k).
- **Reputation**: Currently the most developer-loved email API. High deliverability and spam compliance features built-in.
- **Environment**: Cloud mode native. Standalone users would need to provide their own Resend API key or fallback to a local SMTP server.
**Design Doc**:
- **Trigger**: Tenant drafts a promotional message in the "Customers" tab and clicks "Send Broadcast".
- **Action**: OHC compiles the email using React Email templates and fires it off via the Resend API, tracking open/click webhooks.
- **User Interface**: A simple rich-text editor in the OHC dashboard with audience filtering (e.g., "Send to all customers who haven't booked in 3 months").
**Implementation Prompt**: Integrate the Resend SDK. Create an interface for tenants to draft broadcast emails to their unified customer list. Implement webhook listeners for delivery, bounce, and complaint events to automatically clean the tenant's mailing list.
**Priority**: P2 (medium)
**Estimated Scope**: Medium

## 4. Payment Processing
**Title**: Regional Payment Expansion with Razorpay (India)
**Problem Statement**: Rohan (Handmade Crafts, India) cannot easily use Stripe for local customers who prefer UPI, RuPay, or local net banking. He loses sales because the checkout experience feels foreign and lacks local payment methods.
**Research Report**:
- **Tool evaluated**: Razorpay
- **Ease of Use**: Familiar OAuth connection flow for the business owner.
- **Pricing**: Standard ~2% per transaction in India.
- **Reputation**: The undisputed leader for payments in India.
- **Environment**: Cloud mode fully supported. Standalone mode can utilize direct API keys.
**Design Doc**:
- **Trigger**: Customer proceeds to checkout and their location is identified as India.
- **Action**: OHC overrides the default Stripe processor and initializes a Razorpay checkout session, natively supporting UPI.
- **User Interface**: During checkout, Indian customers see prominent UPI QR codes and local bank options instead of just credit card fields.
**Implementation Prompt**: Implement Razorpay as a secondary payment gateway. Add logic to the checkout flow to detect regional preferences and display the Razorpay widget when appropriate. Handle Razorpay webhooks to transition OHC orders from "Pending" to "Paid".
**Priority**: P1 (high)
**Estimated Scope**: Large

## 5. Shipping & Logistics
**Title**: Automated Label Generation and Rate Calculation via Shippo
**Problem Statement**: Maya (Home Baker) wastes hours manually typing addresses into USPS.com to print shipping labels for her nationwide cookie orders. She needs automatic shipping rate calculation at checkout and 1-click label printing from her dashboard.
**Research Report**:
- **Tool evaluated**: Shippo
- **Ease of Use**: Aggregates 85+ carriers globally. Business owners don't need individual carrier accounts.
- **Pricing**: $0.05 per label or flat monthly fee. Very affordable.
- **Reputation**: Highly reliable shipping API used by major e-commerce platforms.
- **Environment**: Cloud mode supported via OHC central account. Standalone mode supported via user-provided API key.
**Design Doc**:
- **Trigger**: Customer enters shipping address at checkout; Tenant clicks "Fulfill Order" in dashboard.
- **Action**: OHC queries Shippo for real-time rates during checkout. During fulfillment, OHC purchases the label via Shippo API and returns the PDF to the tenant.
- **User Interface**: A "Print Shipping Label" button on the Order Details page that instantly downloads a PDF ready for a thermal printer.
**Implementation Prompt**: Integrate Shippo API for real-time rate calculation during the storefront checkout flow. Add a fulfillment action in the admin dashboard to generate, purchase, and download shipping label PDFs, while automatically emailing tracking numbers to the customer.
**Priority**: P1 (high)
**Estimated Scope**: Medium

## 6. SMS & Notifications
**Title**: Global SMS Notifications with Twilio
**Problem Statement**: Carlos (Landscaper) works in the field. He doesn't check email. When a new quote request comes in, he needs a text message instantly so he can reply before losing the lead.
**Research Report**:
- **Tool evaluated**: Twilio
- **Ease of Use**: Invisible to the end user.
- **Pricing**: ~$0.0079 per message. Extremely cost-effective for high-value alerts.
- **Reputation**: The industry standard for programmatic SMS. Highest global deliverability.
- **Environment**: Cloud mode uses OHC's master Twilio account. Standalone mode requires the user to input their own Twilio SID/Token.
**Design Doc**:
- **Trigger**: A high-priority event occurs (e.g., new order paid, new booking made).
- **Action**: OHC dispatches a payload to Twilio to send a brief SMS to the business owner's verified phone number.
- **User Interface**: A simple toggle in settings: "Send me an SMS when I get a new order."
**Implementation Prompt**: Implement a notification service utilizing the Twilio SDK. Allow users to verify their phone number and opt-in to SMS alerts for specific events (new orders, new messages, agent escalations). Ensure compliance with basic opt-out handling.
**Priority**: P0 (critical)
**Estimated Scope**: Small

## 7. Video Conferencing
**Title**: Auto-generated Virtual Meetings via Zoom API
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom meeting link for every online student, copies it, and emails it to them. He frequently pastes the wrong link, causing missed lessons and frustration.
**Research Report**:
- **Tool evaluated**: Zoom API
- **Ease of Use**: Standard OAuth connection.
- **Pricing**: Free tier works for basic integrations (40-min limit applies to the user's Zoom account, not the API).
- **Reputation**: The most universally understood video conferencing tool for non-technical consumers.
- **Environment**: Cloud and Standalone supported (relies purely on the user's Zoom account connection).
**Design Doc**:
- **Trigger**: A service is booked that is marked as "Online/Virtual" (via Cal.com integration).
- **Action**: OHC calls the Zoom API using the tenant's OAuth token to create a meeting, appending the generated `join_url` to the calendar invite and confirmation email.
- **User Interface**: A "Connect Zoom" button in Integrations. A location dropdown on services offering "Zoom Meeting".
**Implementation Prompt**: Add Zoom as an OAuth provider for tenants. When an online appointment is scheduled, utilize the Zoom API to dynamically generate a meeting link, attach it to the OHC booking record, and ensure it is included in all automated confirmation and reminder communications sent to the customer.
**Priority**: P2 (medium)
**Estimated Scope**: Medium
