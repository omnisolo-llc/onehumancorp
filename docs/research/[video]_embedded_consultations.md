# Title: One-Click Video Calls for Online Consultations
## Problem Statement
Coaches, tutors, and consultants need to run video sessions, but manually creating Zoom links and emailing them is tedious and error-prone.

## Research Report
Zoom API and Google Meet API are the primary targets. Jitsi is an open-source alternative that doesnt require the user to have an account to generate a link.
- **Ease of Use**: Jitsi is easiest (generate a link on the fly). Zoom requires OAuth.
- **Pricing**: Jitsi is free. Zoom requires a paid account for sessions > 40 mins.
- **Reputation**: Zoom is most recognized, Jitsi is great for seamless embedded experiences.

## Design Doc
- **Trigger**: A customer books a service marked as "Online/Video Call".
- **Action**: OHC automatically generates a unique video meeting link (e.g., via Jitsi) and attaches it to the appointment.
- **User View**: The business owner and the customer both see a "Join Video Call" button on their respective appointment details pages, which becomes active 5 minutes before the start time.

## Implementation Prompt
Implement a seamless video conferencing integration using Jitsi (or Zoom API if OAuth is preferred). When an appointment is booked for a virtual service, automatically generate a unique meeting link. Display a clear "Join Call" button in both the business owners dashboard and the customers confirmation page, simplifying the process of entering the meeting.

## Priority
P2

## Estimated Scope
Medium

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported. Meeting links are generated server-side.
- **Standalone Mode**: Fully supported. Outbound API requests can generate links dynamically from the client.
- **Risks**: Free tier limitations, join failures, and varying video quality.
