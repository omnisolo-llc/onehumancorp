# SMS & Notifications: Twilio

## Problem Statement
Emails often get ignored. For urgent things like appointment reminders, order pickups, or quick updates, business owners (and their customers) prefer text messages. This is especially crucial for users with low English proficiency who rely on simple SMS.

## Research Report
Twilio is the industry leader for programmatic SMS.
- *Ease of Use*: Very easy API, hard part is dealing with A2P 10DLC compliance in the US.
- *Pricing*: Pay per message (fractions of a cent in US, varies globally).
- *Reputation*: Extremely reliable, global coverage.

## Design Doc
- *Trigger*: An appointment is coming up in 24 hours, or an order is marked "Ready for Pickup".
- *Action*: OHC calls Twilio API to send a templated SMS to the customer's phone number.
- *User Interface*: In Settings -> Notifications, checkboxes for "Send SMS reminders to customers". In the customer profile, a timeline shows sent SMS messages.

## Implementation Prompt
Integrate Twilio SMS to send automated appointment reminders and order updates. Provide a settings panel for the owner to enable/disable SMS notifications. Ensure the system handles phone number formatting (E.164) and logs SMS delivery status in the customer's history.

## Priority
P1

## Estimated Scope
Medium

## Environment Support
Cloud, Standalone.
