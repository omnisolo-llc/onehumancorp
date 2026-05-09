# OHC Tool Integration Research Report - Q3

This report contains 7 issue briefs evaluating essential tools for small business owners across multiple domains. Our focus is on user-centric integration that addresses tangible pain points and operates seamlessly in both Cloud and Standalone environments.

---

## [Social Media Integration] Unified Inbox via Ayrshare

**Title**: Implement Unified Social Media Inbox using Ayrshare
**Problem Statement**: Small business owners (like bakers or consultants) receive messages across Instagram, Facebook, TikTok, and WhatsApp. Missing a DM often means losing a sale. Checking 4 different apps multiple times a day is exhausting and error-prone for non-technical users.
**Research Report**:
- **Tool Evaluated**: Ayrshare.
- **User Benefit**: Aggregates comments and DMs into a single interface.
- **Pros**: Handles the complex OAuth processes for multiple social networks behind a single API. Reliable webhook delivery.
- **Cons**: Pricing scales with the number of connected profiles (can be costly for users with many brands).
- **Pricing Estimate**: ~$55/month for the Premium tier, scaling up to >$200/month for agencies.
- **Cloud & Standalone Support**: Works well via API in Cloud. For Standalone, we would need to proxy requests through our central gateway or have users bring their own Ayrshare key, which might be a friction point.
- **Ease of Use**: Highly beneficial for the user, hiding the complexity of Meta's Graph API.
**Design Doc**:
- **Trigger**: User connects their social profiles via the Settings > Integrations page.
- **Action**: Webhooks from Ayrshare flow into OHC, creating generic "Message" entities in the unified inbox. Replies from the OHC inbox are routed back to the appropriate social network via Ayrshare API.
- **UI**: A unified inbox view in the dashboard with social platform icons indicating the source.
**Implementation Prompt**: Build an integration with Ayrshare that allows users to authenticate their social profiles and displays incoming messages from all supported platforms in a single OHC inbox. Users must be able to reply directly from OHC.
**Priority**: P1
**Estimated Scope**: Large

---

## [Calendar & Scheduling] Automated Booking via Cal.com

**Title**: Enable 1-Click Client Booking via Cal.com
**Problem Statement**: Service-based businesses waste hours going back and forth over email to find a time to meet with clients.
**Research Report**:
- **Tool Evaluated**: Cal.com.
- **User Benefit**: Generates a customizable booking page that syncs with their existing Google/Outlook calendar.
- **Pros**: Open-source, self-hostable (great for our Standalone mode), clean UI, handles timezone conversions flawlessly.
- **Cons**: Might require some setup for non-technical users to link their primary calendars.
- **Pricing Estimate**: Free for individuals (basic), ~$15/user/month for team features. Self-hosted version is free.
- **Cloud & Standalone Support**: Excellent. We can utilize their managed API for Cloud and self-hosted instances for Standalone.
**Design Doc**:
- **Trigger**: User configures their availability schedule and connects their Google Calendar.
- **Action**: OHC generates a unique Cal.com booking link that the user can put on their OHC storefront. Bookings trigger a new "Appointment" entity in OHC.
- **UI**: A "Schedule" tab where the business owner can view upcoming appointments.
**Implementation Prompt**: Integrate Cal.com so business owners can generate a booking link based on their availability. Appointments booked via this link must automatically appear in the OHC dashboard.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Email Marketing] Customer Engagement via Listmonk

**Title**: Embedded Email Campaigns using Listmonk
**Problem Statement**: Business owners want to email past customers about promotions or holiday specials, but traditional tools like Mailchimp are too complex and expensive.
**Research Report**:
- **Tool Evaluated**: Listmonk.
- **User Benefit**: Simple, newsletter-style emailing directly to their OHC customer list.
- **Pros**: Open-source, self-hostable (perfect for Standalone privacy), lightweight.
- **Cons**: Doesn't come with a built-in SMTP server (requires SES, Sendgrid, or Resend).
- **Pricing Estimate**: Free (open-source software), but requires paying for an external SMTP provider (e.g., AWS SES is ~$0.10 per 1000 emails).
- **Cloud & Standalone Support**: Highly compatible with Standalone (can be bundled). Cloud version can run as a centralized service per tenant.
**Design Doc**:
- **Trigger**: User selects a segment of their customer list in OHC and clicks "Send Promotion".
- **Action**: OHC pushes the list and the plain-text/HTML message to Listmonk via API, which dispatches the emails.
- **UI**: A simple WYSIWYG editor for drafting emails, avoiding complex drag-and-drop builders.
**Implementation Prompt**: Integrate Listmonk as the backend for a simple "Send Email to Customers" feature. It should allow business owners to write a message and send it to all past customers without leaving the OHC interface.
**Priority**: P2
**Estimated Scope**: Medium

---

## [Payment Processing] Localized Checkout via Mercado Pago

