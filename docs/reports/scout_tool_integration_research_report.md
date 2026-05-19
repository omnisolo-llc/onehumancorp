# Scout Tool Integration Research Report

## 1. Social Media Integration

**Title**: [Social Media] Unified Inbox Integration for Instagram, Facebook, WhatsApp, and TikTok

**Problem Statement**: Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive inquiries across multiple social media platforms. Checking Instagram DMs, Facebook comments, WhatsApp, and TikTok messages separately is overwhelming and leads to missed sales opportunities. They need a single place to view and respond to all customer messages.

**Research Report**:
- **Evaluated Tools**: Meta Graph API (for FB/IG/WhatsApp), TikTok for Business API, external aggregators like MessageBird or Twilio.
- **Ease of Use**: Non-technical users struggle with complex OAuth flows (e.g., Meta Business Manager). The integration must abstract the setup into a simple "Connect with Facebook" button.
- **Pricing**: Meta APIs are mostly free for basic messaging, but WhatsApp Business API has per-conversation pricing. Twilio/MessageBird adds a per-message markup.
- **OAuth Complexity**: High for Meta due to app review requirements.
- **Message Parsing Quality**: High for text, variable for media (voice notes, images).
- **Webhook Reliability**: High for Meta, but requires strict SLA compliance to avoid app suspension.
- **Cloud vs Standalone**: Works well in Cloud mode. In Standalone mode, webhooks require a tunneling or polling mechanism, which adds complexity.

**Design Doc**:
- **Triggers**: A customer sends a message on a connected social platform.
- **Actions**: The system receives the webhook, maps it to the corresponding customer profile, and displays it in the OHC unified inbox. The AI Customer Success agent can automatically draft or send replies based on business context.
- **User View**: A single "Inbox" screen on their phone where messages from all platforms appear seamlessly, with the platform logo indicating the source.

**Implementation Prompt**:
Implement a unified inbox feature that allows users to connect their social media accounts. When a customer messages them on Instagram, Facebook, WhatsApp, or TikTok, the message should appear in a central OHC inbox. The business owner should be able to reply from OHC, and the response should be delivered back to the original platform. Ensure the setup process is a simple 1-click OAuth flow without requiring technical configuration.

**Priority**: P1
**Estimated Scope**: Large


## 2. Calendar & Scheduling

**Title**: [Calendar] Sync and Scheduling via Google Calendar and Outlook

**Problem Statement**: Service providers like Carlos (Freelance Handyman) and Leo (Music Tutor) manage their time using personal calendar apps. They need a booking system that automatically reads their availability to prevent double-booking and adds new appointments directly to their calendars.

**Research Report**:
- **Evaluated Tools**: Google Calendar API, Microsoft Graph API (Outlook), Nylas, Cronofy.
- **Ease of Use**: Native APIs require users to grant OAuth permissions, which is standard and well-understood. Aggregators like Nylas simplify multi-provider support but add cost.
- **Pricing**: Google and Microsoft APIs are free for basic usage. Nylas charges per connected account ($1-$2/mo).
- **Calendar Conflict Resolution**: Native APIs provide robust free/busy querying.
- **Timezone Handling**: Complex but manageable using standard IANA timezone databases.
- **Cloud vs Standalone**: Fully supported in both modes, though Standalone may need to handle token refresh locally.

**Design Doc**:
- **Triggers**: A user connects their calendar; a customer views the booking page; a customer books a slot.
- **Actions**: System queries the connected calendar for free/busy times, subtracts them from the business's working hours, and displays available slots. Upon booking, a calendar event is created for both the business owner and the customer.
- **User View**: A "Connect Calendar" button in the Operations settings. The booking page seamlessly reflects real-time availability.

**Implementation Prompt**:
Build a two-way calendar sync feature supporting Google Calendar and Outlook. The business owner should be able to connect their calendar with a single click. The OHC booking page must only show available time slots by checking against the connected calendar's busy times. When a booking is made, it should automatically appear on the owner's personal calendar.

**Priority**: P1
**Estimated Scope**: Medium


