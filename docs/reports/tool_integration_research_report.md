# Tool Integration Research Report

## 1. Social Media Integration

**Title**: Unified Social Inbox: Instagram, Facebook, TikTok, and WhatsApp
**Problem Statement**: Small business owners waste hours checking 4-5 different apps to reply to customer inquiries, often missing sales or responding late because messages are scattered across platforms. They need a single place to see and reply to all messages.
**Research Report**: Unified inbox tools like ManyChat, Sprout Social, and Hootsuite offer social media message aggregation. For our target audience, Meta's official Graph API and WhatsApp Business API provide the most reliable way to pull messages without violating Terms of Service. TikTok also offers an official API for comments and messages. The integration requires handling OAuth flows for different platforms, listening to webhooks for new messages, and parsing text/media. Pricing for APIs is generally volume-based but highly affordable for small businesses. Evaluated tools: Meta Graph API (Free tier available), WhatsApp Cloud API (Pay per conversation), TikTok for Business API.
**Design Doc**: A "Unified Inbox" screen in the OHC platform. When a user connects their social accounts via OAuth, OHC begins receiving webhooks for new messages. The UI displays conversations in a combined chronological feed. The user can reply directly from OHC, and the platform routes the message back to the correct social channel. Cloud mode uses a centralized webhook receiver. Standalone mode polls or uses a local tunnel for webhooks.
**Implementation Prompt**: Create a Unified Inbox view that displays messages from multiple social channels. Implement the OAuth connection flows for Meta (Facebook/Instagram/WhatsApp) and TikTok. Develop webhook endpoints to receive incoming messages, save them to the tenant database, and display them in real-time. Ensure users can reply from the OHC interface and the replies are successfully delivered to the native platform.
**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

**Title**: Seamless Calendar Sync & Booking Pages
**Problem Statement**: Service-based businesses (salons, consultants, tutors) lose time and risk double-booking due to manual scheduling. They need a simple, automated way for clients to book appointments that instantly syncs with their personal Google or Outlook calendars.
**Research Report**: Tools like Calendly, Acuity, and Cal.com dominate this space. Integrating directly with Google Calendar API and Microsoft Graph API is the most robust approach for a platform, avoiding third-party subscription fees for the user. These APIs handle timezone translation and conflict resolution out-of-the-box. Free tier API access is generous. The main challenge is managing OAuth refresh tokens and parsing recurring events properly. Both Cloud and Standalone modes are viable, though Standalone requires local OAuth handling.
**Design Doc**: A "Booking Page" feature that generates a public URL for the business. The business owner authenticates their Google or Outlook calendar in OHC. OHC queries the calendar for free/busy times and displays available slots to clients. When a client books, OHC inserts an event directly into the owner's calendar and sends a confirmation email.
**Implementation Prompt**: Build a public booking page generator for tenants. Integrate Google Calendar and Outlook OAuth flows to fetch free/busy data. Create the booking logic that checks availability, handles timezone conversions, and creates events on the connected calendar. Include a settings page for the business owner to define working hours and meeting durations.
**Priority**: P0
**Estimated Scope**: Medium

---

## 3. Email Marketing

**Title**: Customer List Email Campaigns
**Problem Statement**: Small business owners collect customer emails but don't know how to engage them effectively. Setting up complex tools like Mailchimp is overwhelming. They need a dead-simple way to send announcements or promotions to their existing customer list directly from OHC.
**Research Report**: Providers like SendGrid, Resend, and Postmark offer robust APIs for transactional and marketing emails. Resend is particularly developer-friendly with high deliverability rates. For non-technical users, a drag-and-drop template builder is too complex; they prefer simple, text-based or single-image emails (like Substack). Spam compliance (unsubscribe links, physical address) is critical. Integrating an API like Resend costs fractions of a cent per email, easily subsidized or passed on. Works perfectly in Cloud mode; Standalone mode can use the user's own API key.
**Design Doc**: A "Broadcasts" section in OHC. The owner selects a segment of their customer list, writes a rich-text message with an optional image, and hits send. OHC compiles the list, appends mandatory unsubscribe footers, and dispatches the emails via a transactional email API (e.g., Resend). Analytics (open rates) are fetched via webhooks and displayed in a simple dashboard.
**Implementation Prompt**: Create an Email Campaigns interface allowing users to compose rich-text emails and send them to their customer segments. Integrate an email API (e.g., Resend) for bulk dispatch. Implement mandatory CAN-SPAM compliance features (unsubscribe links). Set up webhooks to track and display basic metrics like open and bounce rates.
**Priority**: P1
**Estimated Scope**: Medium

---

## 4. Payment Processing

