# Title: SMS Notifications for Critical Updates

## Problem Statement
Many small business customers (especially in developing regions) do not check email reliably. Business owners like Fatima need to send appointment reminders and order updates via SMS to reduce no-shows and keep customers informed.

## Research Report
*   **Tool Candidates**: Twilio, MessageBird (Bird), Vonage.
*   **Evaluation**: Twilio is the industry leader with massive global reach. MessageBird is highly competitive in Europe/Asia. Twilio's API is very mature.
*   **Ease of Use**: Invisible to the business owner. They just toggle "Send SMS Reminders" on.
*   **Pricing**: Pay-per-message, varies wildly by destination country.
*   **Modes**: Cloud (requires OHC to manage billing/credits). Standalone (user inputs their own Twilio credentials).

## Design Doc
*   **Integration Trigger**: An appointment is approaching (24h before) or an order ships.
*   **Action**: OHC triggers an SMS payload to the provider API.
*   **User Interface**: A toggle in settings for "Enable SMS Notifications" and a log of messages sent on the customer profile.

## Implementation Prompt
Implement automated SMS notifications for key events (appointment reminders, order shipped). Integrate with an SMS provider. Ensure opt-out mechanisms are respected. Acceptance criteria: user toggles SMS on, and a triggered event successfully delivers an SMS to a test phone number.

## Priority
P1

## Estimated Scope
Medium
