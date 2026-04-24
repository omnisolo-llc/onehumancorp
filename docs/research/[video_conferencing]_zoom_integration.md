# Integrate Zoom API for Auto-Generated Virtual Lessons

## Problem Statement
Leo (Music Tutor) spends too much time manually creating Zoom links for every student and pasting them into calendar invites.

## Research Report
- **Tool Evaluated**: Zoom API
- **Ease of Use**: Ubiquitous for consumers, standard OAuth flow.
- **Pricing**: Free tier supports 40-min meetings, which covers basic needs.
- **Standalone/Cloud**: Requires OAuth 2.0, well-supported in Cloud.
- **Persona Fit**: Essential for Leo and any digital service provider.

## Design Doc
- **Integration Point**: Operations Agent (Booking Flow).
- **Trigger**: New virtual service booking created.
- **Action**: Call Zoom API to generate a meeting, attach the join link to the OHC booking record and calendar invite.
- **User View**: Leo connects his Zoom account once. Every online booking auto-includes a unique Zoom link for him and the student.

## Implementation Prompt
Implement Zoom OAuth flow in the integrations settings. When a booking is confirmed for a "virtual" service, automatically generate a Zoom meeting link and append it to the confirmation email and calendar event.

## Priority
P2

## Estimated Scope
Medium
