# Tool Integration Research Q4 2024

This document outlines evaluations and integration proposals for 7 key software categories, designed specifically to empower non-technical small business owners using OHC in both Cloud and Standalone environments.

---

## 1. Social Media Integration

**Title**: Integrate Social Media Direct Messages and Comments into Unified Inbox
**Problem Statement**: Small business owners waste hours switching between Instagram, Facebook, WhatsApp, and TikTok to reply to customer inquiries, often missing messages and losing sales. They need a single place to view and respond to all customer communications.
**Research Report**: Meta provides a comprehensive Graph API for Messenger, Instagram Direct, and WhatsApp Business. Integrating these is highly valuable, as these channels are where small businesses interact with customers daily. TikTok's Webhook API is available but less mature. Pricing: Mostly free API access, WhatsApp has per-conversation pricing after a free tier (1,000 service conversations/month free). For Standalone users, configuring OAuth might be harder without an OHC intermediary.
**Design Doc**: The user connects their social accounts via a simple OAuth flow in OHC's 'Integrations' settings. Once connected, incoming DMs and comments trigger webhooks to OHC, which parses them and adds them to a 'Unified Inbox'. The user replies from OHC, which uses the provider's API to send the response. In Standalone mode, users may need to provide their own App IDs via an Advanced Mode toggle, or OHC could proxy requests.
**Implementation Prompt**: Build a unified inbox feature that allows users to connect their Instagram and Facebook pages. The user should be able to receive incoming messages in OHC and reply to them directly from the OHC interface. The setup must be straightforward and guide the user through the Meta connection process. Acceptance criteria include successfully receiving an IG DM and sending a reply from OHC that appears in the customer's IG app.
**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

**Title**: Auto-Sync Appointments with Google and Outlook Calendars
**Problem Statement**: Double-booking is a nightmare for service-based businesses. Managing a separate booking system from their personal/work calendar leads to scheduling conflicts and unhappy clients.
**Research Report**: Google Calendar is the dominant player, followed by Outlook. Both offer mature APIs for syncing events. While tools like Calendly exist, building this natively into OHC keeps users on-platform. Ease of use is high for the end user once authenticated. Pricing: Free APIs. Works well in both Cloud and Standalone (requires local OAuth callback handling).
**Design Doc**: OHC users authorize their Google/Outlook calendars. The system fetches busy times and prevents the OHC booking system from offering those slots. When an appointment is created in OHC, it automatically creates a corresponding event in the external calendar. The integration should handle timezone conversions seamlessly and resolve conflicts by treating the external calendar as the source of truth for "busy" blocks.
**Implementation Prompt**: Create a seamless integration for users to connect their Google Calendar or Outlook Calendar. The booking system should automatically block off times when the user has personal events, and new OHC bookings should immediately appear on their external calendar. Acceptance criteria: User connects calendar, creates a test event in Google, and OHC correctly blocks that time slot.
**Priority**: P0
**Estimated Scope**: Medium

---

## 3. Email Marketing

**Title**: Integrated Email Campaign Management for Customer Retention
**Problem Statement**: Business owners struggle to export their customer list from their CRM to send newsletters or promotional blasts, finding standard email marketing tools overly complex and disconnected from their core operations.
**Research Report**: Mailchimp is well-known but can be intimidating and expensive for very small businesses. Resend offers a developer-friendly API that OHC could abstract into a simple "Send Blast" feature. Pricing: Resend is cheap ($20/mo for 50k emails), Mailchimp has a free tier but scales up quickly. If OHC abstracts the sending, it drastically simplifies the UI. Standalone might require the user to plug in their own API key (e.g., SendGrid/Resend) or SMTP details.
**Design Doc**: A "Marketing" tab in OHC where users can select segments of their customer list (e.g., "all customers in the last 6 months") and compose a simple email. The system uses an integrated email provider to send the blast and tracks open rates. In Standalone mode, provide an 'Advanced Mode' toggle to enter custom SMTP credentials or API keys.
**Implementation Prompt**: Implement an email blast feature allowing users to draft a message and send it to their customer list directly from OHC. The interface must be extremely simple—just a subject, body, and audience selector. Acceptance criteria: A user can select 5 test customers, write a promotional email, click send, and all 5 receive the email.
**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing

