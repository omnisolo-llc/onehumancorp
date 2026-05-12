# Title: Implement Frictionless Booking via Cal.com Integration

## Problem Statement
Small business owners waste hours every week playing email ping-pong trying to find a time to meet with clients or schedule services. They need a simple, shareable link that lets customers pick a time that automatically syncs with the owner's actual availability.

## Research Report
- **Tool Evaluated:** Cal.com
- **Benefits:** Open-source, highly customizable, and natively supports two-way sync with Google Calendar, Outlook, and Apple Calendar.
- **Ease of Use:** Business owners connect their calendar once and get a simple link to share. Customers see a clean booking page.
- **Pricing:** Roughly $12/user/month for premium cloud features, but the core engine is open-source.
- **Cloud/Standalone:** Exceptional fit. It can be offered as a SaaS add-on in Cloud, and the open-source version can be bundled directly into the OHC Standalone deployment for ultimate privacy.

## Design Doc
1. **Trigger:** User connects their personal calendar and sets their working hours in the OHC settings.
2. **Action:** OHC generates a personalized booking link (e.g., `ohc.com/book/fatimas-bakery`).
3. **UI Outcome:** The business owner sees their upcoming appointments directly in the OHC dashboard. When an appointment is booked, it automatically blocks out time on their personal calendar to prevent double-booking.

## Implementation Prompt
Integrate Cal.com to provide a booking system for business owners. Build a setup wizard where owners can connect their existing calendars (Google/Outlook) and set their weekly availability. Generate a public booking page that they can share with customers. Ensure booked appointments appear in a central schedule view within the OHC app.

## Priority
P1

## Estimated Scope
Medium
