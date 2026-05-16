# Scout: Tool Integration Research

## Calendar & Scheduling
**Title**: Integrate Acuity Scheduling for Advanced Service Booking
**Problem Statement**: Service-based businesses like salons, tutors, and consultants need more than just a simple calendar event. They need to manage intake forms, specific service durations, buffer times, and integrated deposits during the booking process to prevent no-shows.
**Research Report**:
- Acuity Scheduling (by Squarespace) is a powerhouse for service businesses. It handles complex scheduling rules, multi-staff availability, and up-front payments.
- Highly customizable booking pages that can be seamlessly embedded into websites.
- Pricing: Starts around $16/month. It does not have a free tier like Cal.com, but targets a slightly more mature business needing advanced features.
- Compatibility: Embeddable via IFRAME/JS in both Cloud and Standalone modes. API webhooks can sync appointments back to OHC.
- It solves the "no-show" problem by enforcing deposits at the time of scheduling.
**Design Doc**:
- Users configuring a "Service" item in their OHC Storefront can select "Advanced Booking (Acuity)".
- They provide their Acuity embed link or authenticate via OAuth to select specific appointment types.
- The OHC Storefront renders the Acuity booking widget directly on the product page.
- "The Manager" AI reads webhook events from Acuity to update the OHC dashboard with upcoming appointments and expected revenue.
**Implementation Prompt**: Add support for embedding Acuity Scheduling booking widgets into the OHC storefront. Implement webhook listeners to update the internal OHC dashboard when new appointments are booked or canceled via Acuity.
**Priority**: P2
**Estimated Scope**: Medium