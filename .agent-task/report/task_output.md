# Scout: Tool Integration Research [Q2]

## 1. Social Media Integration

**Title**: Integrate ManyChat for Unified Instagram/Facebook DM Automation
**Problem Statement**: Maya (The Home Baker) gets overwhelmed by Instagram DMs asking "do you do vegan cakes?" while she sleeps. She needs a unified inbox where an AI agent can automatically respond to basic queries on Instagram and Facebook without her having to constantly check her phone.
**Research Report**:
- **Tool Evaluated**: ManyChat.
- **Benefits for Non-Technical Users**: ManyChat offers an extremely intuitive visual flow builder for automating social media interactions. It natively integrates with Instagram, Facebook Messenger, and WhatsApp. For users like Maya, an OHC integration would mean she simply connects her Instagram account, and OHC configures ManyChat behind the scenes to handle common FAQs and route serious inquiries to the OHC unified inbox.
- **Risks/Considerations**: Meta's API restrictions can be strict. The integration must carefully handle authentication (OAuth) without overwhelming the user.
- **Pricing**: ManyChat has a free tier that is very capable for new businesses; Pro starts around $15/month.
- **Environment**: Works seamlessly in Cloud mode. Standalone mode would require the user to configure their own API keys or use OHC as an intermediary proxy.
**Design Doc**:
- **Trigger**: User connects their Instagram/Facebook account via the OHC settings page ("Marketing & Advertising" department).
- **Action**: OHC orchestrates the OAuth flow to link ManyChat, setting up predefined AI response flows based on the user's business profile (e.g., baker FAQs).
- **User Interface**: The unified inbox in OHC simply displays the filtered, important messages. The user never sees ManyChat's complex builder unless they choose to.
**Implementation Prompt**: Implement an OAuth integration flow for ManyChat. When a user connects their Instagram account via OHC, use the ManyChat API to provision a basic AI auto-responder for their DMs. Ensure incoming messages that require human intervention are routed to the OHC unified inbox.
**Priority**: P0
**Estimated Scope**: Large

## 2. Calendar & Scheduling

**Title**: Integrate Calendly for Seamless Service Booking and Sync
**Problem Statement**: Carlos (The Freelance Handyman) and Leo (The Music Tutor) need a way for customers to book specific time slots without double-booking over their personal events on Google Calendar.
**Research Report**:
- **Tool Evaluated**: Calendly.
- **Benefits for Non-Technical Users**: Calendly is the industry standard for scheduling. It prevents double-booking by syncing with Google Calendar, Outlook, etc. By embedding Calendly into OHC, Carlos doesn't need to manually check his schedule; customers just pick an available slot.
- **Risks/Considerations**: The embedding must look native to the OHC Glassmorphism design system. Timezone handling must be foolproof.
- **Pricing**: Generous free tier; premium starts at $10/month.
- **Environment**: Works well in both Cloud and Standalone modes via API and iframe embedding.
**Design Doc**:
- **Trigger**: User enables "Bookings" on their OHC service listing ("Operations" department).
- **Action**: OHC provisions a Calendly event type via API, syncing it with the user's provided calendar credentials.
- **User Interface**: The OHC storefront displays a beautifully styled Calendly widget. The OHC dashboard shows upcoming bookings in the calendar view.
**Implementation Prompt**: Create a seamless Calendly integration. Allow the user to connect their Google/Outlook calendar. Use the Calendly API to automatically generate scheduling links for the services they define in OHC, and embed the booking widget on their public OHC page.
**Priority**: P0
**Estimated Scope**: Medium

## 3. Email Marketing

**Title**: Integrate Mailchimp for Automated Customer Engagement
**Problem Statement**: Priya (The Boutique Owner) wants to automatically email her past customers when new clothing stock arrives, but she finds setting up email campaigns confusing.
**Research Report**:
- **Tool Evaluated**: Mailchimp.
- **Benefits for Non-Technical Users**: Mailchimp is powerful but can be complex. The goal is to hide this complexity. OHC's "Marketing & Advertising" agent would draft the email content, and OHC would push the audience list and campaign to Mailchimp via API to handle the actual delivery and spam compliance.
- **Risks/Considerations**: Managing contact sync between OHC and Mailchimp. Handling unsubscribes correctly.
- **Pricing**: Free tier up to 500 contacts; paid starts at ~$13/month.
- **Environment**: Cloud mode primarily.
**Design Doc**:
- **Trigger**: User adds new inventory and clicks "Notify Customers" in OHC.
- **Action**: OHC's AI drafts the email. Once approved, OHC syncs the customer list to Mailchimp and triggers the campaign send via API.
- **User Interface**: The user sees a simple text box to review the AI-drafted email and a "Send" button. They see open rates later in the OHC dashboard.
**Implementation Prompt**: Build a Mailchimp integration that handles one-way sync of OHC customers to a Mailchimp audience. Implement a feature where the user can approve an AI-drafted email in OHC, which is then sent as a campaign via the Mailchimp API. Show basic campaign stats (open rate) in OHC.
**Priority**: P1
**Estimated Scope**: Medium

## 4. Payment Processing

