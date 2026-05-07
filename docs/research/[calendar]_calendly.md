# Automated Booking with Calendly Integration

## Problem Statement
Service-based small business owners (consultants, tutors, salon owners) spend hours every week going back and forth with clients over email or text to find a time to meet. Double bookings are common, and managing timezone differences is a headache. They need a way to let clients book available times instantly without manual intervention.

## Research Report
**Tool Evaluated:** Calendly API / Webhooks
- **Ease of Use:** Extremely high. Calendly is the most recognized scheduling tool. Business owners can set up their availability rules in Calendly and simply paste their link into OHC or connect their account to sync data.
- **Pricing:** Free tier available for basic scheduling (1 event type). Paid tiers start at $10/user/month for advanced features like routing and multiple event types.
- **Reputation:** The gold standard for scheduling. Highly reliable, excellent timezone handling, and integrations with all major calendars (Google, Outlook, Apple).
- **Deployment:** Fully supported in both Cloud and Standalone modes, as Calendly handles the heavy lifting of calendar sync and conflict resolution externally.

## Design Doc
- **Trigger:** User connects their Calendly account via OAuth or provides their personal Calendly booking link.
- **Action:** When a client books a meeting via the Calendly link, a webhook is sent to OHC. OHC creates a new customer record (if new) and adds an event to the customer's timeline in the CRM.
- **User View:** A "Schedule Meeting" block that can be dragged into their OHC storefront or sent in emails. The CRM shows a clear history of all upcoming and past meetings with that customer.

## Implementation Prompt
Integrate Calendly to allow business owners to embed their booking pages into their OHC storefronts and emails. Implement webhook listeners so that whenever a booking is created, rescheduled, or canceled in Calendly, the corresponding customer record in OHC's CRM is automatically updated with the event details.

## Priority
P1

## Estimated Scope
Medium