# Title: Integrate Cal.com for Automated Booking and Calendar Sync

## Problem Statement
Small business owners (like tutors or consultants) spend too much time going back and forth with clients to find a suitable meeting time. They need a simple, self-serve booking page that automatically respects their availability and generates virtual meeting links.

## Research Report
Cal.com is an open-source scheduling infrastructure alternative to Calendly.
- **Ease of use:** High. Clean interface for users to set availability and simple booking pages for end customers.
- **Pricing:** Generous free tier for individuals; pro features are reasonably priced. Open-source nature allows for deep integration.
- **Reputation:** Rapidly growing, highly respected open-source alternative.
- **Cloud/Standalone:** Exceptional for both. Open-source means it can be self-hosted alongside OHC Standalone, and works easily via API in the Cloud.

## Design Doc
- **Trigger:** Business owner configures their working hours and connects their personal calendar (Google/Outlook) in OHC.
- **Action:** OHC generates a unique public booking URL. When a customer books a slot, an event is added to the calendar, and an automated email/SMS confirmation is dispatched.
- **User Interface:** A "Booking & Schedule" settings panel for availability. A read-only calendar view in the OHC dashboard showing upcoming appointments.

## Implementation Prompt
Build a self-serve scheduling system. Provide a settings page where the user can define their available hours (e.g., Mon-Fri 9-5) and connect their existing Google or Outlook calendar to prevent double-booking. Generate a public booking link that the business owner can share. When a customer uses the link, automatically create a calendar event and send a confirmation to both parties.

## Priority
P0

## Estimated Scope
Large
