# Integrate Zoom for Automated Online Consultations

## Problem Statement
Service-based small business owners (like tutors, therapists, or consultants) often conduct their business virtually. Currently, when a client books a slot, the owner has to manually open Zoom, create a meeting, copy the link, and email it to the client. This is a massive time sink and prone to error (e.g., forgetting to send the link before the meeting starts).

## Research Report
**Tool**: Zoom API
Zoom is the ubiquitous standard for video conferencing.
- **Ease of use**: Everyone knows how to use Zoom. The OAuth flow for the business owner to connect their account is standard and straightforward.
- **Pricing**: The API is free to use; the owner just needs a standard Zoom account (Basic free tier allows up to 40 mins, Pro is $15/mo).
- **Reputation**: The market leader for video conferencing.
- **Environment**: Cloud-based API. For OHC Standalone users, an OAuth app configuration with secure redirect URIs might require a proxy or specific instructions, but it is technically feasible.

## Design Doc
This integration works in tandem with the Calendar/Scheduling module (e.g., Cal.com) or OHC's internal appointment system to generate meeting links on the fly.
- **Trigger**: An appointment marked as "Virtual/Online" is booked in the OHC system.
- **Actions**: OHC makes a server-to-server call to the Zoom API (using the owner's OAuth token) to create a new Meeting. It captures the `join_url` and saves it to the appointment record.
- **User View**: When viewing upcoming appointments, both the business owner and the customer see a clear "Join Meeting" button that links directly to the generated Zoom room.

## Implementation Prompt
Integrate Zoom for automatic meeting generation. Allow the user to connect their Zoom account via OAuth in the Settings panel. When a new appointment is created with the location set to "Online", use the connected Zoom account to create a scheduled meeting for that exact date and time. Save the resulting Zoom join link to the appointment database record. Update the customer email confirmation template to prominently include this join link.

## Priority
P2

## Estimated Scope
Medium
