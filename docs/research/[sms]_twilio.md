# SMS & Notifications: Automated Alerts via Twilio

## Title
Automate SMS Reminders and Notifications

## Problem Statement
Emails often go unread. Small business owners (especially service providers) suffer from no-shows. They need an automated way to send text messages to clients to confirm appointments or provide service updates, without giving out their personal phone number.

## Research Report
- **Tool Evaluated:** Twilio
- **Ease of Use:** Developer-focused, requires OHC to build a simple UI on top.
- **Pricing:** Pay-as-you-go (fractions of a cent per message).
- **Reputation:** Industry gold standard for telecom APIs.
- **Cloud/Standalone Compatibility:** API-only. Standalone instances will need external internet access to dispatch messages.

## Design Doc
- **Integration Point:** Settings -> Notifications, and individual customer profiles.
- **User Experience:** The owner toggles "Send SMS Reminders." OHC provisions a local phone number for their business. When a client books an appointment, they receive an automated text 24 hours prior.
- **System Behavior:** OHC uses Twilio's API to buy numbers and send outbound SMS, parsing inbound replies (e.g., "Confirm") to update OHC state.

## Implementation Prompt
Build an SMS notification settings panel. Allow users to configure simple automated text message templates (e.g., appointment reminders, order ready for pickup). Ensure the system handles opt-outs (STOP messages) automatically to remain compliant.

## Priority
P1

## Estimated Scope
Medium
