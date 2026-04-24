# 🔍 Scout: Acuity Scheduling (Calendar & Scheduling)

## Title
Integrate Acuity Scheduling for Advanced Appointment Management

## Problem Statement
Service-based business owners (like Carlos the Handyman or Leo the Music Tutor) require flexible and robust booking systems. Basic calendar syncing isn't enough; they need to handle complex scheduling scenarios like buffer times between appointments, group classes, recurring bookings, and taking deposits upfront. Without this, they waste time coordinating schedules manually and suffer from no-shows.

## Research Report
**Acuity Scheduling** (owned by Squarespace) is a powerful, highly customizable online appointment scheduling software. It offers a comprehensive API that allows developers to deeply integrate its booking flows into external platforms.

**Pros for Non-Technical Users:**
- Handles complex scheduling logic automatically.
- Sends customizable confirmation and reminder emails/SMS.
- Supports taking payments/deposits at the time of booking.
- Syncs seamlessly with Google Calendar, Outlook, and iCloud.

**Integration Risks:**
- Acuity is a premium product; there is no free tier (only a free trial). This adds an extra cost burden for the small business owner.
- The API is extensive, making a deep integration complex.
- Users must manage two systems (Acuity for complex settings, OHC for the frontend UI).

**Pricing:**
- Starts at $20/month. No free tier.

**Environment Support:**
- Cloud-only. Requires an active Acuity subscription and API access.

## Design Doc
- **Integration:** The OHC user authenticates their existing Acuity account via OAuth or API Key within the OHC settings.
- **Data Flow:** OHC fetches available appointment types and times from Acuity to display on the business's public storefront. When a customer books via OHC, the booking is pushed to Acuity, which then handles the calendar placement and confirmation emails.
- **Action:** The "Operations" agent monitors Acuity webhooks for new bookings, cancellations, or reschedules, and updates the OHC dashboard and customer records accordingly.

## Implementation Prompt
Build an integration module for the Acuity Scheduling API. First, implement a settings page for users to input their Acuity API credentials. Second, create a frontend widget for the OHC storefront that queries Acuity for available time slots for specific appointment types and allows customers to book them. Finally, set up webhook listeners to receive real-time updates from Acuity about booking status changes to keep the OHC database in sync. Do not attempt to replicate Acuity's complex configuration UI; rely on Acuity for setup and OHC for display.

## Priority
P2

## Estimated Scope
Large
