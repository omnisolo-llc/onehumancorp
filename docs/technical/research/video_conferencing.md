# [Video Conferencing] Auto-Generated Meeting Links

## Problem Statement
For online service providers like Leo (music tutor), manually creating a Zoom link and emailing it to a student after every booking is tedious. The system should automatically generate a unique video meeting link and include it in the calendar invite and confirmation email.

## Research Report
- **Target Tools**: Zoom API, Google Meet API (via Google Workspace integration).
- **Competitive Analysis**: Calendly does this perfectly. OHC needs parity to be viable for online consultants/tutors.
- **Ease of Use**: User authenticates their Zoom or Google account once.
- **Pricing**: Free APIs, though Zoom requires the user to have a licensed Zoom account for meetings over 40 minutes.
- **Reputation**: Essential tools for remote work and online services.
- **Advantages and Risks**: Creates a fully automated tutoring business. Risk is OAuth token expiration leading to failed meeting creations.
- **Cloud vs Standalone**: Same constraints as Calendars. Works perfectly in Cloud. Standalone may have trouble with OAuth redirects unless routed through OHC Cloud.

## Design Doc
- **Integration Flow**: When setting up a service, the user selects "Location: Online Video Call" and connects their Zoom or Google account.
- **Actions**: Upon a successful booking, the system calls the respective API to create a scheduled meeting. The returned join URL is saved to the booking record and sent to both the user and the customer.
- **User Experience**: Completely automated. The user just sees the meeting link appear in their calendar, and the customer gets it in their email. No copy-pasting required.

## Implementation Prompt
Integrate video conferencing capabilities allowing users to connect their Zoom or Google Meet accounts. When a customer booked a service designated as "Online Video Call," the system must automatically interact with the external API to generate a unique meeting link. This link must be automatically distributed in the booking confirmation email to the customer and embedded in the event details for the business owner.

## Priority
P1

## Estimated Scope
Medium
