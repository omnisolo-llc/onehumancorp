# Issue Brief: Native Booking & Deposit System (P0)

## Problem Statement
Service-based solopreneurs (like Carlos the handyman or Leo the tutor) struggle to manage their businesses online because major platforms like Shopify are built fundamentally for physical e-commerce (shipping boxes). Currently, these users must stitch together separate tools (e.g., a Wix website + Calendly + separate Stripe invoices) to simply book an appointment and take a deposit. This manual process leads to lost leads, double-bookings, and administrative exhaustion.

## Research Report
*   **Target Persona:** Service providers (Carlos/Leo).
*   **Pain Point Validation:** Reddit (`r/smallbusiness`) is filled with complaints about the cost and complexity of connecting booking software to website builders.
*   **Competitor Analysis:** Shopify requires paid third-party apps for booking. Wix has a native booking tool, but it's complex to set up.
*   **Opportunity:** OHC can capture this market by offering a zero-configuration booking flow that is native to the platform, integrated seamlessly with the AI agent ecosystem (for automatic reminders and follow-ups).

## Design Doc
*   **High-Level Architecture:**
    *   Introduce `Service` as a first-class product entity alongside physical goods.
    *   Add a `Booking` entity linked to a `Service`, a `Customer`, and a specific time slot.
    *   Integrate a generic `CalendarSlot` availability system.
*   **UI/UX Flow (Mobile-First):**
    *   **Customer View:** Browse services -> Select date/time from available slots -> Checkout with deposit -> Receive confirmation.
    *   **Owner View:** Dashboard showing upcoming bookings -> Ability to accept/reschedule -> Tap to charge final balance via saved payment method.
*   **AI Integration:** "The Manager" agent automatically updates availability and sends SMS reminders to the customer 24 hours before the appointment.

## Implementation Prompt
Implement a full-stack booking system where a user can define a "Service" (e.g., "1 Hour Plumbing Fix"), set a price and required deposit, and define their weekly availability. The customer-facing UI must allow selecting an available time slot and paying the deposit. The backend must enforce double-booking prevention using the existing database architecture. The entire flow must be fully functional and testable on a 375px mobile layout.

*   **Priority:** P0
*   **Estimated Scope:** Large
