# [Calendar & Scheduling] Cal.com Integration

**Title**: Implement Cal.com Integration for Unified Booking Management

**Problem Statement**:
Small business owners, like Leo the Music Tutor, struggle with manually managing appointments across multiple platforms and resolving timezone conflicts. They need an automated, reliable system that allows clients to book available slots, automatically generates meeting links (e.g., Zoom/Meet), and syncs natively with Google Calendar or Outlook without requiring technical setup.

**Research Report**:
Cal.com is an open-source, developer-friendly scheduling infrastructure tool. It is an excellent fit for OneHumanCorp because it offers a white-label booking experience and extensive integration capabilities (Google Calendar, Outlook, Zoom).
- **Ease of Use for Non-Technical Users**: The end-user experience is straightforward. Business owners can share a simple booking link or embed it in their storefront.
- **Pricing**: Cal.com has a generous free tier for individuals, which aligns perfectly with OHC's goal of offering a genuinely useful free tier. Enterprise plans exist for scaled usage.
- **Reputation**: Highly regarded in the developer community for its reliability, open-source nature, and clean API.

**Design Doc**:
- **Trigger**: The business owner connects their calendar (Google/Outlook) and sets availability preferences via the "Operations" or "Salesperson" AI agent in the OHC dashboard.
- **Action**: A public booking page is generated and linked to their OHC storefront. When a client books a slot, Cal.com handles the calendar invite and auto-generates a Zoom/Meet link. The booking details are synced back to the OHC unified inbox.
- **User View**: The business owner sees upcoming appointments in their OHC calendar view and receives automated summaries from their AI Advisor.

**Implementation Prompt**:
Integrate Cal.com scheduling into the OHC platform. Ensure that users can connect their preferred calendar, set availability, and expose a booking interface on their public storefront. When a booking is made, it should appear in the OHC dashboard and trigger an event that the AI agents can use to follow up or send reminders. The feature must work seamlessly across both Cloud (via Cal.com API) and Standalone modes.

**Priority**: P1
**Estimated Scope**: Medium
