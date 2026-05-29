# Tool Integration Research Report

## 1. Social Media Integration

### Title
[Social Media] Unified Inbox Integration for Instagram, Facebook, and WhatsApp

### Problem Statement
Small business owners, like boutique shop managers, are overwhelmed by managing messages across multiple platforms (Instagram DMs, Facebook comments, WhatsApp). They often miss customer inquiries or take too long to reply because they have to switch between apps continuously.

### Research Report
- **Tool Evaluated:** Chatwoot (Open Source / Hosted) & Meta Business Suite API
- **Ease of Use:** High for the end user once connected, provides a single interface for all messages.
- **Pricing:** Chatwoot has a free tier; Meta APIs are generally free for standard messaging but have rate limits.
- **Reputation:** Chatwoot is well-regarded for unified inboxes and is already an optional component in OHC's stack.
- **Cloud/Standalone:** Works in both. In Cloud, it can use the hosted Chatwoot or a shared instance. In Standalone, it can connect via external APIs directly or use the user's own Chatwoot instance if set up.

### Design Doc
- **Triggers:** A new message arrives on Instagram, Facebook, or WhatsApp.
- **Actions:** The message is ingested into the OHC unified inbox.
- **User Interface:** The business owner sees all messages in a single chronological feed within the OHC dashboard and can reply directly from there.

### Implementation Prompt
Implement a unified inbox interface in OHC where users can link their Instagram, Facebook, and WhatsApp accounts. Provide a seamless OAuth flow for account linking. When a customer sends a message on any of these platforms, the message should appear in the OHC inbox, and the business owner's reply should be routed back to the correct platform.

### Priority
P1

### Estimated Scope
Large

---

## 2. Calendar & Scheduling

### Title
[Calendar] Automatic Meeting Link Generation and Sync with Google Calendar

### Problem Statement
Small business owners, such as consultants or tutors, waste time manually creating meeting links and managing calendar conflicts. They need an automated way to let clients book available slots that automatically generate meeting links.

### Research Report
- **Tool Evaluated:** Cal.com (Open Source / Hosted)
- **Ease of Use:** Extremely high for both the business owner and the client booking the meeting.
- **Pricing:** Cal.com has a generous free tier for individuals.
- **Reputation:** Highly respected in the developer community, solid alternative to Calendly.
- **Cloud/Standalone:** Works in both. Hosted Cal.com can be used for Cloud, and the open-source version can be integrated or self-hosted for Standalone.

### Design Doc
- **Triggers:** A client books a time slot via the business owner's public OHC booking page.
- **Actions:** The system creates a calendar event, checks for conflicts, and generates a meeting link.
- **User Interface:** The business owner sees a clean "Upcoming Meetings" widget in OHC, and clients see a simple booking calendar.

### Implementation Prompt
Integrate a scheduling feature where business owners can connect their Google Calendar or Outlook. Provide a public booking link for clients. When a booking is made, automatically sync it to the owner's calendar, resolve any timezone differences transparently, and auto-generate a video meeting link (e.g., Google Meet) attached to the calendar invite.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing

### Title
[Email] Integrated Campaign Management for Customer Outreach

### Problem Statement
Business owners want to send newsletters or promotions to their customer list but find complex tools like Mailchimp too expensive or complicated for simple announcements.

### Research Report
- **Tool Evaluated:** Resend or SendGrid
- **Ease of Use:** High for the business owner if OHC provides a simplified template editor.
- **Pricing:** Both have generous free tiers (e.g., Resend offers 3,000 emails/month free).
- **Reputation:** Resend is known for high deliverability and modern developer experience.
- **Cloud/Standalone:** Cloud can use a centralized API key with tenant isolation. Standalone users would need to provide their own API key or use a default OHC-provided relay with limits.

### Design Doc
- **Triggers:** The user drafts an email campaign and clicks "Send to All Customers".
- **Actions:** The system processes the customer list and dispatches the emails, handling unsubscribes.
- **User Interface:** A simple rich-text editor for composing emails and a basic dashboard showing open rates.

### Implementation Prompt
Build a simple email campaign tool within OHC. Allow the business owner to select customers from their CRM, write an email using a basic rich-text editor, and send it. Include basic open-rate tracking and automatic handling of unsubscribe links to ensure spam compliance.

### Priority
P2

### Estimated Scope
Medium

---

## 4. Payment Processing

### Title
[Payments] Localized Payment Provider Integration (Mercado Pago / Paytm)

### Problem Statement
Small business owners outside the US (e.g., LATAM, India) cannot always use Stripe effectively due to high fees or lack of local payment method support (like PIX or UPI).

