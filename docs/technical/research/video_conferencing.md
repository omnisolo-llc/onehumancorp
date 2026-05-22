# [Video Conferencing] Integrate Zoom API for Auto-Meeting Links

## Problem Statement
Service providers who teach or consult online (like Leo the Music Tutor) currently have to manually create a Zoom link, copy it, and email it to the student after they book. They need unique meeting links generated automatically and attached to the calendar invite.

## Research Report
**Evaluated Tool:** Zoom Meeting API
**Alternatives Considered:** Google Meet API, Daily.co
**Pros:** Ubiquitous adoption—almost all customers know how to use Zoom. Robust API for generating scheduled meetings programmatically.
**Cons:** Requires the tenant to have a paid Zoom account to avoid 40-minute limits. OAuth approval process for the app marketplace can be stringent.
**Ease of Use for Non-technical Users:** User clicks "Connect Zoom". When a customer books an "Online Lesson", a unique Zoom link appears on the confirmation screen and calendar invite.
**Pricing:** Free API usage; requires tenant to have a Zoom subscription.
**Deployment:** Cloud-native (Server-to-Server OAuth or standard OAuth).

## Design Doc
**Integration with OHC:**
- **Trigger:** A new booking is created for a service marked as "Online Meeting".
- **Action:** OHC calls the Zoom API to create a meeting associated with the tenant's Zoom account, retrieving the `join_url`.
- **AI Agent Interaction:** "The Ambassador" includes the `join_url` in the booking confirmation email and reminder notifications.
- **User View:** A "Video Conferencing" settings panel to connect Zoom, and an "Online Meeting" toggle when creating a service offering.

## Implementation Prompt
Integrate the Zoom Meeting API via OAuth. When an online service is booked, automatically generate a unique Zoom meeting link. Display this link in the user's booking dashboard, the customer's confirmation page, and include it in automated email/SMS reminders.

## Priority
P2

## Estimated Scope
Medium
