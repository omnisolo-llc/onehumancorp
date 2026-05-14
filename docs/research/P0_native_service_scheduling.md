# [P0] Native Service Scheduling & Booking Engine

**Problem Statement:** Gap, pain point, or opportunity — framed from a non-technical small business owner's perspective.
Service-based SMBs (tutors, handymen, like 'Leo' or 'Carlos') cannot use OHC effectively because we lack a simple way for their customers to book time slots. They are forced to piece together clunky external tools like Calendly or rely on phone calls.

**Research Report:** Findings, data, competitive comparison, sources.
- Our Deep Competitor Audit indicates Shopify fails significantly at service-based businesses.
- Wix has a booking engine but the mobile admin experience is poor.
- Reddit r/smallbusiness is full of service owners begging for a unified "website + booking + payments" tool.
- Service businesses are our recommended beachhead market due to the lack of dedicated, easy-to-use platforms compared to physical goods.

**Design Doc:** High-level architecture, UI wireframes, mobile UX flow, AI agent integration points.
- **Entities:** Service, TimeSlot, Booking, Calendar.
- **Mobile UX Flow (375px first):**
  1. User creates a "Service" (e.g., "1 Hour Guitar Lesson").
  2. User defines working hours in 3 taps (e.g., M-F, 9-5).
  3. Shareable booking link is automatically generated and added to their storefront.
  4. Calendar view optimized for 375px screens to manage upcoming bookings.
- **AI Agent Integration Points:**
  - The 'Operator' agent automatically sends SMS/email reminders to the customer 24h before the booking.
  - The 'Operator' agent drafts a follow-up email asking for a review post-service.

**Implementation Prompt:**
Build a native scheduling primitive. Users must be able to define a service with a duration and price, set their general availability, and accept a booking via a customer-facing interface. The booking UI must be embeddable on their OHC storefront. Ensure strict mobile parity for the admin view of the calendar.

**Priority:** P0
**Estimated Scope:** Large
