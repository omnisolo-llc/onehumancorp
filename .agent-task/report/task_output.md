# 🔍 Scout: Tool Integration Research Report

## 1. Social Media Integration: ManyChat

**Title**: Integrate ManyChat for Unified Social Media Inbox
**Problem Statement**: Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Missing a message often means losing a sale, but logging into four different apps constantly is overwhelming for a non-technical user.
**Research Report**: ManyChat is a leading conversational marketing tool. It connects easily to Meta's suite (FB, IG, WhatsApp) and provides a visual builder for automations. For non-technical users, its interface is highly intuitive. Pricing starts at a very accessible tier (often free for up to 1,000 contacts, then $15/mo). It is highly reputable and has robust webhook support, which makes it reliable for catching messages in real time.
**Design Doc**:
- **Trigger**: User connects their ManyChat account via OAuth in the OHC dashboard.
- **Action**: Inbound messages from connected social channels are routed into the OHC unified inbox.
- **User Experience**: The business owner sees all social messages in one OHC view, and their replies are routed back through ManyChat to the customer's original platform.
**Implementation Prompt**: Build a unified inbox interface in OHC where users can read and reply to messages from Instagram, Facebook, and WhatsApp. The user must be able to click "Connect Social Media", authorize the integration, and immediately see new messages appear in their OHC dashboard. Replies sent from OHC must reach the customer on the platform they used.
**Priority**: P0
**Estimated Scope**: Large
**Environment**: Works in both Cloud and Standalone modes (assuming webhook exposure).

## 2. Calendar & Scheduling: Calendly

**Title**: Implement Calendly Sync for Automated Booking
**Problem Statement**: Scheduling appointments, consultations, or classes involves endless back-and-forth emails. Small business owners often double-book themselves because they manually copy appointments into their personal calendars.
**Research Report**: Calendly is the industry standard for scheduling. It seamlessly syncs with Google Calendar and Outlook to prevent double bookings. The user interface for the person booking is foolproof, and the setup for the business owner is simple. There is a robust free tier, with premium features starting at $10/mo.
**Design Doc**:
- **Trigger**: Business owner pastes their Calendly API key or authenticates via OAuth.
- **Action**: OHC automatically syncs Calendly events into the OHC internal calendar view and triggers reminder flows.
- **User Experience**: The business owner views their upcoming appointments directly inside OHC without needing to check Calendly, and client details are automatically added to the customer list.
**Implementation Prompt**: Create a "Schedule" tab in OHC that displays upcoming appointments fetched from Calendly. Provide a simple connection flow for the business owner to link their Calendly account. New bookings should automatically generate a customer profile in OHC.
**Priority**: P1
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.

## 3. Email Marketing: Mailchimp

**Title**: Mailchimp Integration for Seamless Customer Campaigns
**Problem Statement**: Business owners collect emails but don't know how to engage them. Exporting lists from their customer database to an email tool is tedious and often forgotten, leading to missed marketing opportunities.
**Research Report**: Mailchimp is highly recognizable and tailored for small businesses. It offers intuitive drag-and-drop templates and straightforward list management. It provides a generous free tier for new businesses. Open rate analytics are easy to understand.
**Design Doc**:
- **Trigger**: A new customer is added to OHC.
- **Action**: OHC automatically pushes the customer's email and name to a designated Mailchimp audience list.
- **User Experience**: The business owner sees a "Sync to Mailchimp" toggle. When active, their OHC customer list is always up-to-date in Mailchimp, ready for newsletters.
**Implementation Prompt**: Add a Mailchimp integration setting where the user can log in and select an audience list. Implement a one-way sync that automatically adds new OHC contacts to the selected Mailchimp list. Show a basic summary of the synced list size in the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.

## 4. Payment Processing: Mercado Pago

