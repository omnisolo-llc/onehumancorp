# Title: Automated Booking and Calendar Sync via Cal.com

## Problem Statement
Service-based small business owners (consultants, tutors, repair technicians) waste hours playing phone tag or exchanging emails just to find a meeting time. They need a simple, branded link they can send to customers that only shows their actual free time and books the appointment instantly.

## Research Report
- **Tool Evaluated**: Cal.com API
- **Benefit to Users**: Eliminates scheduling back-and-forth. Customers book directly based on real-time availability.
- **Ease of Use**: The business owner connects their Google/Outlook calendar once. OHC generates a permanent booking link they can put in their bio or text to clients.
- **Pricing**: Open-source core. The hosted API has a generous free tier for basic scheduling, with affordable paid tiers for team routing or advanced workflows.
- **Integration Risks**: Handling timezones correctly is notoriously difficult. Calendar sync edge cases (recurring events, all-day events) can sometimes block availability incorrectly.
- **Environment**: Fully functional in Cloud mode. In Standalone, API calls can be made directly from the local backend to the Cal.com managed service.

## Design Doc
- **Trigger**: User enables the "Booking Link" feature in their OHC settings.
- **Action**: User authenticates their primary calendar (Google/Outlook). OHC provisions a Cal.com booking link on their behalf.
- **User Interface**: The user sees a simple toggle to set their "Working Hours" and is given a shareable URL. New bookings appear on their OHC dashboard schedule automatically.

## Implementation Prompt
Integrate Cal.com to provide automated scheduling capabilities. Allow the user to connect their existing calendar and define their working hours. Generate a unique, shareable booking link. Ensure that when a customer books a slot, the event appears on the user's OHC dashboard and blocks out that time to prevent double-booking.

## Priority
P1

## Estimated Scope
Medium