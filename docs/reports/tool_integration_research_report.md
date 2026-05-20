# Tool Integration Research Report

## Social Media Integration

**Title**: Integrate ManyChat for Unified Social Media Messaging
**Problem Statement**: Small business owners struggle to keep up with customer inquiries scattered across Instagram, Facebook, and WhatsApp. Messages fall through the cracks, leading to lost sales and poor customer service. They need a single, unified inbox to manage all conversations effortlessly.
**Research Report**: Evaluated ManyChat. It offers seamless integration with Instagram DMs, Facebook Messenger, and WhatsApp. It is highly user-friendly with a drag-and-drop flow builder, which is great for setting up basic auto-replies. Pricing starts at $15/month for the Pro plan, making it affordable for small businesses. The OAuth process is standardized and reliable. It works well in Cloud (multi-tenant) mode. In Standalone mode, webhooks might require local tunneling (like ngrok) or polling, but it is technically feasible.
**Design Doc**: OHC will connect to ManyChat via OAuth. When a new message arrives on any connected social platform, ManyChat forwards it to OHC. OHC displays the message in a unified inbox tab within the dashboard. The business owner can reply directly from OHC, which routes the message back through ManyChat to the original platform.
**Implementation Prompt**: Create a "Unified Inbox" view in the dashboard. Allow users to connect their Instagram and Facebook accounts using a "Connect to ManyChat" button. Ensure incoming messages appear in real-time or near real-time, and that replies sent from the dashboard successfully reach the customer on the platform they used.
**Priority**: P1
**Estimated Scope**: Large

---

## Calendar & Scheduling

**Title**: Integrate Cal.com for Automated Meeting Scheduling
**Problem Statement**: Setting up appointments, consultations, or classes often involves endless back-and-forth emails or texts to find a suitable time. This wastes time and frustrates both the business owner and the customer.
**Research Report**: Evaluated Cal.com. It is an open-source scheduling tool that integrates well with Google Calendar, Outlook, and Apple Calendar. It offers robust conflict resolution and timezone handling. For non-technical users, setting up a booking page is straightforward. The free tier is generous, and team plans start at $12/user/month. It supports both Cloud and Standalone environments beautifully since it can be self-hosted or consumed via their API.
**Design Doc**: OHC will integrate with Cal.com via its API. Business owners can generate a unique booking link directly from their OHC dashboard. Customers visiting the link can see available times and book appointments. OHC will sync the scheduled events back to the dashboard's agenda view.
**Implementation Prompt**: Add a "Scheduling" section where users can connect their existing calendars and generate a public booking link. Display a simple agenda of upcoming appointments in the main dashboard. The user should not have to leave OHC to see who they are meeting today.
**Priority**: P1
**Estimated Scope**: Medium

---

## Email Marketing

**Title**: Integrate Mailchimp for Customer Email Campaigns
**Problem Statement**: Small business owners want to send newsletters or promotional offers to their customer list but find complex marketing software intimidating. They need a simple way to stay in touch with past customers to encourage repeat business.
**Research Report**: Evaluated Mailchimp. It remains the industry standard for ease of use. It provides excellent templates, easy list management, and robust spam compliance guardrails. The free tier allows up to 500 contacts, which is perfect for new businesses. Upgraded plans start around $13/month. The API is mature and reliable. Works perfectly in Cloud mode; Standalone users can sync lists via API.
**Design Doc**: OHC will connect to the Mailchimp API. The business owner's customer list in OHC will automatically sync with a designated Mailchimp audience. OHC will provide a button to "Create Campaign" which will launch the Mailchimp template builder in a new tab or iframe, ensuring the owner can use Mailchimp's powerful visual tools while keeping the data synchronized.
**Implementation Prompt**: Implement an "Email Marketing" tab. Allow the user to authenticate with Mailchimp. Ensure that any new customer added to OHC is automatically added to the Mailchimp list. Show basic campaign statistics (open rates, click rates) in the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium

---

## Payment Processing

