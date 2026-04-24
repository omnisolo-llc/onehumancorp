# Research Report: OHC Tool Integrations

## Issue Brief 1: Calendar & Scheduling (Cal.com)

### Title
Integrate Cal.com for Automated Booking and Calendar Sync

### Problem Statement
Small business owners like Leo (Music Tutor) and Carlos (Handyman) struggle with manual back-and-forth messaging to schedule appointments. They need a simple, self-serve booking page that syncs with their personal calendars to prevent double-booking, without requiring them to navigate complex enterprise scheduling software.

### Research Report
**Evaluated Tool:** Cal.com
Cal.com is an open-source scheduling infrastructure platform.
- **Ease of Use for Non-Technical Users:** High. OHC can completely abstract the configuration, providing a simple 1-click generation of booking links directly within the OHC platform.
- **Pricing:** Generous free tier for individuals, which aligns perfectly with OHC's goal to support small businesses. API usage has predictable pricing.
- **Reputation:** Well-regarded open-source alternative to Calendly with modern API capabilities and webhook support.
- **Cloud vs. Standalone:** Supports both. In Cloud mode, we can use Cal.com's managed API. In Standalone mode, users could theoretically self-host or connect to their own Cal.com instance.

### Design Doc
- **Trigger:** Business owner activates the "Bookings" feature in the Operations Department.
- **Action:** OHC automatically provisions a Cal.com booking link via API, syncing it with the owner's Google or Apple Calendar. The Operations AI agent uses webhooks from Cal.com to track new bookings, cancellations, and reschedules.
- **User View:** The business owner sees a clean calendar view within their OHC dashboard (mobile and web) and can share a simple booking link on their storefront.

### Implementation Prompt
Implement the backend integration with Cal.com to generate booking links and listen for booking webhooks. Ensure the user interface allows the business owner to view their upcoming appointments and share their booking link. The solution must support mobile-first viewing (375px) and integrate seamlessly with the existing OHC dashboard.

### Priority
P0

### Estimated Scope
Medium


## Issue Brief 2: SMS & Notifications (Twilio)

### Title
Integrate Twilio for Global SMS Notifications and Order Alerts

### Problem Statement
Users like Fatima (Food Cart Operator) need immediate, reliable notifications on their phones when an order is placed, especially if they are operating in environments with poor data connections where push notifications might fail. SMS is a universal, reliable fallback.

### Research Report
**Evaluated Tool:** Twilio Programmable SMS
Twilio is the industry leader for programmatic SMS and voice communications.
- **Ease of Use for Non-Technical Users:** The complexity is entirely hidden. The user simply inputs their phone number to receive alerts, or enables SMS notifications for their customers.
- **Pricing:** Pay-as-you-go pricing. OHC can absorb costs in premium tiers or pass them through.
- **Reputation:** Highly reliable, excellent documentation, and global carrier coverage.
- **Cloud vs. Standalone:** Works excellently in Cloud mode. Standalone mode users would need to provide their own Twilio API keys.

### Design Doc
- **Trigger:** A customer places an order, or the Customer Success agent needs to send an urgent update.
- **Action:** OHC backend triggers a Twilio API call to send an SMS to the business owner or the end customer.
- **User View:** The business owner configures SMS alerts in the Operations tab with a simple toggle. Customers receive standard SMS texts with order updates.

### Implementation Prompt
Integrate the Twilio SDK to send SMS notifications for critical events (e.g., new order received, order ready for pickup). Add a toggle in the mobile-first OHC frontend allowing the business owner to enable or disable SMS alerts for themselves and their customers. Ensure phone numbers are validated before sending.

### Priority
P1

### Estimated Scope
Small


## Issue Brief 3: Email Marketing (Resend)

### Title
Integrate Resend for Transactional and Marketing Emails

### Problem Statement
Business owners like Priya (Boutique Owner) need to automatically email customers when new stock arrives or send professional-looking digital receipts. Traditional tools like Mailchimp are often too complex and expensive for simple workflows.

### Research Report
**Evaluated Tool:** Resend
Resend is a developer-first email API designed for modern applications.
- **Ease of Use for Non-Technical Users:** Invisible to the user. OHC's Marketing & Advertising agent will auto-draft and send emails via Resend.
- **Pricing:** 3,000 free emails per month, very affordable thereafter.
- **Reputation:** Known for high deliverability, great developer experience, and modern React Email support.
- **Cloud vs. Standalone:** Ideal for Cloud. Standalone mode can easily use custom API keys.

### Design Doc
- **Trigger:** The AI agent determines an email campaign is needed or a transactional event occurs.
- **Action:** OHC backend uses the Resend API to send beautifully formatted, mobile-responsive HTML emails.
- **User View:** The business owner simply clicks "Approve Campaign" generated by the AI. Customers receive polished, branded emails.

### Implementation Prompt
Implement an email service using the Resend API. Create backend endpoints to trigger transactional emails (receipts) and marketing campaigns. The AI agent should be able to draft campaign content and use this service to dispatch emails. Ensure all email templates are mobile-responsive.

### Priority
P1

### Estimated Scope
Medium


## Issue Brief 4: Social Media Integration (Meta Graph API)

### Title
Integrate Meta Graph API for Unified Instagram and Facebook Inbox

