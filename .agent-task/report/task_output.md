# Tool Integration Research Report

## Executive Summary
This report details the evaluation of seven crucial tool categories designed to empower small business owners using the OHC platform. The primary objective is to identify integrations that reduce friction, automate manual workflows, and expand operational capabilities without requiring technical expertise from the user. We have evaluated tools across Social Media, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS Notifications, and Video Conferencing.

## 1. Social Media: TikTok Unified Inbox
### Title
Native TikTok Inbox Integration

### Problem Statement
Small business owners struggle to keep up with customer messages and comments across multiple platforms, specifically losing track of engagement on TikTok. They need a single place to view and reply to all messages without constantly switching apps.

### Research Report
*   **Strategy**: Build a native integration with the TikTok API to pull comments and direct messages into the unified OHC inbox.
*   **Target Persona**: E-commerce merchants, content creators.
*   **Advantages**: TikTok is a massive source of discovery. Managing engagement natively in OHC saves time and prevents lost leads.
*   **Risks**: TikTok API access can be strict, and managing high volumes of comments requires robust backend processing.
*   **Pricing**: API access is generally free for authorized apps, but requires maintaining compliance with TikTok's developer terms.
*   **Compatibility**: Cloud (OAuth). Standalone (OAuth).

### Design Doc
*   Users connect their TikTok account via a "Connect TikTok" button in OHC Settings.
*   Webhooks receive incoming comments and direct messages and route them to a unified "Inbox" view in the OHC app alongside Instagram and WhatsApp.
*   Replies typed in OHC are pushed back to TikTok natively.
*   The AI agent can help categorize or suggest replies for common questions.

### Implementation Prompt
Implement a unified inbox integration with TikTok. Allow users to connect their TikTok accounts. Incoming comments and DMs must appear in the OHC chronological feed. Users must be able to send replies from within OHC.
- **Acceptance Criteria**: Connect TikTok account. Receive a test comment/DM in the OHC inbox. Send a reply from OHC that appears on TikTok.

### Priority
P2

### Estimated Scope
Medium

---

## 2. Calendar & Scheduling: Microsoft Outlook
### Title
Native Integration with Outlook Calendar for Scheduling

### Problem Statement
While some small business owners use Google Calendar, a significant portion rely on Microsoft Outlook. They need appointments booked through OHC to automatically block out time on their Outlook calendar to prevent double-booking. They do not want to use complex third-party tools like Calendly.

### Research Report
*   **Strategy**: Direct integration with the Microsoft Graph API to sync Outlook Calendars.
*   **Target Persona**: Professional services (consultants, tutors, accountants).
*   **Advantages**: Provides parity with Google Calendar integration, catering to businesses ingrained in the Microsoft ecosystem.
*   **Risks**: Microsoft Graph API OAuth flow can be complex. Maintaining real-time bidirectional sync requires robust polling or webhook infrastructure.
*   **Pricing**: Free API access via Microsoft Graph.
*   **Compatibility**: Cloud (OAuth). Standalone (OAuth).

### Design Doc
*   In the "Calendar & Scheduling" settings, users can click "Connect Outlook".
*   Once connected, the OHC booking widget checks the user's Outlook calendar for free/busy slots.
*   When a customer books a service, an event is created directly on the user's Outlook Calendar.
*   If the user deletes or modifies the event in Outlook, webhooks sync the changes back to OHC.

### Implementation Prompt
Implement a bidirectional sync between OHC and Microsoft Outlook Calendar. Ensure free/busy checks accurately reflect the external calendar and that OHC appointments correctly generate Outlook events.
- **Acceptance Criteria**: Connect Outlook account. Booking an OHC service creates an event in Outlook. Deleting the event in Outlook frees the slot in OHC.

### Priority
P1

### Estimated Scope
Medium

---

## 3. Email Marketing: MailerLite
### Title
Native Email Campaigns via MailerLite Integration

### Problem Statement
Small business owners want to send newsletters and promotional emails to their customer base. While SendGrid/SES are great for transactional emails, building a full drag-and-drop template editor internally is too complex. MailerLite provides a user-friendly API for managing campaigns without forcing the user to leave OHC.

