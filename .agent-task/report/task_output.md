# Research Report: Tool Integration Research Q4

## 1. Social Media Integration
### Title: Unified Social Media Inbox Integration
### Problem Statement
Managing messages across Instagram DMs, Facebook, WhatsApp, and TikTok is overwhelming for small business owners. They miss messages, lose track of customer requests, and struggle to switch between apps constantly. They need a single place to view and respond to all customer messages.
### Research Report
- **Evaluated Tools:** Chatwoot, ManyChat, Twilio.
- **Chatwoot:** Open-source, excellent unified inbox UI, supports multiple channels (WhatsApp, Facebook, Instagram, Twitter). Strong support for both Cloud (SaaS) and Standalone (self-hosted) modes. Good API.
- **ManyChat:** Primarily focused on automation and marketing flows rather than a simple, clean unified inbox. Can be complex to set up.
- **Twilio:** Extremely powerful, but very developer-focused. High learning curve and requires custom UI to be built on top.
- **Winner:** **Chatwoot** is the best fit due to its focus on a unified customer inbox, ease of use, and compatibility with both our Cloud and Standalone modes.
- **Risks:** WhatsApp Business API requires Facebook business verification.
- **Pricing:** Chatwoot has a free tier; paid plans start at $19/user/month. Self-hosted version is free.
- **Cloud/Standalone Support:** Yes, works beautifully in both.
### Design Doc
- **User Experience:** The OHC dashboard will feature an "Inbox" tab. When a user clicks "Connect Socials," they are guided through standard OAuth flows for Instagram, Facebook, and WhatsApp. Once connected, all incoming messages from these platforms appear in the OHC Inbox. The business owner can read and reply directly from OHC, and the replies are sent back through the respective social platform.
- **Triggers:** Receiving a new message on a connected platform.
- **Actions:** Display message in OHC, allow sending replies via OHC.
### Implementation Prompt
Implement an integration with Chatwoot to provide a unified social media inbox within OHC. The integration should allow users to connect their Instagram, Facebook, and WhatsApp accounts via standard OAuth or guided setup. Create an "Inbox" interface in OHC where users can view and respond to messages from all connected platforms seamlessly. The feature must function correctly in both Cloud and Standalone environments.
### Priority
P0 (Critical)
### Estimated Scope
Large

---

## 2. Calendar & Scheduling
### Title: Smart Calendar Sync & Booking Pages
### Problem Statement
Business owners spend too much time going back and forth over email or text to schedule appointments, consultations, or services. They need a simple, professional way for clients to book time directly on their calendar without double-booking.
### Research Report
- **Evaluated Tools:** Calendly, Cal.com, Acuity Scheduling.
- **Calendly:** The industry standard. Very easy to use. However, the API and advanced features are locked behind higher pricing tiers. Less ideal for tight integration into a standalone product.
- **Cal.com:** Open-source, developer-friendly, and very flexible. Excellent white-labeling capabilities and API. Strong support for self-hosting (Standalone mode).
- **Acuity Scheduling:** Geared heavily towards specific service businesses (salons, fitness). A bit heavy for a general-purpose scheduling tool.
- **Winner:** **Cal.com** is the top choice due to its open-source nature, strong API, and excellent fit for both Cloud and Standalone environments.
- **Risks:** Handling complex, multi-user scheduling scenarios can be challenging.
- **Pricing:** Cal.com is free for individuals; team plans start at $12/user/month. Self-hosting is free.
- **Cloud/Standalone Support:** Yes, excellent support for both.
### Design Doc
- **User Experience:** An "Appointments" tab in OHC. The owner connects their Google or Outlook calendar. They can define their availability (e.g., "Monday-Friday, 9am-5pm") and create specific event types (e.g., "30-min Consultation"). OHC generates a personalized, professional booking link they can share with clients or embed on their website. When a client books, the event automatically appears on the owner's calendar, and a confirmation email (with an auto-generated Meet/Zoom link) is sent to both parties.
- **Triggers:** Client selects a time slot on the booking page.
- **Actions:** Create calendar event, send confirmation emails, generate video link.
### Implementation Prompt
Integrate Cal.com to provide automated scheduling and calendar sync. Users should be able to connect their Google or Outlook calendars, set their availability, and generate branded booking links. The integration must automatically handle timezone conversions, prevent double-booking, and automatically create calendar events when a client books a slot. The feature must support both Cloud and Standalone modes.
### Priority
P1 (High)
### Estimated Scope
Medium

---