**Title**: Localized Payment Processing Integration (Mercado Pago / Razorpay)
**Problem Statement**: Stripe doesn't support every country, and many small businesses in LATAM or India rely on regional gateways like Mercado Pago or Razorpay. Without these, they cannot accept digital payments easily through OHC.
**Research Report**: Mercado Pago is critical for LATAM; Razorpay is dominant in India. Both have robust APIs and checkout widgets. Fees are competitive regionally. They handle local payment methods (Pix in Brazil, UPI in India) which are absolute must-haves. Ease of use is very high for local users who are already familiar with these platforms.
**Design Doc**: Add alternative payment providers alongside Stripe. When setting up payments, users select their region and connect their respective provider (e.g., Mercado Pago for Brazil/Argentina). OHC generates payment links or checkout sessions using the selected provider's API. Webhooks from the provider update the invoice status in OHC to "Paid".
**Implementation Prompt**: Add support for connecting Mercado Pago (or Razorpay) as an alternative to Stripe for invoice and checkout payments. The user should be able to select their provider, authorize it, and start accepting payments via local methods (like Pix or UPI). Acceptance criteria: An invoice can be paid using the new provider, and OHC correctly marks it as paid automatically.
**Priority**: P1
**Estimated Scope**: Medium

---

## 5. Shipping & Logistics

**Title**: Automated Shipping Label Generation and Rate Calculation
**Problem Statement**: Product-based small businesses spend hours copying customer addresses into shipping carrier websites to print labels, risking typos and wasting time.
**Research Report**: Shippo and EasyPost aggregate multiple carriers (USPS, UPS, FedEx, DHL, local carriers) into a single API. They are ideal for non-technical users as they abstract the carrier-specific complexity. Pricing: usually pennies per label plus carrier costs. EasyPost is highly reliable. Works well in Cloud; Standalone users could input their own API key via an Advanced Mode toggle.
**Design Doc**: On an OHC "Order" screen, add a "Create Shipping Label" button. OHC calls the shipping API (Shippo/EasyPost) with the package dimensions and customer address to fetch rates. The user selects a rate, and OHC generates the PDF label for printing, automatically saving the tracking number and emailing it to the customer.
**Implementation Prompt**: Build a shipping integration that allows users to instantly generate and print shipping labels for orders directly within OHC. The flow must automatically pull the customer's address and email them a tracking link once the label is bought. Acceptance criteria: User can view an order, click to buy a label, download the PDF, and the system saves the tracking info.
**Priority**: P1
**Estimated Scope**: Large

---

## 6. SMS & Notifications

**Title**: Reliable SMS Notifications and Appointment Reminders
**Problem Statement**: Customers miss appointments or ignore emails, costing the business owner money. SMS has a 98% open rate, but setting up SMS automation is too technical for most owners.
**Research Report**: Twilio is the industry standard but heavily developer-focused. MessageBird is another strong option. Small businesses need automated reminders (e.g., "Your appointment is tomorrow at 2 PM"). Compliance (A2P 10DLC in the US) is a major hurdle for self-serve. OHC needs to abstract this. Pricing: ~$0.01 per message. Standalone users will definitely need to bring their own Twilio API keys.
**Design Doc**: A notification settings panel where the user can toggle "Send SMS Reminders". In Cloud mode, OHC handles the backend sending (possibly bundling costs into a premium tier). In Standalone mode, an 'Advanced Mode' toggle reveals fields for Twilio Account SID and Auth Token. When an appointment is 24 hours away, the system automatically dispatches an SMS.
**Implementation Prompt**: Add an automated SMS reminder system for appointments and important customer alerts. The setup must be as simple as a toggle switch for the business owner. For Standalone users, provide a clear, guided setup to input their Twilio credentials. Acceptance criteria: The system successfully sends an SMS to a test phone number 24 hours before a scheduled test appointment.
**Priority**: P0
**Estimated Scope**: Medium

---

## 7. Video Conferencing

**Title**: Auto-Generate Video Links for Online Consultations
**Problem Statement**: Coaches, tutors, and consultants have to manually create Zoom links and email them to clients after they book an online session, looking unprofessional and taking extra time.
**Research Report**: Zoom's API is ubiquitous for this. Google Meet is also highly requested as it integrates seamlessly with Google Calendar. Both allow programmatic creation of meetings. Free tiers are available (Zoom 40-min limit on free, Meet free via Google accounts). Ease of use is high for the end user; they just connect their account once.
**Design Doc**: When configuring a "Service" in OHC, the user can set the location to "Online/Video Call". If connected to Zoom or Google, booking this service automatically triggers an API call to generate a unique meeting link. This link is automatically included in the calendar invite and confirmation emails for both the business owner and the customer.
**Implementation Prompt**: Integrate video conferencing so that online bookings automatically generate a unique Zoom or Google Meet link. The link should be securely shared with the customer upon confirmation. Acceptance criteria: A customer books an "Online Consultation", and the confirmation page and email both contain a functional, unique video meeting link.
**Priority**: P2
**Estimated Scope**: Small
