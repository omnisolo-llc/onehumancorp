## [Calendar & Scheduling] Issue Brief: Native Booking Engine & Google Calendar Sync

**Title**: Scout 🔍: Integrate Cal.com API for Conflict-Free Booking
**Problem Statement**:
Service providers like Leo (Music Tutor) and Carlos (Handyman) need clients to book their time. Doing this manually via text or email leads to double-booking and timezone confusion. They need a simple booking widget that automatically respects their personal calendar availability, without requiring them to set up a separate complex scheduling tool.

**Research Report**:
- **Tool**: Cal.com API.
- **Evaluation**: Cal.com offers an open-source, API-first scheduling infrastructure. It handles timezone math, calendar conflict checks, and event generation seamlessly. It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- **Ease of Use**: High. The business owner connects their Google/Outlook calendar with one click, sets their working hours (e.g., 9 AM - 5 PM), and the platform generates a beautiful booking interface natively.
- **Advantages**: Handles all complex scheduling math. Allows OHC to offer premium scheduling without building it from scratch.
- **Risks**: Dependency on an external service for core functionality.
- **Pricing**: Cal.com API is highly scalable; free tier available for individuals; great for our free tier users.
- **Compatibility**: Cal.com works well in Cloud. For Standalone, it can be self-hosted or user provides their own API key.

**Design Doc**:
- A new "Booking & Schedule" module natively in the OHC platform.
- User sets "Service Types" (e.g., 1-hour lesson).
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect external calendars (Google/Outlook) to block out busy times via a one-click OAuth button in the "Operations" tab.
- The storefront displays available time slots to the end customer seamlessly.
- Upon booking, Cal.com manages the calendar event and conflict resolution transparently.

**Implementation Prompt**:
Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking. Build a booking system that allows business owners to define services with durations and prices natively in OHC. Integrate with Cal.com to handle the complex calendar conflict logic.
- **Acceptance Criteria**: Merchant can connect their external calendar. Customers can view availability and book a timeslot without double-booking. Events sync correctly to the merchant's calendar.
**Priority**: P0
**Estimated Scope**: Large
