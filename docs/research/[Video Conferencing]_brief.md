# Title: Integrate Zoom for Automated Video Meeting Generation

## Problem Statement
Consultants like Priya and educators like Leo rely on video conferencing to deliver their services. Manually creating a meeting link, copying it, pasting it into a calendar invite, and sending it to a client is tedious and prone to error. They need a system that automatically provisions virtual meeting rooms when a client books a session.

## Research Report
**Tool Evaluated:** Zoom
**Ease of Use:** Extremely high user familiarity. Most clients already have the app installed, reducing friction at meeting time.
**Key Features:** Auto-generation of meeting links, robust mobile and desktop apps, recording capabilities, and reliable video/audio quality.
**Pricing:** Generous free tier (up to 40 minutes for groups, usually unlimited for 1-on-1s depending on current policy), with affordable Pro plans.
**Reputation:** The household name in video conferencing.
**Environments:** Cloud API integration.

## Design Doc
**Trigger:** A client books a "Virtual Consultation" service type through the OHC scheduling system (e.g., via Cal.com integration).
**Action:** OHC calls the Zoom API to create a scheduled meeting and retrieves the unique join URL.
**User Experience:** Priya sets up a new service called "1-Hour Consult" and checks a box that says "Make this a Zoom meeting." When a client books, both Priya and the client receive an email/SMS with the exact Zoom link. Priya doesn't have to manually create the link.

## Implementation Prompt
Integrate the Zoom API to allow automated meeting creation. Create an OAuth flow so users can link their Zoom accounts to OHC. Modify the service/appointment creation UI to include a "Location" option where users can select "Zoom." When an appointment is booked with this location, use the Zoom API to generate a meeting link and embed it in the resulting calendar events and notification payloads.

## Priority
P1

## Estimated Scope
Medium