### Research Report
*   **Strategy**: API integration with MailerLite for subscriber management and campaign sending.
*   **Target Persona**: Retail, boutique owners, service providers.
*   **Advantages**: Offloads the complexity of email rendering and template design while keeping the trigger points natively in OHC.
*   **Risks**: MailerLite API rate limits; synchronizing customer lists requires background jobs.
*   **Pricing**: Free tier up to 1,000 subscribers, affordable thereafter.
*   **Compatibility**: Cloud (OAuth or API Key). Standalone (API Key).

### Design Doc
*   In OHC Settings, users input their MailerLite API key (or OAuth in Cloud mode).
*   Customer emails collected during checkout/booking in OHC are automatically synced to a specific MailerLite group.
*   Users can trigger pre-built campaigns natively from OHC (e.g., "Send 'New Collection' email"), which makes API calls to MailerLite to dispatch the campaign.
*   Basic analytics (open rate, click rate) are fetched via API and displayed on the OHC Marketing dashboard.

### Implementation Prompt
Integrate MailerLite to handle email marketing campaigns. Automatically sync OHC customer records to MailerLite subscriber groups. Provide a native UI to trigger specific email campaigns and view basic delivery analytics without needing to log into MailerLite directly.
- **Acceptance Criteria**: Sync a customer email to MailerLite. Trigger an email campaign from OHC. View campaign open/click rates in OHC.

### Priority
P2

### Estimated Scope
Medium

---

## 4. Payment Processing: Alipay
### Title
Expand Payments with Alipay Integration

### Problem Statement
Small business owners targeting the Chinese market or catering to Chinese tourists locally need to accept Alipay. Western payment methods like Stripe often do not adequately support direct local currency transactions for these users, leading to lost sales and poor conversion rates.

### Research Report
*   **Strategy**: Direct API integration with Alipay Global to facilitate cross-border and local transactions.
*   **Target Persona**: Retail businesses in tourist hubs, e-commerce stores shipping internationally.
*   **Advantages**: Unlocks a massive demographic that relies almost exclusively on Alipay or WeChat Pay. Native integration prevents the user from navigating clunky third-party gateways.
*   **Risks**: Alipay's integration documentation can be fragmented; requires specific business entity verification for cross-border payments.
*   **Pricing**: Standard transaction fees apply.
*   **Compatibility**: Cloud (Centralized routing). Standalone (User supplies API keys).

### Design Doc
*   In the "Finance & Payments" settings, users can enable Alipay.
*   The setup process requires providing merchant details for Alipay verification.
*   During checkout, if the user selects Alipay, a QR code is generated (for desktop/in-person) or a deep link opens the Alipay app (on mobile).
*   Webhooks notify OHC when the payment is completed to automatically update the order status.

### Implementation Prompt
Add Alipay as an alternative payment provider. Implement the QR code generation and mobile app deep-linking flow for checkout. Ensure that payment success webhooks accurately map to standard OHC order fulfillment events.
- **Acceptance Criteria**: Merchant can configure Alipay. Customers can select Alipay at checkout and complete a transaction via QR code or app redirect. Webhooks successfully mark the order as paid.

### Priority
P2

### Estimated Scope
Large

---

## 5. Shipping & Logistics: ShipStation
### Title
Automated Label Generation via ShipStation Integration

### Problem Statement
Small business owners selling physical goods spend too much time copying customer addresses into different carrier websites to find the best rate and print labels. They need a single button natively within OHC to calculate rates, buy postage, and print labels.

### Research Report
*   **Strategy**: API integration with ShipStation to aggregate carriers and print labels.
*   **Target Persona**: E-commerce stores, crafters, boutique owners.
*   **Advantages**: Provides access to discounted rates across multiple carriers (USPS, UPS, FedEx) through a single API. High reliability.
*   **Risks**: ShipStation has its own monthly subscription cost, which adds to the merchant's overhead. Complex API for international shipments.
*   **Pricing**: Monthly subscription fee plus postage costs.
*   **Compatibility**: Cloud (OAuth or API Key). Standalone (API Key).

