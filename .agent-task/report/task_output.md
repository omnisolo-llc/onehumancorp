# Chatwoot Integration

## Title
Integrate Chatwoot for Omnichannel Social Media Inbox

## Problem Statement
As a small business owner, I receive messages from customers across Facebook, Instagram, WhatsApp, and email. Jumping between different apps to reply takes too much time, and I often miss messages or reply late, which costs me sales. I need one single inbox where I can see and reply to all my customer messages easily.

## Research Report
Chatwoot is a modern, open-source omnichannel customer support platform. It supports Meta (Facebook/Instagram), WhatsApp, Telegram, Line, and email in a single unified inbox.
- **Ease of use:** High. The interface is intuitive and designed to resemble familiar email/messaging clients.
- **Pricing:** Free tier available for small teams (self-hosted is free), with cloud plans starting around $19/user/month.
- **Reputation:** Well-regarded open-source alternative to Intercom, with 25k+ GitHub stars and strong community support.
- **Integration Risks:** Relies on third-party APIs (Meta, WhatsApp) which have strict rules. OAuth approval processes for Meta can be complex.
- **Cloud/Standalone:** Works perfectly in both. Cloud mode can use the hosted Chatwoot or a central instance, while Standalone mode can easily run Chatwoot as a self-hosted Docker container alongside the local backend.

## Design Doc
When a user connects their social media accounts to OHC, OHC will automatically provision a Chatwoot inbox (either by calling the Chatwoot Cloud API or the local standalone instance API). Social media messages are routed into this inbox via webhooks. The business owner will view a unified "Inbox" tab within the OHC dashboard, which seamlessly embeds or syncs with the Chatwoot interface, allowing them to reply directly from OHC.

## Implementation Prompt
Create an "Inbox" feature in the OHC dashboard that connects to a user's social media accounts (Facebook, Instagram, WhatsApp). When customers message the business on any of these platforms, the messages should appear in this single OHC Inbox. The business owner must be able to read and reply to these messages directly from the OHC Inbox without logging into the separate social media apps.

## Priority
P1

## Estimated Scope
Medium

---

# Cal.com Integration

## Title
Integrate Cal.com for Automated Calendar & Scheduling

## Problem Statement
Scheduling calls and appointments with clients involves too much back-and-forth over email or text to find a time that works. It's frustrating and sometimes clients just give up. I need a way to simply send a link to my clients so they can pick a time that is automatically synced with my Google or Outlook calendar.

## Research Report
Cal.com is an open-source scheduling tool and a direct competitor to Calendly. It supports Google Calendar, Outlook, Apple Calendar, and more.
- **Ease of use:** Very high for both the business owner setting up availability and the client booking the slot.
- **Pricing:** Free for individuals. Team plans are around $12/user/month.
- **Reputation:** Excellent reputation in the developer and startup community (backed by prominent tech figures). Known for clean design and extensive customization.
- **Integration Risks:** Timezone handling can be tricky, but Cal.com handles it natively. OAuth for Google/Microsoft calendars requires maintaining approved API apps.
- **Cloud/Standalone:** Fully supports both. Cloud mode can leverage Cal.com's hosted API, and Standalone mode can run Cal.com's self-hosted version locally.

## Design Doc
OHC will integrate with Cal.com via its API. When a business owner sets up scheduling in OHC, OHC will provision a Cal.com booking link and handle the OAuth flow for the user's primary calendar (Google/Outlook). The OHC dashboard will display upcoming appointments by querying the Cal.com webhooks/API. The booking page will be embedded or linked from the user's OHC website.

## Implementation Prompt
Add a "Scheduling" feature to OHC where the business owner can connect their Google or Outlook calendar and set their available hours. Generate a booking link that the owner can share with clients. When a client books a time using the link, it should automatically appear on the owner's calendar and show up in an "Upcoming Appointments" view in the OHC dashboard.

## Priority
P0

## Estimated Scope
Small

---

# Klaviyo Integration

## Title
Integrate Klaviyo for Email Marketing Automation

