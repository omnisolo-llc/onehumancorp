## [Calendar & Scheduling] Issue Brief: Native Booking Engine & Google Calendar Sync

**Title**: Scout 🔍: Integrate Cal.com API for Conflict-Free Booking
**Problem Statement**:
Service providers like Leo (Music Tutor) and Carlos (Handyman) need clients to book their time. Doing this manually via text or email leads to double-booking and timezone confusion. They need a simple booking widget that automatically respects their personal calendar availability.
**Research Report**:
- **Tool**: Cal.com API (or direct Google Calendar API).
- **Evaluation**: Cal.com offers an open-source, API-first scheduling infrastructure. It handles timezone math, calendar conflict checks, and event generation seamlessly.
- **Ease of Use**: High. The business owner connects their Google/Outlook calendar with one click, sets their working hours (e.g., 9 AM - 5 PM), and the platform generates a beautiful booking interface.
- **Pricing**: Cal.com API is highly scalable; basic usage can be very cheap or self-hosted. Direct Google Calendar API is free but requires more engineering effort to handle timezone and scheduling logic.
- **Cloud vs. Standalone**: Cal.com works well in Cloud. For Standalone, direct Google Calendar OAuth is more appropriate to avoid third-party API dependencies.
**Design Doc**:
- A new "Booking & Schedule" module in the OHC platform.
- User sets "Service Types" (e.g., 1-hour lesson).
- User connects external calendars to block out busy times.
- The storefront displays available time slots to the end customer.
- Upon booking, an event is created in the user's calendar.
**Implementation Prompt**:
Build a booking system that allows business owners to define services with durations and prices. Integrate with Google Calendar via OAuth to check for conflicts before displaying available times to the customer. When a customer books, automatically add the event to the business owner's calendar and send a confirmation email.
**Priority**: P0
**Estimated Scope**: Large
