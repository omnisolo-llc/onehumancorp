## 2. Calendar & Scheduling: Acuity Scheduling

**Title:** Integrate Acuity Scheduling for Advanced Booking Options
**Problem Statement:** Business owners need a way for clients to book appointments, classes, and consultations without endless back-and-forth emails, while also handling timezones properly.
**Research Report:**
- **Tool evaluated:** Acuity Scheduling
- **What problem it solves for which persona:** Solves the scheduling hassle for service-based owners (like tutors or consultants).
- **Ease of Use:** Very customizable but slightly steeper learning curve than Calendly.
- **Pricing:** Starts at around $15/month.
- **Reputation:** Very well-regarded, especially since being acquired by Squarespace.
- **Advantages & Risks:**
  - *Advantages:* Deep customization, built-in payment collection during booking.
  - *Risks:* Might be overkill for very simple needs; pricing is a bit higher.
- **Cloud/Standalone Mode:** Fully supported in Cloud via API webhooks. Standalone might require webhook relays.
**Design Doc:**
- **Trigger:** A customer clicks 'Book Now' on the OHC business storefront.
- **Action:** An inline widget displays available times from Acuity. Upon booking, it syncs back to OHC's internal calendar.
- **User View:** The business owner manages their availability in OHC, which bidirectional-syncs with Acuity.
**Implementation Prompt:**
Embed an appointment booking widget into the customer-facing storefront. In the business dashboard, display a list of upcoming appointments. Acceptance criteria: A customer can select a time slot and the business owner receives a notification inside OHC.
**Priority:** P2
**Estimated Scope:** Small
