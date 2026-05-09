# [Video] Zoom Integration

## Title
Native Zoom Link Generation for Appointments

## Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.

## Research Report
- **Strategy**: Native integration with Zoom.
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Standard connection process. Highly intuitive.
- **Risks**: Platform reviews and compliance checks.
- **Pricing**: Free tier (40-min limit). Pro starts at $15/mo.
- **Compatibility**: Cloud and Standalone.

## Design Doc
- In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- Upon a successful booking, OHC creates a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
- The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.

## Implementation Prompt
Build an integration that automatically creates meeting links for online service bookings. Users should be able to connect their account. When a customer books a service marked as "Online Meeting", the system must dynamically generate a link, store it with the booking, and share it with both the merchant and the customer.

## Priority
P1

## Estimated Scope
Medium
