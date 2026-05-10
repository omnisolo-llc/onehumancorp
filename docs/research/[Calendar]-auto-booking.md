# Calendar & Scheduling: Auto-Booking

## Title
Sync Google Calendar/Outlook and Enable Public Booking Pages

## Problem Statement
Small business owners, like consultants or service providers, waste hours going back-and-forth over email or text trying to find a time to meet. They need a simple way to let clients book available times without double-booking their personal calendars.

## Research Report
- **Tools Evaluated:** Google Calendar API, Microsoft Graph API (Outlook), Calendly API, Cal.com (open source).
- **Ease of Use:** Cal.com has excellent embedded booking components. Direct Google/Outlook OAuth integration is critical.
- **Pricing:** Direct APIs are free. Cal.com API has volume pricing, but hosting the core logic ourselves over Google/Outlook APIs is cheaper for OHC.
- **Reputation:** Google and Microsoft are ubiquitous.
- **Cloud vs Standalone:** Both support OAuth. Cal.com infrastructure might be heavy for standalone, suggesting a direct OAuth + local OHC booking engine approach.

## Design Doc
- **Trigger:** User connects Google/Outlook in "Calendar Settings".
- **Action:** OHC syncs free/busy times. User configures a "Booking Page" with specific available hours.
- **User View:** A public, branded OHC booking page link is generated. Clients see available slots in their local timezone and can book directly.

## Implementation Prompt
Create a "Booking Page" feature. Allow users to authenticate with Google or Microsoft to sync their calendar. Provide a public URL where clients can view available time slots and book appointments. The system must automatically block out times where the user already has events in their connected calendar and handle timezone conversions automatically.

## Priority
P1

## Estimated Scope
Medium
