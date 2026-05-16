# Tool Integration Research Report

## [Social Media] Integrate ManyChat for Unified Inbox
**Title**: Integrate ManyChat for Unified Inbox
**Problem Statement**: Small business owners struggle to manage customer inquiries across Instagram, Facebook, and WhatsApp. A unified inbox would save time and prevent lost sales.
**Research Report**: ManyChat is a top-rated tool for non-technical users. It offers a free tier and starts at $15/mo for Pro. It has reliable webhooks for receiving messages instantly, handles complex OAuth flows seamlessly, and requires no coding from the user. Message parsing quality is generally high, though complex multimedia messages sometimes require fallback handling. It works well in both Cloud and Standalone environments.
**Design Doc**: ManyChat connects via OAuth and routes incoming messages to OHC via webhooks. Business owners see all DMs in a single OHC inbox. OHC automatically parses the platform and sender to construct a unified conversation thread.
**Implementation Prompt**: The user should be able to click "Connect ManyChat" and authenticate. Once connected, a new "Unified Inbox" tab should appear showing messages from all connected social platforms with reply capabilities.
**Priority**: P1
**Estimated Scope**: Medium
**Key advantages**: Very easy to set up; supports all major Meta platforms.
**Risks**: Changes to Meta's API policies frequently break integrations; requires maintaining persistent webhook endpoints.

## [Calendar] Integrate Calendly for Easy Scheduling
**Title**: Integrate Calendly for Easy Scheduling
**Problem Statement**: Small business owners lose time playing email ping-pong to schedule appointments. They need an automated way for clients to book available slots.
**Research Report**: Calendly is the industry standard for scheduling. It integrates seamlessly with Google Calendar and Outlook, handling complex calendar conflict resolutions natively. It resolves time zones automatically based on the user's browser, which is critical for remote services. It offers a free basic tier and starts at $10/mo for professional features, including customizable booking pages. It supports Cloud and Standalone modes.
**Design Doc**: Calendly embeds directly on the OHC dashboard or a public-facing website. A webhook notifies OHC of new bookings, syncing the event data to the business owner's OHC dashboard.
**Implementation Prompt**: Users should be able to paste their Calendly link to embed the booking widget. A dedicated "Appointments" tab in OHC should display upcoming bookings synced from Calendly in real-time.
**Priority**: P0
**Estimated Scope**: Small
**Key advantages**: Intuitive interface; robust timezone and conflict management.
**Risks**: Users might forget to connect their primary calendars, leading to double-booking.

## [Email Marketing] Integrate Mailchimp for Customer Campaigns
**Title**: Integrate Mailchimp for Customer Campaigns
**Problem Statement**: Keeping in touch with past customers is hard. Owners need an easy way to send newsletters or promotions without manually managing email lists or worrying about spam folders.
**Research Report**: Mailchimp is incredibly popular with small businesses due to its easy drag-and-drop template builder and solid free tier (up to 500 contacts). Paid plans start at $13/mo. It provides excellent spam compliance (automatic unsubscribe handling) and detailed open-rate analytics. List management is straightforward via their API. Fully compatible with Cloud and Standalone environments.
**Design Doc**: OHC syncs its customer contact list directly to a specific Mailchimp audience list. Business owners can trigger a sync manually or set it to run automatically when new customers are added.
**Implementation Prompt**: Add a "Sync to Mailchimp" button in the Customers view. The integration should automatically push new OHC contacts to the selected Mailchimp audience and pull back unsubscribe statuses.
**Priority**: P1
**Estimated Scope**: Medium
**Key advantages**: High deliverability rates; excellent drag-and-drop builder for non-technical users.
**Risks**: Pricing scales aggressively as the contact list grows; strict anti-spam policies can block users unexpectedly.

