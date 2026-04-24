# Scout: Tool Integration Research [Q2]

## [Social Media Integration] Unified Customer Inbox (Instagram, Facebook, WhatsApp)

### Title
Implement Unified Customer Inbox for Meta Channels (Instagram, Facebook, WhatsApp)

### Problem Statement
As a small business owner like Maya (the baker), I get customer messages across Instagram DMs, Facebook Messenger, and WhatsApp. It's overwhelming to constantly switch between apps on my phone, and I sometimes miss custom order requests or take too long to reply. I need all my customer messages in one simple inbox so I (and my AI Customer Success agent) can reply quickly and never lose a sale.

### Research Report
- **Evaluated Tools**: Meta Graph API (Instagram Messaging, Facebook Messenger, WhatsApp Business API), Twilio Conversations, Zendesk Smooch.
- **Ease of Use**: Using the native Meta Graph API + Webhooks requires the user to go through an OAuth flow to connect their Facebook Business page. This is a standard flow many users are familiar with. Twilio/Smooch adds a layer of abstraction but increases cost.
- **Pricing**: Meta APIs are mostly free for standard messaging (WhatsApp has some per-conversation costs, but the first 1000 service conversations are often free or very cheap). Twilio charges per active user and message, which hurts our free tier.
- **Reputation**: Meta API is the industry standard. It can be complex to verify apps, but once approved, it's reliable.
- **Environment**: Works seamlessly in Cloud via webhooks. For Standalone mode, users would need to configure their own Meta Developer App or rely on an OHC cloud-relay service, making it more Cloud-oriented.
- **Recommendation**: Direct integration with Meta Graph API for lowest cost to users and direct control.

### Design Doc
- **Integration Flow**: The business owner clicks "Connect Instagram/Facebook" in the Operations or Customer Success tab. An OAuth popup guides them to grant OHC access to their pages.
- **Data Flow**: OHC registers a centralized webhook endpoint. When a customer messages the Instagram page, Meta posts to the webhook. OHC routes it to the specific tenant's inbox.
- **User Interface**: A new "Inbox" screen in the Flutter app. Shows a list of conversations (like a standard messaging app) with icons indicating the source (IG, FB, WA).
- **AI Integration**: The "Ambassador" (Customer Success Agent) monitors this inbox. If configured, it drafts a reply or auto-replies to FAQs (like "do you do vegan cakes?").

### Implementation Prompt
Create a "Unified Inbox" feature where users can connect their Meta accounts (Facebook, Instagram, WhatsApp). Customers' messages from these platforms should appear in a single scrolling list in the OHC app. The business owner must be able to read and reply to messages directly from OHC, and the replies must show up in the customer's native app. Acceptance criteria include a working OAuth connection flow, real-time message receipt, and successful reply delivery.

### Priority
P0

### Estimated Scope
Large

---

## [Calendar & Scheduling] Automated Booking & Calendar Sync

### Title
Google Calendar & Outlook Sync for Service Bookings

### Problem Statement
As a freelancer like Carlos (the handyman) or Leo (the tutor), I manage my availability on my personal Google Calendar. Right now, when someone books me, I have to manually copy it to my calendar, and I sometimes get double-booked. I need my OHC booking page to automatically read my calendar to block off busy times, and automatically add new bookings to my schedule.

### Research Report
- **Evaluated Tools**: Native Google Calendar API, Microsoft Graph API (Outlook), Nylas, Cronofy.
- **Ease of Use**: Nylas and Cronofy provide unified APIs for all calendar providers, drastically simplifying integration. However, they cost per connected account. Given OHC's target audience and free tier, native Google Calendar API covers ~80% of users at zero cost. Microsoft Graph covers another 15%.
- **Pricing**: Google Calendar API is free (subject to generous quotas). Nylas starts around $1-$2/account/month.
- **Reputation**: Google Calendar is the gold standard for small businesses.
- **Environment**: Works in both Cloud and Standalone (with local OAuth credentials).
- **Recommendation**: Build native integration with Google Calendar API first, as it covers the vast majority of our target personas (Leo, Carlos) for free.

### Design Doc
- **Integration Flow**: In the Operations department settings, user clicks "Sync Google Calendar". Standard Google OAuth consent screen asks for calendar read/write permissions.
- **Actions**:
  - *Read*: When displaying available booking slots on the public storefront, OHC queries the synced calendar for "busy" blocks and removes those slots.
  - *Write*: When a customer books a slot and pays the deposit, OHC creates a new Event on the Google Calendar containing the customer details and service description.
- **User Interface**: A simple toggle to connect/disconnect the calendar, and a dropdown to select which specific calendar (e.g., "Work" vs "Personal") to sync with.

### Implementation Prompt
Implement a Google Calendar synchronization feature. Users should be able to link their Google account via OAuth. Once linked, the public booking page must automatically hide time slots that overlap with the user's existing Google Calendar events. When a new booking is confirmed, it must automatically appear on the user's Google Calendar. Acceptance criteria: prevents double-booking, successfully writes events, handles timezone conversions correctly.

