# [Video Conferencing] Google Meet Integration

## Title
Integrate Google Meet for Automated Online Lessons

## Problem Statement
For digital service providers like Leo (Music Tutor), manually creating video links for every remote session is prone to human error. They need a way to auto-generate meeting links upon booking to reduce manual work and look professional.

## Research Report
- **Strategy**: Direct integration with Google Workspace API (Google Meet)
- **Target Persona**: Leo (Music Tutor), Digital Service Providers
- **Advantages**: Google Meet is ubiquitous, free, and creates zero friction for end customers. It can be automatically attached to any Google Calendar event created during the booking process.
- **Risks**: Requires Google Calendar connection.
- **Pricing**: Free via Google Workspace API if using the user's existing Google Calendar/Meet integration.
- **Compatibility**: Cloud (OAuth); Standalone (OAuth).

## Design Doc
- When setting up a service, the user toggles "This is an online meeting".
- When a customer books the service, the OHC backend creates a Google Calendar event.
- The calendar event is configured to auto-generate a Google Meet conference link.
- The confirmation email sent to the customer includes this generated Meet link.

## Implementation Prompt
Extend the calendar booking flow to support online meetings. When a service is marked as "online", ensure the Google Calendar event creation request includes the conference data parameters to auto-generate a Google Meet link. Extract this link from the response and include it in the customer's confirmation email.

## Priority
P1

## Estimated Scope
Small