## 3. Email Marketing
### Title: Integrated Customer Email Campaigns
### Problem Statement
Business owners want to send newsletters, promotions, or updates to their customers but find traditional email marketing tools too complex and disconnected from their main customer list. They need a simple way to email their existing contacts directly.
### Research Report
- **Evaluated Tools:** Mailchimp, Resend, Brevo (formerly Sendinblue).
- **Mailchimp:** Very popular but increasingly expensive and complex. The UI can be overwhelming for simple needs.
- **Resend:** Developer-focused, extremely fast, excellent API. More focused on transactional emails than drag-and-drop campaign building.
- **Brevo:** Good balance of ease-of-use and features. Generous free tier.
- **Winner:** For OHC's needs (simple, integrated campaigns), building a lightweight wrapper around **Resend** or a similar transactional API is best. We control the simple UI, and the API handles delivery.
- **Risks:** Ensuring high deliverability and managing spam compliance (unsubscribe links).
- **Pricing:** Resend offers 3,000 free emails/month. $20/month for 50,000.
- **Cloud/Standalone Support:** Yes, relies on external API so works in both.
### Design Doc
- **User Experience:** A "Marketing" tab in OHC. The owner can select a group of customers from their OHC contacts. They are presented with a very simple, clean text/image editor (no complex HTML drag-and-drop needed for V1). They write the email and click "Send to 50 customers." OHC automatically handles the mass sending and appends mandatory unsubscribe links.
- **Triggers:** User clicks "Send Campaign."
- **Actions:** Queue emails for delivery, track open/click rates (optional V2).
### Implementation Prompt
Create a simple email campaign tool integrated with the OHC customer list. Users should be able to select contacts, draft a simple text/image email, and send it to the group. Use an API like Resend for reliable delivery. The implementation must automatically handle adding unsubscribe links and processing opt-outs to ensure compliance.
### Priority
P2 (Medium)
### Estimated Scope
Medium

---

## 4. Payment Processing
### Title: Global Payment Collection Links
### Problem Statement
Getting paid quickly and easily is a top priority. Business owners need a simple way to generate a payment link they can text or email to a client, without needing a full e-commerce website. They also need options beyond just Stripe for international markets.
### Research Report
- **Evaluated Tools:** Stripe, PayPal, Mercado Pago (LATAM), Razorpay (India).
- **Stripe:** The gold standard for developer experience. Excellent Payment Links feature.
- **Mercado Pago & Razorpay:** Critical for specific regions where Stripe is less dominant or doesn't support local payment methods well.
- **Winner:** A multi-provider approach prioritizing **Stripe** globally, but architected to easily swap in Mercado Pago or Razorpay based on the user's region.
- **Risks:** Handling secure API keys and managing webhook reliability for payment confirmation.
- **Pricing:** Typically a percentage of the transaction (e.g., Stripe is 2.9% + 30¢).
- **Cloud/Standalone Support:** Yes.
### Design Doc
- **User Experience:** A "Payments" tab. The owner clicks "Create Payment Link," enters an amount (e.g., "$150") and a description (e.g., "Plumbing Repair"). OHC generates a secure link. The owner texts this link to the client. The client clicks it, pays via credit card or Apple Pay, and the owner gets an instant notification in OHC that the invoice is paid.
- **Triggers:** Owner generates link; Client completes payment.
- **Actions:** Generate secure URL; Update payment status in OHC based on webhook.
### Implementation Prompt
Implement a feature to generate shareable payment links. Initially integrate with Stripe, but design the system to easily support regional providers like Mercado Pago. Users should be able to enter an amount and description to generate a link. The system must listen for payment completion webhooks and visually update the status of the request to "Paid" in the OHC dashboard.
### Priority
P0 (Critical)
### Estimated Scope
Large

---

