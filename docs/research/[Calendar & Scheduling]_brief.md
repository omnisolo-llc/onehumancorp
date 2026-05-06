# Title: Integrate Cal.com for Seamless Booking and Scheduling

## Problem Statement
Service providers like Carlos and consultants like Priya lose revenue and time to the back-and-forth of scheduling appointments. They experience double bookings, missed calendar invites, and messy timezone conversions. They need a simple, professional way for clients to book available times without manual intervention.

## Research Report
**Tool Evaluated:** Cal.com
**Ease of Use:** Very high for end-users (clients). The business owner setup is streamlined and offers a clean, modern interface.
**Key Features:** Automated booking links, calendar sync to prevent double booking, timezone intelligence, team scheduling, and workflow automations (like email/SMS reminders).
**Pricing:** Generous free tier for individuals (unlimited bookings). Paid team plans are competitively priced.
**Reputation:** Known as the modern, open-source alternative to Calendly. Trusted by fast-growing startups and enterprises alike.
**Environments:** Excellent fit. Works in Cloud mode, and because it is open-source, it is perfectly suited for OHC's Standalone (local/private) environments as well.

## Design Doc
**Trigger:** User sets their working hours and connects their external calendar (Google/Outlook) within the OHC settings.
**Action:** OHC provisions a Cal.com booking link for the user. When a client books via this link, an appointment record is synchronized back to the OHC dashboard.
**User Experience:** The business owner sees a "My Schedule" view in OHC where they manage their availability. They can easily copy their unique booking link to text to clients or embed it on their website. They receive notifications in OHC when a new booking occurs.

## Implementation Prompt
Integrate Cal.com to handle user scheduling. Provide a setup flow where the user can define their weekly working hours and connect an external calendar. Generate a unique, shareable booking link for the user. Display upcoming bookings retrieved from Cal.com on the main OHC dashboard. The UI must hide complex routing or multi-team logic by default, offering a simple "Set Availability" and "Share Link" interface.

## Priority
P0

## Estimated Scope
Large