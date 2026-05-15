# Title: Seamless Calendar Sync and Auto-Scheduling

## Problem Statement
Service-based small businesses (tutors, consultants, salons) waste time in back-and-forth emails to schedule appointments. They need a way to automatically offer available times, avoid double-booking with their personal calendars, and instantly generate meeting links.

## Research Report
*   **Competitors:** Calendly, Acuity Scheduling, Google Workspace appointments.
*   **Ease of Use:** Must be extremely intuitive. The user just connects Google Calendar/Outlook, sets their working hours, and gets a shareable booking link.
*   **Pricing:** Calendly offers a basic free tier, with premium features starting around $10/month.
*   **Reputation:** Calendly is the industry standard due to its simplicity and reliability.

## Design Doc
*   **Trigger:** User connects their calendar (Google/Outlook) and configures availability rules.
*   **Actions:** OHC syncs calendar events to determine busy times. When a customer books, OHC creates an event on the user's calendar and optionally generates a Zoom/Meet link.
*   **User View:** The business owner sees a "Scheduling" section to manage availability and view upcoming appointments. Customers see a branded booking page.

## Implementation Prompt
Create a calendar integration system. Allow users to connect Google Calendar or Outlook. Generate a public booking page based on their configured availability and existing calendar events to prevent double-booking. Include auto-generation of video conferencing links for virtual appointments.

## Priority
P0

## Estimated Scope
Large
