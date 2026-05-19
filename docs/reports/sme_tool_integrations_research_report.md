# Research Report: SME Tool Integrations

This report details evaluations and issue briefs for 7 categories of tools that solve real problems for small business owners. All tool evaluations take a user-first perspective (e.g. non-technical SME owners) and assess viability in both Cloud and Standalone environments.

## 1. Social Media Integration
**Tool Evaluated:** ManyChat

**Problem Statement:**
Business owners are overwhelmed tracking messages across Instagram, WhatsApp, and Facebook. They miss sales inquiries because they're checking three different apps. They need a single, unified inbox to manage all customer communications seamlessly without leaving their command center.

**Research Report:**
- **Overview:** ManyChat excels at unifying messaging channels (IG DMs, WhatsApp, FB Messenger). It is widely used by SMEs and highly rated for its simplicity.
- **Ease of Use:** Exceptional for non-technical users; intuitive setup process.
- **Pricing:** ~$15/month for Pro (very accessible for SMEs).
- **Reputation:** Market leader in omnichannel messaging for small businesses.
- **Hybrid Support:** Fully viable. Cloud environments can utilize standard webhooks; Standalone environments can poll or use a proxy service.

**Design Doc:**
- **Trigger:** A new message arrives on IG, WhatsApp, or FB.
- **Action:** The message appears instantly in the OHC unified inbox.
- **User Experience:** The business owner sees a combined feed of all messages. They can reply directly from OHC, and the response is routed to the correct platform seamlessly.

**Implementation Prompt:**
Implement an integration that pulls messages from Facebook, Instagram, and WhatsApp into a unified inbox view. The user must be able to authenticate their social accounts easily, view incoming messages in a single feed, and reply from within the OHC platform.

**Priority:** P0
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling
**Tool Evaluated:** Calendly

**Problem Statement:**
SMEs waste hours sending emails back and forth trying to find a meeting time. They need an automated way for clients to book available slots that syncs automatically with their existing Google or Outlook calendars, preventing double-booking and saving time.

**Research Report:**
- **Overview:** Calendly is the industry standard for scheduling.
- **Ease of Use:** Extremely user-friendly for both the business owner and their clients.
- **Pricing:** Free tier available; Pro tier is ~$10/month.
- **Reputation:** Exceptional; trusted globally.
- **Hybrid Support:** Viable. Cloud mode handles webhooks easily; Standalone mode can utilize API polling or local OAuth redirects.

**Design Doc:**
- **Trigger:** A client requests a meeting or needs to be scheduled.
- **Action:** OHC generates a personalized booking link and sends it to the client. When booked, the event is synced to the owner's calendar.
- **User Experience:** The owner shares a link (or OHC auto-sends it). The client picks a time, and it magically appears on the owner's calendar with a notification in OHC.

**Implementation Prompt:**
Integrate a scheduling tool that allows users to generate booking links based on their calendar availability. Ensure that booked events automatically reflect in the user's connected calendar and trigger a notification within OHC.

**Priority:** P1
**Estimated Scope:** Medium

---

## 3. Email Marketing
**Tool Evaluated:** Mailchimp

**Problem Statement:**
Business owners struggle to keep their customer lists organized and send professional-looking newsletters. They need a simple way to broadcast updates, promotions, and news to their audience without learning complex marketing software.

**Research Report:**
- **Overview:** Mailchimp is the quintessential entry-level email marketing platform for SMEs.
- **Ease of Use:** Drag-and-drop builder is highly accessible for non-technical users.
- **Pricing:** Generous free tier; paid plans start around $13/month.
- **Reputation:** Excellent; synonymous with SME email marketing.
- **Hybrid Support:** Viable in both modes using standard REST APIs.

**Design Doc:**
- **Trigger:** An SME wants to send a promotional email to their customer base.
- **Action:** OHC pushes the current customer list to Mailchimp and initiates a campaign draft.
- **User Experience:** The owner selects "Send Newsletter" in OHC, chooses a segment of their customers, and is directed to a pre-filled, simple template to write their message.

**Implementation Prompt:**
Create an integration that synchronizes the OHC customer list with an external email marketing platform. Provide a UI for the user to select customer segments and initiate an email campaign draft directly from the OHC dashboard.

**Priority:** P1
**Estimated Scope:** Medium

---

## 4. Payment Processing
**Tool Evaluated:** Mercado Pago (focusing on LATAM market)

**Problem Statement:**
For SMEs in Latin America, standard international payment gateways (like Stripe) often lack local currency support, have slow settlement times, or suffer high failure rates. They need a localized payment solution that works reliably for their regional customers.

