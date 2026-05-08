# [Video Conferencing] Auto-Generated Meeting Links

## Problem Statement
Service-based businesses (tutors, consultants, therapists) waste time manually creating Zoom or Google Meet links and sending them to clients. If they forget, the client shows up to the meeting with nowhere to go.

## Research Report
**Tools Evaluated:** Zoom API, Google Meet API

*   **Ease of Use:** Extremely high for both the business owner and the client. The business owner clicks "Connect Zoom" once. The client simply clicks the link in their confirmation email.
*   **Pricing:** Google Meet is generally included in standard Google quotas. Zoom requires the business owner to have a paid (Pro or higher) Zoom account to generate meetings via the API.
*   **Reputation:** Zoom and Google Meet are the two most recognizable video conferencing platforms.

## Design Doc
**Trigger:** An appointment or service is booked that is flagged as "Virtual Location".
**Action:** OHC calls the Zoom or Google Meet API to generate a unique meeting room.
**User Sees:** During setup, the user connects their Zoom or Google account. When creating a service type, they can set the Location to "Auto-generate Zoom link". When a client books, the generated link is automatically injected into the OHC calendar event, the client's confirmation email, and any SMS reminders.

## Implementation Prompt
Build an integration for Zoom and/or Google Meet. Provide an OAuth flow for the business owner to connect their account. Modify the appointment scheduling system so that a "Virtual Location" type can be selected. When a booking occurs, automatically generate the meeting link via the provider's API and save it to the appointment record so it can be surfaced in UI and notifications.

## Priority
P2

## Estimated Scope
Small

## Mode Compatibility
*   **Cloud:** Fully supported via OAuth.
*   **Standalone:** Fully supported. The standalone instance can handle the OAuth redirect and directly make the API calls to generate links.
