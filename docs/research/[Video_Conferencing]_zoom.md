# Zoom Integration for Video Conferencing

## Title
Auto-Generate Zoom Links for Meetings

## Problem Statement
Service-based small businesses (like tutors, consultants, or therapists) conduct many meetings online. Manually creating a new Zoom meeting, copying the link, and emailing it to the client for every appointment is repetitive and prone to mistakes (like sending the wrong link). They need meetings to automatically include a unique video link upon booking.

## Research Report
Zoom Communications, Inc. is an American communications technology company known primarily for its ubiquitous videoconferencing application (Wikipedia). It exploded in popularity and is now a household name, meaning most clients already have it installed and know how to use it.

Integrating Zoom allows OHC to programmatically generate meeting links. The user experience benefit is immense: completely hands-free meeting setup. Zoom offers a robust API for creating and managing meetings. For small businesses on basic or pro tiers, this saves significant administrative overhead. It operates seamlessly in both Cloud and Standalone modes via OAuth and API calls.

## Design Doc
The business owner will authorize Zoom in the OHC integrations settings. When setting up a service or event type (e.g., "1-Hour Consultation"), they can select "Zoom" as the location. When a client books this service, OHC will automatically call the Zoom API to generate a unique meeting room. The resulting join URL will be automatically injected into the calendar event and the confirmation email sent to the client.

## Implementation Prompt
Build a Zoom integration utilizing OAuth. When creating or scheduling an event in OHC, provide a toggle to "Add Zoom Meeting." If selected, automatically generate a unique Zoom meeting link upon saving/booking and attach it to the event details. Ensure the link is included in all automated confirmation and reminder emails sent to the customer.

## Priority
P2

## Estimated Scope
Small
