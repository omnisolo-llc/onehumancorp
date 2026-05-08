# Title: Zoom Integration for Online Consultations

## Problem Statement
Service-based small business owners (tutors, consultants, therapists) need to manually create video meeting links and email them to clients after an appointment is booked. This manual step often leads to forgotten links and frustrated clients.

## Research Report
Zoom is the dominant video conferencing platform.
- **Ease of use:** OAuth integration is standard. Everyone knows how to join a Zoom call.
- **Pricing:** Free for 40-minute calls, paid for longer.
- **Reputation:** Ubiquitous, highly reliable.
- **Key advantages:** High user familiarity, robust API for meeting creation, and good mobile support for clients.
- **Risks:** The 40-minute limit on free accounts might surprise users. Security updates (like required passcodes or waiting rooms) occasionally break automated flows if not handled correctly.
- **Environment:** Cloud works perfectly via API. Standalone works perfectly via outbound API calls.

## Design Doc
- User goes to "Integrations" and connects their Zoom account via OAuth.
- In the scheduling settings, the user selects "Zoom Meeting" as the location type.
- When a customer books a slot, OHC automatically creates a Zoom meeting via API.
- The Zoom join URL is saved to the appointment record and automatically included in the confirmation email/SMS.

## Implementation Prompt
Integrate the Zoom API to auto-generate meeting links for appointments. Implement an OAuth connection flow for the business owner. When a new appointment is created with "Video Call" selected, call the Zoom API to create a meeting, store the `join_url`, and inject it into the customer notification templates.

## Priority
P2

## Estimated Scope
Medium
