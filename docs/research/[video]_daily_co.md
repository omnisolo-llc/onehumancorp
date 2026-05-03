# Video Conferencing Integration (Daily.co)

## Title
Integrate Daily.co for Embedded Video Consultations

## Problem Statement
Service providers like Leo (The Music Tutor) or consultants need to host virtual sessions. Requiring them to manage Zoom links manually, deal with calendar sync issues, and guide clients to a separate app creates friction. They need video conferencing built directly into their scheduling flow.

## Research Report
- **Tool Evaluated**: Daily.co.
- **Benefits for OHC Users**: Allows video calls to be embedded directly into the OHC platform or client portal. Eliminates the need for external software like Zoom.
- **Ease of Use**: Completely seamless. When a virtual appointment is booked, a unique Daily.co room link is automatically generated and added to the calendar event.
- **Pricing**: Generous free tier (10,000 participant minutes/month), then pay-as-you-go. Very cost-effective.
- **Reputation**: Excellent developer experience, high-quality video/audio, focuses on embedded use cases.
- **Cloud vs. Standalone**: Ideal for Cloud mode.

## Design Doc
- **User Experience**: When a client books a "Virtual Lesson", they receive a calendar invite with a "Join Video" link. Clicking it opens the video call directly in their browser—no app download required.
- **Integration**: Use Daily.co API to programmatically create meeting rooms when virtual appointments are booked. Embed the Daily.co Prebuilt UI into the OHC client portal for a seamless experience.
- **Triggers**: Booking of a service marked as "Virtual".
- **Actions**: Create Daily.co room, store room URL in booking record, include URL in confirmation emails.

## Implementation Prompt
Integrate the Daily.co API to automatically generate video conferencing rooms for virtual appointments. The system should generate a unique meeting link when a relevant booking is made and embed the video call interface within the OHC platform for a seamless user experience. Acceptance criteria include automatic creation of a Daily.co room upon booking, inclusion of the room link in confirmation emails/calendar invites, and a functional embedded video call experience within the OHC web app.

## Priority
P2

## Estimated Scope
Medium
