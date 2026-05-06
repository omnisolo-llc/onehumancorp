# Tool Integration Research Report

## Evidence of Active Research
* Traces collected:
  * `curl -s https://api.github.com/repos/calcom/cal.com > .agent-task/report/calcom_api_trace.json`
  * `curl -s https://api.github.com/repos/twilio/twilio-node > .agent-task/report/twilio_api_trace.json`
  * `curl -s https://api.github.com/repos/resend/resend-node > .agent-task/report/resend_api_trace.json`
  * `curl -s https://api.github.com/repos/stripe/stripe-node > .agent-task/report/stripe_api_trace.json`
  * `curl -s "https://api.github.com/search/repositories?q=email+marketing" > .agent-task/report/email_marketing_search.json`
  * `curl -s "https://api.github.com/search/repositories?q=whatsapp+integration" > .agent-task/report/whatsapp_search.json`
  * `curl -s "https://api.github.com/search/repositories?q=payment+processing+mercadopago" > .agent-task/report/mercadopago_search.json`

---

## [Social Media Integration] WhatsApp Business Unified Inbox

**Title**: Implement WhatsApp Business Messaging into Unified Inbox

**Problem Statement**: Small business owners, especially those running local shops or offering personal services, receive countless customer inquiries via WhatsApp. Currently, they have to constantly switch between their personal phone, WhatsApp Web, and the OHC platform. This leads to missed messages, delayed responses, and lost sales. They need all customer messages in one place so they can reply quickly without juggling apps.

**Research Report**:
* **Tool Evaluated**: WhatsApp Cloud API / Twilio Messaging API
* **Ease of Use**: Once connected, the business owner simply reads and replies to messages in the OHC Inbox like an email. No technical knowledge required for daily use.
* **Advantages**: Massive global reach, especially in LATAM, India, and Europe. High open rates.
* **Risks**: Meta's business verification process can be tedious for small business owners. 24-hour reply window restriction for business-initiated messages.
* **Pricing**: Meta charges per conversation (varies by region, roughly $0.01 to $0.08). Twilio adds a small markup ($0.005/msg).
* **Environment Support**: Works in Cloud mode (webhooks easily received). Standalone mode requires a proxy or polling mechanism if the local machine isn't publicly addressable, making it slightly more complex but feasible with OHC Cloud routing.

**Design Doc**:
* **Trigger**: A customer sends a WhatsApp message to the business's linked number.
* **Action**: OHC receives the message and creates a new conversation thread in the owner's Unified Inbox.
* **User Interface**: The business owner sees a WhatsApp icon next to the message in their OHC Inbox. They type a reply and hit "Send." OHC routes the reply back to the customer's WhatsApp.
* **Why**: To consolidate communication channels so the owner never misses a lead.

**Implementation Prompt**:
* **Outcome**: A business owner can connect their WhatsApp Business account via the Integrations page. Once connected, incoming WhatsApp messages appear in the OHC Inbox. The owner can reply directly from the Inbox, and the customer receives the response on WhatsApp.
* **Acceptance Criteria**:
  * User can authenticate and connect WhatsApp Business.
  * Incoming messages create a notification and appear in the Inbox.
  * Outgoing replies are successfully delivered to the customer's WhatsApp.
  * The integration clearly explains the 24-hour response window to the owner.

**Priority**: P1
**Estimated Scope**: Large

---

## [Calendar & Scheduling] Cal.com Auto-Scheduling Sync

**Title**: Add Cal.com Integration for Automatic Customer Booking

**Problem Statement**: Service-based business owners (consultants, tutors, handymen) waste hours playing "phone tag" or emailing back and forth to find a time to meet with clients. They need a simple link they can share that automatically checks their availability and lets the customer book a slot without double-booking.

**Research Report**:
* **Tool Evaluated**: Cal.com (Open Source Calendly alternative)
* **Ease of Use**: Extremely easy. The owner connects their Google/Outlook calendar once, sets their working hours, and shares a link. Customers see a simple calendar interface.
* **Advantages**: Open-source, highly customizable, supports payments on booking, strong API.
* **Risks**: Calendar sync issues if the owner manually overrides events.
* **Pricing**: Free tier available for individuals. API usage for platforms might require a commercial agreement or self-hosting.
* **Environment Support**: Works perfectly in both Cloud and Standalone modes, as Cal.com handles the external calendar syncing and provides webhooks/API access.

**Design Doc**:
* **Trigger**: The business owner activates the "Scheduling" feature in OHC.
* **Action**: OHC provisions a Cal.com booking link for the business owner and syncs it with their existing calendar.
* **User Interface**: The owner gets a "Share Booking Link" button on their dashboard. When a customer books, an appointment card appears in the OHC "Upcoming Schedule" view.
* **Why**: To automate appointment setting, saving the owner time and providing a professional booking experience for the customer.

