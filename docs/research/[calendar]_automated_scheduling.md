# Automated Scheduling and Calendar Sync

## Problem Statement
Consultants, service providers, and tutors spend too much time going back and forth via email to find a time to meet. Double-booking is a constant fear, and manual calendar management takes time away from actual work.

## Research Report
**Competitive Landscape:**
1. **Calendly:** The industry standard. Easy for users, but expensive on paid tiers.
2. **Cal.com:** Open-source, developer-friendly, great API. White-labeling is possible.
3. **Google Calendar API (Direct):** Requires building scheduling logic from scratch, but avoids third-party fees.

**Evaluation:**
- **Ease of Use:** Cal.com offers a seamless embedded booking experience.
- **Pricing:** Cal.com has favorable pricing for platforms.
- **Cloud vs Standalone:** Cal.com can be self-hosted, making it ideal for OHC Standalone.

## Design Doc
- **Trigger:** User creates a 'Booking Service' in OHC (e.g., '1-Hour Consultation').
- **Action:** OHC generates a public booking link. When a customer books, OHC syncs it to the owner's Google/Outlook calendar and creates a customer record.
- **User Experience:** A settings page to connect their calendar, and a shareable public page for their clients.

## Implementation Prompt
Implement a native scheduling experience. The user connects their Google Calendar. OHC reads their free/busy status. Create a public-facing booking page where customers can select available slots. Upon booking, an event is added to the user's calendar, and a confirmation email is sent to the customer. The UI should be simple, focusing on 'Available Hours' and 'Buffer Times'.

## Priority
P1

## Estimated Scope
Medium