**Research Report:**
- **Overview:** Mercado Pago dominates the LATAM payment space, offering localized payment methods (e.g., PIX in Brazil, OXXO in Mexico).
- **Ease of Use:** Familiar and trusted by LATAM consumers and merchants.
- **Pricing:** Transaction-based; competitive for the region.
- **Reputation:** Highly trusted in its target markets.
- **Hybrid Support:** Viable. APIs are robust for both Cloud (webhooks) and Standalone (polling/redirects) modes.

**Design Doc:**
- **Trigger:** A customer reaches the checkout stage or an invoice is generated.
- **Action:** OHC generates a localized payment link via Mercado Pago.
- **User Experience:** The owner creates an invoice in OHC. OHC provides a local payment link that the customer can pay using familiar local methods. The invoice is automatically marked "Paid" in OHC when the transaction clears.

**Implementation Prompt:**
Implement a payment gateway integration specifically tailored for localized payment methods (e.g., Mercado Pago for LATAM). Enable users to generate payment links for invoices and automatically update invoice status upon successful payment.

**Priority:** P2
**Estimated Scope:** Large

---

## 5. Shipping & Logistics
**Tool Evaluated:** Shippo

**Problem Statement:**
E-commerce SMEs spend excessive time manually calculating shipping rates, printing labels, and tracking packages. They need an automated system that handles shipping logistics seamlessly from the moment an order is placed.

**Research Report:**
- **Overview:** Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into a single API.
- **Ease of Use:** Simplifies complex logistics into a clean dashboard.
- **Pricing:** Pay-as-you-go model (cents per label); highly attractive for small volume shippers.
- **Reputation:** Strong reputation among independent e-commerce merchants.
- **Hybrid Support:** Fully viable via standard REST APIs in both Cloud and Standalone environments.

**Design Doc:**
- **Trigger:** An order is marked as "Ready to Ship" in OHC.
- **Action:** OHC fetches real-time rates, generates a shipping label, and retrieves a tracking number.
- **User Experience:** The owner clicks "Ship Item". They see the cheapest rate, click confirm, and a label prints. The customer automatically receives a tracking link.

**Implementation Prompt:**
Integrate a shipping aggregator to provide real-time shipping rates and label generation within OHC. Users should be able to view carrier options, purchase a label, and automatically email tracking information to the customer.

**Priority:** P2
**Estimated Scope:** Medium

---

## 6. SMS & Notifications
**Tool Evaluated:** Twilio

**Problem Statement:**
Many SMEs serve customers with low email engagement or limited English proficiency (e.g., Fatima's salon). They rely heavily on SMS for appointment reminders and critical updates, and need a reliable way to send text messages automatically.

**Research Report:**
- **Overview:** Twilio is the global leader in programmable SMS.
- **Ease of Use:** While the API is developer-focused, the end-user experience (receiving a text) is universal and simple.
- **Pricing:** Very cheap; fractions of a cent per message.
- **Reputation:** Industry standard for reliability and global carrier coverage.
- **Hybrid Support:** Fully viable in both modes.

**Design Doc:**
- **Trigger:** An appointment is booked, or a critical update occurs.
- **Action:** OHC sends a customized SMS via Twilio to the customer.
- **User Experience:** The business owner configures automated reminders in OHC (e.g., "Remind 24hrs before"). The customer receives a simple text message reminder, reducing no-shows without the owner lifting a finger.

**Implementation Prompt:**
Implement an SMS notification system allowing business owners to configure automated text alerts for specific events (like appointment reminders or shipping updates). The system must handle global phone number formats and provide delivery status indicators.

**Priority:** P0
**Estimated Scope:** Small

---

## 7. Video Conferencing
**Tool Evaluated:** Zoom

**Problem Statement:**
SMEs offering online consultations or lessons waste time manually creating Zoom meetings, copying the links, and emailing them to clients. They need meetings to be auto-generated and attached to calendar invites effortlessly.

**Research Report:**
- **Overview:** Zoom is the ubiquitous video conferencing tool.
- **Ease of Use:** Universally understood by consumers.
- **Pricing:** Free tier available; Pro is ~$15/month.
- **Reputation:** The default choice for online meetings.
- **Hybrid Support:** Viable. OAuth flow works well in Cloud, and can be adapted for Standalone mode via local redirects.

**Design Doc:**
- **Trigger:** An online meeting/appointment is scheduled in OHC.
- **Action:** OHC automatically creates a Zoom meeting and attaches the join link to the calendar invite.
- **User Experience:** When an owner schedules a "Virtual Consultation," OHC instantly generates a Zoom link and emails it to the client. At the meeting time, the owner clicks a "Join Now" button directly inside OHC.

**Implementation Prompt:**
Integrate a video conferencing tool to automatically generate meeting links when a virtual appointment is scheduled. Ensure the meeting link is automatically embedded in calendar invitations and accessible via a "Join" button in the OHC dashboard.

**Priority:** P1
**Estimated Scope:** Medium
