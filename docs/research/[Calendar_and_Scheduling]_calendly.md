# [Calendar and Scheduling] Calendly Integration

**Title**: Integrate Calendly for automatic meeting and consultation booking

**Problem Statement**: Small business owners, such as consultants or tutors (like Carlos or Maya), spend too much time going back and forth over email or text to find a meeting time. They need a simple way for clients to book available slots without double-booking their personal calendars.

**Research Report**: Calendly is a highly popular SaaS scheduling automation platform.
- **Ease of use**: Excellent. It is known for its viral, user-friendly booking links and straightforward setup.
- **Pricing**: Freemium. A robust free tier exists for individuals. Premium versions offer more features (like payments) and start around $10/month.
- **Reputation**: Market leader in scheduling software, valued at $3 billion as of 2021.
- **Cloud/Standalone**: Functions primarily via Cloud webhooks and API. Standalone mode can generate links and fetch schedules if internet is available.

**Design Doc**:
- **Trigger**: User activates "Calendar Sync" in OHC and links their Calendly account.
- **Action**: OHC fetches the user's active event types (e.g., "30 Min Consultation") and generates shareable booking links.
- **User Experience**: The business owner sees a "My Booking Links" section in OHC. They can easily copy the link to send to clients via the unified inbox or SMS. OHC also shows upcoming appointments on the dashboard.

**Implementation Prompt**: Build a feature allowing business owners to connect their Calendly account. Display their active booking links for easy copying and sharing. Show a widget on the main dashboard listing their upcoming scheduled meetings for the day.

**Priority**: P1
**Estimated Scope**: Small