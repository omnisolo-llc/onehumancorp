# Social Media Integration: Meta Business Suite API (WhatsApp, Instagram, Facebook) + TikTok Integration

## Problem Statement
Small business owners lose track of customer inquiries because messages are scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments. Constantly switching apps leads to missed sales, slow response times, and frustration. They need a single, simple unified inbox within OHC to see and reply to all customer messages.

## Research Report
- **Tool Evaluated**: Meta Graph API (Messenger, Instagram Direct, WhatsApp Business) + TikTok App for Business API. (Since Meta provides an official way to combine their own properties, and TikTok provides a separate but similar webhook-based API).
- **Competitors considered**: third-party aggregators like Sprout Social, Buffer, or Hootsuite. However, for a small business standalone tool, a direct integration using official APIs avoids expensive third-party aggregator subscriptions (which can cost $99+/month).
- **Ease of Use for Non-Technical Users**: The user simply clicks "Connect to Facebook/Instagram" and logs in via a standard OAuth popup. The technical complexity (webhooks, token refresh) is entirely handled by OHC.
- **Pricing**: Free for the business owner to receive/reply to organic messages (WhatsApp Business API has some usage-based pricing for initiated conversations, but first 1,000 service conversations are often free per month).
- **Reputation**: Official APIs provided by Meta and TikTok, offering the most stable and feature-rich integration path.

## Design Doc
- **Triggers**: Webhooks from Meta and TikTok send incoming messages directly to the OHC backend.
- **User Interface**: OHC will feature a "Unified Inbox" tab. Messages appear in a chat-like interface. Each thread is labeled with its source (e.g., an Instagram icon or WhatsApp icon).
- **Actions**: The business owner types a reply in the OHC Unified Inbox, hits send, and the OHC backend routes the reply back through the corresponding API to the customer's native app.
- **Cloud vs. Standalone**:
  - *Cloud*: Standard webhook routing and OAuth.
  - *Standalone*: The user provides their own API credentials in "Advanced Mode" or relies on a Cloud-relayed secure webhook proxy (since standalone local networks cannot easily receive external webhooks without a tunnel like ngrok).

## Implementation Prompt
Create a "Unified Inbox" feature that allows users to connect their Instagram, Facebook, and WhatsApp accounts via a simple login button. Once connected, all incoming messages from these platforms should appear in a single chronological feed. The user must be able to click on a message, read the history, and send a text reply that is delivered back to the customer on their original platform. The setup flow must be seamless and hide all technical webhook or token configuration from the user (Simple Mode).

## Priority
P0

## Estimated Scope
Large
# Calendar & Scheduling: Cal.com Integration

## Problem Statement
Small business owners (consultants, tutors, service providers) waste hours going back and forth via email or text to find a meeting time. They need a way to let clients book available slots automatically without double-booking their personal Google or Outlook calendars.

## Research Report
- **Tool Evaluated**: Cal.com
- **Competitors considered**: Calendly, Acuity Scheduling. Cal.com is chosen because it is open-core, API-first, and highly customizable, which aligns perfectly with OHC's cloud and standalone requirements.
- **Ease of Use for Non-Technical Users**: The business owner simply connects their Google/Outlook calendar. The tool generates a clean, public booking page link to share with clients.
- **Pricing**: Generous free tier for individuals. API access available.
- **Reputation**: Highly respected in the developer community for its modern API, reliability, and robust timezone handling.

## Design Doc
- **Triggers**: User configures their availability in OHC. OHC creates a Cal.com event type via API.
- **User Interface**: OHC displays a "Scheduling" dashboard where the owner can set working hours and connect external calendars. A "Share Booking Link" button is prominently displayed.
- **Actions**: When a client books a slot, a webhook notifies OHC to update the internal dashboard and customer CRM record.
- **Cloud vs. Standalone**: Works seamlessly in Cloud via API. In Standalone, can use the Cal.com API with user-provided keys in Advanced Mode or a self-hosted Cal.com instance.

## Implementation Prompt
Implement a scheduling feature powered by Cal.com. The user should be able to connect their Google or Outlook calendar, set their weekly available hours (e.g., Mon-Fri 9 AM - 5 PM), and generate a public booking link. When a client uses the link to book a meeting, the appointment should automatically appear in the OHC "Upcoming Appointments" view, and the time slot should be blocked off on the owner's personal calendar to prevent double-booking.