### Priority
P1

### Estimated Scope
Medium

---

## [Email Marketing] Automated Customer Newsletters

### Title
Simple Email Marketing via Resend

### Problem Statement
As a boutique owner like Priya, I want to let my past customers know when the new summer collection arrives. I don't understand how to use complex tools like Mailchimp, and they are too expensive. I need a "one-click" way to tell my AI to send a beautiful email to everyone who has bought from me before.

### Research Report
- **Evaluated Tools**: Resend, SendGrid, Mailchimp API.
- **Ease of Use**: Resend has the most developer-friendly API and excellent email deliverability without complex setup. Mailchimp is powerful but heavily branded and expensive for the end-user if integrated directly.
- **Pricing**: Resend offers 3,000 free emails/month, which is perfect for our free-tier small businesses. SendGrid is also good but has a steeper learning curve for domain authentication.
- **Reputation**: Resend is highly regarded for transactional and simple marketing emails.
- **Environment**: Cloud-only. Requires domain verification for best results, though we can use a shared sender domain (e.g., `mail.onehumancorp.com`) for non-custom-domain users.
- **Recommendation**: Integrate Resend for sending outbound marketing and transactional emails.

### Design Doc
- **Integration Flow**: Invisible to the user. OHC provisions a Resend sub-domain or uses the primary domain.
- **Actions**: "The Promoter" (Marketing Agent) drafts an email based on Priya's prompt ("Tell customers about summer collection"). Priya reviews the draft in the app. Upon approval, OHC uses the Resend API to blast it to all customers tagged in the CRM.
- **User Interface**: A "Campaigns" section under the Marketing department. Shows a simple text box to instruct the AI, a preview of the generated email (Glassmorphism card), and an "Approve & Send" button. Basic stats: "Sent to 150 people", "Opened by 45".

### Implementation Prompt
Build a simple Email Marketing sender using the Resend API. The Marketing Agent must be able to draft an HTML email template. The user must be able to review the draft and click "Send". The system will then iterate through the user's customer list and dispatch the emails. Acceptance criteria: emails are successfully delivered to test inboxes, open tracking (if available) is reported back to the UI, and the UI remains simple with zero technical jargon.

### Priority
P2

### Estimated Scope
Medium

---

## [Payment Processing] Global Alternative Payments

### Title
Mercado Pago & Razorpay Integration for Emerging Markets

### Problem Statement
As a seller in Latin America or India, my customers don't always have credit cards to use Stripe. I need them to be able to pay using local methods like Pix (Brazil) or UPI (India) so I don't lose sales.

### Research Report
- **Evaluated Tools**: Mercado Pago (LATAM), Razorpay (India), Alipay (China), PayPal (Global).
- **Ease of Use**: Mercado Pago is dominant in LATAM with easy APIs. Razorpay is the standard for India. Both offer drop-in UI components similar to Stripe Elements.
- **Pricing**: Standard localized processing fees (typically 2-3%).
- **Reputation**: Both are market leaders in their respective regions.
- **Environment**: Cloud and Standalone (standard API/Webhook architecture).
- **Recommendation**: Abstract the payment provider layer. Implement Mercado Pago for LATAM users and Razorpay for India users to truly support "anyone".

### Design Doc
- **Integration Flow**: Based on the business's country setting, OHC dynamically offers the correct payment provider connection in the Finance department. User clicks "Connect Mercado Pago" or "Connect Razorpay" and completes OAuth.
- **Actions**: Replaces Stripe Checkout with Mercado Pago Checkout / Razorpay Checkout for public storefront purchases. Webhooks update OHC order status to "Paid".
- **User Interface**: No change to the public storefront other than the payment modal matching the local provider. The Finance dashboard normalizes all transactions into a single "Revenue" view regardless of the underlying processor.

### Implementation Prompt
Abstract the checkout and payment webhook processing to support multiple providers, starting with Mercado Pago and Razorpay alongside the existing Stripe integration. When a user in a supported region connects their account, the storefront must route checkout sessions to the correct provider. Acceptance criteria: A successful mock checkout using Mercado Pago/Razorpay, webhooks successfully marking the order as paid, and revenue appearing in the Finance dashboard.

### Priority
P1

### Estimated Scope
Large

---

## [Shipping & Logistics] Automated Shipping Rates & Labels

### Title
EasyPost Integration for Real-Time Shipping & Labels

### Problem Statement
As an artist selling physical prints, I hate trying to guess how much shipping will cost to different states. I either overcharge the customer or lose money. I need the checkout page to automatically calculate the exact shipping cost, and I need a simple button to print the shipping label from my phone.

