# [Calendar & Scheduling] Automated Meeting Sync via Cronofy

**Title**: Implement Universal Calendar Sync & Auto-Booking with Cronofy

**Problem Statement**:
Service providers like Leo (The Music Tutor) and Carlos (The Freelance Handyman) lose hours every week negotiating meeting times back and forth with clients. They need a system where customers can view their real-time availability and instantly book a slot. Critically, this system must automatically block out times when they have personal appointments in their existing Google or Apple Calendars, so they never get double-booked, and it must auto-generate a video link for online sessions.

**Research Report**:
I evaluated direct Google/Microsoft API integration, Cal.com API, and Cronofy API.
- **Cronofy**: A unified API that connects to Google, Apple, Microsoft Exchange, and Office 365. Handles all the complex timezone logic and conflicting event resolution. Has deep features for generating smart booking links. Excellent for SaaS platforms wanting to embed calendar features. Pricing is structured for platforms.
- **Cal.com API**: Open-source, very strong feature set, but their enterprise API pricing can be steep for early-stage platforms.
- **Direct Google/Apple APIs**: Zero cost but massive engineering overhead to maintain OAuth scopes, refresh tokens, and distinct API quirks across 3-4 different calendar ecosystems.
- **Conclusion**: Cronofy is the best infrastructure choice. It allows OHC to offer a flawless booking experience without the heavy engineering burden of maintaining individual calendar API connections. It supports both Cloud mode securely and Standalone mode (via local OAuth token persistence).

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department.
- **Triggers & Flow**:
  1. User goes to settings and connects their personal calendar (Google, Apple, etc.) via a simple OAuth popup.
  2. The user sets their "Working Hours" in OHC.
  3. OHC generates a public booking page link.
  4. When a customer visits the link, OHC queries the provider for real-time free/busy slots (ignoring private event details).
  5. The customer selects a slot and books.
  6. The provider pushes the new event back to the user's personal calendar and OHC's DB, triggering an auto-generated Zoom/Meet link if required.
- **User View**: A clean, mobile-optimized list of upcoming appointments. A simple toggle to "Sync my personal calendar".

**Implementation Prompt**:
Develop a universal calendar syncing and booking module. Integrate the selected calendar provider to allow the business owner to connect their Google, Apple, or Outlook calendar with a single click. The system must generate a public-facing booking UI for customers that accurately reflects real-time availability by cross-referencing OHC's working hours with the owner's personal calendar events. When a booking is made, the system must create an event in both OHC and the external calendar, and notify the "Manager" AI agent to send confirmation emails and reminders. The booking UI must be flawless on a 375px mobile screen.

**Priority**: P0
**Estimated Scope**: Large
