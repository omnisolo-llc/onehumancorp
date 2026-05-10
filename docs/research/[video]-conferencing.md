# Automated Video Conferencing Links

## Title
Automated Video Conferencing Links

## Problem Statement
Small business owners offering online services (tutors, consultants, therapists) waste time manually creating Zoom or Google Meet links and emailing them to clients after a booking is made. This manual step is prone to errors (sending the wrong link) and creates friction. They need unique video meeting links generated automatically for every online appointment.

## Research Report
*   **Tool:** Zoom API, Google Meet API (via Google Calendar integration).
*   **Market Analysis:** Online consultations are standard. The expectation is that the booking confirmation contains the necessary link.
*   **Competitor Analysis:** Calendly handles this perfectly. Replicating this functionality within OHC's scheduling module is essential to remain competitive.
*   **Ease of Use:** Must be a one-click connection in the settings. The link generation should be entirely invisible to the business owner during daily operations.
*   **Pricing:** API access is generally included with standard Zoom or Google Workspace subscriptions.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Straightforward OAuth integration.
    *   *Standalone:* Doable, utilizing OAuth for API access directly from the local environment.

## Design Doc
*   **User Journey:** The business owner connects their Zoom or Google Workspace account in OHC. When they create an appointment type (e.g., "Virtual Consultation"), they select "Online Video Meeting" as the location. When a client books this appointment type, OHC automatically requests a new meeting link from the Zoom/Google API. The generated link is instantly added to the calendar event and included in the confirmation email sent to both parties.
*   **Triggers:** A new booking for an appointment type designated as "Online".
*   **Actions:**
    *   Call Zoom/Google API to create a scheduled meeting.
    *   Retrieve the join URL.
    *   Embed the join URL into the calendar event and notifications.
*   **Visuals:** A simple dropdown in the appointment type setup to choose the location (Physical Address vs. Zoom vs. Google Meet).

## Implementation Prompt
Enhance the scheduling module to automatically generate video conferencing links for online appointments. Integrate with the Zoom API and Google Meet API (potentially leveraging the calendar integration). When a client books a virtual service, automatically create the meeting in the respective service and embed the unique join link into the confirmation emails and calendar invites. Ensure the integration handles API rate limits gracefully and provides clear error messages if link generation fails.

## Priority
P1

## Estimated Scope
Small
