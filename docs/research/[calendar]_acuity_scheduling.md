# Calendar & Scheduling: Acuity Scheduling (Squarespace)

**Problem Statement:** Small business owners (like consultants, salons, tutors) waste hours playing "email ping-pong" to find meeting times. They need an easy way for clients to book available slots automatically without double-booking.

**Research Report:** Acuity Scheduling is a mature, feature-rich scheduling tool now owned by Squarespace.
- Ease of Use: Very user-friendly for both the business owner and the client booking the appointment.
- Pricing: Affordable for SMBs (~$16-$50/month).
- Reputation: Highly trusted, reliable.
- Key Advantage: Native support for payment collection upon booking.
- Cloud vs. Standalone: Cloud-based. Standalone would use their API and webhooks for syncing.

**Design Doc:**
- User embeds an Acuity booking widget on their OHC-generated storefront or shares a booking link.
- Acuity syncs with the owner's Google/Outlook calendar to block out busy times.
- New bookings trigger a webhook to OHC to create a customer record or update an existing one.
- UI wireframes or screen flow description (375px first): A simple calendar view for the client to select a date, then available time slots.
- Mobile UX flow: The owner views upcoming appointments in a daily agenda view within the OHC app.

**Implementation Prompt:** Integrate Acuity Scheduling. Allow business owners to connect their Acuity account and display their booking page within their OHC site. Sync appointments to the OHC internal dashboard.
- Acceptance Criteria: Appointments made in Acuity appear in the OHC dashboard. Customer details are synced.

**Priority:** P1
**Estimated Scope:** Medium
