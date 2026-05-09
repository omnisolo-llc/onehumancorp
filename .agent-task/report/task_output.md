# OHC Tool Integration Research Q4

## [Social Media] Unify Customer Messaging with ManyChat
**Title**: Integrate ManyChat for Unified Social Media Messaging
**Problem Statement**: As a small business owner, checking Instagram DMs, Facebook Messenger, and WhatsApp separately is overwhelming and causes missed sales opportunities. I need all customer messages in one place so I can respond quickly without juggling apps.
**Research Report**: ManyChat is a leading platform for omnichannel messaging. It consolidates Instagram, Facebook, and WhatsApp into a single inbox. It has a robust free tier and starts around $15/mo for pro features. It is incredibly user-friendly for non-technical users and has an excellent reputation. The OAuth flow is straightforward. Works in both Cloud (via webhooks) and Standalone (polling/local tunneling or cloud-assisted proxy) modes.
**Design Doc**: The business owner clicks "Connect Social Media" in the OHC dashboard, which redirects them to ManyChat's authorization page. Once approved, OHC listens for incoming messages and displays them in a new "Unified Inbox" tab in the OHC UI. When the owner replies from the OHC inbox, the message is sent back to the customer on their original platform.
**Implementation Prompt**: Build a "Unified Inbox" interface where a business owner can authorize their ManyChat account. Display incoming messages from all connected social platforms in a single thread list. Allow the user to type and send replies directly from this inbox. Acceptance criteria include successful authorization, receiving messages, and sending replies.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar & Scheduling] Automated Booking with Calendly
**Title**: Connect Calendly for Automated Client Scheduling
**Problem Statement**: I spend too much time emailing clients back and forth to find a meeting time. I want to share a simple link where they can book a time that automatically syncs with my availability, so I never get double-booked.
**Research Report**: Calendly is the industry standard for scheduling. It handles timezone conversions natively, syncs with Google Calendar and Outlook seamlessly, and prevents conflicts. It offers a generous free tier (one event type) and affordable paid plans (starting at $10/mo). Non-technical users find it very intuitive. Works well in both Cloud and Standalone environments.
**Design Doc**: The user navigates to the "Scheduling" tab and connects their Calendly account via OAuth. OHC imports their active event types and generates a shareable booking link widget for the OHC storefront. When a client books a slot, OHC automatically records the appointment in the customer's profile within OHC.
**Implementation Prompt**: Create an integration flow for Calendly. Add a section in the OHC settings to connect the account. Display the user's booking link prominently. When a booking occurs, automatically log the event details (time, client name, service) in the OHC customer management dashboard. Acceptance criteria include successful connection and automatic recording of new bookings.
**Priority**: P1
**Estimated Scope**: Medium

## [Email Marketing] Simplify Customer Campaigns with Mailchimp
**Title**: Sync Customer Contacts with Mailchimp for Marketing
**Problem Statement**: I want to send newsletters and promotional offers to my customers, but manually exporting contacts from my store and importing them into an email tool is tedious and error-prone.
**Research Report**: Mailchimp remains a top choice for small businesses due to its drag-and-drop builder and strong analytics. Pricing is accessible (free up to 500 contacts, then scales). It has strong spam compliance mechanisms. The integration is straightforward for non-technical users. It operates efficiently in Cloud mode, and Standalone mode can easily sync lists out via API.
**Design Doc**: The business owner connects their Mailchimp account in the OHC marketing settings. OHC sets up an automatic background sync that pushes new customer contacts from OHC directly into a designated Mailchimp audience list. The OHC dashboard will show the date of the last sync and the total number of synced contacts.
**Implementation Prompt**: Implement an OAuth connection to Mailchimp. Create a background sync process that ensures any new customer added to OHC is automatically added to a selected Mailchimp list. Add a status indicator in the UI showing sync health. Acceptance criteria include successful authentication and verified syncing of a test contact.
**Priority**: P1
**Estimated Scope**: Medium

