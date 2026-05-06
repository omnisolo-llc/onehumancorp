# Integrate Zoom for Automated Video Conferencing Links

## Problem Statement
For service-based businesses offering online consultations, tutoring, or virtual classes, manually creating a video meeting link and emailing it to the client for every booking is tedious and prone to human error (e.g., forgetting to send the link or sending the wrong one). They need a system that automatically generates a unique meeting link when an online service is booked.

## Research Report
**Findings & Data**: Zoom (Zoom Workplace) is a proprietary videotelephony software program that became the standard for online meetings.
**Ease of Use**: Very high familiarity among both business owners and clients. The integration involves standard OAuth authentication.
**Features**: The Zoom API allows for the dynamic creation of unique meeting URLs, setting meeting passwords, and managing host controls.
**Pricing**: The free tier covers basic usage (meetings up to 40 minutes for groups, 1-on-1s generally unlimited or loosely restricted). Pro tiers are affordable for small businesses. API access is typically included in standard accounts.
**Reputation**: Highly reliable video and audio quality; globally recognized.

## Design Doc
**Integration flow**:
1.  **Connection**: The user authenticates with Zoom via OAuth in the OHC integration settings.
2.  **Service Configuration**: When creating a bookable service in OHC, the user toggles "Online Meeting". OHC automatically selects Zoom as the provider if connected.
3.  **Booking**: When a client books this service, OHC calls the Zoom API to create a new scheduled meeting for that specific date/time.
4.  **Delivery**: OHC retrieves the generated "Join URL" from Zoom and includes it in the calendar invite and confirmation emails sent to both the business owner and the client.

## Implementation Prompt
**User-Facing Outcome**: The user connects their Zoom account. Whenever a client books a virtual service, a unique Zoom meeting link is automatically generated and added to the calendar event and confirmation emails. Neither the business owner nor the client has to manually coordinate the link.
**Acceptance Criteria**:
- OAuth flow to securely connect a Zoom account.
- Integration into the scheduling module to generate a unique Zoom meeting via API upon a successful booking.
- Storage of the generated meeting URL and password.
- Inclusion of the Zoom link in automated confirmation emails and calendar invites.
- Fully functional in both Cloud and Standalone deployments.

## Priority
P1

## Estimated Scope
Small