**Title**: Enable LATAM Payment Processing via Mercado Pago
**Problem Statement**: Stripe is not widely adopted or supported for micro-businesses in many Latin American countries, leaving these users unable to accept online payments easily.
**Research Report**:
- **Tool Evaluated**: Mercado Pago.
- **User Benefit**: Familiar, trusted payment method for LATAM customers, supporting local currencies and installments.
- **Pros**: Dominant market share in LATAM, supports Pix in Brazil.
- **Cons**: Documentation can be fragmented; settlement times vary by country.
- **Pricing Estimate**: Roughly ~3.99% to ~5.99% per transaction depending on the country and installment plan.
- **Cloud & Standalone Support**: Fully supported via REST API in both modes.
**Design Doc**:
- **Trigger**: User enables "Mercado Pago" in payment settings and completes OAuth.
- **Action**: Checkout pages on the user's OHC storefront generate Mercado Pago payment preferences and display their checkout widget.
- **UI**: "Pay with Mercado Pago" button during the checkout flow for end-customers.
**Implementation Prompt**: Integrate Mercado Pago as an alternative payment gateway. Users should be able to authenticate their MP account and process storefront transactions natively.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Shipping & Logistics] Automated Label Generation via EasyPost

**Title**: 1-Click Shipping Labels via EasyPost
**Problem Statement**: E-commerce sellers manually copy-paste addresses from OHC into their local post office website to print labels, which is slow and error-prone.
**Research Report**:
- **Tool Evaluated**: EasyPost.
- **User Benefit**: Automatically calculates shipping rates and generates printable labels right from the order page.
- **Pros**: Aggregates dozens of carriers under one API. Very reliable.
- **Cons**: Requires users to weigh and measure their packages accurately.
- **Pricing Estimate**: Developer plan is typically free for under 120k shipments/year, charging 1¢-5¢ per label afterward.
- **Cloud & Standalone Support**: API-based, works flawlessly in both modes.
**Design Doc**:
- **Trigger**: User clicks "Fulfill Order" on an open order.
- **Action**: OHC queries EasyPost for rates based on predefined package sizes, purchases the cheapest label, and returns a PDF.
- **UI**: A "Print Label" button directly on the order details page.
**Implementation Prompt**: Integrate EasyPost to allow users to generate and print shipping labels directly from an order's detail view in OHC.
**Priority**: P0
**Estimated Scope**: Large

---

## [SMS & Notifications] Reliable Alerts via Twilio

**Title**: Customer SMS Notifications via Twilio
**Problem Statement**: Users with low English proficiency or those in regions where SMS is king need a reliable way to get order updates. Customers also prefer SMS over email for delivery updates.
**Research Report**:
- **Tool Evaluated**: Twilio.
- **User Benefit**: Automated text messages sent to customers for order confirmations and shipping updates.
- **Pros**: Industry standard, exceptional global coverage, reliable API.
- **Cons**: Strict A2P 10DLC compliance requirements in the US, which can be hard for micro-businesses to navigate.
- **Pricing Estimate**: ~$0.0079 per SMS sent/received in the US; international rates vary (~$0.05 to $0.10+).
- **Cloud & Standalone Support**: API-based, fully supported in both modes.
**Design Doc**:
- **Trigger**: Order status changes (e.g., "Shipped").
- **Action**: OHC triggers an API call to Twilio to send a templated SMS to the customer's phone number.
- **UI**: A toggle in settings: "Send SMS updates to customers" (with a note about potential extra costs).
**Implementation Prompt**: Integrate Twilio to send automated SMS notifications to a customer's phone number when their order is confirmed and when it ships.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Video Conferencing] Auto-Meeting Links via Zoom

**Title**: Auto-Generated Consultation Links via Zoom
**Problem Statement**: Tutors, consultants, and coaches currently have to manually create a Zoom meeting and email the link to their clients after they book a session.
**Research Report**:
- **Tool Evaluated**: Zoom (API).
- **User Benefit**: A Zoom link is automatically generated and added to the calendar invite the moment a client books a session.
- **Pros**: Ubiquitous, everyone knows how to use it.
- **Cons**: Strict OAuth app approval process for the marketplace.
- **Pricing Estimate**: Basic is free (40-minute limit); Pro is ~$15.99/month/user. API access is typically included.
- **Cloud & Standalone Support**: Works well via API. Standalone users will use the standard OAuth flow.
**Design Doc**:
- **Trigger**: A new appointment is booked via the Calendar module.
- **Action**: OHC calls the Zoom API to generate a new meeting for that specific time and attaches the join URL to the appointment record.
- **UI**: A "Join Meeting" button appears on the appointment details page for both the business owner and the client.
**Implementation Prompt**: Integrate the Zoom API so that when a new appointment is booked, a unique Zoom meeting link is automatically generated and shared with both the business owner and the client.
**Priority**: P2
**Estimated Scope**: Medium