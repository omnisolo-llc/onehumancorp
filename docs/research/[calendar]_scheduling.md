**Title**: Automated Meeting Scheduling with Calendly
**Problem Statement**: Service-based owners like Leo (music tutor) spend hours playing "email ping-pong" trying to find a time that works for a lesson or consultation. This wastes time, looks unprofessional, and occasionally results in double bookings. They need a way to just send a link where clients can pick an available time.
**Research Report**:
- **Calendly**: Founded in 2013, Calendly is a market leader and tech unicorn valued at $3 billion (as of 2021). It allows users to share open time slots in their calendars to book meetings by sending a scheduling link.
- **Ease of Use**: Very high. The concept of "send a link, book a time" is universally understood now. It automatically syncs with Google and Microsoft Outlook calendars to prevent double-booking.
- **Pricing**: Offers a strong freemium model. Premium tiers offer team scheduling and payment integrations.
- **Reputation**: Widely recognized and trusted, despite minor internet debates about the "etiquette" of sending a link, it remains the standard.
- **Cloud/Standalone**: Primarily cloud-based SaaS. For Standalone mode, we would need to leverage their robust API or use webhooks to sync events back to a local OHC instance.
**Design Doc**:
- **Trigger**: Leo wants to schedule a new student. He generates a personal booking link from OHC.
- **Action**: The student clicks the link, sees Leo's availability (synced via Calendly API), and picks a time. Calendly handles the calendar invite and sends a webhook to OHC.
- **UI**: OHC dashboard includes a "Scheduling" tab. Here, Leo can see his upcoming appointments. A prominent "Copy Booking Link" button is always available. When an appointment is booked, it appears in his OHC timeline.
**Implementation Prompt**: Integrate Calendly so users can generate and manage scheduling links directly from OHC. Implement webhooks to listen for new bookings, cancellations, or reschedules, and update the OHC database accordingly so the user's OHC dashboard reflects their real schedule. Ensure the setup flow is as simple as clicking "Connect Calendly" and authorizing the app.
**Priority**: P0
**Estimated Scope**: Medium