## [Payment Processing] Integrate Mercado Pago for LATAM Payments
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Small businesses in Latin America need a reliable way to accept local payment methods (like Pix in Brazil or OXXO in Mexico) online without high failure rates or complex setups.
**Research Report**: Mercado Pago is the dominant payment provider in LATAM. Pricing varies by country but is generally around 3.99% + a fixed fee. It supports local currency settlement rapidly and handles market-specific failure rates better than international competitors. Its checkout flow is optimized for local buying habits. Cloud and Standalone compatible.
**Design Doc**: Mercado Pago Checkout is used to handle payments securely, redirecting users to a localized payment page. OHC records the transaction success/failure via webhooks and updates the order status.
**Implementation Prompt**: The checkout page should redirect to a Mercado Pago hosted checkout. Successful payments should trigger a webhook to mark OHC invoices as "Paid" automatically.
**Priority**: P0
**Estimated Scope**: Medium
**Key advantages**: Deep penetration in LATAM; supports local payment methods essential for conversion.
**Risks**: API documentation can be inconsistent; dispute resolution processes can be lengthy for merchants.

## [Shipping] Integrate ShipStation for Logistics
**Title**: Integrate ShipStation for Logistics
**Problem Statement**: Calculating shipping rates and printing labels manually across multiple carriers is tedious, error-prone, and lacks real-time tracking visibility for customers.
**Research Report**: ShipStation aggregates real-time rates from major global carriers and provides discounted labels. It costs $9.99/mo to start. It is well-regarded for its carrier coverage, including solid international support, and robust API reliability. Works in Cloud and Standalone environments.
**Design Doc**: OHC sends order details and dimensions to ShipStation. ShipStation calculates rates, handles label creation, and returns the tracking number and carrier details to OHC.
**Implementation Prompt**: Once an order is ready to ship, users should see a "Generate Label via ShipStation" button. Clicking it should fetch the label, deduct postage, and update the order with a clickable tracking link.
**Priority**: P2
**Estimated Scope**: Large
**Key advantages**: Consolidates multiple carriers into one interface; provides discounted postage rates.
**Risks**: Complex setup for international customs forms; API rate limits can be restrictive during peak seasons.

## [SMS] Integrate Twilio for SMS Notifications
**Title**: Integrate Twilio for SMS Notifications
**Problem Statement**: Not all customers check emails promptly. Business owners need a reliable way to send SMS alerts globally (e.g., appointment reminders), especially for clients with lower English proficiency.
**Research Report**: Twilio offers robust global SMS delivery across almost all carriers with pay-as-you-go pricing (around $0.0079 per message in the US, varying globally). It has high delivery reliability and automatically handles opt-out compliance (STOP messages). The integration itself is technical, but to the business owner, it will function as a simple toggle. Cloud and Standalone supported.
**Design Doc**: OHC triggers SMS messages via Twilio's API based on state changes (like a booking confirmation or shipping update).
**Implementation Prompt**: The user enters their Twilio credentials in the settings. OHC should then automatically send an SMS to the customer when a specific event occurs, using pre-defined localized templates.
**Priority**: P1
**Estimated Scope**: Medium
**Key advantages**: Unmatched global reach and reliability; handles regulatory compliance automatically.
**Risks**: Variable international pricing can lead to unexpected costs; requires A2P 10DLC registration in the US which can be complex for small businesses.

## [Video Conferencing] Integrate Zoom for Online Consultations
**Title**: Integrate Zoom for Online Consultations
**Problem Statement**: Service-based businesses (e.g., tutors, consultants) need automatic video links generated when a client books a virtual session, rather than creating them manually.
**Research Report**: Zoom is ubiquitous and provides a frictionless join experience for clients. It has a free tier (40-min limit) and Pro starts at $15.99/mo. The API allows for rapid link generation speed and creates high-quality calendar invites automatically. It functions perfectly in Cloud and Standalone contexts.
**Design Doc**: OHC calls the Zoom API to generate a meeting link instantly when a virtual appointment is booked, saving the link to the appointment record and appending it to client notifications.
**Implementation Prompt**: When a user creates a new virtual appointment, OHC should automatically generate a unique Zoom link via the API and include it in the confirmation details shown on the dashboard and in client emails.
**Priority**: P2
**Estimated Scope**: Medium
**Key advantages**: Universal client familiarity; highly reliable video quality.
**Risks**: Free tier time limits may cut off client consultations unexpectedly; requires users to authorize OHC via the Zoom App Marketplace.
