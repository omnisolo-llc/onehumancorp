# OHC Tool Integration Research Report

## 1. Social Media Integration

### Title
Unified Social Inbox: Connect Instagram, Facebook, WhatsApp, and TikTok

### Problem Statement
Managing customer inquiries across multiple platforms (Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments) is overwhelming. I keep missing messages or replying too late because I have to check five different apps. I need one single place to see and reply to all my customer messages.

### Research Report
- **Evaluated Tools:** Meta Graph API (Instagram/Facebook/WhatsApp), TikTok for Business API.
- **Ease of Use:** High for users. Connecting accounts is typically a one-click OAuth flow ("Log in with Facebook/TikTok").
- **Pricing:** Meta APIs are generally free for messaging within a 24-hour window, but WhatsApp Business API has per-conversation pricing (starts roughly at $0.01 to $0.08 depending on region/type). TikTok API is free.
- **Reputation:** Meta is the industry standard but has strict policies and approval processes. TikTok is rapidly growing for commerce.
- **Environment Support:** Works in Cloud mode (webhooks to OHC servers). In Standalone mode, requires a proxy or local tunneling (like ngrok) to receive webhooks securely, or aggressive polling if APIs allow.

### Design Doc
- **Triggers:** A customer sends a DM or comments on a linked social profile.
- **Actions:** The message is fetched and categorized. If the customer is recognized, it links to their profile.
- **What the User Sees:** A unified "Inbox" tab. An "Add Channel" button lets the user log into Facebook/TikTok. Incoming messages appear in a single chat interface. Replies typed in OHC are sent back out to the original platform.

### Implementation Prompt
Implement a unified inbox feature where the business owner can connect their Facebook, Instagram, WhatsApp, and TikTok accounts. They should be able to view all incoming messages in a single chronological feed and reply directly from OHC.
**Acceptance Criteria:**
- User can connect Meta and TikTok accounts via simple login buttons.
- Incoming messages from any connected platform appear in the OHC Inbox within 30 seconds.
- User can reply from OHC, and the message is delivered to the customer on the platform they used.

### Priority
P0

### Estimated Scope
Large

---

## 2. Calendar & Scheduling

### Title
Automated Booking System: Google Calendar and Outlook Integration

### Problem Statement
I lose hours every week playing email ping-pong with clients to find a meeting time. I need a link I can send them where they can just pick a time that I'm free, and have it automatically block off my calendar and send us both an invite.

### Research Report
- **Evaluated Tools:** Google Calendar API, Microsoft Graph API (Outlook), Cronofy (Unified API).
- **Ease of Use:** Users just click "Sign in with Google" or "Sign in with Microsoft". Customers get a simple scheduling page.
- **Pricing:** Direct APIs are free (subject to API quotas). Aggregators like Cronofy charge per connected account (~$1-2/month).
- **Reputation:** Google and Microsoft are universally trusted.
- **Environment Support:** Works perfectly in both Cloud and Standalone modes, as the OHC backend/local server can directly authorize and sync via API.

### Design Doc
- **Triggers:** User connects their calendar. Customer selects a time slot on the OHC booking page.
- **Actions:** OHC reads free/busy times from the connected calendar to generate available slots. When booked, an event is inserted into the calendar.
- **What the User Sees:** A "Calendar" settings page to connect their Google/Outlook account. A public, shareable "Booking Page" link they can give to clients. The booking page shows only their available times in the client's local timezone.

### Implementation Prompt
Build a "Booking Page" feature that syncs with Google Calendar and Outlook. The business owner should be able to connect their calendar and get a public link. Clients visiting the link will only see times when the owner is free.
**Acceptance Criteria:**
- Owner can link Google or Outlook calendars.
- Public booking page accurately reflects free/busy availability.
- When a client books, an event is automatically added to the owner's calendar and a confirmation email is sent to both.
- Timezones are automatically handled for both parties.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing

### Title
Customer Email Campaigns: Send Newsletters and Promotions

### Problem Statement
I have a list of past customers, but I don't have an easy way to send them a nice-looking email about my upcoming sale. Buying a separate email tool like Mailchimp is expensive and too complicated. I want to just write an email and send it to everyone who bought something last month.

