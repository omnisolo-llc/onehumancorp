# Scout: Tool Integration Research Q4

## 1. Social Media Integration

**Title:** Unified Inbox for Social Channels (Instagram, FB, WhatsApp, TikTok)
**Problem Statement:** Small business owners are overwhelmed managing customer inquiries across multiple platforms. Messages get lost, response times are slow, and sales are missed. They need a single, unified inbox to view and reply to all social messages without constantly switching apps.
**Research Report:**
- Evaluated Meta Business Suite, Sprout Social, and Chatwoot.
- Non-technical ease of use: Meta's own tools are often clunky and force users to jump through hoops. A simplified, unified inbox like Chatwoot (open-source) or integrating directly with Meta's Graph API for a custom UI provides a much better experience.
- Pricing: Chatwoot has a free tier; Meta Graph API is free but requires WhatsApp business pricing per conversation.
- Reputation: Both are solid. OHC needs to abstract away the OAuth complexity for the user.
- Works in Cloud and Standalone (with appropriate API keys/webhooks).
**Design Doc:**
- User connects accounts via a simple OAuth "Connect" button in settings.
- Inbound messages trigger an event in OHC's unified inbox.
- User sees a combined feed of messages, categorized by source.
- Replying in OHC sends the message back via the respective platform's API.
**Implementation Prompt:**
Create a unified inbox UI that displays messages from Instagram, FB, WhatsApp, and TikTok in a single feed. Provide a simple OAuth connection flow for each platform. Users should be able to read and reply to messages directly from the OHC interface.
**Priority:** P0
**Estimated Scope:** Large

## 2. Calendar & Scheduling

**Title:** Automated Meeting Links and Calendar Sync (Google/Outlook)
**Problem Statement:** Booking consultations or services often involves back-and-forth emails to find a time and manually creating Zoom/Meet links. This is tedious and unprofessional. Business owners need a way to share a booking link that syncs with their calendar and auto-generates video links.
**Research Report:**
- Evaluated Calendly, Cal.com.
- Non-technical ease of use: Calendly is the industry standard for simplicity. Cal.com is open-source and highly customizable.
- Pricing: Both have generous free tiers.
- Reputation: Excellent.
- Cal.com is preferable for deeper integration or self-hosting (Standalone mode).
- Works in both Cloud and Standalone.
**Design Doc:**
- User connects Google/Outlook calendar via OAuth.
- User configures "Event Types" (e.g., 30 min consultation).
- OHC generates a public booking page.
- When a client books, OHC creates a calendar event and auto-generates a Meet/Zoom link.
**Implementation Prompt:**
Build a scheduling interface where business owners can connect their Google/Outlook calendars. Provide a public booking link for their clients. Automatically sync booked appointments to the calendar and generate video conferencing links for remote meetings.
**Priority:** P1
**Estimated Scope:** Medium

## 3. Email Marketing

**Title:** Integrated Email Campaigns and Newsletters
**Problem Statement:** Business owners need to reach out to their customer list for promotions, updates, or newsletters, but exporting lists to external tools like Mailchimp is cumbersome and leads to fragmented data.
**Research Report:**
- Evaluated Mailchimp, Resend, SendGrid.
- Non-technical ease of use: Mailchimp has a great visual editor but is expensive. Resend is developer-focused but simple.
- Pricing: Mailchimp gets expensive quickly. Resend/SendGrid offer cheap volume pricing but require OHC to build the visual editor.
- Reputation: All are reliable. Building a simple visual editor in OHC backed by Resend provides the best UX/cost ratio.
- Works in both Cloud and Standalone (using API keys).
**Design Doc:**
- User selects customers from their OHC CRM list.
- User writes an email using a simple rich-text editor with basic templates.
- OHC sends the emails via integration (e.g., Resend).
- OHC tracks open rates and displays basic analytics.
**Implementation Prompt:**
Implement a simple email campaign tool. Allow users to select a segment of their customer list, compose an email using a rich-text editor, and send it. Provide basic analytics on open rates and clicks.
**Priority:** P1
**Estimated Scope:** Medium

## 4. Payment Processing

