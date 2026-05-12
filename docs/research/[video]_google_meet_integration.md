# OHC Tool Integration: Google Meet for Virtual Services

## Title
Implement Google Meet API for Automated Virtual Consultations

## Problem Statement
Service-based businesses (tutors, consultants) spend too much time manually creating and emailing video conference links for virtual appointments.

## Research Report
- **Tool Evaluated:** Google Meet API (via Google Workspace/Calendar)
- **Why Google Meet?** Ubiquitous, free for standard users, requires no software installation for the customer.
- **Ease of Use:** Once the owner connects their Google account, meeting links are generated automatically.
- **Pricing:** Included with standard Google accounts / Workspace.
- **Reputation:** Extremely reliable and trusted by consumers.

## Design Doc
- **Trigger:** A customer books a service marked as "Virtual/Online".
- **Action:** OHC automatically creates a calendar event via the Google Calendar API, requesting a Google Meet conference link, and attaches it to the booking confirmation.
- **User View:** The owner's calendar automatically populates with the meeting link. The customer's confirmation email includes a "Join Meeting" button.

## Implementation Prompt
Integrate the Google Calendar/Meet API for automated virtual meeting generation. Add a toggle for "Virtual Service" on the service creation form. When a virtual service is booked, the backend must use the merchant's authenticated Google token to create an event with a Meet link attached. Store the generated link in the booking record and include it in the confirmation emails sent to both the merchant and the customer.

## Priority
P2

## Estimated Scope
Medium