## 3. Email Marketing

**Title**: [Email Marketing] Automated Campaigns and Customer Newsletters

**Problem Statement**: Business owners like Priya (The Boutique Owner) want to notify their existing customers about new stock or promotions. They do not have the time or skills to use complex platforms like Mailchimp and need a simple way to send beautiful emails directly from their customer list.

**Research Report**:
- **Evaluated Tools**: SendGrid, Amazon SES, Postmark, Resend.
- **Ease of Use**: End-users will not interact with these tools directly. OHC will provide a simplified UI, and the backend will route via the chosen provider.
- **Pricing**: Amazon SES is the cheapest ($0.10/1k emails). Resend offers a great developer experience but is pricier ($20/mo for 50k).
- **Template Quality**: OHC must provide pre-built, premium glassmorphism-inspired HTML templates.
- **Spam Compliance**: Built-in handling of unsubscribe links and CAN-SPAM requirements is essential.
- **Cloud vs Standalone**: Cloud handles delivery easily. Standalone may require the user to input an SMTP server or use a cloud relay to avoid being flagged as spam.

**Design Doc**:
- **Triggers**: User initiates a campaign, or an automated trigger (e.g., "new stock") fires.
- **Actions**: The Marketing agent generates the email copy and design. The system batches the emails and sends them via the email API provider, handling unsubscribes and bounces.
- **User View**: A simple "Send Announcement" screen where the user selects an AI-generated draft, reviews it, and hits send. They can see an "Opened" or "Clicked" metric later.

**Implementation Prompt**:
Create a simple email marketing tool allowing business owners to send announcements or promotions to their customer list. Provide a clean UI to select an audience, review an AI-generated or custom email, and send it. The system must automatically include compliance requirements like unsubscribe links and handle bounce tracking seamlessly.

**Priority**: P2
**Estimated Scope**: Medium


## 4. Payment Processing

**Title**: [Payments] Global Alternative Payment Methods (Mercado Pago, Paytm, Alipay)

**Problem Statement**: While Stripe is excellent, it is not supported or preferred in all regions. Business owners in LATAM, India, or China need local payment methods to successfully convert sales, as credit card penetration is lower and local wallets are dominant.

**Research Report**:
- **Evaluated Tools**: Mercado Pago (LATAM), Paytm (India), Alipay/WeChat Pay (China), Razorpay.
- **Ease of Use**: Varies. Razorpay provides a Stripe-like experience for India. Mercado Pago is dominant in LATAM.
- **Pricing**: Typically 2-3% per transaction, competitive with Stripe but in local currencies.
- **Settlement Speed**: Often faster than Stripe for local bank transfers (T+1 or instant).
- **Currency Support**: Highly localized.
- **Cloud vs Standalone**: Both modes support API calls, but webhook handling in Standalone requires secure local tunneling.

**Design Doc**:
- **Triggers**: Customer proceeds to checkout.
- **Actions**: The system detects the region or allows the user to select their preferred local payment provider, redirecting to the provider's secure checkout page, and handling the success webhook.
- **User View**: In the Finance settings, owners can toggle specific regional payment methods. Customers see familiar local payment options at checkout.

**Implementation Prompt**:
Integrate alternative payment providers to support global users, starting with Mercado Pago for LATAM and Razorpay for India. Allow the business owner to enable these options in their payment settings. Ensure the checkout flow seamlessly transitions to these providers when selected, and correctly records the payment success in the OHC Finance dashboard.

**Priority**: P2
**Estimated Scope**: Large


## 5. Shipping & Logistics

**Title**: [Shipping] Real-Time Rates and Label Generation

**Problem Statement**: Product sellers like Maya (The Home Baker, if shipping non-perishables) or Priya (The Boutique Owner) struggle with calculating shipping costs and printing labels. They need an automated way to charge customers the correct shipping amount and print ready-to-use courier labels.

