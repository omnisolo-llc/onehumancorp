# Title: Integrate Twilio for SMS Reminders and Notifications

## Problem Statement
Many small business owners have clients who do not check email frequently or are less proficient in reading long text. They face high no-show rates for appointments. They need simple, automated SMS reminders to ensure clients show up and are informed.

## Research Report
Twilio is the industry standard for programmatic SMS and voice calls.
- **Ease of use:** Abstracted away for the user. We handle the Twilio complexities.
- **Pricing:** Pay-as-you-go. Very affordable per message (fractions of a cent in the US), though international rates vary.
- **Reputation:** The largest and most reliable SMS API globally.
- **Cloud/Standalone:** Cloud API. In standalone, users must provide their own Twilio Account SID and Auth Token.

## Design Doc
- **Trigger:** System events: 24 hours before an appointment, or when an invoice is overdue.
- **Action:** Generates a short text message template and dispatches it to the customer's phone number via Twilio.
- **User Interface:** A settings panel to toggle "Enable SMS Reminders" on/off. A history log in the customer profile showing all SMS messages sent.

## Implementation Prompt
Implement automated SMS notifications to reduce appointment no-shows. Provide a toggle in the settings to enable SMS reminders. When enabled, automatically send a brief text message 24 hours before a scheduled event to the customer's phone number. Ensure there is a UI to view the SMS delivery history on the customer's profile.

## Priority
P0

## Estimated Scope
Medium
