# Zoom Integration
## Problem Statement
Small business owners who offer online services, such as Leo the music tutor, need an automated way to generate video conferencing links for their bookings without manually creating meetings and emailing links to clients.

## Research Report
**Tool**: Zoom
**Ease of use**: High. Extremely well-known among consumers and businesses. The API allows for seamless meeting creation.
**Pricing**: Free tier allows 40-minute meetings. Pro tier starts at $14.99/month/user.
**Reputation**: The global standard for video conferencing. Reliable, high video quality, and universally recognized by customers.

## Design Doc
**Cloud Mode**: Integrate via Zoom API using Server-to-Server OAuth or standard OAuth. When a customer books an online service via the OHC platform (e.g., using the Cal.com integration or a native booking feature), OHC automatically calls the Zoom API to create a meeting and attaches the join link to the booking confirmation.
**Standalone Mode**: Same as cloud mode; requires internet connectivity to communicate with the Zoom API and host the meeting.
**Triggers**: Customer schedules an online appointment/class.
**User Experience**: The business owner connects their Zoom account in OHC settings. When a customer books a virtual session, the confirmation email and calendar event automatically include a unique Zoom link. Neither the business owner nor the customer has to manually manage Zoom links.

## Implementation Prompt
Integrate Zoom into the OHC platform to auto-generate meeting links for online bookings.
**Acceptance Criteria**:
1. Business owners can authenticate and connect their Zoom account via OAuth.
2. When configuring a service/appointment type, the owner can set the "Location" to "Zoom".
3. Upon a customer booking an appointment marked as "Zoom", OHC calls the Zoom API to create a unique meeting.
4. The generated Zoom join URL is automatically sent to the customer in their booking confirmation and saved in the OHC dashboard.

## Priority
P2

## Estimated Scope
Medium
