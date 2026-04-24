# Integrate Cal.com for White-Labeled Booking

## Problem Statement
Service-based businesses like Leo (Music Tutor) and Carlos (Handyman) need a way for customers to book time slots without back-and-forth emails. They need a simple link that syncs with their personal calendars.

## Research Report
- **Tool Evaluated**: Cal.com
- **Ease of Use**: Very user-friendly, open API, supports white-labeling out of the box.
- **Pricing**: Free tier available, highly SMB-friendly.
- **Standalone/Cloud**: Excellent for Cloud (API) and Standalone (self-hosted or direct API).
- **Persona Fit**: Ideal for Leo and Carlos to share a booking link without needing technical setup.

## Design Doc
- **Integration Point**: Operations Agent, Sales Agent.
- **Trigger**: Agent identifies booking intent or user sets up scheduling.
- **Action**: Generate single-use or reusable booking links via Cal.com API.
- **User View**: A scheduling widget on the OHC website where customers pick dates. Business owner connects Google/Apple Calendar once.

## Implementation Prompt
Build an integration module with the Cal.com API. Add a "Scheduling" component to the drag-and-drop website builder that embeds the Cal.com widget. Ensure booked events trigger OHC webhook handlers to update the business owner's dashboard.

## Priority
P0

## Estimated Scope
Large
