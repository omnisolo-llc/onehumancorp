# OHC Tool Integration Research Report

## [Social Media] Unified Unified Inbox Integration
**Title**: Integrate Meta & WhatsApp for Unified Unified Inbox
**Problem Statement**: Small business owners like Maya (the home baker) receive orders and inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). Jumping between apps to reply is exhausting, and messages get lost. They need a single, simple inbox within OHC to read and reply to all customer messages, with AI drafting responses for common questions like "do you do vegan cakes?".
**Research Report**:
- **Tool Evaluated**: Meta Graph API (Instagram Messaging, Messenger, WhatsApp Business API).
- **Pros**: Direct access to the largest platforms. Essential for mobile-first businesses.
- **Cons**: High OAuth complexity. Strict 24-hour reply window rules for automated messages.
- **Ease of Use**: High for end-users once connected; challenging for initial Meta App verification.
- **Pricing**: Free for standard messaging APIs; WhatsApp Business has conversation-based pricing.
- **Cloud vs Standalone**: Cloud-friendly (webhooks required). Standalone requires local tunneling or polling, which is difficult for Meta APIs.
**Design Doc**:
- The "Customer Success" department monitors the unified inbox.
- A new section in the OHC app called "Inbox" aggregates messages from Instagram, Facebook, and WhatsApp.
- Incoming messages trigger a notification to the owner's phone.
- The AI agent automatically drafts a reply based on previous answers, pricing, and FAQs. The owner taps "Send" or edits the draft.
- Requires OHC to register a central Meta App and handle per-tenant OAuth flow.
**Implementation Prompt**: Create a unified inbox view in the Flutter app that displays messages from Instagram and WhatsApp in a single feed. Implement the OAuth flow for users to connect their Meta accounts. Ensure the AI 'Ambassador' agent can read incoming messages and generate draft replies visible below the customer's message.
**Priority**: P0
**Estimated Scope**: Large

## [Calendar] Automated Booking & Calendar Sync
**Title**: Integrate Google Calendar for Seamless Booking
**Problem Statement**: Service providers like Leo (the music tutor) and Carlos (the handyman) struggle with double-booking and manual scheduling. They need a way to let customers book available time slots automatically, without back-and-forth emails, while ensuring it syncs with their personal Google Calendar so they never miss an appointment.
**Research Report**:
- **Tool Evaluated**: Google Calendar API & Nylas (as an aggregator).
- **Pros**: Google Calendar is universally used. Nylas simplifies multi-provider (Outlook, Apple) integration.
- **Cons**: Handling timezones and recurring events is notoriously difficult. Nylas adds extra cost per connected account.
- **Ease of Use**: Very high for users; they just log in with Google.
- **Pricing**: Google API is virtually free for basic usage. Nylas is ~$1/account/month.
- **Cloud vs Standalone**: Works in both; OAuth flows can redirect to local or cloud callback URLs.
**Design Doc**:
- The "Operations" department manages the booking calendar.
- The business owner connects their Google Calendar via a simple 1-click OAuth button.
- The OHC storefront displays available time slots based on the owner's free/busy status and defined working hours.
- When a customer books, a calendar event is automatically created in the owner's calendar, and a confirmation email is sent.
- Timezone conversions are handled automatically based on the customer's browser and the owner's profile settings.
**Implementation Prompt**: Implement an OAuth integration with Google Calendar. Create a booking widget for the storefront that reads free/busy times and allows customers to select an available slot. Upon selection, generate a calendar event and update the OHC database with the booking details.
**Priority**: P0
**Estimated Scope**: Medium

## [Email] Automated Customer Engagement & Marketing
**Title**: Integrate Resend for Transactional and Marketing Emails
**Problem Statement**: Boutique owners like Priya need to notify customers when an order ships, or send out a blast when new inventory arrives. Managing a separate tool like Mailchimp is too complex and expensive. They need a built-in, dead-simple way to send beautiful emails directly from OHC.
**Research Report**:
- **Tool Evaluated**: Resend (API-first email platform).
- **Pros**: Developer-friendly, excellent deliverability, built with modern React Email templates (which can be ported/generated).
- **Cons**: Primarily designed for transactional emails; full marketing campaign management (unsubscribe lists, drip campaigns) requires custom building on top.
- **Ease of Use**: Invisible to the user. They just type a message, and OHC handles the styling and sending.
- **Pricing**: Generous free tier (3,000 emails/month). Very cheap beyond that.
- **Cloud vs Standalone**: Cloud-centric. Standalone would require users to bring their own SMTP credentials or API key.
**Design Doc**:
- The "Marketing & Advertising" and "Customer Success" departments handle emails.
- Standard transactional templates (Order Confirmed, Shipped, Refunded) are automatically branded with the tenant's colors and logo.
- A "Broadcast" feature allows the owner to type a simple message, and the AI converts it into a beautiful HTML email sent to all past customers.
- OHC handles the unsubscribe links and bounce tracking in the background.
**Implementation Prompt**: Integrate the Resend API to handle outbound emails. Build a simple email campaign interface where users can type a message and select an audience (e.g., "All Customers", "Recent Buyers"). The system must automatically wrap the message in a beautifully styled, branded HTML template and manage the sending queue.
**Priority**: P1
**Estimated Scope**: Medium

