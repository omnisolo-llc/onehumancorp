# Twilio Integration
## Problem Statement
Small business owners, particularly those who rely heavily on mobile devices or cater to populations with limited English proficiency or internet access (like Fatima the food cart operator), need a reliable way to send and receive text messages (SMS) to confirm orders, send pickup notifications, or alert customers.

## Research Report
**Tool**: Twilio (SMS/Messaging API)
**Ease of use**: High for developers; easy to abstract for end-users. The business owner just needs to provision a number and set up notification rules.
**Pricing**: Pay-as-you-go pricing (approx. $0.0079 per message sent/received in the US). Phone numbers cost around $1.15/month.
**Reputation**: Twilio is the industry standard for programmatic SMS and voice, known for high deliverability and global reach.

## Design Doc
**Cloud Mode**: OHC utilizes the Twilio REST API to provision phone numbers for tenants and send outbound SMS notifications. Webhooks are configured to receive inbound SMS, routing them into the OHC unified customer inbox.
**Standalone Mode**: Since Twilio is a cloud-based API, standalone instances would require internet access to dispatch messages via HTTP requests to Twilio.
**Triggers**: Order status changes (e.g., "Ready for pickup"), appointment reminders, or direct messages from the customer inbox.
**User Experience**: The business owner configures automated SMS triggers in the OHC settings. Customers receive standard text messages and can reply; their replies appear in the OHC unified inbox, allowing the business owner (or the AI Customer Success agent) to respond seamlessly.

## Implementation Prompt
Integrate Twilio SMS into the OHC platform to enable automated and manual text messaging.
**Acceptance Criteria**:
1. Implement backend support to provision Twilio phone numbers for individual tenants.
2. Allow business owners to enable automated SMS notifications for key events (e.g., Order Confirmation, Appointment Reminder).
3. Create a webhook endpoint to receive incoming SMS messages from Twilio and route them to the tenant's unified customer inbox.
4. Allow business owners to reply to inbound SMS directly from the OHC dashboard.

## Priority
P1

## Estimated Scope
Medium
