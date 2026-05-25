# Tool Integration Research Report

## 1. Social Media Integration: Meta Business Suite (Instagram/WhatsApp)
**Title**: Integrate Meta Platforms for Unified Social Inbox
**Problem Statement**: Small business owners are overwhelmed managing DMs across Instagram, Facebook, and WhatsApp. They miss messages, lose leads, and struggle to reply quickly, hurting their customer relationships and sales.
**Research Report**:
- **Findings & Competitive Analysis**: Meta provides the official Graph API for Instagram, Facebook, and WhatsApp Business. Alternatives like ManyChat are powerful but add extra subscription costs for users. Connecting directly to Meta is the most robust long-term solution.
- **Problem Solved**: Centralizes all messages into one unified inbox within OHC.
- **Ease of Use**: Once the initial OAuth flow is completed by the user, the experience is seamless.
- **Advantages**: Official API, most reliable, covers the most popular platforms.
- **Risks**: Meta's OAuth and App Review process is notoriously strict and complex. Webhook delivery can sometimes be delayed during Meta outages.
- **Pricing**: Free for Instagram/Facebook DMs. WhatsApp Business API has per-conversation pricing after a free tier (e.g., ~$0.015 - $0.08 per conversation depending on region).
- **Cloud/Standalone Support**: Works in Cloud mode via central webhook reception. Can work in Standalone mode if the standalone instance is exposed to the internet (e.g., via ngrok/Cloudflare Tunnels) to receive webhooks, or via long-polling if available.
**Design Doc**:
The integration will use a centralized webhook receiver. The business owner will click "Connect Instagram/WhatsApp" in the OHC settings, navigating through the Meta OAuth flow. Upon successful connection, OHC will subscribe to new message webhooks. When a message arrives, OHC will create a notification and show it in a unified inbox UI. Replies sent from the OHC inbox will be pushed back via the Meta Graph API.
**Implementation Prompt**:
Create a feature that allows a business owner to connect their Instagram and WhatsApp accounts. Once connected, all incoming messages should appear in a new "Unified Inbox" section in OHC. The user should be able to read and reply to these messages directly from OHC, with replies showing up instantly for the customer on the respective platform. Acceptance criteria include a working OAuth connection flow, real-time message receiving, and successful message sending.
**Priority**: P0
**Estimated Scope**: Large

## 2. Calendar & Scheduling: Calendly API
**Title**: Integrate Calendly for Automated Appointment Booking
**Problem Statement**: Back-and-forth emails to schedule meetings, consultations, or lessons waste hours each week. Business owners need a simple way to let clients book available times without manual coordination.
**Research Report**:
- **Findings & Competitive Analysis**: Calendly is the industry standard with massive brand recognition. Competitors like Acuity Scheduling are good but Calendly has simpler APIs and better user familiarity.
- **Problem Solved**: Eliminates scheduling back-and-forth by letting clients book directly into the owner's calendar.
- **Ease of Use**: Very high. Most clients already know how to use Calendly links.
- **Advantages**: Brand trust, excellent timezone handling, robust conflict resolution with existing calendars (Google/Outlook).
- **Risks**: Dependency on a third-party service; if Calendly goes down, booking stops.
- **Pricing**: Has a free tier. Paid tiers start around $10/month for advanced features like automated reminders.
- **Cloud/Standalone Support**: Works perfectly in Cloud mode. Works in Standalone mode via API polling or webhooks if the local instance is exposed.
**Design Doc**:
The user connects their Calendly account via OAuth. OHC imports the user's active Calendly event types. On the OHC dashboard, the user can easily copy their booking links to send to clients. OHC listens for Calendly webhooks to automatically create "Upcoming Appointment" records in the OHC CRM whenever a client books a slot.
**Implementation Prompt**:
Build an integration that lets the business owner connect their Calendly account. OHC should display their available scheduling links so they can easily copy them. When a customer books a meeting via Calendly, OHC should automatically log that upcoming meeting on the OHC dashboard and link it to the customer's CRM profile. Acceptance criteria include successful account linking and real-time appointment syncing into OHC.
**Priority**: P1
**Estimated Scope**: Medium

