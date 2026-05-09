# Integration Issue Brief: Video Conferencing (Zoom)

## Title
Automated Meeting Link Generation: Zoom

## Problem Statement
Service-based small business owners (tutors, consultants, therapists) who operate online need to manually create video meeting links and email them to clients for every booking. This is tedious and leads to errors, such as sending the wrong link to a client.

## Research Report
*   **Tool Evaluated**: Zoom
*   **Ease of Use**: Ubiquitous and widely understood by end-users. The API allows for seamless meeting creation.
*   **Market Position & Reputation**: The market leader in video conferencing. High reliability and broad consumer acceptance.
*   **Pricing**:
    *   **Basic**: Free (40-minute limit on group meetings, 1-on-1s used to be unlimited but now often have limits depending on account age).
    *   **Pro**: ~$15.99/month/user (removes time limits).
*   **Cloud vs. Standalone Compatibility**: API-based. Fully compatible.

## Design Doc
*   **Integration Trigger**: User connects their Zoom account via OAuth in OHC settings.
*   **Action Flow**:
    1.  When a calendar event is created in OHC (either manually or via the Cal.com integration) marked as "Online Meeting", OHC calls the Zoom API.
    2.  Zoom creates a scheduled meeting and returns the join URL.
    3.  OHC attaches the join URL to the calendar event and includes it in automated email/SMS reminders to the client.
*   **User Experience**: The business owner never has to open the Zoom app to create a meeting. When they get a booking, the Zoom link is magically there, and the client automatically receives it in their confirmation email.

## Implementation Prompt
Integrate Zoom to automate video meeting creation. Build an OAuth flow for users to link their Zoom accounts. Modify the OHC Calendar/Booking module so that when an event is designated as a video call, OHC automatically requests a new meeting URL from Zoom's API. Save this URL to the event record and ensure it is surfaced in the OHC UI and included in any outbound client notifications regarding the meeting.

## Priority
P1

## Estimated Scope
Small
