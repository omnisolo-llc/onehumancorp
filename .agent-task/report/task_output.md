# Scout: Tool Integration Research Q4

## 1. Social Media Integration: Meta Business Suite (Instagram & Facebook DMs)
**Problem Statement**: Small business owners miss potential sales because they don't check their Instagram DMs, Facebook comments, or WhatsApp messages fast enough. They are too busy running their business to check 3 different apps.
**Research Report**:
Meta provides an official Graph API that covers Facebook Pages, Instagram Professional accounts, and WhatsApp Business. It's the standard for unifying messages.
- *Ease of Use*: High for the user (standard OAuth login), but complex to set up Meta app review on the backend.
- *Pricing*: Free for standard messaging API limits. WhatsApp Business has per-conversation pricing after a free tier.
- *Reputation*: Industry standard. Reliable webhooks but strict policy constraints (e.g. 24-hour reply window).
**Design Doc**:
- *Trigger*: User connects their Facebook/Instagram account via an OAuth modal in OHC settings.
- *Action*: OHC subscribes to webhooks for new messages and comments. Incoming messages are routed into the OHC Unified Inbox.
- *User Interface*: A new "Social Channels" tab in Settings. The Unified Inbox will show a small icon (Instagram, Facebook) next to incoming messages. The user can reply directly from OHC.
**Implementation Prompt**: Implement an OAuth connection flow for Meta Business Suite and a webhook handler that routes incoming DMs/comments into the existing OHC Unified Inbox. The user should be able to click "Connect Facebook/Instagram" in Settings, authenticate, and then immediately start seeing and replying to their social media messages within the OHC Inbox.
**Priority**: P0
**Estimated Scope**: Large
**Environment Support**: Cloud (Webhooks), Standalone (Requires ngrok/tunneling or long-polling if possible, else Cloud-only feature).

---

## 2. Calendar & Scheduling: Google Calendar
**Problem Statement**: Service-based businesses (like consultants, salons) struggle with double-booking. They want clients to book appointments directly on their website, but need it to sync seamlessly with their personal Google Calendar so they don't get double-booked.
**Research Report**:
Google Calendar API is the most widely used calendar sync tool.
- *Ease of Use*: Very easy for users (standard Google login).
- *Pricing*: Free tier is generous enough for almost all small businesses.
- *Reputation*: Highly reliable. Handles timezones perfectly.
**Design Doc**:
- *Trigger*: User connects their Google account in OHC Settings.
- *Action*: OHC reads free/busy times from the user's main calendar and blocks out those times on the OHC public booking widget. When a client books via OHC, an event is created on the user's Google Calendar.
- *User Interface*: A "Calendar Sync" section in Settings. The public website builder gets a "Booking Calendar" block that automatically respects the sync.
**Implementation Prompt**: Implement Google Calendar integration allowing users to sync their availability. Add a settings panel to connect a Google account. The system must fetch free/busy schedules to prevent double-booking and push new OHC-generated appointments as events to the user's Google Calendar.
**Priority**: P0
**Estimated Scope**: Medium
**Environment Support**: Cloud, Standalone.

---

## 3. Email Marketing: Mailchimp
**Problem Statement**: Business owners want to send monthly newsletters or promotions to their customer list, but they don't know how to export contacts, format HTML emails, or track who opened what. They just want a "Send blast to all customers" button.
**Research Report**:
Mailchimp offers a robust API for audience management and campaign creation.
- *Ease of Use*: Good, though their API has a learning curve.
- *Pricing*: Free tier up to 500 contacts, which is sufficient for many new small businesses.
- *Reputation*: Very strong, reliable delivery rates, strict spam compliance.
**Design Doc**:
- *Trigger*: User hits "Sync Contacts" or "Create Campaign" in the OHC Marketing tab.
- *Action*: OHC syncs the local customer database to a Mailchimp Audience. For campaigns, OHC can trigger a draft campaign creation via API.
- *User Interface*: A "Marketing" tab showing total synced contacts and recent campaign stats (open rate). A button to "Design Email in Mailchimp".
**Implementation Prompt**: Build a Mailchimp integration that automatically syncs the OHC customer list to a Mailchimp Audience. Whenever a new customer is added in OHC, they should be added to Mailchimp. Display basic metrics (total subscribers) in the OHC Marketing dashboard.
**Priority**: P1
**Estimated Scope**: Medium
**Environment Support**: Cloud, Standalone.

---

