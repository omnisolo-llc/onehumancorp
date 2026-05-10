# Automated Video Conference Link Generation

## Problem Statement
Manually creating Zoom or Google Meet links and sending them to clients for online consultations is prone to error and looks unprofessional.

## Research Report
Evaluated Zoom and Google Meet integrations for automatic link generation.

- **Ease of Use**: Creates a seamless, professional experience for virtual services.
- **Pricing**: Mostly relies on the user's existing paid or free tiers with Zoom/Google.
- **Risks**: OAuth token expiration, handling meeting cancellations/reschedules correctly.
- **Modes**: Cloud and Standalone work well provided the user authenticates their account.

## Design Doc
When a virtual service is booked, OHC calls the connected video conferencing API to generate a unique meeting room. The link is automatically added to the calendar event and the confirmation email/SMS.

## Implementation Prompt
Allow the business owner to connect their Zoom or Google Meet account. Automatically generate and attach a meeting link whenever a virtual appointment is booked, and include this link in customer notifications.

## Priority
P2

## Estimated Scope
Small
