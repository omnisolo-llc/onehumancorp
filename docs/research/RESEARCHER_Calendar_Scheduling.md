# Calendar & Scheduling Brief

## Problem Statement
The back-and-forth emails required to find a mutually agreeable meeting time are tedious and time-consuming for both the business owner and their clients.

## Research Report
**Tool Evaluated:** Calendly
**Findings:** Calendly is an industry standard for self-serve scheduling. It allows customers to book appointments based on the owner's real-time availability. It's especially useful for consultants, therapists, and service-based businesses.
**Pricing:** Free tier available; paid starts around $8-$12/month.
**Ease of Use:** Extremely user-friendly for both the owner and the customer.
**Risks:** Calendar sync issues can occasionally cause double-booking if multiple calendars aren't managed correctly. Timezone confusion can still occur for international clients despite automated handling.

## Design Doc
**Trigger:** Customer clicks a "Book an Appointment" link on the business's OHC profile or website.
**Action:** The customer is presented with available time slots. Upon selection, an event is created in the owner's calendar, and a confirmation is sent to both parties.
**User Experience:** The business owner sets their availability rules within OHC. Customers see a simple calendar interface to choose a time.

## Implementation Prompt
**Outcome:** An integrated scheduling tool where business owners can define their working hours and share a booking link with clients. Clients can book available slots without needing to communicate back and forth.
**Acceptance Criteria:**
- Owner can set availability and define meeting types (e.g., 30-min consultation).
- Customer can successfully book a time slot.
- Bookings automatically block out time on the owner's calendar.

## Priority
P1

## Estimated Scope
Medium
