# Title: Reliable SMS Notifications for Customers via Vonage

## Problem Statement
Many customers ignore emails, leading to missed appointments or uncollected orders. For businesses serving populations with varying tech literacy or lower English proficiency, a simple SMS is the most reliable way to communicate critical updates. Business owners need to automatically send text reminders without using their personal phone numbers.

## Research Report
Vonage (formerly Nexmo) is a global cloud communications platform.
- **Ease of Use**: Transparent to the user. The business owner just turns on a toggle, and Vonage handles the complex global carrier routing behind the scenes.
- **Pricing**: Pay-per-message. Generally very cost-effective (fractions of a cent per message in the US, varying globally), making it affordable for small businesses to send automated reminders.
- **Reputation**: Excellent global reach and reliability.
- **Comparison**: Twilio is the market leader, but Vonage often provides simpler pricing structures for international SMS routing, which is beneficial for OHC's global user base. Both are excellent choices.
- **Cloud vs Standalone**: Outbound SMS delivery works seamlessly in both Cloud and Standalone environments. Inbound SMS (if supported) would require webhook tunneling for Standalone mode.

## Design Doc
- **Triggers & Actions**: When an appointment is booked, an SMS confirmation is sent. 24 hours before the appointment, an SMS reminder is sent. When an order is ready for pickup, an SMS alert is sent.
- **User Experience**: In OHC "App Settings" under "Notifications", the owner sees toggles: "Send SMS reminders 24h before appointments" and "Send SMS when order is ready". They simply turn these on. OHC handles the rest.

## Implementation Prompt
Integrate automated SMS notifications for critical customer events.
- **User-Facing Outcome**: The business owner can enable SMS reminders with a simple toggle, reducing no-shows and ensuring customers receive urgent updates directly on their phones.
- **Acceptance Criteria**:
  - Toggles exist in the settings UI to enable/disable SMS for specific events (e.g., appointment reminders, order updates).
  - When enabled, the system successfully dispatches an SMS to the customer's provided phone number at the appropriate time.
  - The system must gracefully handle invalid phone numbers without crashing the main application flow.

## Priority
P1

## Estimated Scope
Medium
