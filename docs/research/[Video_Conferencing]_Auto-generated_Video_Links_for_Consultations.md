# Issue Brief: Auto-generated Video Links for Consultations

**Category**: Video Conferencing

## Problem Statement
Consultants and tutors manually create Zoom links and email them to clients for every booking. This process is repetitive and error-prone.

## Research Report

### Tool Evaluations

**1. Zoom API**
- **Ease of Use for User**: Zoom is ubiquitous. Everyone knows how to use it.
- **Integration**: The OAuth flow requires the user to authorize OHC to create meetings on their behalf.
- **Pricing**: The user must have a paid Zoom Pro account if meetings exceed 40 minutes or have multiple participants.
- **API Features**: We can auto-generate unique links with passcodes for every booking.

**2. Google Meet**
- **Ease of Use for User**: Seamless if they are already using Google Calendar for OHC scheduling sync.
- **Integration**: When creating a Google Calendar event via the API, we simply append `conferenceData` to auto-generate a Meet link. No separate API needed.
- **Pricing**: Free with most Google accounts.

**3. Jitsi Meet**
- **Ease of Use for User**: No installation required for participants (runs in browser).
- **Integration**: Open source. We could self-host a Jitsi instance and simply generate URLs like `meet.ohc.com/booking-123`.
- **Brand Trust**: Lower than Zoom or Google. Clients might be hesitant to click unknown links.

**Summary Recommendation**: Implement Google Meet auto-generation first, since we are already building Google Calendar sync. It requires almost zero extra effort. Add Zoom OAuth as a fast-follow for users who prefer it.


## Design Doc
Integrate Zoom API and Google Meet (via Calendar API). Automatically generate a unique meeting link when a virtual service is booked. Embed the link in the confirmation email/SMS.

## Implementation Prompt
Enhance the booking system to automatically generate a Zoom or Google Meet link for online appointments and display it on the booking confirmation page.

## Priority
P2

## Estimated Scope
Small
