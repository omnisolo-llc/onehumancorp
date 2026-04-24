# Title: Auto-Generated Meeting Links via Zoom API

## Problem Statement
Online service providers like Leo (The Music Tutor) waste time manually creating Zoom links for every lesson and sending them to students. They need the meeting link automatically generated and attached to the calendar invite the moment a booking occurs.

## Research Report
**Findings & Evaluation:**
- **Zoom API:** Robust API for creating meetings programmatically. Requires OAuth integration.
- **Alternatives evaluated:** Google Meet (via Google Calendar API), Whereby. Whereby is simpler but Zoom is what customers expect and trust. Google Meet is already partially covered if Cal.com handles the calendar sync, but a dedicated Zoom integration is necessary for power users.
- **Ease of Use:** The owner connects Zoom once via OAuth in the OHC settings.
- **Cloud vs Standalone:** Works well in Cloud. Standalone mode requires proxying the OAuth callback.

## Design Doc
**Integration with OHC:**
During the Cal.com booking flow (or native OHC booking flow), if the service location is set to "Online / Zoom", the Operations Agent triggers a call to the Zoom API using the tenant's OAuth token.
A unique meeting link is generated. This link is injected into the Cal.com event, the confirmation email sent via Resend, and the SMS reminder sent via Twilio.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- Business owners can set a service's location to "Zoom Meeting".
- They can connect their Zoom account with one click.
- When a customer books the service, a unique Zoom link is instantly generated.
- The Zoom link is included in the calendar invite and all reminder communications.

## Priority
P2

## Estimated Scope
Medium