**Implementation Prompt**:
* **Outcome**: A business owner can generate a personalized booking link directly from OHC. They can view, reschedule, or cancel upcoming customer appointments within the OHC interface.
* **Acceptance Criteria**:
  * User can set available working hours in OHC.
  * OHC generates a functional booking link.
  * New bookings automatically appear in the OHC schedule.
  * Cancellations update the schedule dynamically.

**Priority**: P1
**Estimated Scope**: Medium

---

## [Email Marketing] Resend Campaign Manager

**Title**: Implement Simple Email Newsletters via Resend

**Problem Statement**: Small business owners want to send promotions or newsletters to their customer list, but tools like Mailchimp are too complicated and expensive. They just want to select their customers, write a simple message, and click send.

**Research Report**:
* **Tool Evaluated**: Resend (API-first email platform)
* **Ease of Use**: The OHC interface abstracts the complexity. The owner just sees a rich text editor and a "Send to All Customers" button.
* **Advantages**: Developer-friendly API, excellent deliverability, modern architecture.
* **Risks**: Domain verification (DKIM/SPF) is required for good deliverability, which is highly technical for a small business owner. We must automate or simplify this.
* **Pricing**: Free for up to 3,000 emails/month. Very affordable thereafter ($20 for 50k emails).
* **Environment Support**: Works in both Cloud and Standalone modes via outgoing API calls.

**Design Doc**:
* **Trigger**: The owner clicks "Create Campaign" in the Marketing tab.
* **Action**: OHC provides a simple editor, compiles the email list from the CRM, and sends the batch via Resend.
* **User Interface**: A clean, distraction-free writing area. A simple toggle to "Send to all past customers" or "Send to new leads." After sending, a dashboard shows "How many people opened this."
* **Why**: To help owners generate repeat business with zero technical setup.

**Implementation Prompt**:
* **Outcome**: The owner can draft a promotional email and send it to their customer list. They can see basic stats like how many people opened the email.
* **Acceptance Criteria**:
  * User can write an email with basic formatting (bold, links, images).
  * User can select a segment of their customer list.
  * Emails are delivered reliably to the recipients.
  * Open rates are tracked and displayed on the campaign dashboard.
  * Domain verification instructions are simplified and guided.

**Priority**: P2
**Estimated Scope**: Medium

---

## [Payment Processing] Mercado Pago Integration for LATAM

**Title**: Add Mercado Pago Support for Seamless LATAM Transactions

**Problem Statement**: Small business owners in Latin America often cannot use Stripe. They rely on Mercado Pago, Pix (Brazil), and local payment methods. Without this integration, OHC is effectively unusable for commerce in these massive markets, forcing owners to handle payments manually.

**Research Report**:
* **Tool Evaluated**: Mercado Pago API
* **Ease of Use**: Once the owner links their Mercado Pago account, customers can pay invoices or buy products using local credit cards, Pix, or cash deposits at convenience stores (e.g., Oxxo).
* **Advantages**: Dominant market share in LATAM, supports local payment cultures (installments, offline cash).
* **Risks**: API documentation can be fragmented. Settlement times and dispute resolution differ significantly from US norms.
* **Pricing**: Percentage per transaction (varies widely by country and payment method, e.g., 3-5%).
* **Environment Support**: Works in Cloud. In Standalone, webhooks for payment confirmation need a reliable tunnel or polling mechanism if the business is entirely offline.

**Design Doc**:
* **Trigger**: The business owner creates an invoice in OHC and sends it to a customer.
* **Action**: OHC generates a Mercado Pago checkout link. When paid, OHC marks the invoice as "Paid".
* **User Interface**: The owner sees a "Connect Mercado Pago" button in settings. Invoices display a "Pay Now" button for the customer. The owner's dashboard updates in real-time when money is received.
* **Why**: To unlock OHC for the massive LATAM small business market by supporting their default payment ecosystem.

**Implementation Prompt**:
* **Outcome**: A business owner in LATAM can connect their Mercado Pago account. Invoices sent from OHC include a checkout link. When the customer pays using local methods, the OHC invoice is automatically marked as paid.
* **Acceptance Criteria**:
  * User can authenticate and connect Mercado Pago.
  * Generated invoices provide valid Mercado Pago payment links.
  * Successful payments trigger an update in OHC to mark the invoice as paid.
  * Support for Pix (Brazil) and at least one local card method is confirmed.

**Priority**: P1
**Estimated Scope**: Large

---

## [Shipping & Logistics] EasyPost Label Generation

**Title**: Implement 1-Click Shipping Label Generation

**Problem Statement**: Boutique owners and local artisans spend hours copying customer addresses into carrier websites to buy shipping labels. They need to turn an OHC order into a printable shipping label with one click.