**Title**: Global Payment Links & Alternative Methods
**Problem Statement**: Stripe is great, but many small businesses operate in regions where Stripe is unavailable or not the preferred method (e.g., Mercado Pago in LATAM, Paytm in India). Businesses need to generate payment links that accept the local payment methods their customers actually use.
**Research Report**: Aggregators like Rapyd or dLocal offer broad global coverage, but direct integrations with regional leaders (Mercado Pago for LATAM, Razorpay/Paytm for India) often provide lower fees and better merchant experiences. These tools offer API-based "Payment Links" which are incredibly easy for non-technical users to share via WhatsApp. The settlement speed varies by region. The API integration involves creating a payment intent and returning a hosted checkout URL. Works in both Cloud and Standalone modes.
**Design Doc**: A "Get Paid" feature where the owner enters an amount and description, and OHC generates a shareable payment link. Depending on the tenant's region, OHC routes the request to Stripe, Mercado Pago, or Razorpay to generate the link. When the payment succeeds, the respective provider sends a webhook to OHC, marking the internal invoice/order as paid.
**Implementation Prompt**: Implement a Payment Link generator that integrates with at least two regional payment providers (e.g., Mercado Pago and Razorpay) in addition to Stripe. Provide a simple UI for the owner to input amount and details to get a link. Implement webhook handlers for each provider to update the payment status securely in the database.
**Priority**: P1
**Estimated Scope**: Medium

---

## 5. Shipping & Logistics

**Title**: Automated Shipping Labels & Tracking
**Problem Statement**: E-commerce small businesses waste hours manually copying customer addresses into local carrier websites to buy shipping labels. They need a system that calculates rates and prints labels with one click right from the order dashboard.
**Research Report**: APIs like Shippo, EasyPost, and Sendle aggregate multiple carriers (USPS, UPS, FedEx, local international carriers). Shippo and EasyPost provide excellent REST APIs to fetch real-time rates, generate labels (PDF/ZPL), and track packages. Pricing is typically a few cents per label. The integration requires accurate package weight/dimensions and from/to addresses. Works in both Cloud and Standalone (printing locally).
**Design Doc**: On the "Order Details" page, add a "Create Label" button. OHC sends the order's shipping address and default box dimensions to the shipping API (e.g., EasyPost) to fetch rates. The user selects a rate, and OHC purchases the label, displaying the PDF for download/printing. The tracking number is automatically saved and emailed to the customer.
**Implementation Prompt**: Integrate a shipping aggregator API (e.g., EasyPost) into the order fulfillment flow. Build the UI to display live shipping rates, allow the user to purchase a label, and render the resulting PDF. Automate the process of attaching the tracking number to the order and sending a notification to the customer.
**Priority**: P2
**Estimated Scope**: Large

---

## 6. SMS & Notifications

**Title**: Instant SMS Notifications for Customers
**Problem Statement**: Many customers, especially in certain demographics or regions, ignore emails but read every text message. Businesses need to send appointment reminders, order updates, and simple marketing blasts via SMS to ensure they are seen.
**Research Report**: Twilio is the industry standard, but alternatives like MessageBird or Plivo often offer better pricing for specific regions. SMS pricing varies wildly by destination country. A critical challenge is 10DLC compliance in the US, which requires business registration to avoid carrier filtering. For non-US regions, alphanumeric sender IDs are common. Cloud mode handles API keys centrally; Standalone could require the user to input their own Twilio credentials (revealed via Advanced Mode).
**Design Doc**: A "Text Alerts" settings panel where owners can toggle automatic SMS reminders for appointments or order shipments. OHC formats short, concise messages and dispatches them via the SMS API (e.g., Twilio). For campaigns, a simple broadcast tool similar to the email feature, but restricted by character limits and strictly enforcing opt-out tracking.
**Implementation Prompt**: Integrate an SMS provider (e.g., Twilio) to send automated transactional messages (e.g., appointment reminders). Build a preferences UI for the business owner to toggle these notifications. Ensure the system handles phone number formatting (E.164) and automatically processes "STOP" replies to update opt-out lists in the database.
**Priority**: P1
**Estimated Scope**: Medium

---

## 7. Video Conferencing

**Title**: Auto-Generated Meeting Links
**Problem Statement**: Tutors, consultants, and remote service providers struggle with manually creating Zoom or Google Meet links and sending them to clients for every booking. They need video links to be automatically generated and attached to calendar invites.
**Research Report**: Google Meet is free and automatically included if we integrate Google Calendar. Zoom requires a separate OAuth integration and API calls to create meetings. Zoom's API is robust but their app approval process is stringent. Microsoft Teams works via Microsoft Graph API. Generating links automatically saves the user ~2 minutes per booking and eliminates "where is the link" emails. Works seamlessly in both Cloud and Standalone modes.
**Design Doc**: Tied closely to the Calendar Scheduling feature. When an owner sets a service type as "Online Meeting", OHC automatically adds a Google Meet link (via Google Calendar API) or a Zoom link (via Zoom OAuth API) to the created calendar event. The generated link is displayed on the booking confirmation page and in reminder emails.
**Implementation Prompt**: Extend the calendar booking system to support location types (Physical vs. Online). Integrate the Zoom API via OAuth to create unique meeting URLs on demand. Ensure that if Google Calendar is used, the native Google Meet generation flag is passed. Update confirmation emails and the client-facing UI to prominently display the "Join Meeting" button.
**Priority**: P2
**Estimated Scope**: Small