## 3. Email Marketing: Mailchimp
**Title**: Integrate Mailchimp for Seamless Email Campaigns
**Problem Statement**: Business owners manually export CSVs of their customer data from their CRM to their email tool, leading to outdated lists and missed marketing opportunities.
**Research Report**:
- **Findings & Competitive Analysis**: Mailchimp is highly recognizable and user-friendly for non-technical users. Alternatives like SendGrid or AWS SES are too developer-focused. ConvertKit is good but skewed towards creators rather than general retail/service businesses.
- **Problem Solved**: Keeps the email marketing list automatically in sync with the OHC customer database.
- **Ease of Use**: Mailchimp's drag-and-drop builder is excellent for beginners.
- **Advantages**: Great templates, strong analytics, built-in spam compliance and unsubscribe handling.
- **Risks**: Mailchimp pricing scales steeply as the audience grows.
- **Pricing**: Free tier up to 500 contacts. Starts at $13/month thereafter.
- **Cloud/Standalone Support**: Fully supported in both Cloud and Standalone modes via outgoing REST API calls.
**Design Doc**:
The business owner connects their Mailchimp account via OAuth in the settings. They map an OHC customer group (e.g., "All Active Customers") to a specific Mailchimp Audience list. Whenever a new customer is added to OHC or an email address is updated, OHC automatically triggers a background job to push the update to the Mailchimp API.
**Implementation Prompt**:
Develop a feature that syncs OHC contacts with Mailchimp. The user should be able to authenticate with Mailchimp and select a target audience list. OHC should automatically add new customers to this Mailchimp list in the background. Acceptance criteria include successful authentication, initial sync of existing customers, and real-time syncing of newly added customers without manual intervention.
**Priority**: P1
**Estimated Scope**: Medium

## 4. Payment Processing: Mercado Pago (LATAM Focus)
**Title**: Integrate Mercado Pago for LATAM Payment Processing
**Problem Statement**: Stripe is not available or heavily used in many Latin American countries. Business owners in these regions need a trusted local payment gateway that supports local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- **Findings & Competitive Analysis**: Mercado Pago dominates the LATAM market. Stripe is expanding but lacks the deep local penetration and alternative payment method support of Mercado Pago.
- **Problem Solved**: Allows LATAM businesses to accept digital payments easily using methods their customers trust.
- **Ease of Use**: Very familiar to LATAM users; checkout flows are highly optimized.
- **Advantages**: Supports local cash-based and bank-transfer payment methods natively. High brand trust in the region.
- **Risks**: Settlement speeds can vary by country. API documentation is sometimes fragmented compared to Stripe.
- **Pricing**: Varies by country, typically around 3-5% per transaction depending on when the merchant chooses to settle funds.
- **Cloud/Standalone Support**: Works in Cloud mode. Standalone mode requires webhook reception capabilities (exposing local instance) to receive asynchronous payment success notifications (like Pix payments).
**Design Doc**:
The user adds their Mercado Pago access credentials in the OHC billing settings. When generating an invoice in OHC, a new option "Pay via Mercado Pago" becomes available. Selecting this generates a unique payment link via the Mercado Pago API. OHC tracks the payment status via incoming webhooks and marks the OHC invoice as "Paid" automatically.
**Implementation Prompt**:
Add Mercado Pago as an alternative payment gateway. The business owner should be able to enter their API keys. When creating a customer invoice, there should be a button to "Generate Mercado Pago Link". When the customer pays via that link, the invoice in OHC should automatically update to "Paid". Acceptance criteria include generating valid payment links and correctly updating invoice status upon payment completion.
**Priority**: P2
**Estimated Scope**: Large

## 5. Shipping & Logistics: Shippo
**Title**: Integrate Shippo for Automated Label Generation
**Problem Statement**: Creating shipping labels manually on carrier websites (USPS, UPS) is incredibly tedious and prone to typos, costing product-based businesses hours per day.
**Research Report**:
- **Findings & Competitive Analysis**: Shippo and EasyPost are the main contenders. Shippo has a slightly more business-owner-friendly dashboard if they need to log in directly, whereas EasyPost is very developer-centric.
- **Problem Solved**: Automatically calculates rates and generates printable shipping labels directly from OHC orders.
- **Ease of Use**: Very simple; aggregates dozens of carriers into one interface.
- **Advantages**: Excellent carrier coverage (global), negotiated discounted rates for USPS/UPS built-in.
- **Risks**: Physical printing issues (printer alignment) are a common user complaint that OHC can't easily fix.
- **Pricing**: Pay-as-you-go model (e.g., $0.05 per label plus postage), or $10/month for no per-label fee.
- **Cloud/Standalone Support**: Fully supported in both Cloud and Standalone modes via outgoing REST API calls.
**Design Doc**:
The user connects their Shippo account. When an order is marked as "Ready to Ship" in OHC, the user inputs the package weight and dimensions. OHC queries the Shippo API to show shipping rates. The user selects a rate, and OHC calls Shippo to purchase the label, downloading the PDF directly to the OHC UI for the user to print. Tracking numbers are automatically saved to the order.
**Implementation Prompt**:
Create a shipping fulfillment flow using Shippo. For any physical order, the user should be able to click "Create Shipping Label", see price options from carriers, and purchase the label. OHC should display the printable PDF label and automatically save the tracking number to the order details. Acceptance criteria include successfully fetching rates, purchasing a test label, and displaying the tracking link.
**Priority**: P2
**Estimated Scope**: Medium