### Research Report
- **Tool Evaluated:** Mercado Pago (LATAM) & Paytm (India)
- **Ease of Use:** Very high for the end consumer as it supports familiar local payment methods.
- **Pricing:** Competitive local rates compared to international gateways.
- **Reputation:** Dominant players in their respective regions.
- **Cloud/Standalone:** Works in both. Webhooks will need to be handled securely. Standalone mode might require a relay service for webhooks if the user is behind a NAT.

### Design Doc
- **Triggers:** A customer reaches the checkout step for an invoice or product.
- **Actions:** The system generates a localized payment link or QR code.
- **User Interface:** The business owner sees payment statuses (Pending, Paid) on their invoices in the OHC dashboard.

### Implementation Prompt
Expand the payment capabilities beyond Stripe to support Mercado Pago (for LATAM) and Paytm (for India). Allow the business owner to select their preferred regional payment gateway. When an invoice is generated, the customer should be able to pay using local methods (e.g., PIX, UPI), and the invoice status in OHC should automatically update to "Paid" upon successful settlement.

### Priority
P0

### Estimated Scope
Large

---

## 5. Shipping & Logistics

### Title
[Shipping] Real-time Rate Calculation and Label Generation

### Problem Statement
E-commerce business owners spend too much time manually calculating shipping costs at the post office and writing labels by hand.

### Research Report
- **Tool Evaluated:** Shippo or EasyPost
- **Ease of Use:** High; abstracts multiple carriers into one interface.
- **Pricing:** Pay-as-you-go per label (cents per label), plus carrier costs.
- **Reputation:** Both are industry standards for multi-carrier shipping APIs.
- **Cloud/Standalone:** Works perfectly in both as it's an API-driven service.

### Design Doc
- **Triggers:** An order is marked as "Ready to Ship".
- **Actions:** The system fetches rates, allows the user to select one, and generates a printable PDF label.
- **User Interface:** A "Create Label" button on the order details page, showing carrier options and prices, resulting in a downloadable PDF.

### Implementation Prompt
Add a shipping module that connects to a logistics provider like EasyPost or Shippo. When a business owner views an order, they should be able to click "Generate Shipping Label", compare rates from different carriers (e.g., USPS, FedEx, local carriers), purchase the label, and download it as a PDF directly from the OHC dashboard.

### Priority
P2

### Estimated Scope
Medium

---

## 6. SMS & Notifications

### Title
[SMS] Automated Order and Appointment Reminders via SMS

### Problem Statement
Many customers, particularly in demographics with lower English proficiency or limited email usage (like Fatima's clients), prefer or require SMS for order updates and appointment reminders to avoid missed sessions.

### Research Report
- **Tool Evaluated:** Twilio or MessageBird
- **Ease of Use:** Transparent to the business owner once enabled; automated.
- **Pricing:** Pay-per-message (usually a few cents per SMS).
- **Reputation:** Twilio is the gold standard for global SMS delivery.
- **Cloud/Standalone:** Cloud handles this easily. Standalone may require the user to input their own API credentials or buy a credit package via OHC.

### Design Doc
- **Triggers:** An appointment is coming up in 24 hours, or an order ships.
- **Actions:** The system dispatches an SMS template localized to the customer's preferred language.
- **User Interface:** A settings toggle for "Enable SMS Reminders" and a simple template editor for the message text.

### Implementation Prompt
Implement automated SMS notifications for critical events like upcoming appointments or shipped orders. Ensure the business owner can toggle this feature on/off and customize the message template. The system must reliably deliver these messages globally and respect standard SMS opt-out rules (e.g., handling "STOP" replies).

### Priority
P1

### Estimated Scope
Medium

---

## 7. Video Conferencing

### Title
[Video] Instant Zoom/Meet Link Generation for Online Consultations

### Problem Statement
Online tutors, therapists, and consultants need a seamless way to generate unique video meeting links for every booked session without manually copying and pasting URLs.

### Research Report
- **Tool Evaluated:** Zoom API & Google Meet (via Google Workspace API)
- **Ease of Use:** Flawless for the end user; links appear automatically.
- **Pricing:** Free tiers available for both, though Zoom has time limits on free accounts.
- **Reputation:** The most universally recognized video tools.
- **Cloud/Standalone:** Works in both via OAuth integrations.

### Design Doc
- **Triggers:** A new online consultation is booked.
- **Actions:** The system calls the respective API to create a meeting room and retrieves the join URL.
- **User Interface:** The meeting details page shows a prominent "Join Video Call" button for both the business owner and the client.

### Implementation Prompt
Create an integration with Zoom and Google Meet. When a business owner sets a service type as "Online Consultation," automatically generate a unique video conferencing link upon each new booking. Display a clear "Join Meeting" button in the OHC dashboard for the owner, and include the link in the confirmation email sent to the client.

### Priority
P1

### Estimated Scope
Small