## 5. Shipping & Logistics
### Title: Instant Shipping Label Generation
### Problem Statement
For businesses shipping physical goods, manually calculating shipping rates and copy-pasting addresses into carrier websites to buy labels is tedious and error-prone. They need a 1-click way to buy and print shipping labels.
### Research Report
- **Evaluated Tools:** Shippo, EasyPost, ShipStation.
- **Shippo:** Very developer-friendly API, good pricing, easy to use.
- **EasyPost:** Excellent API, very robust, but sometimes slightly more complex to configure than Shippo.
- **ShipStation:** More of a standalone web app; less ideal for tight API integration into another platform.
- **Winner:** **Shippo** for its excellent API and focus on making label generation easy.
- **Risks:** Accurately handling package dimensions and weights to ensure correct pricing.
- **Pricing:** Shippo offers a pay-as-you-go model (5¢ per label + postage).
- **Cloud/Standalone Support:** Yes, API-based.
### Design Doc
- **User Experience:** On an order details page in OHC, there is a "Buy Shipping Label" button. OHC automatically pulls the customer's address. The owner enters the package weight (or selects a predefined box size). OHC displays rates from USPS, UPS, etc. The owner clicks a rate, purchases the label, and OHC generates a printable PDF label and automatically sends the tracking number to the customer.
- **Triggers:** User requests shipping rates; User purchases label.
- **Actions:** Fetch rates via API; Generate PDF label; Update order with tracking.
### Implementation Prompt
Integrate a shipping API (like Shippo) to allow users to generate shipping labels directly from OHC. The integration should auto-populate customer addresses, allow the user to input package weight/dimensions, fetch live rates from carriers, and return a printable PDF label upon purchase. It must also automatically capture the tracking number.
### Priority
P2 (Medium)
### Estimated Scope
Large

---

## 6. SMS & Notifications
### Title: Reliable Customer SMS Notifications
### Problem Statement
Emails often go unread. For time-sensitive updates (like "Your order is ready" or appointment reminders), business owners need a reliable way to text their customers directly from their business system, especially for customers who prefer texting or have low English proficiency.
### Research Report
- **Evaluated Tools:** Twilio, MessageBird, Vonage.
- **Twilio:** The industry leader. Extremely reliable, global coverage, excellent API.
- **MessageBird (Bird):** Strong competitor, often better pricing internationally.
- **Vonage:** Good alternative, solid API.
- **Winner:** **Twilio** due to its ubiquity and reliability.
- **Risks:** Strict compliance rules around SMS marketing (A2P 10DLC registration in the US). We must ensure we are only sending transactional/opt-in messages.
- **Pricing:** Twilio charges roughly $0.0079 per SMS in the US.
- **Cloud/Standalone Support:** Yes, API-based.
### Design Doc
- **User Experience:** In the OHC settings, the owner can enable "SMS Notifications." They might need to fill out a short form to register their business for SMS compliance. Once active, they can check a box on an order or appointment to "Send SMS update." The customer receives a text message with the update.
- **Triggers:** System events (order status change, upcoming appointment) or manual user action.
- **Actions:** Send SMS via API.
### Implementation Prompt
Implement transactional SMS notifications using Twilio. Users should be able to trigger SMS updates for key events (like appointment reminders or order status changes). The implementation must handle basic phone number validation and formatting (E.164) and must include a clear pathway for handling SMS compliance (like mandatory "Reply STOP to unsubscribe" footers).
### Priority
P1 (High)
### Estimated Scope
Medium

---

## 7. Video Conferencing
### Title: Auto-Generated Meeting Links
### Problem Statement
When a business owner schedules an online consultation or lesson, they manually create a Zoom link and email it to the client, leading to lost links and confusion. The link should be generated and shared automatically when the meeting is booked.
### Research Report
- **Evaluated Tools:** Zoom API, Google Meet API, Daily.co.
- **Zoom API:** Very popular, but the OAuth flow and API can be clunky.
- **Google Meet:** Excellent if the user is already using Google Workspace.
- **Daily.co:** Incredibly easy developer experience for embedding video, but users usually prefer standard tools like Zoom/Meet.
- **Winner:** Integrating with **Google Meet** (via Calendar integration) and **Zoom** as primary options. This ties in closely with the Calendar & Scheduling feature.
- **Risks:** Managing OAuth tokens for Zoom securely.
- **Pricing:** Zoom requires a Pro plan for API access; Meet is included in Workspace.
- **Cloud/Standalone Support:** Yes.
### Design Doc
- **User Experience:** This is an extension of the Scheduling feature. When an owner sets up an event type (e.g., "Virtual Consultation"), they select "Location: Zoom." When a client books, OHC silently calls the Zoom API, generates a unique meeting room, and includes that specific URL in the calendar invite and confirmation email sent to the client.
- **Triggers:** A new virtual appointment is booked.
- **Actions:** Call Zoom/Meet API to create a meeting, attach the URL to the appointment record.
### Implementation Prompt
Enhance the scheduling system to support auto-generating video conferencing links via Zoom and Google Meet. When an appointment is booked that requires a virtual meeting, automatically generate a unique meeting link via the respective API and attach it to the calendar event and confirmation notifications. Securely manage the required OAuth tokens.
### Priority
P1 (High)
### Estimated Scope
Medium