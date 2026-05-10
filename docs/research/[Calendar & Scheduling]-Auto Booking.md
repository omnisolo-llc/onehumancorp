# Automated Customer Meeting Scheduler

## Problem Statement
Coordinating appointment times with clients involves endless back-and-forth emails. Business owners need a simple booking link that syncs with their real availability.

## Research Report
Evaluated Google Calendar and Outlook sync, plus automated meeting link generation.

- **Ease of Use**: Essential for service-based businesses. Eliminates double-booking.
- **Pricing**: Calendly charges around $10-$15/mo per user for premium features.
- **Risks**: Timezone confusion, calendar conflict resolution complexities.
- **Modes**: Cloud (multi-tenant) and Standalone support via user's own OAuth tokens.

## Design Doc
Business owner connects their Google/Outlook calendar. OHC reads busy slots to determine availability. Customers visit a public booking page, select a time slot, and book. OHC creates the calendar event on both the owner's and customer's calendars.

## Implementation Prompt
Implement a public-facing booking page for customers and a settings page for the business owner to set their availability rules and connect their calendar. Ensure booked slots are blocked off from future bookings.

## Priority
P1

## Estimated Scope
Medium
