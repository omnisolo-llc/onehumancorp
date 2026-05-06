# Title: Integrate Zoom for Auto-generated Online Meetings

## Problem Statement
Coaches and tutors who offer online sessions waste time manually creating Zoom links and copy-pasting them into calendar invites. They need unique, secure video meeting links generated automatically when a customer books a session.

## Research Report
Zoom provides a Server-to-Server OAuth API for creating and managing meetings.
- **Ease of use:** High for the end user once their Zoom account is linked.
- **Pricing:** Free tier supports 40-minute meetings. Pro tier required for longer sessions.
- **Reputation:** The most recognizable video conferencing tool globally.
- **Cloud/Standalone:** Cloud API. In standalone, users would configure their own Server-to-Server OAuth app.

## Design Doc
- **Trigger:** A customer completes a booking for a service marked as "Online Meeting".
- **Action:** OHC calls the Zoom API to create a scheduled meeting and retrieves the join URL. The URL is appended to the calendar event and confirmation emails.
- **User Interface:** A "Video Conferencing" settings page to connect a Zoom account. A toggle on service offerings to mark them as "Requires Zoom Link".

## Implementation Prompt
Integrate automatic Zoom link generation for online services. Provide a settings page for the user to link their Zoom account via OAuth. Add a setting to the service catalog allowing a service to be marked as an "Online Meeting". When a customer books an online meeting, automatically generate a unique Zoom link and include it in the confirmation email and calendar event.

## Priority
P1

## Estimated Scope
Medium
