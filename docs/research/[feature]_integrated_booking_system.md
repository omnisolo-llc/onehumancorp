# [Feature] Integrated Booking System

## Problem Statement
Service-based solopreneurs (like Leo, music tutor, and Carlos, handyman) struggle with manual booking chaos. Missing leads when busy and double-booking are major pain points. Traditional platforms treat booking as an afterthought or require third-party add-ons.

## Research Report
- **Finding:** Constant context-switching between website builders and calendar tools creates friction and lost sales.
- **Source:** Trustpilot reviews for platforms lacking native booking show significant user frustration.
- **Comparison:** Shopify and Wix require add-ons or separate apps for robust booking. OHC must offer an integrated solution.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `Service`, `TimeSlot`, `Booking`.
  - **Key Relationships:** `Booking` linked to `Tenant` and `Customer`.
  - **Integration Points:** Syncs with external calendars (Google, Outlook).
- **UI Wireframes/Screen Flow:**
  - **Mobile UX (375px first):**
    1. Calendar view showing upcoming bookings.
    2. Tap slot -> View booking details.
    3. New booking -> Simple form for customer info and service selection.
- **AI Agent Integration:** Proactive reminders for upcoming bookings and automated follow-ups for feedback.

## Implementation Prompt
**User-Facing Outcome:** Business owners can easily manage their availability and receive bookings directly through their OHC storefront without relying on external tools.
**Critical User Journey:**
1. Owner sets availability.
2. Customer selects a service and time slot.
3. Owner receives a booking confirmation.
**Acceptance Criteria:**
- Must sync with external calendars.
- Booking flow must be mobile-first and intuitive.
- Notifications sent for new bookings and reminders.

## Priority
P1

## Estimated Scope
Large
