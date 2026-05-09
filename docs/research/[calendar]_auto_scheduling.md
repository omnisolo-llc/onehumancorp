# Calendar & Scheduling: Auto-Scheduling

## Problem Statement
Coordinating meeting times via back-and-forth emails is frustrating and wastes time. Business owners need a simple way to let clients book available slots on their calendar without double-booking over existing personal or business events.

## Research Report
**Selected Tool:** Cal.com
We evaluated Cal.com vs Calendly. Cal.com provides an open-source, highly customizable alternative that aligns perfectly with our Standalone and privacy-first goals.
- **Ease of use for non-technical users:** The end-customer booking experience is intuitive. For the owner, setting availability rules requires simple UI guidance.
- **Pricing:** Free for individuals. OHC can utilize the open-source version or API.
- **Reputation:** Highly regarded developer-friendly platform with strong momentum.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer clicks a "Book Now" link on the owner's OHC storefront.
- **Action:** OHC checks the owner's connected calendar (via Cal.com integration) for free slots and displays them. Upon booking, an event is created, and confirmation emails are sent.
- **User Interface:** The owner sees a simple "Availability" setting in OHC. The customer sees a clean, mobile-optimized booking calendar.
- **Environment:** Compatible with both Cloud (SaaS integration) and Standalone (can run alongside or integrate securely).

## Implementation Prompt
**User-Facing Outcome:** Business owners can connect their Google or Outlook calendar and share a single booking link. Customers pick a time, and the appointment appears automatically on the owner's calendar.
**Acceptance Criteria:**
- Easy calendar connection flow.
- Custom booking page matching the business's branding.
- Automatic prevention of double bookings.
- Automated confirmation and reminder emails to the customer.

## Priority
P1

## Estimated Scope
Medium
