# Title: Auto-Generate Meeting Links for Online Consultations via Zoom

## Problem Statement
Service-based businesses (tutors, consultants, therapists) who offer online sessions struggle with the manual work of creating a Zoom link for every new booking and emailing it to the client. If they forget, the client is confused and the meeting starts late. They need the video link to be created and shared automatically the moment a booking is made.

## Research Report
Zoom is the ubiquitous standard for video conferencing.
- **Ease of Use**: Everyone knows how to click a Zoom link. Connecting a Zoom account via OAuth is a standard, trusted process for business owners.
- **Pricing**: Zoom has a free tier that allows 40-minute meetings, which is sufficient for many quick consultations. Paid tiers remove this limit.
- **Reputation**: Unmatched brand recognition. High reliability for video quality.
- **Comparison**: Google Meet is a strong alternative, especially if the user is already heavily integrated into Google Workspace. However, Zoom is often preferred as a standalone tool that doesn't require a Google account from the customer.
- **Cloud vs Standalone**: Outbound link generation works natively in both Cloud and Standalone modes as it relies on outbound API requests. Webhooks for meeting state (e.g. participant joined) would require tunnel routing for Standalone mode.

## Design Doc
- **Triggers & Actions**: When an online service is booked via the OHC scheduling system, OHC calls the Zoom API to generate a unique meeting room. This link is automatically added to the calendar invite and the confirmation email sent to the customer.
- **User Experience**: When setting up a service in OHC, the owner selects "Location: Online (Zoom)". OHC prompts them to log in to Zoom once. From then on, every booking for that service automatically includes a unique Zoom link in the dashboard and in the customer's emails.

## Implementation Prompt
Integrate automated video conferencing link generation for scheduled events.
- **User-Facing Outcome**: When a customer books an online service, a unique video meeting link is automatically generated and shared with both the business owner and the customer, eliminating manual setup.
- **Acceptance Criteria**:
  - The business owner can authenticate their Zoom account within the OHC settings.
  - Services can be designated as "Online".
  - Booking an "Online" service automatically generates a unique meeting URL.
  - The generated URL is displayed on the appointment details in OHC and included in customer notification payloads.

## Priority
P2

## Estimated Scope
Medium
