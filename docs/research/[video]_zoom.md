# Integrate Zoom for Auto-Generated Video Consultations

## Problem Statement
Small business owners who offer digital services, tutoring, or online consultations struggle with the manual process of creating video meeting links and emailing them to clients. Often, clients lose the link, resulting in delayed meetings and frustrated customers. Owners need video links to be generated and shared automatically when an appointment is booked.

## Research Report
*   **Tool:** Zoom (or Google Meet via Google Calendar API)
*   **Problem Solved:** Automatically generates secure video conferencing links for scheduled meetings.
*   **Ease of Use:** High. Once the owner connects their Zoom account, they never have to manually create a meeting again.
*   **Pricing:** Free tier available (40-min limit); Pro is $15/month.
*   **Reputation:** The ubiquitous standard for video conferencing.
*   **Environment:** Works seamlessly in both Cloud and Standalone modes via OAuth and API.
*   **Advantages:** Widely recognized by clients; extremely reliable; integrates well with existing calendar flows.
*   **Risks:** OAuth token expiration requiring re-authentication; clients sometimes struggle with downloading the Zoom app if they don't have it.

## Design Doc
1.  **Trigger:** A client books an "Online Consultation" via the OHC scheduling feature.
2.  **Action:** OHC automatically calls the Zoom API, creates a new meeting for the specified time, and retrieves the join URL.
3.  **User Interface:** The OHC dashboard shows a "Join Meeting" button next to the appointment. The client receives an automated calendar invite and email containing the clear, clickable Zoom link.
4.  **Integration:** This feature must act as an add-on to the Calendar & Scheduling module.

## Implementation Prompt
Add automatic video conferencing link generation to the appointment scheduling flow. When a business owner defines a service as "Online", the system must prompt them to connect their Zoom account. Upon a client booking, the system must automatically generate a unique Zoom meeting link and embed it directly into the calendar invite and confirmation emails sent to the client. Provide a clear "Join Now" button on the business owner's dashboard that becomes active 5 minutes before the meeting starts.

## Priority
P2

## Estimated Scope
Small
