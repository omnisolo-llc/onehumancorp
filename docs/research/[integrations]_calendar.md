# Calendar & Scheduling

## Title
[Calendar] Sync and Scheduling via Google Calendar and Outlook

## Problem Statement
Service providers like Carlos (Freelance Handyman) and Leo (Music Tutor) manage their time using personal calendar apps. They need a booking system that automatically reads their availability to prevent double-booking and adds new appointments directly to their calendars.

## Research Report
- **Evaluated Tools**: Google Calendar API, Microsoft Graph API (Outlook), Nylas, Cronofy.
- **Ease of Use**: Native APIs require users to grant OAuth permissions, which is standard and well-understood. Aggregators like Nylas simplify multi-provider support but add cost.
- **Pricing**: Google and Microsoft APIs are free for basic usage. Nylas charges per connected account ($1-$2/mo).
- **Calendar Conflict Resolution**: Native APIs provide robust free/busy querying.
- **Timezone Handling**: Complex but manageable using standard IANA timezone databases.
- **Cloud vs Standalone**: Fully supported in both modes, though Standalone may need to handle token refresh locally.

## Design Doc
- **Triggers**: A user connects their calendar; a customer views the booking page; a customer books a slot.
- **Actions**: System queries the connected calendar for free/busy times, subtracts them from the business's working hours, and displays available slots. Upon booking, a calendar event is created for both the business owner and the customer.
- **User View**: A "Connect Calendar" button in the Operations settings. The booking page seamlessly reflects real-time availability.

## Implementation Prompt
Build a two-way calendar sync feature supporting Google Calendar and Outlook. The business owner should be able to connect their calendar with a single click. The OHC booking page must only show available time slots by checking against the connected calendar's busy times. When a booking is made, it should automatically appear on the owner's personal calendar.
- **Acceptance Criteria**: User can connect Google Calendar and Outlook with a single click. The public booking page only displays time slots that are free according to the connected calendar's availability and the business's working hours. New bookings automatically create an event on the connected calendar.

## Priority
P1

## Estimated Scope
Medium