## [Payment Processing] Expand Global Sales with Mercado Pago
**Title**: Enable Mercado Pago Checkout for LATAM Markets
**Problem Statement**: Many of my customers in Latin America don't use traditional credit cards or prefer local payment methods. I need to accept payments through Mercado Pago to avoid losing sales in these regions.
**Research Report**: Mercado Pago is essential for LATAM, supporting local cards, cash payments (like OXXO in Mexico or Boleto in Brazil), and installments. Settlement speeds are competitive, and pricing varies by country but is standard for the region. It is trusted and widely used. Integration is via a standard redirect or checkout widget, suitable for Cloud and Standalone (with appropriate webhook handling).
**Design Doc**: In the "Payments" settings, the user can toggle Mercado Pago as a checkout option. They enter their public and access credentials. On the storefront checkout page, customers see Mercado Pago as an option, which opens a secure payment modal or redirects them to complete the purchase. OHC marks the order as paid once the payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a supported payment gateway. Build a configuration screen for the business owner to enter their credentials. Update the checkout flow to present Mercado Pago to buyers and handle the payment completion status to update the order. Acceptance criteria include successful configuration and completion of a test transaction.
**Priority**: P2
**Estimated Scope**: Large

## [Shipping & Logistics] Automated Shipping Labels with Shippo
**Title**: Streamline Order Fulfillment with Shippo
**Problem Statement**: Calculating shipping rates manually and buying labels at the post office is incredibly time-consuming. I need a way to see cheap shipping rates and print labels directly from my online orders.
**Research Report**: Shippo aggregates multiple carriers (USPS, UPS, DHL, FedEx) and provides discounted rates without volume minimums. It is pay-as-you-go (5¢ per label) or $10/mo for the pro tier. It handles international customs forms well. Very friendly for small businesses. Works smoothly in Cloud, and Standalone environments can fetch labels securely.
**Design Doc**: After a customer places an order, the business owner opens the order details in OHC and clicks "Create Shipping Label." OHC sends the package weight/dimensions and addresses to Shippo, retrieves the best rates, and allows the owner to purchase and download the PDF label directly within the OHC interface. The tracking number is then automatically emailed to the customer.
**Implementation Prompt**: Integrate Shippo for label generation. Add a "Fulfillment" section to order details where users can input box sizes and get live rates. Allow them to buy the label and download the PDF. Automatically save the tracking number and update the order status. Acceptance criteria include rate fetching, label generation, and tracking number assignment.
**Priority**: P1
**Estimated Scope**: Large

## [SMS & Notifications] Reliable Text Alerts with Twilio
**Title**: Connect Twilio for Automated SMS Customer Alerts
**Problem Statement**: My customers don't always check their email, but they always check their texts. I need to send automatic text messages for order confirmations and appointment reminders to reduce no-shows and keep them informed.
**Research Report**: Twilio is the most reliable global SMS infrastructure. While slightly more technical to set up initially, a simplified OHC integration can abstract the complexity. Pricing is very low per message (fractions of a cent). It strictly handles opt-out compliance (STOP messages). Ideal for Cloud; Standalone instances can use it via standard outbound API calls.
**Design Doc**: The business owner configures their Twilio Account SID and Auth Token in the "Notifications" tab. OHC provides a set of simple toggle switches: "Send SMS on new order" and "Send SMS 24h before appointment." OHC handles formatting the message with the customer's details and sending it out.
**Implementation Prompt**: Build a Twilio integration settings page for credentials. Create automated triggers for key events (order placed, appointment soon) that send a templated SMS to the customer's phone number. Acceptance criteria include successful credential validation and successful delivery of a test SMS.
**Priority**: P2
**Estimated Scope**: Medium

## [Video Conferencing] One-Click Online Meetings with Zoom
**Title**: Auto-Generate Zoom Links for Online Consultations
**Problem Statement**: When a client books an online lesson or consultation, I currently have to manually create a Zoom link and email it to them. I want a Zoom link generated automatically and attached to their appointment so neither of us has to search for it.
**Research Report**: Zoom is universally recognized and used by non-technical customers. The API allows for instant meeting creation. The free tier has a 40-minute limit, which works for many small consultations, while Pro is ~$15/mo. Integration is standard OAuth. Cloud mode handles this natively; Standalone works with proper redirect URIs.
**Design Doc**: The business owner authenticates their Zoom account in the OHC "Services" setup. When creating a service, they can check a box saying "This is an online meeting." When a customer books that service, OHC instantly generates a unique Zoom meeting link and includes it in the confirmation screen and calendar invites.
**Implementation Prompt**: Implement an OAuth flow for Zoom. Add a toggle on service creation for "Online Meeting." When an appointment is booked for this service, call the Zoom integration to create a meeting, and display the join URL in the appointment details and customer-facing confirmations. Acceptance criteria include successful authentication and generation of a valid Zoom link upon booking.
**Priority**: P1
**Estimated Scope**: Medium
