# Issue Brief: Proactive Service Booking & Subscription Engine

## Problem Statement
Service-based founders like Leo (Music Tutor) and Carlos (Handyman) lose hours to "calendar tetris" and manual follow-ups. They often rely on one-time payments, missing out on the stability of recurring revenue (subscriptions). Existing tools (Wix Bookings, Calendly) are passive—they wait for the customer to act.

## Research Report
- **Competitor Audit:**
    - **Wix Bookings:** Comprehensive but complex. Supports recurring classes but setup is "spaceship cockpit" style.
    - **Calendly:** Great for scheduling, but poor for integrated payments and business management.
- **SMB Pain Point:** Service owners report a 25% "no-show" or "forgot to rebook" rate (Source: Reddit r/smallbusiness synthesis).
- **Leapfrog Advantage:** OHC's "The Advisor" and "The Ambassador" agents work together to proactively fill the calendar. If a student hasn't booked in 2 weeks, the agent drafts a personalized re-engagement message with a direct booking link.

## Design Doc
### High-Level Architecture
- **Unified Booking & Subscriptions:** Treat a "service" as a product that can be bought once or as a recurring "Pass" (e.g., 4 lessons/month).
- **Agent Interventions:** "The Ambassador" monitors booking history and automatically triggers re-engagement tasks in the "Action Required" feed.
- **Calendar Sync:** Two-way sync with Google/iCal to prevent double-booking.

### Mobile UX Flow (375px First)
1. **Setup:** "The Manager" asks 3 questions: "What do you do?", "When are you free?", "Do you want recurring clients?".
2. **Storefront:** Customer sees a simple "Book Now" button with options for "One-time" or "Monthly Membership".
3. **Follow-up:** 2 weeks later, Leo gets a notification: "Leo, Sarah hasn't booked her guitar lesson. I've drafted a follow-up with her favorite Tuesday 4 PM slot. Approve?"

## Implementation Prompt
Create a "Service & Subscription" module. Allow products to be defined as "Bookable Services" with availability slots. Implement a subscription billing layer using Stripe Billing. Connect "The Ambassador" agent to the booking data to generate proactive re-engagement drafts for customers who have stopped booking.

## Priority
P0

## Estimated Scope
Large
