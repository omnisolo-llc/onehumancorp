# Scout: Tool Integration Research [Q2]

## Title
Scout 🔍 Social Media Integration: Unified Inbox for Facebook, Instagram, WhatsApp

## Problem Statement
Small business owners (like Maya the Baker or Fatima the Food Cart Operator) manage customer inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Jumping between apps is overwhelming and leads to missed sales opportunities. They need a single, unified inbox to view and respond to all customer messages, and an AI agent to draft responses or auto-reply while they sleep.

## Research Report
- **Goal**: Evaluate tools that provide a unified API for social media messaging (Facebook, Instagram, WhatsApp) to integrate into the OHC platform.
- **Top Candidate**: Meta Graph API (specifically Messenger API for Instagram, WhatsApp Business API).
  - **Ease of Use for Non-Technical Users**: High. OHC handles the technical integration; the user just connects their accounts via a simple OAuth flow. The result is a single "Inbox" in the OHC app.
  - **Pricing**: Meta Graph API is mostly free, but WhatsApp Business API has per-conversation pricing after a free tier (1,000 service conversations/month free).
  - **Reputation**: Official Meta APIs are standard and reliable.
- **Alternative**: Chatwoot (Open Source).
  - **Ease of Use**: OHC can use Chatwoot as a middle layer to connect to Meta channels, simplifying our backend. Chatwoot is already mentioned in our `docker-compose` setup!
  - **Pricing**: Free (self-hosted).
  - **Reputation**: Highly regarded open-source omnichannel platform.

## Design Doc
- **Integration**: Use Chatwoot (already part of OHC dev stack) as the unified messaging layer. Connect Meta channels to Chatwoot.
- **Trigger**: Incoming message from IG/FB/WhatsApp -> Chatwoot Webhook -> OHC KAIROS Orchestrator -> Customer Success Agent.
- **Action**: Agent drafts a reply based on business context (inventory, FAQs).
- **User Experience**: The business owner sees all messages in the OHC "Customer Success" inbox. They see AI-drafted replies and can 1-tap approve or edit them.

## Implementation Prompt
"Integrate the Chatwoot omnichannel inbox with the KAIROS Orchestrator's Customer Success Agent. When a new message arrives via Chatwoot webhook, route it to the Customer Success Agent to draft a reply. Display the drafted reply in the OHC mobile dashboard requiring 1-tap approval from the user before sending it back via Chatwoot."

## Priority
P0

## Estimated Scope
Medium

---

## Title
Scout 🔍 Calendar & Scheduling: Simplified Booking & Auto-Zoom Links

## Problem Statement
Service providers (like Carlos the Handyman or Leo the Music Tutor) need a way for customers to book appointments without endless back-and-forth messaging. They also need automatic video links for online sessions and calendar sync so they don't get double-booked.

## Research Report
- **Goal**: Evaluate booking tools that integrate Google Calendar sync and Zoom/Meet link generation.
- **Top Candidate**: Cal.com (Open Source / API).
  - **Ease of Use**: Very high for the end-user. Customers see a simple calendar selection UI.
  - **Pricing**: Generous free tier, white-label API available.
  - **Reputation**: Excellent developer experience, open-source alternative to Calendly.
- **Alternative**: Google Workspace Calendar API + Zoom API directly.
  - **Complexity**: High. Managing OAuth for both platforms and handling conflict resolution is complex to build from scratch.

## Design Doc
- **Integration**: Use Cal.com API for scheduling logic, conflict resolution, and video link generation.
- **Trigger**: Customer clicks "Book Now" on the OHC generated website.
- **Action**: Cal.com handles available slot calculation and creates the event with a video link. Webhook updates OHC-SIP DB.
- **User Experience**: Leo the Music Tutor simply connects his Google account. His public page shows available slots. When a student books, Leo gets a notification, and the event (with Zoom link) appears in his OHC dashboard and Google Calendar.

## Implementation Prompt
"Integrate Cal.com API to handle booking scheduling. Provide a simple 'Connect Calendar' OAuth flow for the business owner. Expose the Cal.com booking widget on the business's public website. Ensure the Operations Agent is notified via webhook when a new booking is created to update the OHC dashboard."

## Priority
P1

## Estimated Scope
Medium

---

## Title
Scout 🔍 Email Marketing: Automated Customer Engagement

## Problem Statement
Boutique owners (like Priya) need to send newsletters or product updates to their customer list, but find Mailchimp too complex. They need a simple way to email all past customers without leaving the OHC platform.

## Research Report
- **Goal**: Evaluate simple email marketing APIs.
- **Top Candidate**: Resend.
  - **Ease of Use**: For the user, it's invisible. The Marketing & Advertising Agent drafts the email, the user approves, and Resend delivers it.
  - **Pricing**: Free up to 3,000 emails/month. Very startup-friendly.
  - **Reputation**: High deliverability, developer-focused, fast.
- **Alternative**: SendGrid.
  - **Complexity**: Older, clunkier API, but reliable. Free tier available.

## Design Doc
- **Integration**: Use Resend API for sending bulk/transactional emails.
- **Trigger**: User requests "Send a newsletter about new summer dresses" -> Marketing Agent drafts email -> User approves.
- **Action**: OHC backend fetches customer emails from OHC-SIP DB and sends via Resend API.
- **User Experience**: Priya tells the Marketing Agent what she wants to say. The Agent drafts a beautiful email. Priya taps "Approve and Send". The agent handles the rest.

## Implementation Prompt
"Integrate the Resend API for email delivery. The Marketing & Advertising Agent should draft email content, which is then sent via Resend upon user approval. Ensure unsubscribe links and bounce handling are automatically managed by the integration."

