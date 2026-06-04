# [Calendar & Scheduling] Integrate Nylas for Booking Sync

## Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) struggle with double bookings. They need a simple booking page where customers can choose a time, and it must sync seamlessly with their existing personal calendars (Google, Outlook) to block out unavailable times automatically.

## Research Report
**Evaluated Tool:** Nylas Calendar API
**Alternatives Considered:** Cronofy, Cal.com
**Pros:** Highly reliable, broad support for almost all calendar providers (Google, Exchange, Office365, generic IMAP/CalDAV). Provides excellent unified data models and handles timezones gracefully.
**Cons:** Can be expensive at high volume.
**Ease of Use for Non-technical Users:** Simple "Connect Calendar" OAuth flow. Once connected, sync is automatic and invisible.
**Pricing:** Volume-based, typically per connected account.
**Deployment:** Fully functional in Cloud mode. Standalone may require BYO API keys.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer visits a tenant's booking page.
- **Action:** OHC queries the Nylas API for the tenant's free/busy schedule to render available slots.
- **AI Agent Interaction:** "The Operations Manager" uses this availability to schedule, reschedule, or cancel bookings.
- **User View:** A clean "Booking Configuration" screen where the owner sets working hours, and a public booking page showing available time slots.

## Implementation Prompt
Integrate the Nylas API to enable bi-directional calendar sync. Implement an OAuth flow for tenants to connect their Google/Outlook calendars. Build a frontend scheduling component that calculates and displays available time slots based on the synced calendar data and predefined working hours.

## Priority
P0

## Estimated Scope
Medium
