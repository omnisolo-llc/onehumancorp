# OHC Tool Integration: Twilio for SMS Notifications

## Title
Implement Twilio Integration for Critical SMS Alerts

## Problem Statement
Important notifications (appointment reminders, last-minute cancellations, shipping updates) often go unread when sent via email. SMS has a much higher open rate, critical for immediate communication.

## Research Report
- **Tool Evaluated:** Twilio
- **Why Twilio?** The industry standard for programmable SMS, offering reliable global delivery and robust APIs.
- **Ease of Use:** Business owners opt-in to SMS features; OHC handles the routing automatically.
- **Pricing:** Pay-per-message (varies by country, e.g., ~$0.0079 in US). Requires handling A2P 10DLC registration for US numbers.
- **Reputation:** Highly reliable, though complex regulatory compliance (A2P) is required in some regions.

## Design Doc
- **Trigger:** System events such as an upcoming appointment (24h reminder) or a shipped order.
- **Action:** OHC formats a short text message and uses the Twilio API to send it to the customer's phone number on file.
- **User View:** Owners can toggle "Enable SMS Reminders" in settings. Customers receive simple text updates.

## Implementation Prompt
Integrate Twilio for automated transactional SMS notifications. Add settings for merchants to enable SMS alerts for specific events (e.g., order shipped, appointment reminder). Implement backend logic to format and queue SMS messages via the Twilio API when these events occur. Ensure phone number validation and opt-out handling are correctly implemented.

## Priority
P1

## Estimated Scope
Medium
