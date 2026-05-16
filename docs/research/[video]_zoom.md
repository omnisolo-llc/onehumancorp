# Title: Automated Video Conferencing Link Generation

## Problem Statement
Service providers like online tutors or remote consultants waste time manually creating Zoom or Google Meet links for every appointment and sending them to clients. If a meeting is rescheduled, they have to manually update the links. They need video links to be auto-generated and attached to appointments.

## Research Report
**Tool Analyzed**: Zoom
**Ease of Use**: Very high for end-users. The host just connects their account once.
**Reputation**: Ubiquitous. Most consumers already have the client installed and know how to use it.
**Pricing**: Free for meetings up to 40 minutes. Pro plans start at $15/month. Very accessible for SMBs.
**Environment**: Cloud API. Can be integrated from a Standalone environment via external API requests.
**AI Integration**: Potential for post-meeting AI transcripts and summaries to be automatically saved to the customer's CRM profile.

## Design Doc
**Integration Trigger**: The user links their Zoom account via OAuth in the OHC Settings and marks an appointment type as "Virtual".
**Actions Taken**:
- When a client books a "Virtual" appointment, OHC calls the Zoom API to create a scheduled meeting.
- The unique Join URL is saved to the appointment record.
- The Join URL is automatically included in the confirmation email/SMS sent to the client.
**User View**: The owner sees a "Join Meeting" button appear next to the appointment in their dashboard 5 minutes before the start time. The client receives the link automatically without the owner lifting a finger.

## Implementation Prompt
Integrate the Zoom API for automated meeting link generation. Add an OAuth flow for the owner to connect their Zoom account. Modify the appointment booking flow so that if an appointment is flagged as "online/virtual", a Zoom meeting is programmatically created. Store the resulting join link and ensure it is displayed prominently in the appointment details UI for the owner and included in the automated confirmation messages sent to the client.

## Priority
P2

## Estimated Scope
Medium