**Research Report**:
- **Evaluated Tools**: Shippo, EasyPost, ShipStation.
- **Ease of Use**: Shippo and EasyPost offer excellent APIs. The complexity of package dimensions and weights must be abstracted for the user.
- **Pricing**: Shippo is $0.05 per label + postage. EasyPost is similar.
- **Carrier Coverage**: Excellent global coverage (USPS, FedEx, UPS, DHL, local carriers).
- **Cloud vs Standalone**: Fully supported in both environments via standard REST APIs.

**Design Doc**:
- **Triggers**: Customer views cart (rate calculation); owner fulfills order (label generation).
- **Actions**: System calculates real-time shipping rates based on cart contents and destination. Upon fulfillment, it purchases the label and retrieves a printable PDF and tracking number.
- **User View**: Customer sees accurate shipping costs at checkout. The owner clicks "Print Shipping Label" on an order, automatically generating the PDF and emailing the tracking link to the customer.

**Implementation Prompt**:
Implement a shipping management integration using a provider like Shippo or EasyPost. The system must automatically calculate shipping rates at checkout based on standard package sizes. Add a feature to the order management screen allowing the business owner to generate and download a shipping label with one click, automatically updating the order status to "Shipped" and notifying the customer.

**Priority**: P2
**Estimated Scope**: Medium


## 6. SMS & Notifications

**Title**: [SMS] Automated Order Notifications and Alerts

**Problem Statement**: Users like Fatima (The Food Cart Operator) may not always be looking at the app or have reliable mobile data. SMS notifications are a critical, low-tech way to alert owners of new orders instantly and inform customers that their food is ready for pickup.

**Research Report**:
- **Evaluated Tools**: Twilio, Vonage, Plivo, MessageBird.
- **Ease of Use**: High via API. The owner simply provides their phone number.
- **Pricing**: Twilio is ~$0.0079 per SMS in the US, but international rates vary wildly (up to $0.10+ in some regions).
- **Delivery Reliability**: Very high.
- **Opt-Out Compliance**: Mandatory compliance (STOP messages) is required.
- **Cloud vs Standalone**: Fully supported.

**Design Doc**:
- **Triggers**: New order received; order status changed to "Ready".
- **Actions**: System dispatches a formatted SMS via the provider's API.
- **User View**: The owner receives a text: "New Order #102: 2x Chicken Over Rice. Paid." The customer receives a text: "Your order is ready for pickup!"

**Implementation Prompt**:
Integrate an SMS notification system (e.g., using Twilio) to send critical alerts. Allow business owners to opt-in to receive SMS alerts for new orders, which is especially important for fast-paced environments like food carts. Additionally, enable automatic SMS notifications to customers when their order status changes to "Ready for Pickup" or "Shipped".

**Priority**: P1
**Estimated Scope**: Small


## 7. Video Conferencing

**Title**: [Video] Automated Zoom/Meet Link Generation for Services

**Problem Statement**: Service providers like Leo (The Music Tutor) conduct their business online. Manually creating a Zoom link for every booked lesson and emailing it to the student is tedious and prone to human error.

**Research Report**:
- **Evaluated Tools**: Zoom API, Google Meet API (via Google Calendar), Whereby.
- **Ease of Use**: Whereby embedded links are easiest to generate via API. Zoom requires OAuth setup. Google Meet is seamless if the user already connected Google Calendar.
- **Pricing**: Zoom requires a paid plan for API access. Google Meet is free with Calendar. Whereby has API pricing.
- **Join Experience**: Critical that students can join without installing new software if possible.
- **Cloud vs Standalone**: Fully supported in both modes.

**Design Doc**:
- **Triggers**: A customer books an online service.
- **Actions**: System creates a meeting via the provider's API and attaches the join URL to the booking confirmation and calendar event.
- **User View**: A setting to "Enable Video Meeting for this Service". The booking confirmation automatically includes a big "Join Meeting" button.

**Implementation Prompt**:
Build an automated video meeting integration. For services marked as "Online", the system should automatically generate a unique meeting link (e.g., using Google Meet or Zoom) upon booking. This link must be included in the customer's confirmation email, the calendar invite, and displayed prominently in the business owner's upcoming appointments dashboard.

**Priority**: P2
**Estimated Scope**: Small
