# Auto-Generated Video Links for Services

## Problem Statement
Online tutors, therapists, and consultants need to manually create Zoom links and email them to clients after an appointment is booked. This manual step often leads to errors ('wrong link') and a poor client experience.

## Research Report
**Competitive Landscape:**
1. **Zoom API:** Ubiquitous, but API can be clunky. High user familiarity.
2. **Google Meet API:** Great if the user is already in the Google ecosystem.
3. **Whereby / Jitsi:** Excellent for embedding video directly into the OHC platform, removing the need to download apps.

**Evaluation:**
- **Ease of Use:** Embedded Whereby/Jitsi is the best UX (1-click join in browser). Zoom requires app installation.
- **Pricing:** Jitsi is open source. Whereby has a good embedded API. Zoom requires paid plans for longer meetings.
- **Cloud vs Standalone:** Embedded WebRTC (Jitsi) is perfect for Standalone to avoid third-party dependencies.

## Design Doc
- **Trigger:** A customer books an 'Online Service' via the OHC scheduling feature.
- **Action:** OHC automatically generates a unique video meeting link and includes it in the calendar invite and confirmation email.
- **User Experience:** Both the owner and the customer just click the link in their calendar when it's time to meet.

## Implementation Prompt
Integrate auto-generated video links into the scheduling module. Support Google Meet (via Google Calendar integration) and Zoom. When a service is configured as 'Online', automatically provision a meeting link upon booking. Display this link prominently in the appointment details UI for both the business owner and the customer.

## Priority
P2

## Estimated Scope
Medium
