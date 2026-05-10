# Reliable Global SMS Notifications

## Problem Statement
Email notifications often go unread. Business owners need a reliable way to send critical updates (like appointment reminders or order confirmations) via SMS, especially for customers with low digital literacy.

## Research Report
Evaluated SMS tools for reliable notifications.

- **Ease of Use**: High impact for reducing no-shows and increasing engagement.
- **Pricing**: Per-message costs can add up; requires clear pricing visibility for the owner.
- **Risks**: Global carrier coverage variations, strict opt-out compliance (10DLC regulations).
- **Modes**: Cloud easily integrates with Twilio/MessageBird; Standalone requires the user to supply their own API keys.

## Design Doc
OHC triggers SMS notifications based on workflow events (e.g., appointment in 24h). The platform routes the message to an SMS provider API. The user interface allows the business owner to customize the SMS templates and manage opt-outs.

## Implementation Prompt
Implement an SMS template editor in the settings. Integrate a backend service to send SMS messages on specific triggers (e.g., new order, appointment reminder) and handle STOP replies automatically.

## Priority
P0

## Estimated Scope
Medium