**Title**: Integrate Mercado Pago for LATAM Payment Processing
**Problem Statement**: Business owners in Latin America often cannot use Stripe. They need a localized, reliable payment processor that supports local payment methods (like Pix in Brazil or cash payments via convenience stores) with fast settlement times.
**Research Report**: Evaluated Mercado Pago. It is highly trusted across LATAM and supports critical local payment methods. Settlement times vary but are generally competitive, and failure rates for local cards are significantly lower than international processors. Pricing is transparent, typically a percentage of the transaction. It supports both Cloud and Standalone environments via standard REST APIs.
**Design Doc**: OHC will integrate the Mercado Pago checkout SDK. When generating an invoice or checkout link in OHC, the owner can select Mercado Pago as the gateway. Customers will be redirected to the secure Mercado Pago checkout page to complete the transaction, and webhooks will update the invoice status in OHC to "Paid".
**Implementation Prompt**: Add Mercado Pago to the "Payment Methods" settings. Allow the owner to connect their account. Update the invoice generation flow to output Mercado Pago payment links. Ensure the dashboard accurately reflects payment success or failure.
**Priority**: P0
**Estimated Scope**: Large

---

## Shipping & Logistics

**Title**: Integrate Shippo for Streamlined Shipping and Label Generation
**Problem Statement**: Fulfilling physical product orders is tedious. Business owners have to manually copy addresses into carrier websites, calculate shipping rates, and buy labels one by one. This is error-prone and time-consuming.
**Research Report**: Evaluated Shippo. It aggregates multiple carriers (USPS, UPS, FedEx, DHL, and local carriers) into one interface. It provides discounted shipping rates, which is a massive benefit for small businesses. The API is robust and handles rate calculation, label generation, and tracking. Pricing is pay-as-you-go (per label) or subscription-based starting at $19/month. Suitable for both Cloud and Standalone modes.
**Design Doc**: OHC will use the Shippo API. For any physical order in OHC, the system will query Shippo for real-time shipping rates based on package weight and dimensions. The owner can select a carrier, purchase the label directly within OHC, and print it. OHC will store the tracking number and provide status updates.
**Implementation Prompt**: In the "Orders" view, add a "Fulfill Order" button. This should open a modal to enter package weight and dimensions, display shipping options with prices, and allow one-click label purchase and printing. Automatically attach the generated tracking number to the order details.
**Priority**: P1
**Estimated Scope**: Large

---

## SMS & Notifications

**Title**: Integrate Twilio for Reliable SMS Customer Notifications
**Problem Statement**: Many customers, especially those with lower English proficiency or limited email access, prefer or rely on SMS text messages for order updates, appointment reminders, and critical communications.
**Research Report**: Evaluated Twilio. It is the gold standard for global SMS delivery. It offers excellent reliability, global carrier coverage, and handles complex opt-out compliance (STOP messages) natively. Pricing is per-message (e.g., ~$0.0079 per SMS in the US), making it affordable. It works flawlessly in both Cloud and Standalone modes.
**Design Doc**: OHC will connect to the Twilio Programmable SMS API. Business owners can configure automated SMS templates (e.g., "Your order is ready!", "Reminder: Appointment tomorrow at 2 PM"). OHC will trigger these messages based on system events (order status change, calendar event approaching).
**Implementation Prompt**: Create an "SMS Notifications" settings page. Allow the owner to input their Twilio credentials (or use a managed OHC Twilio pool for Cloud users). Provide toggle switches to enable automated SMS for order confirmations and appointment reminders.
**Priority**: P0
**Estimated Scope**: Medium

---

## Video Conferencing

**Title**: Integrate Zoom for Auto-Generating Consultation Links
**Problem Statement**: For businesses offering online services (tutoring, coaching, consultations), manually creating Zoom links and sending them to clients is a repetitive and error-prone chore that can lead to missed meetings.
**Research Report**: Evaluated Zoom API. Zoom is universally recognized and easy for clients to join. The API allows for instantaneous generation of meeting links. The join experience is highly polished. The free tier allows 40-minute meetings, and Pro starts at $14.99/month. Works in both Cloud and Standalone modes.
**Design Doc**: OHC will authenticate with Zoom via OAuth. When an online appointment is scheduled (either manually by the owner or via the scheduling integration), OHC will automatically request a Zoom meeting link and append it to the calendar invite and confirmation notifications.
**Implementation Prompt**: Add a "Connect Zoom" option in the settings. Modify the appointment creation flow to include an "Online Meeting" toggle. When toggled, automatically generate a Zoom link and display it on the appointment details page and include it in any customer confirmation messages.
**Priority**: P2
**Estimated Scope**: Medium