## Problem Statement
I have a list of customer emails from my sales, but I don't have a good way to send them professional-looking newsletters, promotional offers, or automated "thank you" emails. I want an easy way to email my customers to keep them coming back, without needing to be a graphic designer.

## Research Report
Klaviyo is a powerful email and SMS marketing automation platform heavily tailored toward ecommerce and retail businesses.
- **Ease of use:** Moderate to High. Excellent drag-and-drop template builder, though its advanced segmentation can have a learning curve.
- **Pricing:** Free tier up to 250 contacts. Paid plans scale with the number of contacts.
- **Reputation:** Extremely strong, especially in the Shopify/ecommerce ecosystem. Known for high deliverability and deep analytics.
- **Integration Risks:** Syncing large customer lists efficiently requires robust background jobs. Strict compliance rules for spam (CAN-SPAM/GDPR) must be adhered to.
- **Cloud/Standalone:** Supported in both. As a SaaS product, integration relies on cloud APIs regardless of OHC's deployment mode.

## Design Doc
OHC will sync the business owner's customer directory (CRM data) directly to a Klaviyo list via the Klaviyo API. When a new customer is added in OHC, a webhook or background job will push the contact to Klaviyo. OHC will provide a simplified UI to select Klaviyo templates and trigger campaign sends via the API, or simply provide a single-sign-on link to the Klaviyo dashboard for advanced campaign management.

## Implementation Prompt
Build a "Marketing" tab that syncs our OHC customer list with Klaviyo. The business owner should be able to see their total subscriber count in OHC. Provide a flow for the owner to draft an email update and send it to their customer list, ensuring that any new customers added to OHC are automatically subscribed to the email list.

## Priority
P1

## Estimated Scope
Medium

---

# Paytm Integration

## Title
Integrate Paytm for Regional Payment Processing (India)

## Problem Statement
My customers in India prefer to pay using UPI, Paytm wallets, or local bank transfers. Using international payment gateways doesn't support these local methods well, leading to failed transactions and lost sales. I need a reliable way to accept local Indian payments effortlessly.

## Research Report
Paytm is one of India's leading digital payments and financial services platforms, offering Payment Gateway services, UPI, wallets, and POS billing.
- **Ease of use:** Familiar to almost all Indian consumers. For merchants, the onboarding requires KYC verification which can be tedious but is standard.
- **Pricing:** Standard payment gateway fees (often 0% for UPI, ~2% for cards).
- **Reputation:** Ubiquitous in India. Very high trust among local consumers.
- **Integration Risks:** Requires strict adherence to Indian financial regulations (RBI guidelines). The KYC process for merchants cannot be fully automated by OHC.
- **Cloud/Standalone:** Cloud API integration works for both OHC modes.

## Design Doc
OHC will integrate the Paytm Payment Gateway API as an alternative checkout provider. In the OHC billing/invoicing module, if the merchant selects Paytm, OHC will generate Paytm payment links or dynamic UPI QR codes for invoices. Webhooks from Paytm will update the invoice status in OHC to "Paid" once the transaction is successful.

## Implementation Prompt
Add Paytm as a payment option for business owners to accept money. When the business owner creates an invoice in OHC, allow them to generate a Paytm payment link or a UPI QR code. Once the customer pays using the link or QR code, the invoice in OHC should automatically mark itself as paid.

## Priority
P2

## Estimated Scope
Medium

---

# Shippo Integration

## Title
Integrate Shippo for Automated Shipping & Label Generation

## Problem Statement
When I sell a physical product, calculating shipping costs, buying postage, and typing out shipping labels manually is incredibly tedious. I need a system that automatically gets the cheapest shipping rates, prints the label for me, and sends the tracking number to my customer.

## Research Report
Shippo is a multi-carrier shipping software that connects businesses with carriers like USPS, UPS, FedEx, and DHL.
- **Ease of use:** High. Simplifies complex shipping carrier APIs into a single, user-friendly interface.
- **Pricing:** Free starter plan (pay-as-you-go per label), with professional plans for high volume.
- **Reputation:** Very positive. Highly reliable API and strong discounts on USPS/UPS rates compared to retail.
- **Integration Risks:** Physical address validation errors can cause label generation failures. Carrier APIs can occasionally go down, requiring robust error handling.
- **Cloud/Standalone:** Cloud API integration perfectly suits both OHC modes.

