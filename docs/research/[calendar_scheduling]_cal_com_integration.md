# Title: Seamless Booking & Scheduling via Cal.com Integration

## Problem Statement
Service providers like Leo (The Music Tutor) and Carlos (The Freelance Handyman) need a way for clients to book their time without the back-and-forth of "when are you free?". They need a reliable scheduling system that syncs with their personal calendars (Google/Outlook) so they don't get double-booked, and it needs to be as simple as sharing a link.

## Research Report
**Findings & Evaluation:**
- **Cal.com:** An open-source scheduling infrastructure. It provides robust APIs, webhook support, and pre-built booking components. It handles timezone math, calendar conflict resolution (Google Calendar, Outlook, Apple Calendar), and availability logic.
- **Calendly:** The market leader, but their API is less developer-friendly for deep white-label integration and their pricing model is rigid.
- **Ease of Use:** With Cal.com API, we can embed the booking flow natively into the OHC storefront. The business owner only needs to connect their Google/Outlook account once.
- **Pricing:** Cal.com has an open-source tier which is highly attractive for OHC's margins, allowing us to offer scheduling in the free tier.
- **Cloud vs Standalone:** Excellent for both. In Standalone, users can self-host the Cal.com core or rely on the API.

## Design Doc
**Integration with OHC:**
The Operations Agent ("The Manager") handles service creation. When a user creates a "Service" (e.g., 1-hour plumbing repair), they define their general availability in the OHC app.
OHC communicates with Cal.com via API to create an event type. The user connects their personal calendar using Cal.com's OAuth flow.
On the public OHC storefront, a native-looking booking widget (powered by Cal.com's embed or API) allows customers to pick a slot. When booked, Cal.com fires a webhook to OHC, triggering the Finance Agent to request a deposit and the Operations Agent to update the internal dashboard.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- The business owner can define a service duration and available hours in the OHC app.
- The owner can connect their Google or Outlook calendar with one click to prevent double bookings.
- Customers visiting the public storefront see a calendar view to select an available date and time.
- Timezones are automatically handled for both the business owner and the customer.
- Booked appointments appear in the OHC dashboard and the owner's personal calendar.

## Priority
P1

## Estimated Scope
Medium