### Design Doc
*   Merchant connects their ShipStation account in the OHC "Fulfillment" settings.
*   During checkout, OHC queries the ShipStation API with package dimensions to display live shipping rates to the customer.
*   In the OHC Merchant Dashboard, the Operations Agent highlights unfulfilled orders.
*   Merchant clicks a native "Buy Label" button. OHC calls ShipStation to purchase the label and saves the PDF.
*   The tracking number is automatically retrieved and emailed to the customer.

### Implementation Prompt
Integrate ShipStation to automate shipping label generation. The checkout flow must query live rates. The merchant order management view must allow purchasing and printing of shipping labels natively. Tracking numbers must be automatically saved and sent to the customer.
- **Acceptance Criteria**: Live shipping rates shown at checkout. Merchant can purchase and print a shipping label from the OHC dashboard. Tracking number is emailed to the customer.

### Priority
P1

### Estimated Scope
Large

---

## 6. SMS & Notifications: MessageBird
### Title
Global SMS Notifications via MessageBird Integration

### Problem Statement
Small business owners, especially those running food trucks or physical services, often miss push notifications or emails. They and their customers need reliable, instant SMS alerts for order updates, appointment reminders, and promotions, regardless of their global location.

### Research Report
*   **Strategy**: Direct API integration with MessageBird for global outbound SMS delivery.
*   **Target Persona**: Food service operators, local service providers, international merchants.
*   **Advantages**: Excellent global coverage and competitive pricing outside the US compared to Twilio. Simple API.
*   **Risks**: US A2P 10DLC compliance is still a hurdle for merchants sending to US numbers.
*   **Pricing**: Pay-per-message. OHC will need a credit system or bill the merchant directly.
*   **Compatibility**: Cloud (Centralized OHC MessageBird account). Standalone (User provides API key).

### Design Doc
*   In OHC Settings, merchants can toggle SMS notifications on/off for specific events (e.g., "Order Ready", "Appointment Reminder").
*   When a qualifying event occurs, the OHC backend dispatches an async job to the MessageBird API.
*   The system formats phone numbers globally (E.164) before sending.
*   The AI Operations Agent can intelligently delay non-urgent SMS messages to avoid waking customers at night.

### Implementation Prompt
Integrate MessageBird for global SMS notifications. Allow merchants to configure which events trigger an SMS to the customer or themselves. Ensure strict E.164 phone number formatting and handle delivery failure webhooks.
- **Acceptance Criteria**: Merchant can enable SMS for "Order Ready". Customer receives an SMS when the order status changes. Phone numbers are validated and formatted correctly.

### Priority
P2

### Estimated Scope
Medium

---

## 7. Video Conferencing: Microsoft Teams
### Title
Auto-Generate Microsoft Teams Meeting Links for Appointments

### Problem Statement
Small business owners offering virtual consultations or tutoring spend unnecessary time manually creating Microsoft Teams meeting links and emailing them to clients. This workflow is error-prone and unprofessional. They need links generated automatically upon booking within OHC.

### Research Report
*   **Strategy**: Native OAuth integration with the Microsoft Graph API to create Teams meetings.
*   **Target Persona**: Professional services, tutors, B2B consultants using the Microsoft ecosystem.
*   **Advantages**: Highly professional. Parity with Zoom/Google Meet integrations. Keeps the user flow entirely automated.
*   **Risks**: Microsoft Graph API OAuth permissions can be granular and confusing to configure for the initial developer setup.
*   **Pricing**: Free for users with a Microsoft 365 account that includes Teams.
*   **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth or User OAuth).

### Design Doc
*   During service creation, the user sets the location type to "Online Meeting" and selects "Microsoft Teams" (after connecting their Microsoft account).
*   When a customer books this service, OHC makes a call to the Graph API to schedule an online meeting.
*   The generated Teams join URL is embedded directly into the OHC confirmation email, the calendar invite, and the customer portal.

### Implementation Prompt
Build a Microsoft Teams integration that dynamically creates meeting links for online service bookings. Users must be able to authenticate their Microsoft account. Upon booking, OHC must generate a unique Teams link and attach it to the appointment record and outgoing notifications.
- **Acceptance Criteria**: Merchant connects Microsoft account. Customer books an online service. Unique Teams link is generated and sent to both parties in the confirmation email.

### Priority
P2

### Estimated Scope
Medium
