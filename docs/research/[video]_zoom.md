# Zoom Integration for OHC

## Problem Statement
Service providers who offer online consultations, tutoring, or classes (like Leo the Music Tutor) need a frictionless way to generate and share video conferencing links when a booking is made. Manually creating a Zoom meeting and emailing the link to the client is tedious and prone to human error.

## Research Report
- **Features & API Suitability**: Zoom provides a robust REST API for managing users, meetings, and webinars. It supports Server-to-Server OAuth.
- **Pricing**: Basic API access is available on free accounts, but some features require Pro.
- **Ease of Use for Non-Technical Users**: High via standard OAuth.
- **Cloud vs. Standalone**: Works well in Cloud. Standalone requires Server-to-Server OAuth configuration.
- **Advantages**: Ubiquitous, highly recognized by clients.
- **Risks**: Security features (passcodes, waiting rooms) must be configured correctly via API to avoid confusing users.

## Design Doc
- **Integration Point**: "The Manager" (Operations).
- **Trigger**: A customer books an online service (e.g., "1 Hour Guitar Lesson").
- **Action**: OHC creates a new scheduled meeting via the Zoom API, retrieves the join URL, and embeds it in the booking confirmation email and calendar invite.
- **User View**: A "Connect Zoom" button in the Service configuration. When enabled, all bookings automatically show a "Join Video Call" button in the customer dashboard and emails.

## Implementation Prompt
Integrate Zoom to automate video meeting generation for bookings. Allow the business owner to authorize their Zoom account. When configuring a service, allow them to mark it as "Online via Zoom". Upon a successful booking, use the Zoom API to dynamically generate a unique meeting link. Include this link in the customer's confirmation email and the owner's booking dashboard.

## Priority
P1

## Estimated Scope
Medium
