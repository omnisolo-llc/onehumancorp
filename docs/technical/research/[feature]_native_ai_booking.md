# [feature] Native AI Booking & Scheduling Agent

**Problem Statement:** Service businesses like tutors and handymen lose leads because they have no automated way to take bookings. They face "Integration Chaos" trying to connect third-party calendars.

**Research Report:** Competitors like Wix and Squarespace offer booking, but it's a manual setup process. Shopify requires paid third-party apps. Real users (like Carlos the Handyman) miss leads when busy. Zyro relies entirely on third-party embed codes.

**Design Doc:**
- **Entities:** Service, Availability, Booking, Customer.
- **UX:** Mobile-first calendar interface where the user sets availability. An autonomous AI agent handles the actual booking conversation via SMS/Web chat.
- **Mobile Flow (375px):**
  1. Tap "Services"
  2. Tap "Add Service"
  3. Set Price and Time
  4. AI generates booking link.

**Implementation Prompt:** Implement a native booking engine where customers can schedule services. The AI should manage the calendar, handle rescheduling requests via natural language, and send automated reminders.

**Priority:** P0

**Estimated Scope:** Large
