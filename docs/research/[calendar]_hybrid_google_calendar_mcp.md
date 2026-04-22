# Google Calendar Integration
## Problem Statement
Small business owners, such as Carlos the handyman or Leo the music tutor, need their business appointments (created in OHC) to sync with their personal or business Google Calendars to avoid double-booking and manage their daily schedules effectively from a single place.

## Research Report
**Tool**: Google Calendar (Google Workspace APIs)
**Ease of use**: Very High. The standard for personal and business calendar management.
**Pricing**: Free for personal Google accounts. Business features available via Google Workspace (starts at $6/user/month).
**Reputation**: Ubiquitous, highly reliable, and universally understood by users.

## Design Doc
**Cloud Mode**: Integrate via Google Calendar API using OAuth 2.0. OHC will request calendar read/write scopes. When an appointment is booked on OHC, an event is created on the user's connected Google Calendar. OHC also reads busy slots from Google Calendar to prevent customers from booking during those times.
**Standalone Mode**: Same as cloud mode; requires internet to sync with Google servers.
**Triggers**: A customer books, modifies, or cancels an appointment on the OHC platform. The business owner creates a busy block on their Google Calendar.
**User Experience**: The business owner connects their Google account in OHC. They select which calendars to check for conflicts and which calendar to add new OHC bookings to. Customers cannot book slots that are blocked on the owner's Google Calendar.

## Implementation Prompt
Integrate Google Calendar into the OHC platform for two-way calendar syncing.
**Acceptance Criteria**:
1. Business owners can connect their Google account via OAuth.
2. Business owners can select which specific Google Calendars to sync.
3. OHC checks the connected Google Calendars for "Busy" blocks and removes those time slots from customer-facing booking availability.
4. New bookings made via OHC are automatically pushed as new events to the designated Google Calendar, including relevant customer details in the event description.

## Priority
P1

## Estimated Scope
Large