## Priority
P1

## Estimated Scope
Medium
# Email Marketing: Resend Integration

## Problem Statement
Small business owners want to send newsletters, promotions, or updates to their customer list but find traditional tools like Mailchimp too complex, bloated, or expensive. They need a simple way to blast a message to their contacts directly from within the tool where their contacts already live.

## Research Report
- **Tool Evaluated**: Resend
- **Competitors considered**: Mailchimp, SendGrid, Amazon SES. Resend is chosen for its exceptional developer experience, modern API, built-in React Email support for templates, and high deliverability focus. Mailchimp is too bloated; SES is too complex for simple setups.
- **Ease of Use for Non-Technical Users**: The user writes an email as if they are using Gmail. They select a group of customers and hit send. No complex drag-and-drop builders unless requested.
- **Pricing**: Very generous free tier (3,000 emails/month). Excellent for small businesses just starting out.
- **Reputation**: Rapidly growing modern email API with fantastic documentation and reliability.

## Design Doc
- **Triggers**: The business owner clicks "Send Campaign" from the CRM view in OHC.
- **User Interface**: A clean, distraction-free text editor. The user selects a list of contacts (e.g., "All past clients"). OHC handles the unsubscription links automatically.
- **Actions**: OHC compiles the email using a clean template, chunks the recipient list, and sends via the Resend API.
- **Cloud vs. Standalone**: Cloud can use a centralized Resend account with domain verification per tenant. Standalone requires the user to input their own Resend API key in Advanced Mode.

## Implementation Prompt
Create an "Email Broadcast" feature. The user should be able to select a group of contacts from their OHC customer list, write a rich-text message, and send it to everyone at once. The system must automatically append an unsubscribe link to the bottom of every email and track open rates. The interface should feel as simple as sending a regular email, shielding the user from complex marketing terminology.

## Priority
P2

## Estimated Scope
Medium
# Payment Processing: Mercado Pago Integration

## Problem Statement
While Stripe is excellent for the US and Europe, small businesses in LATAM heavily rely on local payment methods (PIX in Brazil, local credit cards, cash payments via OXXO/Boleto). A business owner in LATAM using OHC cannot easily collect payments if only Stripe is supported.

## Research Report
- **Tool Evaluated**: Mercado Pago API
- **Competitors considered**: dLocal, EBANX. Mercado Pago is chosen because it is the most recognized and trusted consumer brand in LATAM, offering easy onboarding for small businesses without requiring enterprise-level legal entities.
- **Ease of Use for Non-Technical Users**: Business owners connect their existing Mercado Pago account. They can generate payment links with one click to send via WhatsApp.
- **Pricing**: Standard localized transaction fees. No monthly fixed costs.
- **Reputation**: The undisputed leader in LATAM payments.

## Design Doc
- **Triggers**: User creates an invoice or a "Payment Link" in OHC.
- **User Interface**: A button saying "Generate Mercado Pago Link". The link is copied to the clipboard.
- **Actions**: OHC calls the Mercado Pago Checkout Pro API to generate a secure payment URL. Webhooks update the invoice status in OHC to "Paid" when the transaction clears.
- **Cloud vs. Standalone**: Requires API keys. Cloud can manage OAuth. Standalone requires the user to provide their Access Token in Advanced Mode.

## Implementation Prompt
Build a Mercado Pago payment integration tailored for LATAM users. The business owner should be able to connect their Mercado Pago account and generate a simple payment link for a specific amount. When the customer pays the link, the corresponding invoice in OHC should automatically be marked as "Paid". Ensure the setup process clearly explains how to find and input the necessary credentials if using Standalone mode.

## Priority
P1

## Estimated Scope
Medium
# Shipping & Logistics: EasyPost Integration

## Problem Statement
Small e-commerce or craft business owners waste significant time manually copying customer addresses into carrier websites (USPS, FedEx, UPS) to check rates and buy shipping labels. They need to automatically generate shipping labels and tracking numbers directly from their OHC order dashboard.

## Research Report
- **Tool Evaluated**: EasyPost API
- **Competitors considered**: Shippo, ShipStation. EasyPost offers a highly developer-friendly API, transparent pricing, and aggregates hundreds of carriers behind a single integration, making it ideal for a software platform like OHC.
- **Ease of Use for Non-Technical Users**: The user clicks "Buy Shipping Label", selects the box size, and OHC provides the cheapest rate. One more click prints the label.
- **Pricing**: Free tier up to 120,000 shipments/year (only paying carrier postage costs).
- **Reputation**: Highly reliable, standard choice for modern e-commerce platforms.

