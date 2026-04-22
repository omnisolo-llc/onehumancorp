# 📹 Video Conferencing: Meeting Generator

## Title
Automated Video Meeting Link Generation

## Problem Statement
Service providers offering online sessions, like Leo (The Music Tutor), need an automated way to generate and share video meeting links when a booking is confirmed. Manually creating Zoom links and emailing them to clients is tedious and prone to errors.

## Research Report
- **Goal**: Evaluate APIs for auto-generating video conferencing links.
- **Tools Evaluated**:
    - **Zoom API**: The most popular platform. Requires OAuth integration per user. API is comprehensive but can be complex.
    - **Google Meet (via Calendar API)**: Seamless if the user already connects Google Calendar. Generates links natively when creating an event.
    - **Whereby**: Developer-friendly API for generating embedded video rooms. Great for a white-labeled experience.
    - **Daily.co**: Excellent WebRTC API, highly customizable.
- **Recommendation**: Prioritize **Google Meet via Calendar API** as it comes "for free" with the Google Calendar sync integration (see Calendar Brief). For users needing a dedicated platform, integrate **Zoom** via OAuth. Both integrations are external API calls that are fully supported in both Cloud and Standalone modes.
- **User Impact**: A student books a guitar lesson. The OHC system automatically schedules the event on Leo's calendar, generates a Google Meet link, and includes it in the confirmation email and calendar invite. Leo simply clicks the link at the scheduled time.

## Design Doc
- **Component**: `VideoMeetingAgent` (often co-located with `BookingAgent`)
- **Responsibilities**:
    - If using Google Meet, append conference data requests to the calendar event creation payload.
    - If using Zoom, handle OAuth and call the Zoom API to create a meeting for the specific time slot.
    - Store the generated meeting URL with the booking record.
- **Integration Point**: The `BookingAgent` requests meeting creation when a booking is finalized.

## Implementation Prompt
Implement the Video Meeting link generation. Extend the Calendar Sync integration to request Google Meet links upon event creation. Alternatively, implement a standalone Zoom OAuth integration and API client to create scheduled meetings. Ensure the generated meeting link is stored in the database and included in customer confirmation notifications.

## Priority
P2

## Estimated Scope
Small
