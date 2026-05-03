# Issue Brief: Video Consultations via Zoom

## Title
Automated Video Links for Online Lessons & Consultations

## Problem Statement
"I have to manually create a Zoom link and email it for every lesson. Sometimes I forget." Music tutors like Leo need a "Zero Friction" way to teach. They want the link created, the calendar invite sent, and the "Join" button to appear in OHC automatically.

## Research Report
- **Tool**: Zoom Meeting API.
- **Ease of Use**: High. The industry standard for video.
- **Persona Fit**:
    - **Leo (Music Tutor)**: Auto-generates a Zoom link for every guitar lesson.
    - **Carlos (Handyman)**: Can offer "Video Consultations" for quick quotes.
- **Cloud vs. Standalone**:
    - **Cloud**: Required for meeting hosting.
    - **Standalone**: Can embed the Zoom Web Client for a seamless in-app experience.
- **Pricing**: Free tier (40-min limit); Pro at $15/mo. API access is included in the developer plan.
- **Competitive Analysis**: Google Meet is great but Zoom has better "White-label" and "Embed" capabilities for a "Premium" OHC experience.

## Design Doc
- **Integration**: "The Advisor" (Business Advisory) suggests "Video Consultations" as a new service.
- **User Experience**:
    - Customer books a "Video Consultation".
    - OHC (via Zoom API) creates the meeting.
    - A "Join Meeting" glassmorphism button appears on the customer's and owner's dashboard 5 mins before the start.

## Implementation Prompt
Integrate the Zoom Meeting API to automate meeting link generation for "Service & Booking" products. Implement the Zoom Web Meeting SDK to allow users to join calls directly from the OHC app. Ensure that "The Advisor" agent can track "Video Sales" as a separate revenue stream.

## Priority
P2 (Medium)

## Estimated Scope
Medium
