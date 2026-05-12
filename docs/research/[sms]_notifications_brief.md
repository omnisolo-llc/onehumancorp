# Title: Reliable Global SMS Delivery via Twilio

## Problem Statement
Not all customers have smartphones or reliable internet. For critical notifications (like appointment reminders or order ready alerts), small business owners need guaranteed delivery via SMS to reduce no-shows.

## Research Report
- **Tool Evaluated:** Twilio
- **Benefits:** The gold standard for programmatic SMS. Massive global reach and high deliverability rates.
- **Ease of Use:** Transparent to the business owner. They just see a "Send SMS Reminder" toggle.
- **Pricing:** Pay-as-you-go per message segment.
- **Cloud/Standalone:** In Cloud, OHC acts as the Twilio account holder. In Standalone, the user must provide their own Twilio API keys due to billing.

## Design Doc
1. **Trigger:** An appointment is approaching, or an order status changes to "Ready for Pickup".
2. **Action:** OHC automatically triggers an SMS via the Twilio API to the customer's phone number.
3. **UI Outcome:** Business owners configure notification templates in the settings. They can view an SMS delivery log to confirm messages were sent successfully.

## Implementation Prompt
Integrate Twilio to handle automated SMS notifications for critical events (appointment reminders, order updates). Create a settings panel where business owners can customize the SMS templates. Ensure compliance with opt-out mechanisms (e.g., handling "STOP" replies).

## Priority
P1

## Estimated Scope
Medium
