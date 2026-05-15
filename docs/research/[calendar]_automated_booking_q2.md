# [calendar] Issue Brief: Automated Booking & Calendar Sync

**Title**: Google Calendar & Outlook Sync for Service Bookings
**Problem Statement**: As a freelancer like Carlos (the handyman) or Leo (the tutor), I manage my availability on my personal Google Calendar. Right now, when someone books me, I have to manually copy it to my calendar, and I sometimes get double-booked. I need my OHC booking page to automatically read my calendar to block off busy times, and automatically add new bookings to my schedule.
**Research Report**:
- Evaluated Tools: Native Google Calendar API, Microsoft Graph API (Outlook), Nylas, Cronofy.
- Ease of Use: Nylas and Cronofy provide unified APIs for all calendar providers, drastically simplifying integration. However, they cost per connected account. Given OHC's target audience and free tier, native Google Calendar API covers ~80% of users at zero cost. Microsoft Graph covers another 15%.
- Pricing: Google Calendar API is free (subject to generous quotas). Nylas starts around $1-$2/account/month.
- Reputation: Google Calendar is the gold standard for small businesses.
- Environment: Works in both Cloud and Standalone (with local OAuth credentials).
- Recommendation: Build native integration with Google Calendar API first, as it covers the vast majority of our target personas (Leo, Carlos) for free.
**Design Doc**:
- **Integration Flow**: In the Operations department settings, user clicks "Sync Google Calendar". Standard Google OAuth consent screen asks for calendar read/write permissions.
- **Actions**:
  - *Read*: When displaying available booking slots on the public storefront, OHC queries the synced calendar for "busy" blocks and removes those slots.
  - *Write*: When a customer books a slot and pays the deposit, OHC creates a new Event on the Google Calendar containing the customer details and service description.
- **User Interface**: A simple toggle to connect/disconnect the calendar, and a dropdown to select which specific calendar (e.g., "Work" vs "Personal") to sync with.
**Implementation Prompt**: Implement a Google Calendar synchronization feature. Users should be able to link their Google account via OAuth. Once linked, the public booking page must automatically hide time slots that overlap with the user's existing Google Calendar events. When a new booking is confirmed, it must automatically appear on the user's Google Calendar. Acceptance criteria: prevents double-booking, successfully writes events, handles timezone conversions correctly.
**Priority**: P1
**Estimated Scope**: Medium
