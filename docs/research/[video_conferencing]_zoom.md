# Video Conferencing - Zoom

## Problem Statement
Online tutors, consultants, and therapists need to generate video meeting links for their sessions without manually creating and emailing them for every booking.

## Research Report
Zoom is the most widely used video conferencing tool.
- **Ease of Use**: Ubiquitous. Users already know how to use it.
- **Pricing**: Free for 40-min meetings. Pro is $15.99/month.
- **Reputation**: Standard for business video calls.
- **Cloud/Standalone**: Cloud-based.

## Design Doc
- **Trigger**: A customer books an online service (e.g., "1hr Guitar Lesson").
- **Action**: OHC calls the Zoom API to create a meeting for that specific time block.
- **User View**: The business owner connects their Zoom account. The customer's booking confirmation email automatically includes a unique Zoom join link.

## Implementation Prompt
Integrate Zoom API for online service bookings. When a customer books an online service, automatically generate a Zoom meeting and attach the link to the booking details and confirmation emails.
- Acceptance Criteria: User can connect Zoom. Booking an online service generates a unique Zoom link. Link is visible in OHC dashboard and customer email.

## Priority
P1

## Estimated Scope
Small
