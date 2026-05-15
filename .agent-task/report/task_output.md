# Scout Tool Integration Research Report

## [Social Media] Issue Brief: Unified Inbox for Instagram & Facebook DMs

**Title**: Scout 🔍: Integrate Meta Graph API for Unified Customer Inbox
**Problem Statement**:
Small business owners (like a boutique owner or a custom cake maker) receive orders and inquiries across multiple platforms, especially Instagram and Facebook DMs. Checking multiple apps constantly leads to missed messages and lost sales. They need a single, unified inbox within their dashboard where all customer messages appear in one place.
**Research Report**:
- **Tool**: Meta Graph API (Instagram Messaging API & Messenger API).
- **Evaluation**: This is the industry standard for connecting to Meta platforms. It supports webhooks for real-time messaging, allowing the business owner to receive and reply directly from our app.
- **Ease of Use**: High for the end-user. They authenticate via Facebook Login, grant page permissions, and messages start flowing into the app automatically.
- **Pricing**: Free to use the API. Costs only occur if running ad campaigns or using advanced WhatsApp Business tiers.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode via central webhooks. In Standalone mode, webhooks are challenging to receive behind a NAT, so polling or a cloud-relay architecture may be required.
**Design Doc**:
- A "Unified Inbox" section in the OHC platform.
- The business owner clicks "Connect Facebook/Instagram", logging into their Meta account.
- Incoming DMs from customers trigger a notification in OHC.
- The owner types a reply in OHC, which is sent back to the customer's Instagram or Facebook app.
**Implementation Prompt**:
Create a feature that allows a business owner to securely connect their Instagram and Facebook Business pages. Build a unified chat interface that displays incoming DMs and allows the owner to reply seamlessly. Ensure the experience handles connection drops and gracefully prompts the user to re-authenticate if their token expires.
**Priority**: P1
**Estimated Scope**: Large

---

## [Calendar & Scheduling] Issue Brief: Native Google Calendar Sync

**Title**: Scout 🔍: Integrate Google Calendar for Conflict-Free Booking
**Problem Statement**:
Service providers (like a music tutor or a handyman) need clients to book their time. Doing this manually via text or email leads to double-booking and timezone confusion. They need a simple booking widget that automatically respects their personal calendar availability.
**Research Report**:
- **Tool**: Google Calendar API.
- **Evaluation**: Direct integration with Google Workspace/Gmail accounts. Offers robust conflict checking and event generation.
- **Ease of Use**: Very high for the business owner. A simple OAuth "Sign in with Google" flow connects their calendar.
- **Pricing**: Free for the core scheduling operations within reasonable rate limits.
- **Cloud vs. Standalone**: Works seamlessly in both. Standalone users can authenticate directly using local loopback redirects for OAuth.
**Design Doc**:
- A "Booking & Schedule" module in the OHC platform.
- The business owner defines "Service Types" (e.g., 1-hour consultation) and connects their Google Calendar.
- The storefront displays available time slots based on the business owner's busy periods.
- Upon a customer booking, an event is instantly created in the owner's Google Calendar.
**Implementation Prompt**:
Build a booking configuration flow where a business owner can define services and availability. Integrate Google Calendar OAuth to pull busy times and prevent double-booking. When a customer confirms an appointment, generate a calendar invite for both parties and update the dashboard schedule.
**Priority**: P0
**Estimated Scope**: Medium

---

## [Email Marketing] Issue Brief: Automated Customer Campaigns via Resend

**Title**: Scout 🔍: Integrate Resend for Customer Newsletters and Updates
**Problem Statement**:
Small businesses struggle to keep their customers engaged. They want to send newsletters or promotions (e.g., "Holiday Sale!") to their client list but find traditional tools like Mailchimp too complex or expensive. They need a simple way to email their existing customers directly from their management dashboard.
**Research Report**:
- **Tool**: Resend.
- **Evaluation**: Resend provides a developer-friendly, modern email delivery platform. It offers high deliverability and easy domain verification.
- **Ease of Use**: High. The business owner types an email in a simple rich-text editor, selects their customer list, and hits send. Domain setup (DNS records) can be guided or fully managed by the platform.
- **Pricing**: Generous free tier (up to 3,000 emails/month), which is perfect for most small businesses. Affordable scaling thereafter.
- **Cloud vs. Standalone**: Great for Cloud using shared IP/domains or verified tenant domains. In Standalone, users might need to provide their own API key or SMTP credentials if a central relay isn't used.
**Design Doc**:
- A "Marketing" or "Campaigns" tab in the OHC platform.
- Business owners can view their unified customer list and select segments.
- They draft an email using a simple WYSIWYG editor.
- The system handles sending the emails out and tracking basic open rates.
**Implementation Prompt**:
Develop a simple email campaign tool that lets business owners draft messages and send them to their customer database. Integrate with an email delivery service to handle the actual sending. Provide basic feedback to the user on whether the emails were successfully delivered.
**Priority**: P2
**Estimated Scope**: Medium

---

## [Payment Processing] Issue Brief: Alternative Payment Gateways for LATAM

