# SMS & Notifications Research Brief

## Title
Automated SMS Notifications and Reminders

## Problem Statement
Email open rates hover around 20%, whereas SMS open rates are above 90%. For critical updates—like appointment reminders or delivery notifications—email is insufficient. Small business owners need automated SMS to reduce no-shows and keep customers informed, especially in regions or demographics where email is rarely used.

## Research Report
### Market Context
SMS is highly regulated. Carriers aggressively filter spam, and compliance (10DLC in the US) is complex. Choosing the right provider is critical for deliverability.

### Tool Evaluations

#### 1. Twilio
- **Ease of Use:** Developer-focused, extremely powerful.
- **Pricing:** Pay-as-you-go (~$0.0079 per message in US).
- **Capabilities:** Global reach, WhatsApp API integration, voice capabilities.
- **Reputation:** The undisputed market leader, but requires significant engineering to handle compliance and routing.

#### 2. MessageBird
- **Ease of Use:** Good API, better omnichannel dashboard than Twilio.
- **Pricing:** Competitive, slightly better rates in Europe/Asia.
- **Capabilities:** Deep focus on omnichannel (SMS, WhatsApp, WeChat).
- **Reputation:** Strong alternative, especially for non-US markets.

#### 3. Vonage (formerly Nexmo)
- **Ease of Use:** Similar to Twilio.
- **Pricing:** Competitive.
- **Capabilities:** Strong global routing, good verification API.
- **Reputation:** Reliable fallback or primary provider for global SMS.

### Recommended Direction
Use Twilio as the primary engine for programmable SMS. However, OHC must heavily abstract the complexity of A2P 10DLC registration so the business owner doesn't have to deal with carrier compliance manually.

## Design Doc
### Trigger & Action
1. **Trigger:** An appointment is scheduled for tomorrow, or an order is out for delivery.
2. **Action:** A background job triggers an SMS payload to the provider API.
3. **User View:** The business owner configures SMS templates in settings (e.g., "Reminder: Your appointment with {{BusinessName}} is tomorrow at {{Time}}"). They can see a log of sent messages and any delivery failures.

### Environment Support
- **Cloud Mode:** OHC pools resources or registers users as sub-accounts to handle compliance.
- **Standalone Mode:** User inputs their own Twilio Account SID and Auth Token.

## Implementation Prompt
Build an "SMS Reminder" feature for the scheduling or order module.
- Allow the user to toggle "Send SMS Reminder 24 hours before."
- Integrate with a mock Twilio client to log the SMS sending action.
- Provide a UI for the user to customize the SMS template using basic variables (e.g., {{ClientName}}).
- Ensure phone numbers are validated (e.g., E.164 format) before attempting to send.
- Acceptance criteria: Scheduling an event successfully triggers the mock SMS job, which logs the formatted message to the console or database.

## Priority
P1 (High)

## Estimated Scope
Medium

### Extended Messaging Architecture Analysis
#### Global Phone Number Formatting
Invalid numbers are the leading cause of failed SMS deliveries. The system must rigorously enforce E.164 formatting standards. The UI must utilize country code selectors and validate the digit length based on regional rules before committing a customer's phone number to the database.

#### Compliance and Opt-Outs
In many jurisdictions, providing a clear opt-out mechanism (like replying "STOP") is a strict legal requirement. The SMS integration must seamlessly handle inbound STOP messages, automatically blacklisting the number from future automated communications without requiring manual intervention from the business owner.

#### Cost Control Mechanisms
Unlike email, SMS can become expensive quickly, especially for international destinations. The integration should enforce configurable spending limits or daily message caps to prevent malicious actors from racking up massive bills through automated form submissions.

#### Delivery Receipts
A "sent" status does not guarantee delivery. Carriers may silently drop messages. The system should process Delivery Receipt webhooks to provide accurate status updates to the business owner, highlighting messages that failed to reach the handset.

### User Persona Match
- **Fatima (Boutique Owner):** Medium value. Useful for shipping updates or flash sale alerts.
- **Carlos (Consultant):** High value. Appointment reminders via SMS drastically reduce his client no-show rate.

### Conclusion
Integrating reliable SMS communications bridges the gap between the digital platform and the physical reality of the customer, ensuring critical updates are read almost instantaneously.
