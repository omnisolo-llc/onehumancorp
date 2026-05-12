# Title: Automated Booking and Meeting Links via Google Calendar
## Problem Statement
Business owners spend too much time playing "email ping-pong" trying to find a time to meet with clients. They also forget to send Zoom or Google Meet links, leading to confusion and missed appointments.

## Research Report
Google Calendar API combined with Google Meet API provides a robust solution for reading availability, booking slots, and auto-generating video links.
- **Ease of Use**: Connect via Google OAuth. Very familiar to most users.
- **Pricing**: Free with standard Google accounts (limits apply, but sufficient for SMBs).
- **Reputation**: Industry standard, highly reliable.

## Design Doc
- **Trigger**: User connects their Google account and sets up a "Booking Page" with their available hours (e.g., 9 AM - 5 PM).
- **Action**: OHC queries the Google Calendar API to find free slots. When a customer books, OHC creates a calendar event and auto-generates a Google Meet link.
- **User View**: Business owner gets a public booking link (`ohc.app/book/business-name`) to share. They see new appointments automatically appear in their OHC dashboard and personal Google Calendar.

## Implementation Prompt
Implement an integration with the Google Calendar API using OAuth. Create a simple UI for the business owner to set their weekly availability. Generate a public booking page where customers can select a time slot. Upon booking, automatically create a calendar event with a Google Meet link and send a confirmation email/SMS to the customer.

## Priority
P1

## Estimated Scope
Medium

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported via OHC backend polling or webhooks from Google.
- **Standalone Mode**: Fully supported as outbound API requests and local OAuth flows can be handled by the desktop client.
- **Risks**: Calendar sync conflicts and API rate limits during peak booking times.
