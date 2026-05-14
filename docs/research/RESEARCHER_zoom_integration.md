# Automated Video Conferencing via Zoom

## Problem Statement
Service providers who offer online consultations or lessons waste time manually creating Zoom links and sending them to clients for every booking.

## Research Report
Zoom is the most recognized video conferencing tool globally. Integrating its API allows for seamless meeting creation tied to bookings.
*   **Ease of use (end user):** Very high. The link is generated automatically and included in invites.
*   **Pricing:** API access is included with standard paid Zoom accounts.
*   **Reputation:** Ubiquitous and reliable.

## Design Doc
OHC will integrate Zoom to auto-generate meeting links for online services.
1.  **Trigger:** A client books an online service (via the Cal.com integration or a native booking flow).
2.  **Action:** OHC calls the Zoom API to create a meeting for the scheduled time.
3.  **User Sees:** The meeting link is automatically added to the booking details, calendar invite, and confirmation email sent to the client.

## Implementation Prompt
Integrate the Zoom API to automatically generate video conferencing links for online bookings.
*   Create an OAuth flow for the business owner to connect their Zoom account.
*   Update the booking workflow to detect "online" services and automatically create a Zoom meeting.
*   Ensure the generated Zoom link is saved to the appointment record and included in customer notifications.
*   Acceptance Criteria: An owner connects Zoom, a customer books an online service, and a unique Zoom link is generated and associated with the booking.

## Priority
P3

## Estimated Scope
Medium