## Design Doc
- **Triggers**: User views an unfulfilled order in OHC and clicks "Create Label".
- **User Interface**: A modal shows the customer's address (pre-filled). The user selects a saved package size. The system fetches live rates. The user selects a rate and clicks "Purchase".
- **Actions**: OHC calls the EasyPost API to buy the postage, downloads the PDF label for printing, and automatically emails the tracking number to the customer.
- **Cloud vs. Standalone**: EasyPost requires an API key. Standalone users will input their key in Advanced Mode.

## Implementation Prompt
Implement a shipping label generation feature using EasyPost. From an order view, the business owner should be able to instantly see shipping rates for different carriers based on the customer's address. With one click, they should be able to purchase the label, download it as a PDF for printing, and automatically save the tracking number to the order record.

## Priority
P2

## Estimated Scope
Large
# SMS & Notifications: Twilio Integration

## Problem Statement
For users with low English proficiency or in regions where email is rarely checked (like Fatima from the personas), SMS is the only reliable way to ensure a customer sees an appointment reminder, order update, or payment link.

## Research Report
- **Tool Evaluated**: Twilio Programmable SMS
- **Competitors considered**: Plivo, MessageBird, AWS SNS. Twilio is the industry standard with the best global carrier routing, comprehensive documentation, and robust compliance tools (like A2P 10DLC registration).
- **Ease of Use for Non-Technical Users**: They don't interact with Twilio directly. They just toggle "Send SMS Reminder" in OHC.
- **Pricing**: Pay-as-you-go per message (fractions of a cent in the US, varies globally).
- **Reputation**: The gold standard for programmatic SMS.

## Design Doc
- **Triggers**: Scheduled cron jobs (for reminders) or status changes (for order updates) in OHC.
- **User Interface**: A simple toggle in the settings: "Enable SMS Notifications for Customers".
- **Actions**: OHC formats a brief text message and sends it via the Twilio API to the customer's phone number.
- **Cloud vs. Standalone**: Cloud requires OHC to handle the billing and pass costs to the tenant, or use Twilio Connect. Standalone requires the user to input their Twilio Account SID and Auth Token.

## Implementation Prompt
Add automatic SMS notifications for critical customer events, such as appointment reminders (sent 24 hours before) and payment link deliveries. The system should allow the business owner to toggle these notifications on or off globally. Ensure the messages are concise and include clear opt-out instructions (e.g., "Reply STOP to cancel"). Provide a setup flow for Standalone users to input their Twilio credentials.

## Priority
P1

## Estimated Scope
Medium
# Video Conferencing: Zoom Integration

## Problem Statement
Small business owners who offer online services (tutoring, consulting, telehealth) currently have to manually create a Zoom meeting, copy the link, and email it to the client for every single appointment, leading to mistakes and lost links.

## Research Report
- **Tool Evaluated**: Zoom API
- **Competitors considered**: Google Meet, Microsoft Teams. Zoom is chosen due to its ubiquitous adoption across all demographics; almost every client already has it installed.
- **Ease of Use for Non-Technical Users**: The user connects their Zoom account once. Meeting links magically appear on calendar invites.
- **Pricing**: Free tier API access for standard meetings.
- **Reputation**: The default standard for video conferencing.

## Design Doc
- **Triggers**: A new appointment is scheduled in OHC and marked as "Online/Virtual".
- **User Interface**: When viewing an appointment, both the owner and the customer see a prominent "Join Video Call" button.
- **Actions**: OHC calls the Zoom API to create a meeting for the scheduled time, retrieves the join URL, and saves it to the appointment record.
- **Cloud vs. Standalone**: Requires OAuth app approval from Zoom. Cloud handles OAuth natively.

## Implementation Prompt
Build a Zoom integration that automatically generates a unique video meeting link whenever an "Online Appointment" is booked. The business owner must be able to authenticate their Zoom account with a single click. The generated Zoom link should be automatically included in the confirmation email sent to the customer and displayed prominently on the OHC appointment dashboard.

## Priority
P2

## Estimated Scope
Medium