## Design Doc
OHC will connect to the Shippo API to handle order fulfillment. When an order is marked for shipping in OHC, OHC will send the package dimensions, weight, and addresses to Shippo to fetch rate quotes. The user selects a rate, and OHC triggers the label purchase via Shippo, downloads the PDF label for printing, and stores the tracking number to display to the user and email to the customer.

## Implementation Prompt
Create a "Shipping" feature for orders. When an owner has a physical order to ship, allow them to enter the box weight and size, and show them a list of shipping rates (e.g., USPS, UPS). Let them buy the shipping label with one click, print it out, and automatically send an email to the customer with their tracking link.

## Priority
P1

## Estimated Scope
Large

---

# Twilio Integration

## Title
Integrate Twilio for SMS Notifications and Alerts

## Problem Statement
Not all of my customers check their emails regularly, especially for time-sensitive things like appointment reminders, order pickups, or payment links. I need a way to send automatic text messages to their phones so I know they get the information immediately.

## Research Report
Twilio is the industry-standard cloud communications platform for programmatic SMS, voice, and WhatsApp.
- **Ease of use:** Developer-centric API, but the end-user (business owner) will experience it seamlessly through OHC's UI.
- **Pricing:** Pay-as-you-go, very affordable (fractions of a cent per SMS depending on the region).
- **Reputation:** Gold standard for CPaaS (Communications Platform as a Service). High deliverability and reliability.
- **Integration Risks:** Strict telecom regulations (like A2P 10DLC in the US) require business registration campaigns to avoid carrier filtering, which adds friction to merchant onboarding.
- **Cloud/Standalone:** Cloud API integration perfectly suits both OHC modes.

## Design Doc
OHC will utilize the Twilio Programmable Messaging API to send outbound SMS. OHC will handle the logic for triggering messages based on state changes (e.g., Appointment = 'Upcoming' triggers a reminder SMS 24 hours prior). For regulatory compliance, OHC will need to provide a UI for merchants to submit their A2P 10DLC registration details, which OHC will forward to Twilio.

## Implementation Prompt
Add automatic SMS notifications for important customer events. The business owner should be able to turn on a toggle that says "Send SMS reminders to customers." When enabled, customers will receive a text message 24 hours before their scheduled appointment, or when their order is ready for pickup.

## Priority
P0

## Estimated Scope
Medium

---

# Zoom Integration

## Title
Integrate Zoom for Automated Video Conferencing Links

## Problem Statement
I offer online consultations and lessons, but manually creating a new Zoom meeting for every client and emailing them the link is repetitive and error-prone. I want a video meeting link to be automatically generated and sent to the client as soon as they book a time with me.

## Research Report
Zoom is the market leader in video conferencing and unified communications.
- **Ease of use:** Extremely familiar to most users; joining a meeting is virtually frictionless.
- **Pricing:** Free tier available (40-minute limit for groups, but 1:1 is flexible). Pro plans start around $15/month.
- **Reputation:** Ubiquitous and highly reliable.
- **Integration Risks:** Requires OAuth integration for user-level access to generate meetings on their behalf. Zoom's API rate limits and token expirations must be managed carefully.
- **Cloud/Standalone:** Cloud API integration perfectly suits both OHC modes.

## Design Doc
OHC will implement the Zoom API via an OAuth app. When a user connects their Zoom account, OHC receives refresh/access tokens. When an online appointment is booked (e.g., via the Cal.com integration), OHC will call the Zoom API to create a new meeting. The returned `join_url` will be saved to the appointment record and automatically injected into the confirmation emails sent to both the business owner and the client.

## Implementation Prompt
Integrate Zoom so that when an online service is booked, a unique Zoom meeting is automatically created. The business owner should simply connect their Zoom account once. From then on, any online booking will automatically include a fresh Zoom link in the calendar invite and confirmation email, so both the owner and the client know exactly where to meet.

## Priority
P1

## Estimated Scope
Medium