**Title**: Scout 🔍: Integrate Mercado Pago for Localized Payment Options
**Problem Statement**:
While Stripe is great, many business owners in Latin America rely on local payment methods like PIX, OXXO, or local credit cards that Stripe may not fully support or offer good rates on. They need a payment processor tailored to their region to ensure customers can actually complete their purchases.
**Research Report**:
- **Tool**: Mercado Pago API.
- **Evaluation**: The dominant payment gateway in LATAM, supporting country-specific payment methods seamlessly.
- **Ease of Use**: Moderate to High. The owner connects their Mercado Pago account via OAuth.
- **Pricing**: Transaction fees vary by country and settlement speed, but it is competitive and culturally trusted in the region.
- **Cloud vs. Standalone**: Fully functional in both modes, relying on webhook callbacks for payment confirmation.
**Design Doc**:
- A "Payments & Checkout" settings page.
- Option to enable "Mercado Pago" alongside or instead of other gateways.
- Customers on the storefront are redirected to a secure Mercado Pago checkout or use an embedded localized form.
- Successful payments instantly update the order status in OHC.
**Implementation Prompt**:
Provide a payment integration option for Mercado Pago. Create an onboarding flow for the business owner to link their account. Update the storefront checkout experience to offer Mercado Pago as a payment method, and reliably capture payment success to update the customer's order.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Shipping & Logistics] Issue Brief: Automated Shipping Labels & Rates

**Title**: Scout 🔍: Integrate Shippo for Real-Time Rates and Label Printing
**Problem Statement**:
Sellers of physical goods waste hours manually calculating shipping costs at the post office and writing out labels by hand. They need a system that calculates the correct shipping cost at checkout and lets them print a prepaid shipping label with one click when an order comes in.
**Research Report**:
- **Tool**: Shippo API.
- **Evaluation**: Shippo aggregates dozens of carriers (USPS, UPS, FedEx, DHL, etc.) into one platform, offering heavily discounted rates for small businesses.
- **Ease of Use**: Very High. The business owner enters package dimensions and weight; the platform does the rest.
- **Pricing**: Pay-as-you-go model (cents per label plus postage), which is highly accessible for low-volume shippers.
- **Cloud vs. Standalone**: Works identically in both environments via standard API calls.
**Design Doc**:
- A "Fulfillment" view within the Orders section.
- For physical orders, the owner clicks "Purchase Shipping Label."
- The system fetches the best rate, purchases the label, and provides a PDF for printing.
- Tracking numbers are automatically attached to the order and emailed to the customer.
**Implementation Prompt**:
Integrate a shipping aggregation service to allow business owners to generate and print shipping labels directly from their order dashboard. Automatically update the order status to "Shipped" and attach the tracking number once the label is purchased.
**Priority**: P1
**Estimated Scope**: Large

---

## [SMS & Notifications] Issue Brief: Reliable Customer SMS Alerts

**Title**: Scout 🔍: Integrate Twilio for Global SMS Notifications
**Problem Statement**:
Many customers, particularly in regions or demographics with lower email usage, prefer text messages. Business owners need to send critical updates (e.g., "Your table is ready", "Your appointment is tomorrow", "Your package is out for delivery") via SMS to ensure they are seen immediately.
**Research Report**:
- **Tool**: Twilio Programmable SMS.
- **Evaluation**: The industry leader in SMS. Highly reliable, global carrier coverage, and handles complex opt-out compliance (STOP messages) automatically.
- **Ease of Use**: Business owner sets up automated notification rules (e.g., "Send text when order ships"). They don't interact with Twilio directly.
- **Pricing**: Pay per message segment. Very cheap, though international rates vary.
- **Cloud vs. Standalone**: Easy to integrate in Cloud (OHC provides the Twilio account and bills the user). In Standalone, the user would need to provide their own Twilio API credentials.
**Design Doc**:
- A "Notifications" settings panel.
- Owner can toggle "Send SMS to customers for Order Updates".
- When an order state changes, the system triggers an SMS alert to the customer's provided phone number.
**Implementation Prompt**:
Build an SMS notification layer that business owners can enable for critical customer touchpoints (like order confirmations or appointment reminders). Ensure the system handles phone number formatting securely and respects user privacy.
**Priority**: P1
**Estimated Scope**: Medium

---

## [Video Conferencing] Issue Brief: Auto-Generated Meeting Links

**Title**: Scout 🔍: Integrate Google Meet for Seamless Online Consultations
**Problem Statement**:
Consultants, tutors, and coaches who offer online services struggle with manually creating video links and emailing them to clients before every session. They need a system where an online booking automatically generates a unique video call link and shares it with the client.
**Research Report**:
- **Tool**: Google Meet API (via Google Workspace/Calendar integration).
- **Evaluation**: Extremely widespread usage. No app installation required for clients (works in browser).
- **Ease of Use**: High. If the user has connected Google Calendar (see Calendar brief), Meet links can be generated automatically with the calendar event.
- **Pricing**: Free with any Google account.
- **Cloud vs. Standalone**: Works securely in both environments using the owner's OAuth tokens.
**Design Doc**:
- When defining a Service Type, the owner can mark it as "Online Video Call".
- Upon booking, the system creates a calendar event and requests a Meet conference link attached to it.
- Both the owner's dashboard and the customer's confirmation email display the bright "Join Video Call" button.
**Implementation Prompt**:
Extend the booking system to support virtual meetings. When an online service is booked, automatically generate a unique video conferencing link and distribute it to both the business owner and the customer via their confirmation notifications.
**Priority**: P2
**Estimated Scope**: Small
