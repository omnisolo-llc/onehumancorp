# Issue Brief: Automated Scheduling via Cal.com

## Title
AI-Powered Booking & Calendar Sync (Google, Outlook, Apple)

## Problem Statement
"I spend half my day texting back and forth just to pick a time for a repair." Handymen like Carlos and music tutors like Leo waste hours on scheduling. They need a system where customers can see their availability, book a slot, and have it automatically appear on their phone's calendar without any manual effort.

## Research Report
- **Tool**: Cal.com API.
- **Ease of Use**: Very High. Known for "minimalist" and "clean" UI.
- **Persona Fit**:
    - **Carlos (Handyman)**: Customers book "Plumbing Repair" slots directly from his OHC site.
    - **Leo (Music Tutor)**: Syncs with his Google Calendar so he never double-books a lesson.
- **Cloud vs. Standalone**:
    - **Cloud**: Full sync with external providers.
    - **Standalone**: Cal.com is open-source and can be self-hosted, making it perfect for OHC Standalone mode.
- **Pricing**: Free for individuals (perfect for Leo); $12/mo for teams. API access is robust.
- **Competitive Analysis**: Calendly is the standard but Cal.com is more "OHC-aligned" (Open-source, white-label friendly).

## Design Doc
- **Integration**: OHC "Operations Department" manages the Cal.com API keys.
- **User Experience**:
    - User selects "Enable Bookings" in the dashboard.
    - OHC generates a "Premium" glassmorphism booking page.
    - When a booking happens, OHC sends a push notification to the owner and a confirmation email to the customer.

## Implementation Prompt
Integrate the Cal.com API to allow tenants to manage their availability and bookings. The integration should support 2-way sync with Google and Outlook calendars. When a booking is made, create a corresponding task in the KAIROS Shared Task List for "The Manager" to track. Ensure the booking UI follows OHC Glassmorphism standards.

## Priority
P1 (High)

## Estimated Scope
Medium