**Research Report**:
* **Tool Evaluated**: EasyPost API
* **Ease of Use**: The owner clicks "Buy Label" on an order, confirms the box weight, and prints. Extremely simple.
* **Advantages**: Aggregates USPS, UPS, FedEx, and international carriers. Real-time rate comparison.
* **Risks**: Requires accurate weight/dimension data from the owner to prevent chargebacks from carriers.
* **Pricing**: Free for less than 120,000 shipments/year (just pay carrier postage costs).
* **Environment Support**: Works perfectly in Cloud and Standalone (outgoing API calls to fetch rates and PDFs).

**Design Doc**:
* **Trigger**: An order is marked as "Ready to Ship".
* **Action**: OHC fetches the best carrier rate, purchases the label via EasyPost, and provides a PDF.
* **User Interface**: On the order details page, a button says "Buy Shipping Label." A modal asks for the package weight. It shows the price (e.g., "$4.50 via USPS"). Clicking confirm deducts the money and opens the PDF label for printing. Tracking numbers are automatically emailed to the customer.
* **Why**: To remove the manual data entry error risk and save time fulfilling physical product orders.

**Implementation Prompt**:
* **Outcome**: The business owner can purchase and print a shipping label directly from an order page. The customer automatically receives the tracking number.
* **Acceptance Criteria**:
  * User can enter package weight and dimensions.
  * System displays live shipping rates from major carriers.
  * User can purchase the label and download a printable PDF.
  * System automatically sends the tracking link to the customer.

**Priority**: P2
**Estimated Scope**: Medium

---

## [SMS & Notifications] Twilio Global SMS Alerts

**Title**: Enable Critical SMS Notifications for Customers

**Problem Statement**: Many customers ignore emails but read every text. For businesses like hair salons or auto repair shops, a missed appointment costs money. Owners need an automated way to text customers appointment reminders or "Your order is ready" alerts.

**Research Report**:
* **Tool Evaluated**: Twilio Programmable SMS
* **Ease of Use**: Completely invisible to the business owner once enabled. They just see a toggle saying "Send text reminders to customers."
* **Advantages**: Rock-solid reliability, global reach.
* **Risks**: A2P 10DLC compliance in the US requires businesses to register their brand to send texts, which is a massive regulatory headache for small business owners.
* **Pricing**: ~$0.0079 per message in the US, higher internationally.
* **Environment Support**: Fully supported in both Cloud and Standalone modes via API.

**Design Doc**:
* **Trigger**: An appointment is 24 hours away, or an order is marked "Ready for Pickup."
* **Action**: OHC dispatches a short SMS to the customer's phone number.
* **User Interface**: In the settings panel, the owner toggles "Enable SMS Reminders." On the customer's profile, the owner can see a log of texts sent.
* **Why**: To reduce no-shows and improve customer satisfaction with timely, unmissable updates.

**Implementation Prompt**:
* **Outcome**: Business owners can enable automated SMS reminders for appointments and order updates. Customers receive a simple text message at the appropriate time.
* **Acceptance Criteria**:
  * User can toggle SMS notifications on/off.
  * System automatically sends a reminder 24 hours before a scheduled event.
  * System handles invalid phone numbers gracefully without crashing.
  * OHC provides a simplified flow to help the owner register for US A2P 10DLC compliance if applicable.

**Priority**: P1
**Estimated Scope**: Large

---

## [Video Conferencing] Auto-generated Zoom Links for Meetings

**Title**: Automatic Zoom Link Generation for Online Services

**Problem Statement**: Tutors, therapists, and consultants who work online currently have to manually create a Zoom meeting, copy the link, and paste it into an email for every client. They need the system to do this automatically whenever an online appointment is booked.

**Research Report**:
* **Tool Evaluated**: Zoom API
* **Ease of Use**: The owner authorizes Zoom once. After that, any appointment marked "Online" automatically gets a link.
* **Advantages**: Zoom is universally recognized by customers. Reliable video quality.
* **Risks**: Zoom's OAuth approval process for marketplace apps requires strict security audits from our side.
* **Pricing**: API access is included in standard Zoom Pro accounts.
* **Environment Support**: Works in Cloud and Standalone modes via API.

**Design Doc**:
* **Trigger**: A new appointment is scheduled and the location is set to "Online/Video".
* **Action**: OHC calls the Zoom API, creates a unique meeting room, and attaches the link to the calendar invite.
* **User Interface**: The business owner's schedule view shows a "Join Meeting" button next to the appointment. The customer's confirmation email includes a large, clear "Click here to join video call" link.
* **Why**: To eliminate the manual administrative work of creating and sharing video links for service providers.

**Implementation Prompt**:
* **Outcome**: When an online appointment is booked, OHC automatically generates a unique Zoom link. Both the business owner and the customer can click a button to instantly join the meeting at the scheduled time.
* **Acceptance Criteria**:
  * User can authenticate their Zoom account via OAuth.
  * New online bookings automatically generate a unique Zoom meeting URL.
  * The Zoom link is included in customer confirmation emails.
  * The business owner can click "Join Meeting" directly from the OHC dashboard.

**Priority**: P2
**Estimated Scope**: Medium