# Title: Automated Booking & Calendar Sync

## Problem Statement
Service-based small business owners spend too much time going back and forth with clients to find a meeting time. Double bookings happen frequently because personal and business calendars aren't synced. They need a way to let clients book available slots automatically.

## Research Report
*   **Tool Candidates**: Calendly API, Cal.com, Google Calendar API direct.
*   **Evaluation**: Cal.com is open-source, highly customizable, and offers a white-label API. Calendly is the industry standard but less flexible for white-labeling. Direct Google Calendar integration requires building the scheduling logic from scratch.
*   **Ease of Use**: Cal.com API allows us to embed the booking flow seamlessly into OHC so the business owner just sees "Availability Settings".
*   **Pricing**: Cal.com has team plans; direct Google API is free but high development cost.
*   **Modes**: Cloud (easy). Standalone (requires managing OAuth tokens locally).

## Design Doc
*   **Integration Trigger**: User sets their working hours and connects their Google/Outlook calendar.
*   **Action**: OHC generates a public booking link. When a client books, it creates an event on the owner's connected calendar and blocks that time in OHC.
*   **User Interface**: An "Availability" settings page, and a generated public-facing booking page for clients.

## Implementation Prompt
Build a scheduling feature that allows users to set their weekly availability and connect a third-party calendar (Google/Outlook). Generate a shareable booking link where clients can pick an available time slot. When booked, the event must appear on the connected calendar. Acceptance criteria: user can set hours, connect calendar, and a test booking successfully blocks out that time.

## Priority
P0

## Estimated Scope
Medium