## [Payments] Alternative Local Payment Processing
**Title**: Integrate Mercado Pago & Paytm for Global Reach
**Problem Statement**: Not all business owners are in regions well-served by Stripe. A food cart operator in LATAM or India needs to accept payments via local, trusted methods. They need an alternative payment processor that works out-of-the-box in their country without complex setup.
**Research Report**:
- **Tool Evaluated**: Mercado Pago API (LATAM) & Paytm API (India).
- **Pros**: Dominant market share in their respective regions. Familiar to local customers.
- **Cons**: Fragmentation. Each API has entirely different webhooks, settlement processes, and testing environments.
- **Ease of Use**: High for local users familiar with the platforms.
- **Pricing**: Varies by region, typically 2-3% per transaction.
- **Cloud vs Standalone**: Works in both; requires secure webhook endpoints for payment confirmation.
**Design Doc**:
- The "Finance & Payments" department manages multiple gateways.
- Introduce a "Payment Provider" abstraction layer in the backend, allowing the frontend checkout to dynamically render either Stripe, Mercado Pago, or Paytm based on the tenant's region.
- Webhooks from all providers are normalized into a standard OHC `PaymentEvent` before processing.
**Implementation Prompt**: Create a generic payment interface in the backend. Implement a Mercado Pago adapter that generates a checkout preference and handles the corresponding payment webhooks. Update the frontend checkout flow to support redirecting to Mercado Pago for users in supported LATAM countries.
**Priority**: P2
**Estimated Scope**: Large

## [Shipping] Automated Shipping Rates & Labels
**Title**: Integrate Shippo for Seamless Logistics
**Problem Statement**: Creators selling physical goods struggle with calculating shipping costs and manually writing addresses on boxes. They need a system that automatically charges the customer the correct shipping amount and lets the owner print a ready-to-go shipping label with one tap.
**Research Report**:
- **Tool Evaluated**: Shippo API.
- **Pros**: Aggregates dozens of carriers (USPS, FedEx, UPS, DHL). Simplifies international customs forms.
- **Cons**: Label generation requires accurate package weights and dimensions, which users often forget to input.
- **Ease of Use**: Very high; eliminates trips to the post office.
- **Pricing**: Pay-as-you-go per label (cents) + postage costs.
- **Cloud vs Standalone**: Cloud-centric.
**Design Doc**:
- The "Operations" department handles fulfillment.
- During checkout, the storefront pings Shippo with the destination address and cart weights to get real-time shipping rates.
- In the OHC dashboard, an "Orders" view shows paid items. A "Print Label" button automatically purchases postage from Shippo and opens a PDF label for printing.
- The tracking number is automatically emailed to the customer via the Customer Success agent.
**Implementation Prompt**: Integrate the Shippo API to fetch real-time shipping rates during the checkout flow. Add a "Buy & Print Label" button to the order management screen that purchases a USPS/carrier label and returns the PDF. Ensure the tracking number is automatically saved to the order and sent to the customer.
**Priority**: P1
**Estimated Scope**: Medium

## [SMS] Reliable Global SMS Notifications
**Title**: Integrate Twilio for Instant SMS Alerts
**Problem Statement**: Users like Fatima (food cart) rely on their phones but may have poor data connections or don't want push notifications. Customers need instant SMS updates when their food is ready for pickup. SMS is critical for urgent, real-time communication.
**Research Report**:
- **Tool Evaluated**: Twilio SMS API.
- **Pros**: Global reach, extremely reliable, supports WhatsApp messaging as well.
- **Cons**: Strict A2P 10DLC compliance rules in the US make onboarding difficult for small businesses. SMS costs can add up quickly.
- **Ease of Use**: Invisible to the user. They just toggle "Send SMS alerts".
- **Pricing**: ~$0.0079 per message in the US, higher internationally.
- **Cloud vs Standalone**: Works in both.
**Design Doc**:
- The "Customer Success" department handles outbound notifications.
- When an order state changes (e.g., "Ready for Pickup"), an SMS is dispatched via Twilio to the customer's phone number.
- OHC handles the generic A2P compliance centrally to shield the small business owner from the paperwork.
- A daily digest SMS can be sent to the business owner summarizing sales.
**Implementation Prompt**: Integrate the Twilio API to send SMS notifications. Implement a trigger in the order fulfillment flow that sends a "Your order is ready" text message when the status changes. Add a setting in the user dashboard to enable/disable SMS notifications for both the owner and their customers.
**Priority**: P2
**Estimated Scope**: Medium

## [Video] Auto-Generated Video Conferencing Links
**Title**: Integrate Zoom/Google Meet for Online Services
**Problem Statement**: Tutors and consultants need an effortless way to host online sessions. Manually creating a Zoom link and emailing it to a client for every booking is tedious and error-prone. They need OHC to automatically generate a meeting room and include it in the calendar invite.
**Research Report**:
- **Tool Evaluated**: Google Meet (via Google Calendar API) & Zoom API.
- **Pros**: Meet is already bundled with Google Calendar. Zoom is widely expected by clients.
- **Cons**: Zoom requires a separate OAuth flow and app approval.
- **Ease of Use**: High. The owner simply selects "Online Meeting" for their service type.
- **Pricing**: Free for Google Meet; Zoom API requires a Pro account.
- **Cloud vs Standalone**: Works in both.
**Design Doc**:
- The "Operations" department handles booking logistics.
- When configuring a service, the owner can set the location to "Online via Video Call".
- If the owner uses Google Calendar sync, OHC automatically appends a Google Meet link to the event payload.
- Both the owner and the customer receive the joining link in their confirmation emails and calendar events.
**Implementation Prompt**: Enhance the existing booking system to support an "Online Meeting" location type. When a booking is finalized, automatically generate a video conferencing link (using Google Meet via Calendar API) and embed it in the confirmation email and calendar event sent to the customer and the business owner.
**Priority**: P1
**Estimated Scope**: Small