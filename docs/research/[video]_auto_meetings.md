# Video Conferencing: Auto Meetings

## Problem Statement
Service-based businesses (tutors, consultants) waste time manually creating Zoom links and emailing them to clients for every booked session.

## Research Report
**Selected Tool:** Google Meet API
While Zoom is popular, Google Meet requires zero client-side installation for the customer and is free with Google accounts, removing a massive point of friction.
- **Ease of use for non-technical users:** Seamless if they already use Google Calendar.
- **Pricing:** Free.
- **Reputation:** Extremely reliable and ubiquitous.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer books a service marked as "Online Meeting".
- **Action:** OHC (often via the scheduling integration like Cal.com) provisions a Google Meet link and attaches it to the calendar invite.
- **User Interface:** A simple toggle on the service definition: "Location: Online (Google Meet)".
- **Environment:** Cloud and Standalone.

## Implementation Prompt
**User-Facing Outcome:** When a customer books an online service, they and the business owner automatically receive a calendar invite containing a unique Google Meet link.
**Acceptance Criteria:**
- OAuth flow to authorize Google Calendar/Meet.
- Automatic link generation upon booking.
- Link is included in confirmation emails/SMS automatically.

## Priority
P2

## Estimated Scope
Small
