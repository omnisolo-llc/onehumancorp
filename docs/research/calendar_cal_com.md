# Integrate Cal.com for Automated Calendar & Scheduling

## Problem Statement
Manually scheduling meetings, calls, or appointments leads to endless back-and-forth emails, double bookings, and timezone confusion. For a small business owner (like a consultant or tutor), this administrative overhead wastes hours every week and can frustrate potential clients. They need a simple link they can share that lets clients pick an available time automatically.

## Research Report
**Tool**: Cal.com
Cal.com is an open-source scheduling infrastructure that connects with Google Calendar, Outlook, and automatically generates video conferencing links (like Zoom or Google Meet).
- **Ease of use**: Very clean, modern interface. Easy to set up event types (e.g., "30 Min Consultation").
- **Pricing**: Free for individuals (perfect for solo small business owners). Team plans start at $12/user/month.
- **Reputation**: High-quality open-source alternative to Calendly, trusted by developers and businesses alike.
- **Environment**: Exceptional for both Cloud and Standalone modes, as it can be self-hosted or used via their managed cloud API. This makes it an ideal fit for OHC's dual-environment architecture.

## Design Doc
The integration will embed Cal.com scheduling functionality into the OHC platform.
- **Trigger**: The business owner configures their scheduling link in the OHC settings (authenticating their Cal.com account or providing their public link).
- **Actions**: OHC will fetch their public booking page or upcoming appointments via the Cal.com API and display them in a "Schedule" overview panel.
- **User View**: The owner sees a list of upcoming booked appointments in their dashboard. They also get a one-click button to copy their scheduling link to share with clients or embed it on their public-facing storefront.

## Implementation Prompt
Build a "Scheduling" widget in the main dashboard. Allow the user to connect their Cal.com account. Once connected, the widget should display a chronological list of upcoming appointments, showing the attendee's name, time (in the owner's local timezone), and a direct link to join the meeting (if it's a video call). Additionally, provide a prominent "Share Booking Link" button that copies their primary Cal.com link to the clipboard.

## Priority
P0

## Estimated Scope
Small
