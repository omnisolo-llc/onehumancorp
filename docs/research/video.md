# Title: Auto-Generated Video Conferencing Links

## Problem Statement
Coaches, tutors, and consultants have to manually create Zoom or Google Meet links and email them to clients after an online booking is made. This manual step is error-prone and tedious.

## Research Report
*   **Tool Candidates**: Zoom API, Google Meet (via Google Workspace API), Daily.co.
*   **Evaluation**: Daily.co allows embedding the video call directly in the browser (white-labeled). Zoom is what clients expect but requires app installation. Google Meet is ubiquitous but requires Google Auth.
*   **Ease of Use**: Daily.co provides the most seamless experience—just click a link and join in the browser. No downloads.
*   **Pricing**: Daily.co has a generous free tier for 1:1 calls.
*   **Modes**: Cloud (works perfectly). Standalone (works perfectly).

## Design Doc
*   **Integration Trigger**: An online meeting is booked.
*   **Action**: OHC calls the video provider API to generate a unique room link and attaches it to the calendar invite and confirmation email.
*   **User Interface**: A "Join Call" button appears on the appointment details page for both the owner and the client.

## Implementation Prompt
Integrate a video conferencing API to automatically generate unique meeting links when an online service is booked. The link should be included in the confirmation notifications. Acceptance criteria: booking an online service generates a valid video link, and both parties can click the link to join the room.

## Priority
P2

## Estimated Scope
Medium
