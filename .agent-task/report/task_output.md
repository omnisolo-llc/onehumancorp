# Scout: Tool Integration Research Q4

## 1. Social Media Integration
**Title**: Integrate WhatsApp Business API for Unified Inbox
**Problem Statement**: Small business owners (like local bakers or plumbers) receive critical customer orders and questions via WhatsApp. Managing multiple apps is chaotic, leading to missed messages, slow replies, and lost revenue. They need one simple inbox.
**Research Report**:
- **Tool**: WhatsApp Cloud API (Meta)
- **Problem it solves for which persona**: Allows service-based small businesses (plumbers, bakers) to view and respond to customer inquiries from WhatsApp directly inside their OHC dashboard.
- **Ease of Use**: Very easy for the non-technical owner. They just link their Meta account once. After that, messages appear like regular chats in the OHC interface.
- **Pricing**: The first 1000 service conversations per month are free. Afterwards, conversation-based pricing varies by region (e.g., $0.015/message in NA). Highly affordable for SMBs.
- **Key Advantages**: Massive user base globally; high open rates for messages; native rich media support (images of broken pipes or cake designs).
- **Integration Risks**: Meta's review process can be strict. The 24-hour customer service window requires careful handling of outbound replies.
- **Environment**: Works well in Cloud. Standalone mode might require the business owner to register their own Meta App ID or use a proxy service.
**Design Doc**:
- **Trigger**: Customer sends a message to the business's WhatsApp number.
- **Action**: Webhook receives the message, parses text/media, and routes it to the OHC unified inbox as a new chat thread.
- **User Interface**: Business owner sees a "WhatsApp" icon next to messages in their OHC inbox. Replying sends the message back via the API.
**Implementation Prompt**: Implement a webhook endpoint to receive incoming WhatsApp messages via Meta's Cloud API and surface them in the OHC unified inbox. Enable the business owner to reply from the OHC UI, ensuring their message is sent back to the customer's WhatsApp. Handle text and basic image media types.
**Priority**: P0
**Estimated Scope**: Large

## 2. Calendar & Scheduling
**Title**: Integrate Google Calendar for Auto-Scheduling and Sync
**Problem Statement**: Small business owners spend hours doing "email ping-pong" to find a time to meet with clients. When they get booked, they often forget to manually update their personal calendar, leading to embarrassing double-bookings.
**Research Report**:
- **Tool**: Google Calendar API
- **Problem it solves for which persona**: Helps consultants, tutors, and salon owners automate booking and prevent double-booking.
- **Ease of Use**: Business owner clicks "Connect Google Calendar", completes OAuth, and it's done.
- **Pricing**: Free (included in Google Workspace / free Gmail accounts).
- **Key Advantages**: Ubiquitous usage; reliable conflict resolution; free.
- **Integration Risks**: Managing OAuth refresh tokens can be brittle; timezone complexities between client and owner.
- **Environment**: Works well in Cloud via standard OAuth. Standalone mode requires users to supply their own Google Cloud Project credentials.
**Design Doc**:
- **Trigger**: OHC booking widget checks available slots; client selects a time.
- **Action**: OHC creates an event on the owner's Google Calendar and sends a confirmation email to the client.
- **User Interface**: Owner sees a "Connect Calendar" button. Once connected, they can generate a booking link to share with clients.
**Implementation Prompt**: Create an OAuth flow for Google Calendar. Implement two-way sync: read free/busy times from the owner's calendar to power a public booking page, and create new calendar events when a client books a slot. Include basic timezone handling.
**Priority**: P1
**Estimated Scope**: Medium

## 3. Email Marketing
**Title**: Integrate Mailchimp for Customer Newsletters
**Problem Statement**: Local shops capture customer emails during checkout but have no easy way to send promotions or newsletters without exporting/importing CSVs to another tool, which they rarely have time to do.
**Research Report**:
- **Tool**: Mailchimp Marketing API
- **Problem it solves for which persona**: Allows retail shops and service businesses to send bulk promotional emails easily.
- **Ease of Use**: Well-known brand. Owner links their account. OHC automatically syncs new customer emails.
- **Pricing**: Free tier up to 500 contacts / 1000 sends per month. Then starts at $13/mo.
- **Key Advantages**: Industry standard, excellent template builder, reliable deliverability.
- **Integration Risks**: Strict spam compliance rules; API rate limits on free tiers.
- **Environment**: Cloud and Standalone supported (API key based).
**Design Doc**:
- **Trigger**: New customer added to OHC CRM.
- **Action**: OHC automatically adds the customer to a designated Mailchimp audience (list).
- **User Interface**: Owner sees a toggle in settings: "Sync new customers to Mailchimp".
**Implementation Prompt**: Build a background synchronization worker that pushes new customer contacts from the OHC CRM to a designated Mailchimp Audience using their API. Support OAuth or API key connection.
**Priority**: P2
**Estimated Scope**: Small

