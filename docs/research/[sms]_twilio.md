# Title: SMS & Notifications via Twilio

## Problem Statement
Fatima, the food cart operator, relies on immediate notifications when a pre-order is placed. She doesn't always have reliable internet for push notifications, and she needs to text her customers when their food is ready. Email is too slow for food pickup. She needs robust, instant SMS capabilities.

## Research Report
Twilio is the industry standard for programmable SMS and voice.
- **Ease of Use for Non-Technical Users**: Fatima doesn't know what Twilio is. She just turns on "SMS Notifications" in the OHC app. The platform provisions a local number for her business automatically.
- **Pricing**: Pay-as-you-go per message (fractions of a cent). Very affordable.
## Risks
- **Risks**: A2P 10DLC compliance strictness, risk of numbers being blocked if opt-outs are not handled perfectly.

## Reliability & Reputation**: The most reliable telecom API globally. Strong compliance tools for opt-outs (STOP messages).
- **Environment Support**: Works perfectly via API in all modes.

## Design Doc
The "Operations" and "Customer Success" agents handle SMS.
1. **Trigger**: A customer places a food order.
2. **Action**: Twilio sends an SMS alert to Fatima's phone. When she taps "Ready" in the OHC app, Twilio sends an SMS to the customer.
3. **User View**: Fatima gets a text: "New Order: 2x Falafel Wrap. Reply 1 to confirm." She replies '1' and the customer gets a text: "Your order is confirmed and will be ready in 15 mins."

## Implementation Prompt
Integrate the Twilio SMS API. Implement an automated phone number provisioning flow so businesses get a dedicated local number. Build notification logic that sends SMS alerts to the business owner for critical events (like new orders) and allows sending status updates to customers. Ensure compliance by handling standard opt-out replies automatically.

## Priority
P1

## Estimated Scope
Medium