## 4. Payment Processing: Mercado Pago (LATAM focus)
**Problem Statement**: Stripe doesn't support many countries in Latin America well. Business owners in these regions need a localized payment gateway that supports local currency, installments, and local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
Mercado Pago is the dominant player in LATAM e-commerce.
- *Ease of Use*: Simple checkout integration, widely recognized by local consumers.
- *Pricing*: Variable by country, usually a percentage + flat fee per transaction.
- *Reputation*: Highly trusted in LATAM, excellent support for local payment methods.
**Design Doc**:
- *Trigger*: User selects "Mercado Pago" as their payment provider in OHC Store Settings.
- *Action*: OHC generates Mercado Pago checkout preferences and redirects buyers to their secure checkout page, then listens for IPN (Instant Payment Notification) webhooks.
- *User Interface*: In Settings -> Payments, an option to connect Mercado Pago. In the storefront, the checkout button says "Pagar con Mercado Pago".
**Implementation Prompt**: Implement Mercado Pago as an alternative payment gateway. Users should be able to connect their account via access tokens. The storefront checkout must be able to generate a Mercado Pago payment link and handle the return webhook to mark orders as paid.
**Priority**: P1
**Estimated Scope**: Medium
**Environment Support**: Cloud, Standalone.

---

## 5. Shipping & Logistics: Shippo
**Problem Statement**: E-commerce business owners hate going to the post office to figure out shipping costs. They want to automatically charge the customer the exact shipping rate and print the label from their desk.
**Research Report**:
Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into one API.
- *Ease of Use*: Excellent for the developer, abstracts away carrier specifics.
- *Pricing*: Pay-as-you-go (5¢ per label) or monthly subscription. Often provides discounted USPS rates.
- *Reputation*: Very reliable, great documentation.
**Design Doc**:
- *Trigger*: Customer enters their address at checkout (fetches live rates). Business owner clicks "Fulfill Order" in OHC dashboard.
- *Action*: OHC calls Shippo to get rates during checkout. During fulfillment, OHC calls Shippo to purchase a label and generates a PDF for the owner to print.
- *User Interface*: In the Order details view, a "Buy & Print Shipping Label" button. A modal shows carrier options and prices.
**Implementation Prompt**: Integrate Shippo to handle real-time shipping rates at checkout and label generation in the admin dashboard. The business owner should be able to view an order, select a box size, see rate quotes from USPS/UPS, purchase the label, and download the PDF directly within OHC.
**Priority**: P2
**Estimated Scope**: Large
**Environment Support**: Cloud, Standalone.

---

## 6. SMS & Notifications: Twilio
**Problem Statement**: Emails often get ignored. For urgent things like appointment reminders, order pickups, or quick updates, business owners (and their customers) prefer text messages. This is especially crucial for users with low English proficiency who rely on simple SMS.
**Research Report**:
Twilio is the industry leader for programmatic SMS.
- *Ease of Use*: Very easy API, hard part is dealing with A2P 10DLC compliance in the US.
- *Pricing*: Pay per message (fractions of a cent in US, varies globally).
- *Reputation*: Extremely reliable, global coverage.
**Design Doc**:
- *Trigger*: An appointment is coming up in 24 hours, or an order is marked "Ready for Pickup".
- *Action*: OHC calls Twilio API to send a templated SMS to the customer's phone number.
- *User Interface*: In Settings -> Notifications, checkboxes for "Send SMS reminders to customers". In the customer profile, a timeline shows sent SMS messages.
**Implementation Prompt**: Integrate Twilio SMS to send automated appointment reminders and order updates. Provide a settings panel for the owner to enable/disable SMS notifications. Ensure the system handles phone number formatting (E.164) and logs SMS delivery status in the customer's history.
**Priority**: P1
**Estimated Scope**: Medium
**Environment Support**: Cloud, Standalone.

---

## 7. Video Conferencing: Zoom
**Problem Statement**: Tutors, consultants, and therapists need to easily generate a video meeting link when a client books an online session. Manually creating links and emailing them is tedious and prone to errors.
**Research Report**:
Zoom API allows programmatic meeting creation.
- *Ease of Use*: OAuth flow is standard, but the API has many configuration options to simplify.
- *Pricing*: Free tier allows basic API usage; Pro accounts needed for longer meetings.
- *Reputation*: Ubiquitous, everyone knows how to use Zoom.
**Design Doc**:
- *Trigger*: A customer books a service marked as "Online Meeting".
- *Action*: OHC creates a Zoom meeting via API and attaches the join link to the booking confirmation email and calendar invite.
- *User Interface*: In Services setup, a toggle for "This is an online meeting (Zoom)". The upcoming appointments dashboard shows a "Join Meeting" button.
**Implementation Prompt**: Implement Zoom integration so that when a customer books a virtual service, a unique Zoom meeting is automatically generated. The user connects their Zoom account via OAuth. The resulting join link must be displayed in the business owner's dashboard and included in the customer's confirmation email.
**Priority**: P2
**Estimated Scope**: Medium
**Environment Support**: Cloud, Standalone.
