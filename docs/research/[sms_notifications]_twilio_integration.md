# Title: Global SMS Notifications & Reminders via Twilio

## Problem Statement
Users like Fatima (The Food Cart Operator) rely on their phones but may have poor internet connectivity or turn off app notifications. They need reliable SMS alerts when a new order arrives. Similarly, their customers (especially for food or appointments) expect SMS reminders, not just emails, which have lower open rates.

## Research Report
**Findings & Evaluation:**
- **Twilio:** The industry standard for programmatic SMS. Exceptional global carrier coverage and reliability.
- **Alternatives evaluated:** MessageBird, Plivo. Twilio remains the most reliable for critical transactional alerts, despite being slightly more expensive.
- **Ease of Use:** Completely invisible. The business owner toggles "Receive SMS Alerts" in the OHC app.
- **Cloud vs Standalone:** Fully supported in Cloud. Standalone users must provide their own Twilio credentials.

## Design Doc
**Integration with OHC:**
The OHC backend implements a Notification Service abstraction. When a high-priority event occurs (e.g., "New pre-order paid"), the Operations Agent publishes an event. The Notification Service checks the user's preferences. If SMS is enabled, it formats a concise message and dispatches it via the Twilio API.
For customer reminders (e.g., "Your appointment with Carlos is tomorrow"), the Customer Success Agent schedules a background job that triggers the Twilio API 24 hours before the Cal.com event time.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- Business owners can opt-in to receive instant SMS notifications for new orders or bookings.
- Customers automatically receive an SMS reminder 24 hours before a booked appointment.
- SMS content is concise and localized.
- The business owner does not need to create a Twilio account; it is handled natively via OHC.

## Priority
P0

## Estimated Scope
Medium
