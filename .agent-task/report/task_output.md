# Scout: Tool Integration Research [Q2]

## [Social Media] Unified Inbox: ManyChat Integration
**Title**: Integrate ManyChat for Unified Social Media Inbox
**Problem Statement**: Maya the baker gets custom cake inquiries on Instagram DMs, Facebook, and WhatsApp. She misses messages because she has to check 3 different apps while baking. She needs all customer messages to show up in one place so her Customer Success AI can draft replies to them automatically.
**Research Report**: ManyChat is a leading platform for automating social media messaging, supporting Instagram, Facebook Messenger, and WhatsApp. It handles the complex Meta OAuth flows and provides reliable webhooks for incoming messages. For non-technical users, OHC can abstract ManyChat's complexity by provisioning a sub-account or using their API to sync messages to OHC's unified inbox. Pricing starts at $15/mo for Pro, which OHC can either bundle or pass-through. It is highly reliable but requires careful handling of Meta's 24-hour messaging window rules. Works in Cloud mode; Standalone mode might need a proxy or direct API credentials.
**Design Doc**:
- A "Social Media Connect" UI where the user clicks "Connect Instagram/Facebook/WhatsApp".
- Upon connection, incoming messages from these channels trigger webhooks to OHC.
- OHC routes these messages to the Customer Success AI ("The Ambassador") to draft a reply.
- The user sees the message and the drafted reply in their OHC inbox, and can click "Send" to push it back through the integration.
**Implementation Prompt**: Build a unified inbox connection flow where a user can authenticate their Instagram and WhatsApp accounts. Incoming messages should appear in the OHC customer inbox, and the user must be able to reply directly from OHC, with the message being delivered back to the customer on their original platform. The Customer Success AI should automatically draft suggested replies.
**Priority**: P0
**Estimated Scope**: Large

---

## [Calendar & Scheduling] Nylas Integration for Cross-Provider Calendar Sync
**Title**: Integrate Nylas for Universal Calendar Sync
**Problem Statement**: Leo the music tutor uses Google Calendar, but some of his students use Outlook. When they book a lesson, he sometimes gets double-booked if his OHC calendar doesn't instantly sync with his personal calendar. He needs a foolproof way to block off times he's busy, regardless of what calendar app he uses.
**Research Report**: Nylas provides a universal API for calendar, email, and contacts, covering Google Workspace, Microsoft Exchange, Office 365, and iCloud. This eliminates the need to build individual OAuth flows and sync logic for each provider. Nylas handles timezone complexities, recurring events, and conflict resolution out of the box. Pricing is usage-based (around $0.99-$1.50 per connected account per month). It provides a seamless experience for non-technical users: they just log in to their email provider once. Works well in Cloud mode; Standalone might require the user to bring their own Nylas keys or use a free alternative like direct Google/Microsoft integrations.
**Design Doc**:
- A "Sync My Calendar" button in the Operations dashboard.
- Opens the Nylas hosted authentication widget.
- Once connected, OHC subscribes to calendar webhooks to get real-time updates on the user's availability.
- When a customer views the user's booking page, OHC dynamically queries availability, filtering out times blocked on the synced calendar.
- When a booking is made, OHC pushes the event to the user's calendar.
**Implementation Prompt**: Create a calendar integration feature that allows a business owner to link their Google or Outlook calendar. Ensure that the business's public booking page automatically hides time slots where the owner has existing events on their personal calendar. Ensure new bookings made via OHC are added to their personal calendar immediately.
**Priority**: P0
**Estimated Scope**: Medium

---

## [Email Marketing] Resend Integration for Automated Campaigns
**Title**: Integrate Resend for Customer Email Campaigns
**Problem Statement**: Priya the boutique owner wants to email her 500 past customers when she gets new summer dresses in stock. She finds Mailchimp too complicated and expensive. She just wants to click "Email all past customers" and have a beautiful email sent.
**Research Report**: Resend is a modern, developer-friendly email sending platform that focuses on high deliverability and simple templates (using React Email). For OHC, we can use Resend's API to send beautiful, AI-generated email campaigns. It handles bounce tracking, spam compliance (unsubscribe links), and open/click analytics reliably. Pricing is very affordable ($20/mo for 50,000 emails), making it easy to absorb into OHC's premium tier. The user never sees "Resend" — they just see OHC's Marketing AI. Works natively in Cloud mode.
**Design Doc**:
- Marketing AI ("The Promoter") generates an email campaign draft with text and images based on the user's new product.
- The user reviews the draft in the OHC UI and clicks "Send to all customers".
- OHC compiles the email into a responsive HTML template and sends it via the Resend API to the customer list.
- OHC listens to Resend webhooks to track opens and clicks, displaying a simple analytics summary ("50 people opened this email") in the Marketing dashboard.
**Implementation Prompt**: Build an email campaign sender where the Marketing AI drafts a promotional email. Allow the user to review it and send it to all their tagged customers in one click. Display a simple report showing how many people received and opened the email.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Payment Processing] Mercado Pago Integration for LATAM
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Carlos the handyman is based in Mexico and many of his clients prefer to pay via local bank transfers (SPEI) or cash at OXXO. Stripe doesn't support these local payment methods effectively. He needs a way to accept deposits online that his local customers trust.
**Research Report**: Mercado Pago is the leading payment gateway in Latin America, supporting local cards, bank transfers (like SPEI in Mexico or PIX in Brazil), and cash payments (like OXXO). It has a robust API similar to Stripe's. Integrating Mercado Pago opens up OHC to the massive LATAM small business market. Settlement speed is typically fast, and fees vary by country but are competitive locally. It supports complex webhook flows for asynchronous payments (like paying cash at a store). Works in both Cloud and Standalone modes.
**Design Doc**:
- In the "Finance & Payments" settings, add a "Connect Mercado Pago" option for users in supported countries.
- When a customer checks out, the OHC checkout flow displays Mercado Pago as a payment option, redirecting to their secure checkout or using a transparent checkout UI.
- OHC handles asynchronous webhooks (e.g., when an OXXO payment is completed a day later) to update the order status from "Pending" to "Paid" and notify the user.
**Implementation Prompt**: Add Mercado Pago as an alternative payment provider to Stripe. Allow a business owner to connect their account and let customers pay using local methods. Ensure that asynchronous payments (like cash deposits) correctly update the order status in OHC once cleared.
**Priority**: P1
**Estimated Scope**: Large

