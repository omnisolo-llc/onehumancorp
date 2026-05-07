# Auto-Generate Zoom Links for Online Consultations

## Problem Statement
Coaches, tutors, and consultants using OHC for bookings currently have to manually generate a video conferencing link and email it to the client after a booking is confirmed. This creates friction, looks unprofessional, and often leads to the client losing the link right before the meeting.

## Research Report
**Tool Evaluated:** Zoom API
- **Ease of Use:** User connects their Zoom account via OAuth. From then on, links are generated magically.
- **Pricing:** Zoom is ubiquitous. The API is available for free, though meeting time limits apply to free Zoom accounts.
- **Reputation:** The most recognized video conferencing tool globally. Clients know how to use it instantly.
- **Deployment:** Cloud mode is fully supported. Standalone mode works well via outbound REST API calls.

## Design Doc
- **Trigger:** An appointment is booked in OHC (either natively or via Calendar sync) that is marked as "Virtual".
- **Action:** OHC calls the Zoom API to create a scheduled meeting. The returned join URL is attached to the OHC appointment record and included in the confirmation email to the client.
- **User View:** When setting up a service, the owner just toggles "Online Meeting (Zoom)". The system handles the rest.

## Implementation Prompt
Integrate the Zoom API to automatically create meeting links for virtual appointments. Allow users to authenticate their Zoom account via OAuth. When a new virtual appointment is created in OHC, automatically generate a Zoom meeting, store the join and start URLs, and ensure these links are included in the automated confirmation and reminder emails sent to the customer.

## Priority
P2

## Estimated Scope
Medium