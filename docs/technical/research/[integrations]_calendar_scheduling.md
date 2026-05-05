# [Calendar] Integrate Google Calendar for Bi-Directional Sync

## Problem Statement
Service-based businesses (like Leo the Music Tutor) manage their personal and business schedules primarily in Google Calendar. If OHC allows a booking when the owner is at a dentist appointment, it causes double-booking friction. They need OHC to read their external availability and sync new bookings back to their personal calendar.

## Research Report
**Evaluated Tool:** Google Calendar API
**Alternatives Considered:** Nylas, Cronofy
**Pros:** Native integration with the most popular calendar platform. Free API usage within standard quotas. Avoids third-party aggregator costs.
**Cons:** Only covers Google users (Outlook/Apple require separate integrations later). Google OAuth verification process can be tedious.
**Ease of Use for Non-technical Users:** The user clicks "Sign in with Google", grants calendar access, and OHC immediately blocks off time slots where the user is already busy.
**Pricing:** Free API tier is generally sufficient for SMB volume.
**Deployment:** Cloud-native (OAuth requires web redirects).

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer views the booking page, or a new booking is created.
- **Action:** OHC queries the connected Google Calendar for "busy" blocks before displaying available slots. When a booking is confirmed, OHC inserts a new event into the Google Calendar.
- **AI Agent Interaction:** "The Operations Manager" monitors calendar conflicts and can suggest rescheduling if the owner manually double-books themselves in Google Calendar.
- **User View:** A "Calendar Sync" settings page, and a calendar view in the OHC dashboard that overlays external events (read-only) with OHC bookings.

## Implementation Prompt
Integrate the Google Calendar API for bi-directional synchronization. Implement the Google OAuth flow requesting calendar read/write scopes. Update the availability calculation logic to exclude times marked as "busy" in the connected Google Calendar. Implement a background sync to push OHC bookings to Google Calendar.

## Priority
P0

## Estimated Scope
Medium
