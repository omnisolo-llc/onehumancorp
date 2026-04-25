# Two-Way Google Calendar Sync for Bookings

## Problem Statement
Service providers like Carlos (handyman) and Leo (music tutor) rely heavily on Google Calendar for their personal lives. Without calendar sync, customers might book a plumbing repair or guitar lesson during a personal doctor's appointment, causing frustrating double-bookings.

## Research Report
- **Tool**: Google Calendar API
- **Evaluation**: The industry standard for calendar sync. It allows applications to read free/busy times and create/modify calendar events.
- **Ease of Use for Persona**: The user simply clicks "Sign in with Google" and approves access. It requires no technical configuration.
- **Pricing**: Generous free tier (up to 1,000,000,000 queries per day).
- **Reputation**: Official, robust, and universally trusted.

## Design Doc
- **Integration Point**: "Operations" department.
- **Trigger**: User connects their Google account via OAuth.
- **Actions**:
  - When displaying available time slots to customers, OHC queries Google Calendar API for `freebusy` status and removes conflicts.
  - When a customer books a service, OHC creates a new event in the user's Google Calendar.
- **User View**: A calendar view in OHC showing both OHC bookings and personal events (marked as "Busy"). Time slots are automatically hidden from the public booking page if there's a conflict.

## Implementation Prompt
Add a "Connect Google Calendar" button in the Booking Settings. Implement Google OAuth to request calendar access. Update the booking availability logic to check the user's connected Google Calendar for conflicts, and ensure new OHC bookings are automatically pushed to their Google Calendar.

## Priority
P0

## Estimated Scope
Medium