## Priority
P2

## Estimated Scope
Small

---

## Title
Scout 🔍 Payment Processing: Alternative Global Payments

## Problem Statement
While OHC supports Stripe, many international sellers (like those in LATAM or Asia) require alternative payment processors. They need a simple way to accept local payment methods online.

## Research Report
- **Goal**: Evaluate alternative payment providers for specific markets.
- **Top Candidate**: Adyen / Mercado Pago (for LATAM).
  - **Ease of Use**: OHC can abstract the complexity. The user just signs in with Adyen/Mercado Pago.
  - **Pricing**: Varies by region, generally competitive.
  - **Reputation**: Reliable, wide global reach.
- **Alternative**: PayPal.
  - **Complexity**: Widespread but can have high dispute rates for sellers.

## Design Doc
- **Integration**: Add a new payment provider interface to the Finance & Payments Agent.
- **Trigger**: Customer proceeds to checkout.
- **Action**: OHC dynamically routes the payment to the appropriate configured provider (e.g., Mercado Pago for LATAM).
- **User Experience**: The business owner enables their local payment provider with a single click. Their customers see familiar local payment options.

## Implementation Prompt
"Add support for Mercado Pago as a secondary payment provider in the Finance & Payments Agent. Implement a unified checkout flow that can route to either Stripe or Mercado Pago based on the user's configuration. Ensure the transaction is correctly tracked in the OHC dashboard."

## Priority
P2

## Estimated Scope
Large

---

## Title
Scout 🔍 Shipping & Logistics: Real-Time Rates & Labels

## Problem Statement
Sellers shipping physical products (like Priya's Boutique) need to calculate shipping rates accurately and print shipping labels without manually typing addresses into a separate carrier website.

## Research Report
- **Goal**: Evaluate shipping APIs for real-time rates and label generation.
- **Top Candidate**: Shippo / EasyPost.
  - **Ease of Use**: Very easy. The user just clicks "Print Label".
  - **Pricing**: Small per-label fee.
  - **Reputation**: Both are standard in e-commerce. Shippo often has slightly better international support.
- **Alternative**: Direct carrier APIs (USPS, FedEx).
  - **Complexity**: Much higher. Requires managing multiple separate integrations.

## Design Doc
- **Integration**: Use Shippo API.
- **Trigger**: Order is marked as "Ready to Ship" by the Operations Agent.
- **Action**: OHC requests a shipping label from Shippo based on package weight and destination.
- **User Experience**: The business owner sees a "Print Label" button on the order page. Clicking it downloads a PDF label, and the customer automatically receives tracking information.

## Implementation Prompt
"Integrate the Shippo API into the Operations Agent. Add functionality to calculate real-time shipping rates at checkout and generate PDF shipping labels from the OHC dashboard. Automatically update the customer with tracking details when a label is created."

## Priority
P1

## Estimated Scope
Medium

---

## Title
Scout 🔍 SMS & Notifications: Reliable Global Reach

## Problem Statement
Food cart operators (like Fatima) or any business dealing with local, immediate services need to send order updates via SMS, especially in areas with poor internet connectivity where app notifications might fail.

## Research Report
- **Goal**: Evaluate SMS APIs.
- **Top Candidate**: Twilio.
  - **Ease of Use**: OHC abstracts it completely. The Customer Success Agent drafts the SMS.
  - **Pricing**: Pay-as-you-go per message.
  - **Reputation**: Industry standard, extremely reliable globally.
- **Alternative**: Plivo.
  - **Complexity**: Similar to Twilio, sometimes slightly cheaper, but Twilio's ecosystem is more robust.

## Design Doc
- **Integration**: Use Twilio API.
- **Trigger**: Order status changes (e.g., "Ready for Pickup").
- **Action**: Customer Success Agent triggers an SMS via Twilio.
- **User Experience**: Fatima clicks "Order Ready". The customer immediately receives an SMS in their local language.

## Implementation Prompt
"Integrate the Twilio API into the Customer Success Agent for SMS notifications. Add a setting for business owners to enable SMS updates for their customers. Ensure the agent can handle multi-language SMS templates based on the customer's profile."

## Priority
P1

## Estimated Scope
Small

---

## Title
Scout 🔍 Video Conferencing: Seamless Virtual Meetings

## Problem Statement
Tutors and consultants (like Leo) need to automatically generate video meeting links for booked sessions without manually creating and copying links from Zoom.

## Research Report
- **Goal**: Evaluate video conferencing APIs.
- **Top Candidate**: Zoom API / Google Meet (via Calendar integration).
  - **Ease of Use**: Invisible. The link just appears in the booking confirmation.
  - **Pricing**: Zoom has limits on free accounts. Google Meet is free with a Google account.
  - **Reputation**: Both are ubiquitous.
- **Alternative**: Daily.co.
  - **Complexity**: Allows embedding video directly into the OHC app, but most users prefer familiar platforms like Zoom.

## Design Doc
- **Integration**: If Cal.com handles booking, rely on their Zoom/Meet integration. If building custom, use Zoom API for link generation.
- **Trigger**: A new online appointment is booked.
- **Action**: OHC requests a meeting link from Zoom API.
- **User Experience**: When a student books a lesson, the confirmation email automatically includes a unique Zoom link. The link also appears in Leo's OHC dashboard.

## Implementation Prompt
"Integrate the Zoom API to automatically generate meeting links for online bookings. If using Cal.com, ensure the Zoom integration is properly configured. The meeting link should be included in the customer confirmation and displayed on the business owner's dashboard."

## Priority
P2

## Estimated Scope
Medium
