# SMS & Notifications: Global SMS

## Problem Statement
For users with low English proficiency or limited reliable internet, email is often ignored. SMS is the most reliable way to ensure they see order updates or appointment reminders.

## Research Report
**Selected Tool:** Twilio
Twilio remains the industry standard for reliable global SMS delivery.
- **Ease of use for non-technical users:** Completely invisible. The owner just sees "SMS enabled" in settings.
- **Pricing:** Pay-per-message. Can get expensive internationally, so we must offer WhatsApp fallback or explicit opt-ins.
- **Reputation:** The gold standard for telecom APIs.

## Design Doc
**Integration with OHC:**
- **Trigger:** System events (Order Confirmed, Appointment Reminder).
- **Action:** OHC formats a brief text message and dispatches it via Twilio API.
- **User Interface:** Toggle in settings: "Send SMS notifications to customers."
- **Environment:** Cloud and Standalone.

## Implementation Prompt
**User-Facing Outcome:** Customers receive timely SMS text messages confirming their orders or reminding them of appointments, reducing no-shows.
**Acceptance Criteria:**
- System handles basic number formatting (E.164).
- Automatic handling of "STOP" replies for compliance.
- Fallback logic if SMS fails.

## Priority
P1

## Estimated Scope
Medium
