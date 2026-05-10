# Video Conferencing: Auto-Meet Links

## Title
Auto-Generate Video Links for Appointments

## Problem Statement
Coaches, tutors, and consultants doing online sessions currently have to manually create a Zoom link and email it to the client after every booking. This manual step often leads to errors or forgotten links, causing missed meetings.

## Research Report
- **Tools Evaluated:** Zoom API, Google Meet (via Calendar API), Whereby, Daily.co.
- **Ease of Use:** Google Meet is included for free if they connect Google Calendar. Zoom requires OAuth. Whereby/Daily.co allow embedding video directly in OHC without external apps.
- **Pricing:** Google/Zoom APIs are free (tied to user's existing account limits).
- **Reputation:** Zoom and Meet are universally understood by clients.
- **Cloud vs Standalone:** Works in both via OAuth and outbound API calls.

## Design Doc
- **Trigger:** A new appointment is booked through the OHC booking page.
- **Action:** OHC calls the Zoom/Meet API to create a meeting, or generates a unique Whereby room.
- **User View:** The business owner and the client both receive calendar invites with the video link automatically included. No manual copy-pasting required.

## Implementation Prompt
Enhance the booking system to automatically generate a video conferencing link (e.g., Google Meet or Zoom) when an online appointment is scheduled. The generated link must be saved to the appointment details, displayed to the user, and included in the confirmation emails sent to the client.

## Priority
P2

## Estimated Scope
Small
