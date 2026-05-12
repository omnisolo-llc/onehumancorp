# Video Conferencing: Embedded Meetings via Jitsi

## Title
Embed Video Consultations and Online Lessons

## Problem Statement
Coaches, tutors, and consultants waste time manually creating and sending Zoom links. Clients get confused installing apps or finding the right link right before a meeting.

## Research Report
- **Tool Evaluated:** Jitsi Meet
- **Ease of Use:** Extremely high. Browser-based, no installs required.
- **Pricing:** Open source (100% free if self-hosted).
- **Reputation:** The most mature open-source WebRTC video conferencing platform.
- **Cloud/Standalone Compatibility:** Perfect. Can be self-hosted fully locally (Standalone) or managed centrally (Cloud).

## Design Doc
- **Integration Point:** "Meetings" section and the Calendar Booking flow.
- **User Experience:** When a client books an online service, OHC automatically generates a unique meeting room URL (e.g., `ohc.page/meet/12345`). At the scheduled time, both the owner and client click the link and join a video call directly inside their browser, fully branded with the owner's logo.
- **System Behavior:** OHC generates unique Jitsi room names and securely embeds the Jitsi iframe into the OHC web client.

## Implementation Prompt
Integrate video conferencing directly into OHC. When an online appointment is scheduled, automatically generate a unique meeting room link. Create a "Meeting Room" view in OHC that embeds the video call so the user doesn't have to leave the platform. Ensure the video interface works seamlessly on mobile web browsers.

## Priority
P2

## Estimated Scope
Medium
