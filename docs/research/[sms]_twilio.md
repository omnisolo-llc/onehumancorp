# Title: SMS Customer Notifications

## Problem Statement
For many customers, especially those with lower digital literacy or those outside major urban centers, email is rarely checked. Small businesses need a way to send critical updates (like "Your order is ready for pickup" or "Appointment reminder") directly via SMS to ensure the message is seen quickly.

## Research Report
**Tool Analyzed**: Twilio
**Ease of Use**: Developer-centric, but the end-user experience can be abstracted completely behind simple OHC toggles.
**Reputation**: The industry standard for programmatic SMS. Extremely reliable with global reach.
**Pricing**: Pay-as-you-go, typically around $0.0079 per SMS in the US. Very cost-effective for transactional messages. Compliance fees (A2P 10DLC) apply in the US.
**Environment**: Cloud API. Perfectly viable for Standalone mode via standard outbound HTTP requests.
**AI Integration**: AI can help summarize long notifications into concise, 160-character SMS formats to save costs.

## Design Doc
**Integration Trigger**: The owner enables "SMS Notifications" in settings and provides a payment method to cover usage costs (or uses an included quota).
**Actions Taken**:
- Customers are asked for a phone number and opt-in consent during checkout/booking.
- Key events (order fulfilled, appointment tomorrow) trigger an outbound API call to Twilio.
- Twilio dispatches the SMS to the customer.
**User View**: The owner sees a simple toggle list of which events should trigger an SMS (e.g., [x] Order Shipped, [x] Appointment Reminder). The customer receives a standard text message.

## Implementation Prompt
Integrate Twilio to handle transactional SMS notifications. Create a settings UI where the business owner can enable SMS for specific triggers (Order Confirmation, Shipping Update, Appointment Reminder). Update the checkout/booking flows to collect phone numbers and explicit SMS opt-in. Implement the backend logic to dispatch these messages via the Twilio API when the respective events occur.

## Priority
P1

## Estimated Scope
Medium
