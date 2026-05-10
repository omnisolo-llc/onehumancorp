# Calendar & Scheduling: Google Calendar

## Problem Statement
Service-based businesses (like consultants, salons) struggle with double-booking. They want clients to book appointments directly on their website, but need it to sync seamlessly with their personal Google Calendar so they don't get double-booked.

## Research Report
Google Calendar API is the most widely used calendar sync tool.
- *Ease of Use*: Very easy for users (standard Google login).
- *Pricing*: Free tier is generous enough for almost all small businesses.
- *Reputation*: Highly reliable. Handles timezones perfectly.

## Design Doc
- *Trigger*: User connects their Google account in OHC Settings.
- *Action*: OHC reads free/busy times from the user's main calendar and blocks out those times on the OHC public booking widget. When a client books via OHC, an event is created on the user's Google Calendar.
- *User Interface*: A "Calendar Sync" section in Settings. The public website builder gets a "Booking Calendar" block that automatically respects the sync.

## Implementation Prompt
Implement Google Calendar integration allowing users to sync their availability. Add a settings panel to connect a Google account. The system must fetch free/busy schedules to prevent double-booking and push new OHC-generated appointments as events to the user's Google Calendar.

## Priority
P0

## Estimated Scope
Medium

## Environment Support
Cloud, Standalone.
