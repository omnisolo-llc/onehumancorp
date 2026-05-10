# Title: Seamless Virtual Consultations via Microsoft Teams
## Problem Statement
Consultants and service providers need to generate and share video meeting links instantly when a client booked a session, without manually copying and pasting URLs.

## Research Report
**Tool Evaluated:** Microsoft Teams
- **Ease of Use:** Familiar to many, especially B2B clients.
- **Pricing:** Included with Microsoft 365 (starts at $6/user/month).
- **Reputation:** Enterprise-grade reliability, massive adoption.
- **Advantages:** Instant link generation, strong calendar invite quality, robust join experience.
- **Risks:** OAuth flow for Microsoft can be slightly more rigid than competitors.
- **Environment:** Cloud and Standalone compatible.

## Design Doc
When a meeting is scheduled in OHC, an API call to Microsoft Graph will generate a Teams meeting link. This link is automatically injected into the calendar event and the confirmation email sent to the client. The business owner can join directly from their OHC dashboard.

## Implementation Prompt
Implement a Microsoft Teams integration that automatically creates a video meeting link when a new appointment is booked. The link should be securely stored and easily accessible by both the business owner and the client.

## Priority
P2

## Estimated Scope
Medium
