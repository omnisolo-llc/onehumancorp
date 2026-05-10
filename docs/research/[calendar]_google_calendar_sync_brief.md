# Google Calendar Synchronization

## Problem Statement
Small business owners (like consultants, barbers, or fitness instructors) spend hours every week managing their schedules. Double booking is a massive pain point that leads to unhappy customers and lost revenue. They often rely on manual diary entries or basic Google Calendar usage. They need a system that automatically blocks out time when an appointment is made, and allows clients to book available slots without requiring back-and-forth emails or calls.

## Research Report
Google Calendar integration allows OHC to read free/busy times and create events directly on the owner's behalf.
- **Benefits for Users:** Fully automated scheduling. Customers can book times on a custom OHC-generated booking page, which instantly reflects on the owner's phone via Google Calendar.
- **Ease of Use:** "Connect Google" button is universally understood by non-technical users. Once authenticated, the synchronization happens transparently in the background.
- **Reputation:** Google APIs are rock-solid and universally trusted.
- **Pricing:** Free tier provides vast amounts of API calls, easily sufficient for standard small business needs.
- **Environment Compatibility:** Works perfectly in both Cloud and Standalone modes. For Standalone, OHC handles local OAuth callback handling to securely store tokens in the local SQLite database.

## Design Doc
```mermaid
graph TD
    User(Small Business Owner) -->|Clicks Connect| OHC_UI[OHC Setup UI]
    OHC_UI -->|OAuth Consent| Google[Google OAuth]
    Google -->|Returns Token| OHC_Backend[OHC Backend]
    OHC_Backend -->|Stores Token Securely| DB[(SIPDB / Postgres)]
    Client(Customer) -->|Books Appointment| BookingPage[OHC Booking Page]
    BookingPage --> OHC_Backend
    OHC_Backend -->|Creates Event| GCal[Google Calendar API]
    GCal -->|Syncs to Phone| Mobile[Owner's Phone Calendar]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class OHC_UI,OHC_Backend,DB,BookingPage premium;
```

The user authorizes OHC via OAuth. OHC fetches free/busy availability to construct the booking page. When an appointment is booked, OHC writes a new event to Google Calendar, instantly updating the business owner's schedule on all their personal devices.

## Implementation Prompt
Integrate Google Calendar sync for scheduling automation.
- **User Outcome:** A user connects their Google account and receives a personalized booking link. Any booked appointment immediately appears on their personal Google Calendar.
- **Acceptance Criteria:**
  - Complete OAuth2 flow integration.
  - Capability to fetch "free/busy" slots.
  - Capability to create and update calendar events.
  - Provide a visually premium generated booking page for the end-customer.

## Priority
P1

## Estimated Scope
Large