### Research Report
- **Evaluated Tools:** SendGrid, Amazon SES, Resend.
- **Ease of Use:** Resend is exceptionally developer-friendly and easy to manage. SendGrid is powerful but can be complex. For the business owner, they won't see the underlying tool—they'll just see an OHC text editor.
- **Pricing:** Amazon SES is very cheap ($0.10 per 1000 emails). Resend has a generous free tier and starts at $20/mo. SendGrid starts at $19/mo.
- **Reputation:** All have high deliverability rates if domain authentication is configured properly.
- **Environment Support:** Works in Cloud mode. In Standalone mode, the user would need to provide their own API keys (Advanced Mode) or use a central OHC relay service.

### Design Doc
- **Triggers:** User selects a group of customers (e.g., "All Customers") and clicks "Send Campaign".
- **Actions:** The system compiles the list, merges basic tags (like "First Name"), and dispatches the emails via the provider.
- **What the User Sees:** A "Marketing" tab with a list of past campaigns. A simple "New Campaign" button opens a clean, distraction-free editor (like Notion) where they can write the email, add images, pick their audience, and hit "Send".

### Implementation Prompt
Create an Email Campaign tool that allows business owners to send bulk emails to their customer segments directly from OHC. Keep the editor simple and focused on text and basic images.
**Acceptance Criteria:**
- User can draft an email with a subject line and body.
- User can select which customer segment receives the email.
- The system accurately delivers the emails without exposing complex SMTP or technical settings to the user (unless in Advanced Mode).
- Basic stats (Open Rate) are shown after sending.

### Priority
P1

### Estimated Scope
Medium

---

## 4. Payment Processing

### Title
Global Checkout: Alternative Payment Methods (Mercado Pago, Paytm, Alipay)

### Problem Statement
Stripe is great, but it's not popular or supported where my customers live. In my country, everyone pays with local digital wallets or bank transfers. If I only offer credit cards, I lose half of my sales.

### Research Report
- **Evaluated Tools:** Mercado Pago (LATAM), Paytm (India), Alipay/WeChat Pay (China/Asia), Razorpay (India).
- **Ease of Use:** Usually requires the business owner to submit some KYC documents to the provider, but connecting to OHC should be via simple API keys or OAuth.
- **Pricing:** Varies heavily by region, typically 1.5% to 3% plus a fixed fee per transaction.
- **Reputation:** Mercado Pago dominates LATAM. Razorpay/Paytm dominate India. Alipay is essential for Chinese consumers.
- **Environment Support:** Cloud mode supports webhooks easily. Standalone mode requires webhook relay or polling for payment status.

### Design Doc
- **Triggers:** Customer reaches the checkout page.
- **Actions:** OHC dynamically shows payment options based on the business owner's enabled providers and the customer's region.
- **What the User Sees:** A "Payments" settings page where they can turn on "Stripe", "Mercado Pago", "Razorpay", etc. The customer sees a familiar local payment button at checkout instead of just a credit card form.

### Implementation Prompt
Expand the checkout system to support regional payment providers alongside Stripe. Allow the business owner to easily toggle which payment methods they want to accept based on their target market.
**Acceptance Criteria:**
- Settings page includes toggles for at least one major alternative payment gateway (e.g., Mercado Pago or Razorpay).
- The checkout page dynamically renders the correct payment widget.
- Orders are only marked as "Paid" when the external provider confirms the transaction.

### Priority
P1

### Estimated Scope
Medium

---

## 5. Shipping & Logistics

### Title
One-Click Shipping Labels and Real-Time Rates

### Problem Statement
Figuring out how much to charge for shipping is a guessing game, and I often lose money on it. Then, when an order comes in, I have to copy-paste the address into the post office website to buy a label. It takes forever.

