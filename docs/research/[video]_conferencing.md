# Video Conferencing Integration

## Title
Automated Video Meeting Links (Google Meet)

## Problem Statement
Small business owners offering online consultations, tutoring, or virtual classes struggle with generating and sharing meeting links. They often create meetings manually right before an appointment and email the link to the client, which looks unprofessional and leads to missed meetings if the email is delayed or lost.

## Research Report
*   **Target Tools:** Google Workspace API (Google Meet).
*   **Pros:** Ubiquitous. Most users already have a Google account. Generating a Meet link is free and deeply integrated with Google Calendar.
*   **Cons:** Requires users to strictly authenticate with a Google account (won't work for pure Outlook/Apple users without creating friction).
*   **Ease of Use for Non-Technical Users:** Very high. If they use Gmail, they know how to use Google Meet.
*   **Pricing:** Free for basic use.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Simple OAuth 2.0 flow.
    *   *Standalone:* Can require the user to configure their own Google Cloud Console project for OAuth credentials, which violates the non-technical requirement. OHC would need to provide a unified OAuth proxy.

## Design Doc
1.  **Connection:** User links their Google Account in Settings (specifically granting Calendar/Meet permissions).
2.  **Meeting Creation:** When creating a new appointment or responding to a booking request, the user checks a box: "Add Video Conferencing".
3.  **Link Generation:** OHC calls the Google API to generate a Meet link, attaches it to the calendar invite, and displays it in the OHC dashboard for that appointment.
4.  **Client Experience:** The client receives an email with a clear "Join Meeting" button.

## Implementation Prompt
Add a "Virtual Meeting" option to the appointment creation flow. Allow the user to connect their Google Account. When scheduling an appointment, if the user selects "Virtual Meeting", automatically generate a Google Meet link and embed it into the appointment details. Ensure this link is prominently displayed in the confirmation email sent to the client and on the appointment summary page in the dashboard.

## Priority
P2 (medium)

## Estimated Scope
Small
