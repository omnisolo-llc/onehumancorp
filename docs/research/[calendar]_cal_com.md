## 1. Calendar & Scheduling: Cal.com
**Problem Statement:** Small business owners (like consultants, therapists, or tutors) spend hours coordinating meeting times via email, leading to lost bookings and frustration. They need a simple, professional way for clients to book time that automatically respects their personal calendar.
**Research Report:**
- **Tool:** Cal.com
- **Persona Benefit:** Non-technical owners can share a link and clients can pick available slots.
- **Key Advantages:** Open-source, highly customizable, supports white-labeling, and works well for both cloud and standalone instances. It has a generous free tier for individuals and a solid $12/user/month tier for teams.
- **Risks:** Might be overwhelming if configured with too many advanced workflows for very basic users.
- **Pricing:** Free for individuals; $12/user/month for Teams.
- **Environment:** Works in both Cloud (multi-tenant) and Standalone (self-hosted).
**Design Doc:**
- **Trigger:** A "Book Consultation" button on the business's public profile or shared via SMS/Email.
- **Action:** Opens a clean, branded booking page showing available slots synchronized with the owner's Google/Outlook calendar.
- **User View:** The business owner sees upcoming appointments in their unified dashboard; the client receives an automated calendar invite.
**Implementation Prompt:** Integrate Cal.com so business owners can generate personal booking links directly from their dashboard. The UI should display their upcoming bookings and allow them to copy their booking link to share with customers.
**Priority:** P1
**Estimated Scope:** Medium
