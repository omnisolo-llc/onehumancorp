# Issue Brief: Global SMS Notifications for Customer Alerts

**Category**: SMS & Notifications

## Problem Statement
Emails often go unread. Business owners need to send SMS alerts for appointment reminders and order updates, especially to customers who prefer text.

## Research Report

### Tool Evaluations

**1. Twilio**
- **Capabilities**: The industry leader in programmatic SMS. Supports global routing, alphanumeric sender IDs, and WhatsApp messaging.
- **Pricing**: Pay per message (varies wildly by country, e.g., $0.0079 in US, much higher in Europe/Asia).
- **Compliance**: Twilio handles local regulatory compliance (like 10DLC registration in the US), but the business owner must still fill out the forms. OHC must build a UI to guide the user through this registration.
- **Opt-outs**: Twilio automatically handles STOP messages, but OHC must process the webhook to mark the customer record as 'Do Not Contact'.

**2. MessageBird (now Bird)**
- **Capabilities**: Very strong in Europe and Asia. Often better international routing and pricing than Twilio.
- **Integration**: API is clean and similar to Twilio.

**3. Plivo**
- **Capabilities**: Another strong alternative, often slightly cheaper than Twilio for volume sending.

**Summary Recommendation**: Use Twilio as the primary gateway due to its robust documentation and reliability. Build a dedicated "SMS Settings" page in OHC where the user can purchase a phone number (via the Twilio API) and write their notification templates.


## Design Doc
Integrate Twilio or MessageBird API. Allow business owners to configure automated SMS triggers (e.g., 'Appointment in 24h'). Handle opt-outs (STOP messages) securely in both Cloud and Standalone modes.

## Implementation Prompt
Add an SMS notification settings panel where users can write templates for order confirmations and appointment reminders. Ensure compliance with SMS opt-out rules is handled automatically.

## Priority
P1

## Estimated Scope
Medium