## 6. SMS & Notifications: Twilio
**Title**: Integrate Twilio for Reliable SMS Notifications
**Problem Statement**: Email open rates are low. For urgent updates (like appointment reminders or order ready for pickup), business owners need to send SMS messages to ensure customers actually see them, especially for older or non-English speaking demographics.
**Research Report**:
- **Findings & Competitive Analysis**: Twilio is the global leader. Plivo and MessageBird are alternatives, but Twilio's reliability, documentation, and global carrier network are unmatched.
- **Problem Solved**: Ensures critical notifications are seen immediately by customers.
- **Ease of Use**: Business owners just need to buy a number; the sending happens invisibly in the background.
- **Advantages**: Near 100% global delivery rate, highly scalable.
- **Risks**: SMS compliance (10DLC regulations in the US) is very strict and requires business registration, which can be a huge hurdle for small businesses.
- **Pricing**: ~$0.0079 per message in the US, higher internationally. Plus monthly phone number rental (~$1.15/mo).
- **Cloud/Standalone Support**: Fully supported in both Cloud and Standalone modes via outgoing REST calls.
**Design Doc**:
In settings, the user goes through a guided flow to purchase a Twilio phone number (or OHC provides a shared pool in Cloud mode, handling 10DLC behind the scenes). The user can toggle "Send SMS Reminders" on appointment types or order statuses. OHC uses the Twilio API to dispatch standard templated messages based on these triggers.
**Implementation Prompt**:
Build an SMS notification system. The business owner should be able to enable SMS alerts for key events (like "Appointment Tomorrow"). When the event occurs, OHC should automatically send a text message to the customer's phone number. Acceptance criteria include successfully sending an SMS to a test number and providing a UI toggle to turn the feature on/off.
**Priority**: P1
**Estimated Scope**: Medium

## 7. Video Conferencing: Zoom
**Title**: Integrate Zoom for Auto-Generated Online Meeting Links
**Problem Statement**: Tutors, consultants, and therapists waste time manually creating Zoom meetings and emailing links to clients before every session.
**Research Report**:
- **Findings & Competitive Analysis**: Zoom is universally understood by consumers. Google Meet is an alternative, but Zoom is often preferred for dedicated paid sessions due to recording and waiting room features.
- **Problem Solved**: Automatically attaches a unique video meeting link to scheduled appointments.
- **Ease of Use**: Extremely high for the end consumer.
- **Advantages**: Universal brand recognition, robust connection stability, great host controls.
- **Risks**: Zoom requires users to download an app (unlike Meet which is purely browser-based), which sometimes causes friction for less tech-savvy clients right before a meeting.
- **Pricing**: Free tier has a 40-minute limit. Pro tier is ~$15/month.
- **Cloud/Standalone Support**: Fully supported in both Cloud and Standalone modes via outgoing API calls.
**Design Doc**:
The user links their Zoom account via OAuth. When creating a new appointment or event in OHC, they can check a box saying "Make this an online meeting". OHC instantly calls the Zoom API to create a scheduled meeting and saves the `join_url` to the OHC appointment record. This link is then included in any automated confirmation emails or SMS sent to the client.
**Implementation Prompt**:
Add a Zoom integration for online appointments. The business owner should be able to connect their Zoom account. When scheduling a client in OHC, there should be an option to auto-generate a Zoom link. If selected, OHC should create the meeting in Zoom and display the join link on the appointment details page. Acceptance criteria include successful account linking, successful meeting creation via API, and displaying the correct join link to the user.
**Priority**: P2
**Estimated Scope**: Medium
