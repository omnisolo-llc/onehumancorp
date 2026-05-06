# Integrate Google Calendar for Automated Scheduling

## Problem Statement
Small business owners, such as consultants, tutors, or salon owners, spend a disproportionate amount of time managing appointments, negotiating meeting times over email or text, and dealing with double-booking. They need a way to automatically sync their availability and allow clients to book open slots without manual intervention, directly integrated into their daily operational toolset.

## Research Report
**Findings & Data**: Google Calendar is the ubiquitous time-management and scheduling service used by professionals globally. Integrating it is table stakes for any scheduling workflow.
**Ease of Use**: Very high. Most users already have a Google account and use Google Calendar. The OAuth connection process is standard and trusted.
**Features**: It handles timezone conversions automatically, supports recurring events, and offers robust conflict resolution (checking for existing busy slots).
**Pricing**: Google Calendar is free for standard use. Google Workspace plans (which include Calendar) start at a low monthly cost, which many business owners already pay. The API usage is generally free within standard quotas.
**Reputation**: Exceptional reliability and uptime.

## Design Doc
**Integration flow**:
1.  **Connection**: The business owner clicks "Connect Google Calendar" in OHC settings and completes the Google OAuth flow, granting read/write calendar access.
2.  **Configuration**: The user selects which calendars to check for conflicts and which calendar to add new bookings to. They define their available working hours and meeting durations within OHC.
3.  **Booking Page**: OHC generates a public-facing booking link for the business owner.
4.  **Action**: When a client visits the link, OHC queries the Google Calendar API to present only available slots. When a slot is chosen, OHC creates an event on the connected Google Calendar and sends confirmation emails.

## Implementation Prompt
**User-Facing Outcome**: The user can link their Google Calendar and set their general availability hours. They receive a personal booking link to share with clients. When a client books a time through the link, the appointment automatically appears on the user's Google Calendar, and the time slot is no longer offered to other clients.
**Acceptance Criteria**:
- OAuth integration flow for Google Calendar is functional.
- User can specify working hours and meeting length.
- A public booking page is generated showing available slots, dynamically calculated based on Google Calendar busy times.
- Successful bookings create an event on the user's Google Calendar.
- Works in both Cloud and Standalone modes (API calls made directly from the user's instance).

## Priority
P0

## Estimated Scope
Medium