**Title:** Localized Payment Processing (Beyond Stripe)
**Problem Statement:** Stripe is not universally available or preferred. Businesses in LATAM prefer Mercado Pago; in India, Paytm/Razorpay. Without local payment options, businesses lose sales due to friction or high fees.
**Research Report:**
- Evaluated Mercado Pago (LATAM), Razorpay (India), Alipay (China).
- Non-technical ease of use: These tools often provide simple drop-in checkout UIs.
- Pricing: Varies by region, generally competitive locally.
- Reputation: High trust in their respective regions.
- Works in both Cloud and Standalone.
**Design Doc:**
- User selects their preferred local payment gateway in settings and enters API keys.
- During checkout on OHC storefronts/invoices, the relevant payment gateway UI is displayed.
- OHC handles webhooks to mark invoices as paid.
**Implementation Prompt:**
Add support for alternative payment gateways (e.g., Mercado Pago, Razorpay) to the checkout flow. Allow users to configure their regional provider in settings. Ensure the checkout experience is seamless and localized.
**Priority:** P2
**Estimated Scope:** Medium

## 5. Shipping & Logistics

**Title:** Real-time Shipping Rates and Label Generation
**Problem Statement:** Calculating shipping costs manually is error-prone, and going to the post office to buy labels is time-consuming. E-commerce businesses need automatic rate calculation at checkout and printable labels.
**Research Report:**
- Evaluated Shippo, EasyPost.
- Non-technical ease of use: Both offer APIs to abstract away carrier complexities. Shippo has a slightly more user-friendly dashboard if they need to log in directly.
- Pricing: Usually a few cents per label + carrier fees.
- Reputation: Both are industry standards.
- Works in both Cloud and Standalone.
**Design Doc:**
- User connects their Shippo/EasyPost account.
- At checkout, OHC queries the API for live rates based on cart weight/dimensions.
- In the OHC admin panel, user clicks "Generate Label" for an order to get a printable PDF.
**Implementation Prompt:**
Integrate a shipping API (like Shippo or EasyPost) to provide real-time shipping quotes during checkout. Add a "Generate Label" button in the order management view that creates a printable PDF shipping label.
**Priority:** P2
**Estimated Scope:** Large

## 6. SMS & Notifications

**Title:** Global SMS Notifications for Customers
**Problem Statement:** Many customers prefer SMS over email, especially in regions with lower email adoption or for time-sensitive alerts (e.g., appointment reminders, order updates). Business owners need a reliable way to send these texts.
**Research Report:**
- Evaluated Twilio, MessageBird.
- Non-technical ease of use: Requires OHC to build a friendly UI on top of their APIs.
- Pricing: Twilio is standard but can get pricey internationally.
- Reputation: Excellent reliability.
- Works in both Cloud and Standalone.
**Design Doc:**
- User configures SMS settings (often using an OHC-managed pool in Cloud, or their own keys in Standalone).
- OHC automatically triggers SMS for key events (e.g., "Your order has shipped").
- User can send manual 1:1 texts from the unified inbox.
**Implementation Prompt:**
Add SMS notification capabilities for critical customer touchpoints (order confirmations, appointment reminders). Provide an interface for the business owner to configure templates and enable/disable specific SMS triggers.
**Priority:** P1
**Estimated Scope:** Medium

## 7. Video Conferencing

**Title:** Auto-Generated Video Meeting Links
**Problem Statement:** Manually creating and sharing Zoom or Google Meet links for remote services or consultations causes friction and looks unprofessional.
**Research Report:**
- Evaluated Zoom API, Google Workspace API.
- Non-technical ease of use: Both require OAuth setup. Google Meet is often easier if they already use Google Calendar.
- Pricing: Free tiers cover basic needs.
- Reputation: Ubiquitous.
- Works in both Cloud and Standalone.
**Design Doc:**
- User authenticates with Zoom or Google.
- When an online appointment is booked, OHC automatically calls the respective API to create a meeting.
- The generated join link is added to the calendar event and emailed to the customer.
**Implementation Prompt:**
Integrate with Zoom and Google Meet APIs to automatically generate meeting links when a remote appointment is scheduled. Ensure the link is securely shared with the customer and attached to the corresponding calendar event.
**Priority:** P2
**Estimated Scope:** Small
