# SMS Notifications via Twilio

## Problem Statement
Customers frequently miss important email updates regarding orders or appointments. SMS notifications provide a more immediate and reliable communication channel, which is especially important for users with lower technical proficiency.

## Research Report
Twilio is the industry standard for programmatic SMS and voice communication. It offers global reach and high reliability.
*   **Ease of use (end user):** The business owner toggles a setting; the system handles the rest.
*   **Pricing:** Pay-as-you-go per message (around $0.0079 in the US). Requires purchasing a phone number.
*   **Reputation:** Extremely reliable and scalable.

## Design Doc
OHC will integrate Twilio for outbound SMS notifications.
1.  **Trigger:** User enables "SMS Notifications" in settings.
2.  **Action:** OHC sends order confirmations, shipping updates, or appointment reminders via SMS.
3.  **User Sees:** A settings toggle. Their customers receive text messages for critical updates.

## Implementation Prompt
Implement automated SMS notifications for key customer events using Twilio.
*   Add an "SMS Notifications" toggle in the communication settings.
*   Implement backend logic to trigger SMS messages for events like "Order Confirmed", "Order Shipped", and "Appointment Reminder".
*   Allow the business owner to configure a dedicated Twilio phone number or use a shared pool (if applicable).
*   Acceptance Criteria: The owner can enable SMS, and simulating an order confirmation triggers a simulated SMS dispatch.

## Priority
P2

## Estimated Scope
Small
