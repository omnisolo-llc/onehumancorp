# Title: Automated Video Conferencing Links for Online Services

## Problem Statement
Small business owners offering remote services (tutoring, therapy, consulting) currently have to manually create a Zoom or Google Meet link for every single booking, copy it, and email it to the client. If they forget, the client is confused and the session is delayed. They need video conferencing links to be generated instantly and automatically attached to the calendar invite the moment a customer books a session.

## Research Report
We evaluated video conferencing integration strategies:
- **Zoom API:** Extremely popular. Requires a Server-to-Server OAuth setup or a standard OAuth flow for the user. Managing the OAuth flow for non-technical users can be slightly confusing, but it is necessary given Zoom's dominance.
- **Google Meet (via Calendar API):** If we integrate calendar sync (e.g., via Nylas or Google APIs), Google Meet links can be automatically attached to events with zero extra configuration. This is by far the easiest solution for users already in the Google ecosystem.
- **Whereby / Daily.co:** These provide embeddable video rooms. While excellent technically, they force the business owner and the customer to use an unfamiliar platform instead of the tools they already have installed (Zoom/Meet).
- **Cloud vs. Standalone Compatibility:** Generating a link is an outbound API call (e.g., to Zoom) or handled during the Calendar event creation (Google Meet). This works identically in **Cloud** and **Standalone** modes without complex webhook requirements.

**Recommendation:** Prioritize the Google Meet integration as an automatic side-effect of Google Calendar sync. Secondarily, implement a standard Zoom OAuth integration for users who prefer Zoom.

## Design Doc
In the "Service Settings" where a user defines a bookable service, there will be a "Location" dropdown. The options will include "Physical Location," "Phone Call," "Google Meet (Automatic)," and "Zoom." If they select Zoom, they click "Connect Zoom" and authorize their account once. When a customer books this service, OHC calls the respective API to generate a unique meeting room link. This link is displayed on the confirmation page, sent in the confirmation email, and embedded in the calendar invite.

## Implementation Prompt
Integrate automatic video conferencing link generation for bookable services. Modify the service creation UI to allow the owner to select a virtual location. Implement an OAuth flow for connecting a Zoom account. When a booking occurs for a virtual service, automatically generate a unique meeting link via the Zoom API (or Google Meet if using Calendar sync). Ensure this link is securely attached to the booking record and exposed in all automated customer communications (emails, calendar invites, and SMS).

## Priority
P1

## Estimated Scope
Medium