### Problem Statement
Maya (Home Baker) receives custom cake orders via Instagram DMs. Managing requests across multiple apps is overwhelming, and she needs her AI agent to automatically respond to common questions (e.g., "Do you do vegan cakes?") directly in Instagram.

### Research Report
**Evaluated Tool:** Meta Graph API (Messenger API for Instagram)
- **Ease of Use for Non-Technical Users:** Once authenticated via a simple OAuth flow, the user never has to leave OHC to manage DMs.
- **Pricing:** Free API usage.
- **Reputation:** The official and only reliable way to integrate with Meta properties.
- **Cloud vs. Standalone:** Requires registered Meta App, best managed centrally in Cloud mode.

### Design Doc
- **Trigger:** Customer sends a DM to the business's connected Instagram account.
- **Action:** Meta sends a webhook to OHC. The Customer Success AI agent processes the message, drafts a response, and sends it back via the API.
- **User View:** Business owner sees Instagram DMs in their OHC unified inbox and can manually override AI responses.

### Implementation Prompt
Implement the OAuth flow to connect user Instagram Business accounts using the Meta Graph API. Set up webhook listeners to receive incoming DMs and route them to the unified inbox and the Customer Success AI agent for automated drafting. Ensure the UI clearly indicates which messages are from Instagram.

### Priority
P0

### Estimated Scope
Large


## Issue Brief 5: Payment Processing (Mercado Pago)

### Title
Integrate Mercado Pago for LATAM Payment Processing

### Problem Statement
Small business owners in Latin America often cannot use Stripe due to regional restrictions or customer preference for local payment methods. They need a localized payment gateway to accept deposits and online orders.

### Research Report
**Evaluated Tool:** Mercado Pago API
- **Ease of Use for Non-Technical Users:** High. Users authenticate with their Mercado Pago account to start accepting local payments immediately.
- **Pricing:** Standard payment gateway fees, familiar to the target demographic.
- **Reputation:** The dominant payment processor in Latin America with extensive local payment method support.
- **Cloud vs. Standalone:** Supported in both; relies entirely on the user's connected account.

### Design Doc
- **Trigger:** A customer in a LATAM region proceeds to checkout.
- **Action:** OHC generates a Mercado Pago checkout preference and redirects the user to the secure payment flow. Webhooks confirm payment status.
- **User View:** The business owner connects their Mercado Pago account in the Finance & Payments tab. The customer sees local payment options at checkout.

### Implementation Prompt
Integrate the Mercado Pago API to support checkout sessions and webhook processing. Update the Finance & Payments UI to allow users to select Mercado Pago as their primary payment gateway if operating in supported LATAM countries.

### Priority
P2

### Estimated Scope
Medium


## Issue Brief 6: Shipping & Logistics (Shippo)

### Title
Integrate Shippo for Real-Time Shipping Rates and Label Generation

### Problem Statement
Business owners selling physical goods need to calculate accurate shipping costs at checkout and print shipping labels without manually copying customer addresses into a separate postal service website.

### Research Report
**Evaluated Tool:** Shippo API
- **Ease of Use for Non-Technical Users:** Very high. Automatically pulls in orders and generates labels with 1-click.
- **Pricing:** Pay-as-you-go per label or monthly subscription. Competitive carrier rates.
- **Reputation:** Excellent developer experience, supports over 85 global carriers.
- **Cloud vs. Standalone:** Best suited for Cloud. Standalone could work if the user supplies their own API key.

### Design Doc
- **Trigger:** A physical product order is placed, or the user clicks "Fulfill Order".
- **Action:** OHC requests shipping rates during checkout. Upon fulfillment, OHC uses the API to purchase and generate a PDF shipping label.
- **User View:** The business owner clicks "Print Label" on the order details page. The customer sees accurate shipping rates at checkout.

### Implementation Prompt
Implement the Shippo API to fetch real-time shipping rates during checkout for physical products. Create a backend flow to purchase labels and store the PDF securely in the OHC file storage. Add a "Print Label" button in the Operations dashboard.

### Priority
P1

### Estimated Scope
Large


## Issue Brief 7: Video Conferencing (Zoom)

### Title
Integrate Zoom API for Automated Online Lesson Links

### Problem Statement
Service providers like Leo (Music Tutor) who teach online need secure, unique video call links automatically generated and sent to students upon booking, avoiding the manual work of creating and emailing Zoom links.

### Research Report
**Evaluated Tool:** Zoom API
- **Ease of Use for Non-Technical Users:** Requires a one-time OAuth connection to their Zoom account.
- **Pricing:** Free to use the API; requires the user to have a Zoom account (free tier works for 40-min calls).
- **Reputation:** Industry standard for video conferencing.
- **Cloud vs. Standalone:** Cloud mode will handle the OAuth app centrally.

### Design Doc
- **Trigger:** A customer books an online service (e.g., guitar lesson).
- **Action:** OHC calls the Zoom API to create a meeting, retrieves the join URL, and attaches it to the booking record and calendar invite.
- **User View:** The business owner sees the Zoom link automatically attached to their upcoming appointments in the OHC dashboard.

### Implementation Prompt
Integrate the Zoom API via OAuth to allow business owners to connect their accounts. Update the booking creation flow to automatically generate a Zoom meeting for services marked as 'online'. Include the generated join link in the automated confirmation emails and the booking details UI.

### Priority
P2

### Estimated Scope
Medium