**Title**: Mercado Pago Integration for LATAM Payments
**Problem Statement**: While Stripe is great, it doesn't support many local payment methods essential in Latin America (like Pix in Brazil or OXXO in Mexico). Business owners in these regions lose sales because customers cannot pay with their preferred local methods.
**Research Report**: Mercado Pago dominates the LATAM market. It supports a vast array of local payment options, settles relatively quickly, and provides a simple checkout experience. Fees vary by country but are competitive for the region. The brand is highly trusted by consumers in LATAM.
**Design Doc**:
- **Trigger**: Customer proceeds to checkout for an invoice or product.
- **Action**: OHC generates a Mercado Pago checkout link or renders their checkout widget.
- **User Experience**: The business owner connects their Mercado Pago account with one click. Their LATAM customers see localized payment options at checkout, and the invoice is automatically marked "Paid" in OHC upon success.
**Implementation Prompt**: Integrate Mercado Pago as an alternative payment provider to Stripe. Build a connection flow for the business owner. When a customer views an OHC invoice, they should be able to pay via Mercado Pago, and the OHC system should listen for the payment success webhook to update the invoice status.
**Priority**: P1
**Estimated Scope**: Large
**Environment**: Works in Cloud; Standalone requires webhook forwarding setup.

## 5. Shipping & Logistics: Shippo

**Title**: Shippo Integration for Automated Label Generation
**Problem Statement**: Fulfilling physical orders is a massive time sink. Business owners have to manually copy customer addresses into carrier websites, calculate rates, and paste tracking numbers back into emails.
**Research Report**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and offers discounted rates. Its API is highly reliable, and its dashboard is friendly for small businesses. It has a pay-as-you-go model which is perfect for low-volume shippers.
**Design Doc**:
- **Trigger**: An order is marked as "Ready to Ship" in OHC.
- **Action**: OHC requests a shipping label from Shippo using the customer's address and package dimensions.
- **User Experience**: The business owner clicks "Print Label" on an order. OHC provides a PDF of the label and automatically emails the tracking number to the customer.
**Implementation Prompt**: Create a shipping fulfillment flow for orders. Allow the business owner to connect Shippo, input default box sizes, and generate a shipping label PDF directly from the order details page. Automatically extract the tracking link and display it to the user.
**Priority**: P2
**Estimated Scope**: Large
**Environment**: Works in both Cloud and Standalone modes.

## 6. SMS & Notifications: Twilio

**Title**: Twilio Integration for Reliable SMS Notifications
**Problem Statement**: Email open rates are low, and for non-English speaking or less tech-savvy users, SMS is the only reliable way to confirm appointments, send payment links, or provide updates.
**Research Report**: Twilio is the global leader in SMS infrastructure. While its raw API is developer-focused, OHC can abstract this away. Twilio has excellent global carrier coverage and handles opt-out compliance well. Pricing is very low per message.
**Design Doc**:
- **Trigger**: An appointment is booked, or an invoice is due.
- **Action**: OHC sends a templated SMS via Twilio to the customer.
- **User Experience**: The business owner buys a phone number through OHC (powered by Twilio) or connects an existing Twilio account. They toggle "Send SMS Reminders" on, and OHC handles the rest transparently.
**Implementation Prompt**: Build an SMS notification toggle for appointment reminders and invoice links. Integrate Twilio in the backend to deliver these messages. Allow the business owner to customize the text message template using simple placeholders like `[Customer Name]` and `[Time]`.
**Priority**: P0
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.

## 7. Video Conferencing: Zoom

**Title**: Zoom Integration for Auto-Generated Meeting Links
**Problem Statement**: Virtual service providers (tutors, consultants) waste time manually creating Zoom links for every booking and emailing them to clients. This often leads to wrong links being sent or clients losing the link.
**Research Report**: Zoom is universally recognized. Its API allows for instantaneous meeting creation. The free tier covers most 1-on-1 needs for 40 minutes, and Pro accounts are affordable. The join experience is frictionless for attendees.
**Design Doc**:
- **Trigger**: A virtual appointment is booked via OHC.
- **Action**: OHC calls the Zoom API to generate a unique meeting link.
- **User Experience**: The business owner connects Zoom. When a client books a service marked as "Virtual", OHC instantly generates a Zoom link, adds it to both parties' calendar invites, and displays it on the OHC appointment detail page.
**Implementation Prompt**: Add a Zoom integration option in the scheduling settings. When a new virtual booking occurs, automatically generate a Zoom meeting URL and embed it in the confirmation screen and notification emails. Display a "Join Meeting" button in the OHC dashboard for the business owner.
**Priority**: P1
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.