### Research Report
- **Evaluated Tools:** Shippo, EasyPost.
- **Ease of Use:** Excellent. Business owners link their carrier accounts (or use the tool's default discounted rates) and can print labels instantly.
- **Pricing:** Usually a small fee per label (e.g., $0.05) or a low monthly subscription, plus the actual cost of postage.
- **Reputation:** Both are highly regarded and reliable.
- **Environment Support:** Fully supported in both Cloud and Standalone modes via direct API calls.

### Design Doc
- **Triggers:** A customer views their cart (calculates rate). An owner clicks "Fulfill" on an order.
- **Actions:** API calculates shipping costs based on weight/dimensions. API generates a PDF label and tracking number.
- **What the User Sees:** Customers see accurate shipping costs at checkout. The owner sees a "Buy Label" button on the order page, and a PDF pops up ready to print. The customer automatically gets an email with their tracking link.

### Implementation Prompt
Integrate a shipping API so business owners can calculate shipping rates at checkout and print shipping labels directly from the order management screen.
**Acceptance Criteria:**
- Customers see accurate, real-time shipping quotes at checkout based on their address.
- Business owner can click "Print Label" on a paid order to generate a PDF shipping label.
- A tracking number is automatically saved to the order and emailed to the customer.

### Priority
P2

### Estimated Scope
Large

---

## 6. SMS & Notifications

### Title
SMS Order Updates and Reminders

### Problem Statement
My customers don't check their email often, so they miss appointment reminders or order delivery updates. I need a way to text them automatically so they actually see the message.

### Research Report
- **Evaluated Tools:** Twilio, MessageBird, Vonage.
- **Ease of Use:** For the business owner, zero setup required if OHC handles the integration natively. They just flip a switch to "Enable SMS Updates".
- **Pricing:** Twilio charges per SMS (approx $0.0079 in the US, higher internationally). Costs can add up quickly.
- **Reputation:** Twilio is the gold standard for reliability.
- **Environment Support:** Cloud mode handles this natively. Standalone mode requires the user to input their own Twilio API keys due to per-message costs.

### Design Doc
- **Triggers:** An order ships, or an appointment is 24 hours away.
- **Actions:** OHC sends a templated text message to the customer's phone number.
- **What the User Sees:** A toggle in settings: "Send SMS updates to customers". If enabled, they might see a counter of their monthly SMS usage to manage costs. Customers get a standard text: "Hi, your order from [Store Name] has shipped!"

### Implementation Prompt
Add an automated SMS notification system for critical customer touchpoints like order shipping confirmations and appointment reminders.
**Acceptance Criteria:**
- Business owner can toggle SMS notifications on or off.
- The system automatically sends a text message when an order's status changes to "Shipped".
- In Standalone Mode, the Advanced Settings must reveal fields for Twilio API keys to enable this feature.

### Priority
P2

### Estimated Scope
Small

---

## 7. Video Conferencing

### Title
Automatic Meeting Links for Online Consultations

### Problem Statement
When someone books an online lesson or consultation with me, I have to manually go into Zoom, create a meeting, and email them the link. Sometimes I forget or send the wrong link. It should just happen automatically.

### Research Report
- **Evaluated Tools:** Zoom API, Google Meet (via Google Calendar API).
- **Ease of Use:** "Sign in with Zoom/Google" is a one-click process.
- **Pricing:** Zoom API is free for basic usage, but requires a paid Zoom account for longer meetings. Google Meet is free with a Google account.
- **Reputation:** Zoom and Google Meet are universally used and understood by clients.
- **Environment Support:** Fully supported in both Cloud and Standalone modes via OAuth API calls.

### Design Doc
- **Triggers:** A customer books a service marked as "Online Meeting".
- **Actions:** OHC calls the Zoom/Meet API to generate a unique meeting URL and attaches it to the calendar event.
- **What the User Sees:** When creating a service in OHC, they check a box that says "This is an online meeting". When a client books it, both the owner and the client automatically receive an email with the "Join Meeting" button.

### Implementation Prompt
Integrate video conferencing link generation into the booking system. When a client books a virtual service, automatically generate a unique Zoom or Google Meet link for that session.
**Acceptance Criteria:**
- Business owner can link their Zoom account (or use Google Meet if Calendar is linked).
- Virtual services automatically generate a unique video link upon booking.
- The link is included in the calendar invite and the confirmation emails for both parties.

### Priority
P2

### Estimated Scope
Medium
