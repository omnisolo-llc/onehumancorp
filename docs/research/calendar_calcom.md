# Integrate Cal.com for Scheduling
## Problem Statement
Business owners spend too much time going back and forth with clients trying to find a time to meet or provide a service. They need a simple, self-serve booking page that syncs with their real calendar so they don't get double-booked.
## Research Report
Cal.com is an open-source scheduling tool that offers robust calendar sync (Google, Outlook, etc.).
- **Ease of Use**: Very easy to set up event types. The booking page is clean and professional.
- **Pricing**: Generous free tier for individuals.
- **Reputation**: Excellent open-source alternative to Calendly, highly customizable.
## Design Doc
The user will connect their primary calendar (e.g., Google Calendar) to OHC. They can create "Service Types" (e.g., 30-min consultation). OHC generates a unique booking link they can share with clients.
## Implementation Prompt
Build a "Scheduling" section. Allow the user to "Connect Calendar" (Google/Outlook). Let them define simple services (Name, Duration, Price). Generate a public booking URL that they can copy/paste to clients. When a client books, the event must appear in both the OHC dashboard and the owner's personal calendar. Must work in both Cloud and Standalone modes.
## Priority
P0
## Estimated Scope
Large
