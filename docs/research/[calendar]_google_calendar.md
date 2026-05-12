# Issue Brief: Google Calendar Sync

## Title
Implement Google Calendar Sync for Small Business Owners

## Problem Statement
Leo the tutor manages his personal life on Google Calendar. If a student books a session on OHC during his daughter's piano recital, he has to manually cancel and reschedule.

## Research Report
This connects the user's personal Google Calendar directly to their OHC availability.

**Persona Impact:** Leo only has to manage one calendar. If he blocks out Tuesday afternoon for personal time, the OHC public booking page immediately removes Tuesday afternoon from his available slots.

**Advantages:** Google Calendar is the most widely used personal calendar globally. Familiarity is extremely high.

**Risks:** Users might accidentally sync the wrong calendar (e.g., a shared family calendar).

**Pricing Estimate:** Completely free for the user.

**Environment:** Fully functional in both Cloud and Standalone environments.

## Design Doc
1.  **Calendar Selection:** After 1-click Google login, present a clear list of the user's calendars with simple toggles to select which ones block availability.
2.  **Event Creation:** When a customer books via OHC, the event appears beautifully formatted on the user's Google Calendar.

## Implementation Prompt
Create a seamless two-way sync with Google Calendar so the OHC booking system respects the user's personal busy times and automatically adds new bookings to their daily view.

## Priority
P0

## Estimated Scope
Large

### Unique Considerations
When OHC writes an event to Google Calendar, it must include a deep link back to the OHC appointment details page in the event description. This allows Leo to click directly from his phone's calendar app into the OHC client profile to review notes before the session starts.
