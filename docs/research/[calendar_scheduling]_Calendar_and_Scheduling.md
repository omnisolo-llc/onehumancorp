# Calendar & Scheduling Integration

## Title
Integrate Calendly for Calendar & Scheduling

## Problem Statement
Booking consultations or service appointments involves endless back-and-forth emails. Small business owners need a simple way to let clients book available times without double-booking.

## Research Report
**Tool Evaluated:** Calendly
**Pricing:** Free tier; $10/mo premium
**Cloud/Standalone Support:** Cloud: Yes. Standalone: Yes (API driven).

**Findings:**
Calendly is the industry standard. It handles Google/Outlook calendar sync, timezone math, and conflict resolution perfectly. It has a robust API and iframe embedding. Non-technical users understand the Calendly interface intuitively. Free tier available, paid starts at $10/mo.

## Design Doc
Business owners can connect their Calendly account via OAuth. OHC will embed their Calendly booking page directly into their storefront or customer portal. Appointments booked via Calendly will sync back into OHC's internal CRM to track customer interactions.

## Implementation Prompt
Implement a Calendly integration that allows business owners to link their account and display a booking widget on their public profile. New bookings should automatically create an event in the business owner's OHC dashboard.

## Priority
P1

## Estimated Scope
Small