### Research Report
- **Evaluated Tools**: EasyPost, Shippo, ShipEngine.
- **Ease of Use**: EasyPost offers a very clean REST API that normalizes dozens of carriers (USPS, UPS, FedEx, DHL).
- **Pricing**: EasyPost charges pennies per label and gives access to discounted USPS rates (Commercial Plus), which is a huge benefit for small businesses.
- **Reputation**: Reliable, high uptime, developer-friendly.
- **Environment**: Cloud and Standalone.
- **Recommendation**: Integrate EasyPost to handle rate calculation at checkout and label generation in the Operations dashboard.

### Design Doc
- **Integration Flow**: User inputs their package dimensions and weight in the Product settings.
- **Actions**:
  - *Checkout*: OHC sends the cart items and destination address to EasyPost, retrieves real-time rates, and adds them to the customer's total.
  - *Fulfillment*: In the Operations tab, the user clicks "Buy Label". OHC buys the label via EasyPost and returns a PDF.
- **User Interface**: At checkout, a dynamic "Shipping" line item. In the manager app, an "Orders" screen with a prominent "Print Shipping Label" button for unfulfilled physical orders.

### Implementation Prompt
Implement shipping rate calculation and label generation using EasyPost. Products must have weight/dimension fields. The checkout flow must dynamically fetch shipping rates based on the buyer's address. The Operations dashboard must allow the business owner to generate and download a printable PDF shipping label for paid orders. Acceptance criteria: Accurate rate calculation at checkout, successful generation of a test label, and marking the order as "Shipped" with a tracking number.

### Priority
P2

### Estimated Scope
Medium

---

## [SMS & Notifications] Reliable Order Alerts via SMS

### Title
Twilio SMS Notifications for Low-Data/Low-English Users

### Problem Statement
As a food cart operator like Fatima, I don't always have a strong 4G/5G connection to receive push notifications reliably from an app. When someone pre-orders food, I need a simple, immediate SMS text message with the order details so I can start cooking right away.

### Research Report
- **Evaluated Tools**: Twilio, MessageBird, Vonage.
- **Ease of Use**: Twilio is the global standard for SMS APIs. Very simple to integrate.
- **Pricing**: Twilio costs ~$0.0079 per SMS in the US, slightly more internationally. This is cheap but must be metered or limited per tenant to avoid abuse.
- **Reputation**: Highly reliable, excellent global carrier coverage.
- **Environment**: Cloud mostly.
- **Recommendation**: Use Twilio to send immediate order alerts to the business owner, and optional shipping updates to the customer.

### Design Doc
- **Integration Flow**: In the Operations settings, the user enables "SMS Alerts for New Orders" and verifies their phone number.
- **Actions**: When a checkout webhook fires indicating a paid order, OHC enqueues an SMS job. Twilio sends a short text: "New Order! $15.40 - 2x Falafel Platter. Pickup in 15 mins."
- **User Interface**: Simple toggle in the app settings.

### Implementation Prompt
Integrate Twilio to send SMS notifications to the business owner when a new order is placed. Add a setting for the owner to opt-in and verify their mobile number. The message must be concise and contain the order total, items, and customer name. Acceptance criteria: A text message is successfully delivered to a verified number upon a completed checkout, and failures are logged but do not break the checkout flow.

### Priority
P1

### Estimated Scope
Small

---

## [Video Conferencing] Auto-Generated Lesson Links

### Title
Zoom API Integration for Online Consultations

### Problem Statement
As an online music tutor like Leo, when a student books a lesson, I have to manually create a Zoom meeting and email them the link. I sometimes forget, leading to confusion. I need the system to automatically generate a unique Zoom link and send it to the student when they book.

### Research Report
- **Evaluated Tools**: Zoom API, Google Meet API (via Google Calendar), Daily.co.
- **Ease of Use**: Google Meet is essentially free and included if we build the Google Calendar sync. Zoom API requires OAuth but is heavily requested.
- **Pricing**: Zoom requires the user to have a Zoom account (free tier works for 40 mins). Google Meet is completely free.
- **Reputation**: Zoom is ubiquitous for online learning.
- **Environment**: Cloud and Standalone.
- **Recommendation**: If Google Calendar integration (Category 2) is built, automatically attach Google Meet links. Add Zoom API as a secondary option for users who specifically prefer Zoom.

### Design Doc
- **Integration Flow**: User connects Zoom via OAuth in the Operations settings.
- **Actions**: When a service is booked that is marked as "Online/Video", OHC calls the Zoom API to create a meeting for the scheduled time. The `join_url` is saved to the database and included in the confirmation email/calendar invite sent to the customer.
- **User Interface**: A toggle on the Service creation page: "Location: [In-Person | Online Video]". If Online Video is selected, a dropdown lets them choose their provider (Zoom or Meet).

### Implementation Prompt
Implement Zoom API integration to automatically generate video meeting links for online service bookings. When a customer books an "Online Video" service, create a scheduled meeting via Zoom. Display the join link in the business owner's upcoming appointments view and the customer's confirmation page. Acceptance criteria: A valid Zoom link is generated upon booking, correctly scheduled for the booked time, and both parties can access the link.

### Priority
P3

### Estimated Scope
Medium