---

## [Shipping & Logistics] Shippo Integration for Real-Time Rates & Labels
**Title**: Integrate Shippo for Shipping Labels and Tracking
**Problem Statement**: Maya needs to ship a box of cookies to another state. She currently goes to the post office, waits in line, and types the tracking number manually into an email. She needs OHC to calculate the shipping cost at checkout and let her print a label at home.
**Research Report**: Shippo offers a multi-carrier shipping API that aggregates rates from USPS, UPS, FedEx, and international carriers. It allows platforms to generate shipping labels and track packages. It is very startup-friendly with a pay-as-you-go model (cents per label). By integrating Shippo, OHC can provide real-time shipping quotes during customer checkout and a 1-click "Print Label" button for the business owner. Works perfectly in Cloud mode.
**Design Doc**:
- During checkout, OHC sends cart weight/dimensions to Shippo to get live shipping rates for the customer to choose from.
- In the Operations dashboard, the user views an order and clicks "Buy Shipping Label".
- OHC purchases the label via Shippo and displays a PDF for the user to print.
- OHC retrieves the tracking number from Shippo and triggers the Customer Success AI to email the customer with a tracking link.
**Implementation Prompt**: Implement a shipping module where users can buy and print shipping labels directly from an order page. The system must automatically fetch tracking numbers and send a shipping confirmation email to the customer. Ensure real-time shipping costs can be calculated during checkout based on product weight.
**Priority**: P1
**Estimated Scope**: Large

---

## [SMS & Notifications] Twilio Integration for Critical SMS Alerts
**Title**: Integrate Twilio for SMS Order Notifications
**Problem Statement**: Fatima the food cart operator doesn't check her email often and her data connection is slow. When someone places a pre-order for pickup, she needs an immediate text message on her phone so she can start cooking.
**Research Report**: Twilio is the industry standard for programmatic SMS. It offers global carrier coverage and high deliverability. For non-English speakers or users with limited tech literacy, an SMS notification is the most reliable way to alert them of a new order or a canceled booking. Pricing is low (less than $0.01 per message in the US). OHC can use Twilio to send critical alerts to the business owner and order updates to customers. Opt-out compliance (STOP replies) is handled natively by Twilio.
**Design Doc**:
- A notification settings page where the user enters their phone number and verifies it via a code.
- When a high-priority event occurs (e.g., "New Order Paid"), the Operations AI triggers a Twilio API call to send a brief SMS to the owner.
- Similarly, customers can opt-in at checkout to receive an SMS when their order is ready for pickup.
**Implementation Prompt**: Build an SMS notification system. Allow the business owner to add their phone number and receive instant text messages when a new order is placed or a booking is made. Provide a toggle for the owner to enable SMS updates for their customers (e.g., "Your food is ready for pickup").
**Priority**: P1
**Estimated Scope**: Medium

---

## [Video Conferencing] Zoom Integration for Auto-Generated Meeting Links
**Title**: Integrate Zoom for Automated Online Lesson Links
**Problem Statement**: Leo the music tutor spends 10 minutes before every online lesson creating a Zoom link and emailing it to his student. He wants the link to be created automatically the moment the student pays for the booking.
**Research Report**: Zoom's API allows platforms to automatically schedule meetings and generate join links. It is the most recognized video conferencing tool for consumers. By integrating Zoom, OHC can seamlessly support digital service providers (tutors, consultants). The integration requires standard OAuth2. Zoom offers a generous free tier for users, making it accessible.
**Design Doc**:
- User connects their Zoom account in the "Integrations" tab.
- When creating a service, the user toggles "Location: Online (Zoom)".
- When a customer books and pays for this service, the Operations AI calls the Zoom API to schedule a meeting for that specific time.
- The generated join URL is saved to the booking record.
- The Customer Success AI includes the unique Zoom link in the confirmation email and calendar invite sent to both the student and the tutor.
**Implementation Prompt**: Add a Zoom integration that allows business owners to authenticate their Zoom accounts. When a customer books an online service, automatically generate a unique Zoom meeting link for that specific time slot and include it in the booking confirmation details for both parties.
**Priority**: P2
**Estimated Scope**: Medium
