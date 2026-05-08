# Title: Google Calendar Synchronization for Bookings
## Problem Statement
Small business owners, like consultants or tutors, get double-booked when clients schedule appointments on OHC while the owner already has personal or other business events on their personal Google Calendar. They need a hands-off way to block OHC availability based on their existing calendar and push new OHC bookings directly to Google Calendar.

## Research Report
* **Tool:** Google Calendar API
* **What it does:** Syncs events two-way and checks free/busy schedules.
* **Ease of Use for Owners:** Very high. "Sign in with Google" is a familiar pattern.
* **Pricing:** Free tier is extremely generous, effectively free for SMB usage.
* **Cloud vs. Standalone:**
  * Cloud: Straightforward OAuth using OHC's Google Cloud project.
  * Standalone: Same proxy requirement as social media, or local OAuth flow with a bundled client ID (easier than Meta).

## Design Doc
* **Trigger:** User clicks "Sync with Google Calendar" in their booking settings.
* **Action:** Two-way sync. OHC reads "busy" times to prevent double-booking and writes new OHC appointments as events to the selected calendar.
* **User Experience:** The owner's OHC booking page dynamically removes time slots where they have Google Calendar events. OHC bookings appear natively on their phone's Google Calendar app.

## Implementation Prompt
Build the Google Calendar integration so that an owner can connect their Google account, select a primary calendar, and have their OHC scheduling availability automatically updated. The acceptance criteria is that a manually created event in Google Calendar must instantly remove that time slot from the OHC public booking page, and an OHC booking must appear in Google Calendar.

## Priority
P1

## Estimated Scope
Medium
