# [operations] Native Unified Booking System

## Title
Develop a Native, Zero-Config Booking and Scheduling System

## Problem Statement
Service-based businesses (e.g., Carlos the Handyman, Leo the Tutor) need to accept appointments and deposits simultaneously. Currently, they often string together Calendly, a separate payment link, and manual Google Calendar updates. This is confusing for both the owner and the customer.

## Research Report
*   **Competitor Analysis**: Squarespace offers Acuity Scheduling (an add-on). Wix has a built-in booking system but it can be clunky to set up on mobile.
*   **User Need**: A simple, unified booking flow where a customer selects a service, picks an available time slot, pays a deposit, and both parties receive calendar invites automatically.

## Design Doc
*   **Architecture**:
    *   Core `Booking` entity associated with `Service` and `Tenant`.
    *   Integration with Google Calendar API (OAuth) for sync.
    *   Payment intent generation via Stripe Checkout Session linked to the booking.
*   **Mobile UX Flow**:
    *   Owner adds a "Service" -> toggles "Requires Booking".
    *   Sets availability (e.g., Mon-Fri 9-5).
    *   Sets deposit amount.

## Implementation Prompt
Implement the backend core booking logic and the mobile-first frontend flow. The feature must allow a user to define a service with an associated duration and price, set their weekly availability, and provide a public booking link. The customer flow must show available slots, collect a deposit via Stripe, and confirm the booking. Include Google Calendar sync capability.

## Priority
P0

## Estimated Scope
Large
