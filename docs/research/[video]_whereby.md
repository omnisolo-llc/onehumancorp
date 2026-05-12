# Issue Brief: Embedded Video Consultations

## Title
Implement Embedded Video Consultations for Small Business Owners

## Problem Statement
A therapist wants to offer virtual sessions, but asking clients to download an app and find a meeting link creates anxiety right before a session.

## Research Report
Whereby allows video calls to happen directly inside a web browser without any downloads.

**Persona Impact:** The therapist's client simply clicks a link in their appointment email. A video window opens directly inside the OHC-branded portal. No downloads, no passwords. It feels like a premium telehealth platform.

**Advantages:** The absolute best, lowest-friction experience for the end-customer. Keeps branding focused on the small business.

**Risks:** Users on very old devices or browsers might experience performance issues.

**Pricing Estimate:** Very affordable pay-as-you-go pricing based on meeting minutes.

**Environment:** Works beautifully in both Cloud and Standalone modes.

## Design Doc
1.  **1-Click Join:** A prominent 'Join Video Call' button that appears on the appointment details page.
2.  **Embedded Player:** The video call takes place inside a clean, distraction-free view within the OHC interface.

## Implementation Prompt
Integrate Whereby to provide a totally frictionless, download-free video consultation experience embedded directly into the OHC platform.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
The Whereby integration must allow the business owner to 'lock' the room. If a client arrives early, they are placed in an OHC-branded waiting room until the professional admits them, ensuring privacy between back-to-back sessions.
