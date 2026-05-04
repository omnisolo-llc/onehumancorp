# [calendar] AI-Assisted Calendar & Scheduling Sync

## Title
Implement AI-Assisted Calendar & Scheduling Sync

## Problem Statement
Service-based small business owners like Carlos (Freelance Handyman) and Leo (Music Tutor) rely on accurate scheduling to manage their time and revenue. Managing separate calendars for personal life, OHC bookings, and other platforms leads to double-booking and frustration. They need a seamless, automated way to sync their external calendars (like Google Calendar or Outlook) with OHC so that their availability is always accurate, and they want AI to handle the back-and-forth of rescheduling and reminders.

## Research Report
### Market Evaluation
- **Google Calendar API**: The dominant calendar platform for SMBs and personal use.
    - *Ease of use (for user)*: Simple "Sign in with Google" OAuth flow. Familiar to almost everyone.
    - *Pricing*: Free API access within standard GCP limits.
    - *Cloud vs. Standalone*: Works well in Cloud. In Standalone, requires users to provide their own Google Cloud Console API credentials, which is complex for non-technical users.
- **Microsoft Graph API (Outlook)**: Second most common, especially for established businesses or B2B contexts.
    - *Ease of use (for user)*: Microsoft OAuth flow. Can be confusing if users have mixed personal/work Microsoft accounts.
    - *Pricing*: Free API access within limits.
    - *Cloud vs. Standalone*: Same as Google; Standalone requires personal Azure AD app credentials.
- **CalDAV**: Open standard supported by Apple Calendar and others.
    - *Ease of use (for user)*: Complex. Often requires generating app-specific passwords or manual server configuration. Not suitable for non-technical users.
- **Scheduling Aggregators (e.g., Cronofy, Nylas)**:
    - *Pros*: Unified API for all calendar providers. Handles the complex sync logic.
    - *Cons*: High cost per connected account, which breaks OHC's accessible pricing model for free-tier users.

### Integration Risks & Considerations
- **Sync Directionality**: Two-way sync is complex. OHC needs to read external events to block time slots (read) and write OHC bookings back to the external calendar (write). Deletions and modifications require careful state management.
- **Timezone Hell**: Handling timezones correctly across user devices, external calendars, and servers is notoriously difficult and a major source of bugs.
- **API Quotas**: Frequent polling for calendar updates can quickly exhaust free API limits. Push notifications (webhooks) from Google/Microsoft are necessary but complex to manage per tenant.

## Design Doc
### User Experience
1. **Connection**: The user goes to the "Operations" department tab and clicks "Connect Calendar". They select Google or Outlook and complete the standard OAuth flow.
2. **Availability Rules**: The user sets their general working hours (e.g., 9 AM - 5 PM) in OHC.
3. **Smart Booking**: When a customer tries to book Carlos on his OHC public page, the OHC system checks his connected Google Calendar. Any events marked "Busy" dynamically remove those time slots from the OHC booking page.
4. **AI Rescheduling**: If a user needs to reschedule, they can reply to the confirmation email or text. "The Operations Manager" agent reads the request, checks availability, and proposes new times automatically, updating the calendar when agreed.

### System Flow
- User connects Google Calendar via OAuth → OHC requests `calendar.readonly` and `calendar.events` scopes.
- OHC stores the refresh token securely.
- When a customer views a booking page, OHC fetches the user's availability window, queries the connected calendar API for events within that window, and calculates the true free slots.
- When a booking is confirmed, OHC writes the event to the connected calendar with details and optionally a video link (handled separately).
- OHC registers for calendar push notifications (webhooks) to invalidate cached availability if the user adds a personal event.

## Implementation Prompt
Implement a two-way calendar sync feature prioritizing Google Calendar via OAuth. Users must be able to connect their calendar, which should act as the source of truth for their availability on their public OHC booking pages. When an OHC booking is made, it must automatically create an event on the user's connected calendar. Integrate the "Operations" AI agent to handle natural language rescheduling requests from customers. Ensure timezones are handled robustly and the UI for connecting the calendar is foolproof. Do not prescribe specific database schemas or API endpoints; focus on the user flow of connecting, calculating availability, and booking.

## Priority
P1

## Estimated Scope
Medium