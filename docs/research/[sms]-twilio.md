# Title: Reliable SMS Notifications and Reminders via Twilio

## Problem Statement
Many small business owners serve clients who are not tech-savvy or don't check email frequently (e.g., local service appointments). Relying on email for reminders leads to no-shows. They need automated text messages to confirm appointments and send quick updates.

## Research Report
- **Tool Evaluated**: Twilio SMS API
- **Benefit to Users**: Drastically reduces no-shows for appointments and improves customer communication for urgent updates.
- **Ease of Use**: Owner toggles "Send SMS Reminders" in settings. It works silently in the background.
- **Pricing**: Pay-per-message. Very cheap in the US, but international SMS can become expensive quickly.
- **Integration Risks**: A2P 10DLC compliance in the US is a massive hurdle. Businesses must register their brand and campaign to send SMS, which is a complex bureaucratic process.
- **Environment**: Cloud and Standalone compatible, but Cloud mode could abstract away the A2P registration complexity if OHC acts as the primary registered entity.

## Design Doc
- **Trigger**: An appointment is scheduled, or an order is marked ready for pickup.
- **Action**: OHC dispatches a short SMS to the customer's phone number via Twilio.
- **User Interface**: SMS templates are predefined and uneditable to ensure compliance. The user just sees a toggle: "Remind customers via text message 24 hours before."

## Implementation Prompt
Integrate Twilio to send automated SMS notifications for critical events like appointment reminders or order pickups. Ensure phone numbers are validated (E.164 format) before sending. Provide a simple toggle for users to opt-in to SMS features, and clearly display the cost implications or limits. Handle basic STOP/UNSUBSCRIBE replies automatically.

## Priority
P0

## Estimated Scope
Medium