## 4. Payment Processing
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Stripe is not available or preferred in many Latin American countries. Businesses in these regions need a localized payment processor that supports local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- **Tool**: Mercado Pago API
- **Problem it solves for which persona**: Allows LATAM-based small businesses to accept online payments securely using methods their customers trust.
- **Ease of Use**: Familiar to the target market. Owner connects their Mercado Pago account via OAuth.
- **Pricing**: Varies by country, typically ~3-5% + flat fee. No monthly fixed costs.
- **Key Advantages**: Massive market share in LATAM; supports local cash-based and instant transfer methods.
- **Integration Risks**: Complex webhook verification; documentation can be fragmented.
- **Environment**: Cloud and Standalone supported.
**Design Doc**:
- **Trigger**: Customer clicks "Pay" on an OHC invoice.
- **Action**: OHC redirects to a Mercado Pago Checkout Pro link or renders a Web Tokenized Checkout.
- **User Interface**: Owner selects "Mercado Pago" as their payment provider in settings. Customers see localized payment options.
**Implementation Prompt**: Implement an alternative payment provider module using Mercado Pago. Generate Checkout Pro preference links for invoices and handle incoming webhooks to mark OHC invoices as paid.
**Priority**: P1
**Estimated Scope**: Medium

## 5. Shipping & Logistics
**Title**: Integrate Shippo for Multi-Carrier Label Generation
**Problem Statement**: E-commerce micro-businesses waste hours standing in line at the post office and guessing shipping costs. They need to print discounted shipping labels from home automatically when an order is placed.
**Research Report**:
- **Tool**: Shippo API
- **Problem it solves for which persona**: Helps independent makers, crafters, and boutique shops fulfill physical orders quickly from their home or small warehouse.
- **Ease of Use**: Owner connects Shippo, enters box dimensions, and clicks "Print Label".
- **Pricing**: Pay-as-you-go ($0.05 per label) or $10/mo for the pro tier. Excellent USPS/UPS discounts.
- **Key Advantages**: Aggregates many carriers (USPS, UPS, FedEx, DHL) behind one clean API.
- **Integration Risks**: Handling edge cases like rural addresses, customs forms for international shipping.
- **Environment**: Cloud and Standalone supported.
**Design Doc**:
- **Trigger**: Owner clicks "Fulfill Order" in OHC.
- **Action**: OHC fetches rates from Shippo, creates a transaction, and downloads the PDF label.
- **User Interface**: Order details page has a "Buy Shipping Label" button, showing rates from different carriers.
**Implementation Prompt**: Build an integration with the Shippo API to fetch shipping rates for a given order, purchase a shipping label, and retrieve the PDF label and tracking number for the business owner to print.
**Priority**: P2
**Estimated Scope**: Medium

## 6. SMS & Notifications
**Title**: Integrate Twilio for SMS Order Updates
**Problem Statement**: Customers often miss email notifications. Small business owners (like food delivery or repair services) need to send instant text message updates (e.g., "Your repair is done") to reduce no-shows and incoming "is it ready yet?" calls.
**Research Report**:
- **Tool**: Twilio Programmable SMS API
- **Problem it solves for which persona**: Helps service and local retail businesses keep customers informed instantly.
- **Ease of Use**: Owner enables SMS in OHC; OHC provisions a number via Twilio behind the scenes.
- **Pricing**: ~$0.0079 per message in the US, varies globally. Often requires A2P 10DLC registration fees in the US.
- **Key Advantages**: Most robust telecom API, global reach.
- **Integration Risks**: Strict regulatory compliance (A2P 10DLC in the US) can make onboarding small businesses complicated; toll fraud risks.
- **Environment**: Cloud (OHC manages Twilio account) and Standalone (User brings their own Twilio SID/Auth Token).
**Design Doc**:
- **Trigger**: Order status changes to "Ready for Pickup".
- **Action**: OHC sends an SMS via Twilio to the customer's phone number.
- **User Interface**: Owner can toggle "Send SMS Updates" on order statuses.
**Implementation Prompt**: Integrate Twilio's SMS API to send automated, customizable text messages to customers when their order status changes. Implement phone number validation and handle opt-out (STOP) webhooks securely.
**Priority**: P1
**Estimated Scope**: Medium

## 7. Video Conferencing
**Title**: Integrate Zoom for Automated Virtual Meetings
**Problem Statement**: Tutors, therapists, and consultants offer virtual sessions but struggle with manually creating Zoom links, sending them to clients, and remembering which link goes with which meeting.
**Research Report**:
- **Tool**: Zoom API (Server-to-Server OAuth or standard OAuth)
- **Problem it solves for which persona**: Automates the creation of virtual meeting rooms for service professionals offering online sessions.
- **Ease of Use**: Owner connects Zoom once. Every online booking automatically gets a unique Zoom link.
- **Pricing**: Free tier allows 40-min meetings. Pro tier ($15/mo) required for longer or more advanced features.
- **Key Advantages**: The most recognized video conferencing brand; clients already have it installed.
- **Integration Risks**: Zoom OAuth approval process is notoriously stringent for public apps; managing token lifecycles.
- **Environment**: Works well in Cloud via OAuth. Standalone mode might require Server-to-Server OAuth credentials.
**Design Doc**:
- **Trigger**: Client books a "Virtual Consultation" service via OHC scheduling.
- **Action**: OHC calls Zoom API to create a meeting and attaches the `join_url` to the calendar event and confirmation email.
- **User Interface**: Service settings have a "Location" dropdown where the owner can select "Zoom Meeting".
**Implementation Prompt**: Implement a Zoom OAuth integration. When a service marked as "Virtual via Zoom" is booked, automatically generate a Zoom Meeting link via the API and include it in the customer confirmation emails and owner's calendar event.
**Priority**: P2
**Estimated Scope**: Medium