**Title**: Integrate Mercado Pago for LATAM Payment Processing
**Problem Statement**: While OHC supports Stripe, business owners in Latin America often need local payment methods like PIX in Brazil, which are best supported by regional providers.
**Research Report**:
- **Tool Evaluated**: Mercado Pago.
- **Benefits for Non-Technical Users**: Allows businesses in LATAM to accept local credit cards, bank transfers, and mobile payments (like PIX) seamlessly, which drastically increases conversion rates.
- **Risks/Considerations**: API differences from Stripe. Requires careful handling of different currencies and settlement times.
- **Pricing**: Varies by country; typically a percentage + fixed fee per transaction.
- **Environment**: Works in Cloud and Standalone (with user-provided keys).
**Design Doc**:
- **Trigger**: User selects their region as a LATAM country during setup ("Finance & Payments" department).
- **Action**: OHC offers Mercado Pago as the primary payment gateway, guiding them through the OAuth connection.
- **User Interface**: Standard OHC checkout flow, but with Mercado Pago payment options presented.
**Implementation Prompt**: Integrate Mercado Pago as an alternative payment gateway to Stripe. Implement the checkout flow to support Mercado Pago's API, ensuring local payment methods are presented to the customer. Handle payment webhooks to mark orders as paid in OHC.
**Priority**: P1
**Estimated Scope**: Large

## 5. Shipping & Logistics

**Title**: Integrate Shippo for Simplified Shipping and Label Generation
**Problem Statement**: Priya (The Boutique Owner) struggles with calculating shipping costs for out-of-state orders and hates manually writing shipping labels.
**Research Report**:
- **Tool Evaluated**: Shippo.
- **Benefits for Non-Technical Users**: Shippo aggregates multiple carriers (USPS, UPS, FedEx) and provides discounted rates. OHC can use Shippo to automatically calculate shipping at checkout and generate a printable PDF label when the order is fulfilled.
- **Risks/Considerations**: Accurately estimating package dimensions and weights for diverse products.
- **Pricing**: Free to install, pay per label (e.g., $0.05 + postage).
- **Environment**: Cloud mode.
**Design Doc**:
- **Trigger**: Customer proceeds to checkout with physical goods; Business owner clicks "Fulfill Order".
- **Action**: OHC fetches live rates via Shippo API for checkout. On fulfillment, OHC generates the label via Shippo and updates the order with the tracking number.
- **User Interface**: Business owner sees a "Print Shipping Label" button on the order details page.
**Implementation Prompt**: Integrate Shippo to provide real-time shipping rates during the OHC checkout process. Add a feature in the Operations dashboard allowing the business owner to generate and download a shipping label for a paid order, automatically attaching the tracking number.
**Priority**: P0
**Estimated Scope**: Large

## 6. SMS & Notifications

**Title**: Integrate Twilio for Reliable SMS Order Alerts
**Problem Statement**: Fatima (The Food Cart Operator) needs instant notifications on her phone when a pre-order is placed, as she may not be actively looking at the app or have great data coverage.
**Research Report**:
- **Tool Evaluated**: Twilio.
- **Benefits for Non-Technical Users**: SMS is universally reliable, even on low-end phones or poor data connections. OHC can send simple text alerts ("New order: 2x Falafel. Pickup 12:30.") ensuring Fatima never misses an order.
- **Risks/Considerations**: SMS costs can add up. Global carrier deliverability varies. Compliance with SMS opt-in rules.
- **Pricing**: Pay-as-you-go (e.g., ~$0.0079 per SMS in the US).
- **Environment**: Cloud mode (OHC manages the Twilio account) or Standalone (user provides Twilio SID/Token).
**Design Doc**:
- **Trigger**: A customer successfully places and pays for an order.
- **Action**: OHC's backend triggers a Twilio API call to send a formatted SMS to the business owner's registered phone number.
- **User Interface**: User enables "SMS Alerts" in settings and provides their phone number. No complex configuration.
**Implementation Prompt**: Implement SMS notifications using the Twilio API. When a new order is confirmed, dispatch a short, localized text message to the business owner detailing the order items and pickup time. Provide a toggle in settings to enable/disable these alerts.
**Priority**: P1
**Estimated Scope**: Small

## 7. Video Conferencing

**Title**: Integrate Zoom for Automated Online Lesson Links
**Problem Statement**: Leo (The Music Tutor) spends too much time manually creating Zoom links for every booked student and emailing them the details.
**Research Report**:
- **Tool Evaluated**: Zoom (via API).
- **Benefits for Non-Technical Users**: Total automation of online meeting logistics. When a student books a lesson, the Zoom link is automatically generated, attached to the calendar invite, and emailed to both parties.
- **Risks/Considerations**: Zoom OAuth approval process for the OHC app. Managing meeting lifecycle (cancellations/rescheduling).
- **Pricing**: Zoom has a free tier; Pro is ~$15/month.
- **Environment**: Cloud and Standalone modes.
**Design Doc**:
- **Trigger**: A customer books a service marked as "Online Meeting" in OHC.
- **Action**: OHC calls the Zoom API to create a meeting for that specific time, retrieves the join URL, and embeds it in the confirmation email and calendar event.
- **User Interface**: The business owner simply toggles "Online Meeting (Zoom)" on their service listing and authorizes Zoom once.
**Implementation Prompt**: Create a Zoom OAuth integration. Modify the booking flow so that if a service is flagged as an online meeting, OHC automatically creates a Zoom meeting via API and includes the generated join link in the customer confirmation page and email.
**Priority**: P1
**Estimated Scope**: Medium
