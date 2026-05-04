# Issue Brief: Native Service Booking System

## Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) struggle with disjointed tools. They currently use combinations of Calendly, Venmo, text messages, and Google Calendar. They need a simple booking system where customers can view a service, pick a date/time, pay a deposit, and auto-sync with their native calendar.

## Research Report
- **Competitor Gap:** Shopify lacks native service bookings (requires complex app plugins). Wix has a native booking module, but it's desktop-heavy and overly complex for a 1-person operation.
- **Pain Point Alignment:** Addresses "Operational Fatigue" (Rank #2).
- **Opportunity:** Provide a zero-jargon booking tool built strictly for the 375px mobile screen that handles the flow: Discovery -> Select Time -> Deposit -> Confirmation.

## Design Doc
### High-Level Architecture
- **Booking Entity:** Related to the `Tenant` and `Service` (a subset of Product). Includes start/end times, customer info, and payment status.
- **UI Flow (Mobile First):** A tap-friendly calendar interface where the user sets their availability rules (e.g., "Monday-Friday 9am-5pm"). Customers see available slots based on the service duration and buffer times.
- **AI Integration (The Operations Agent):** Automatically block time for travel (if applicable) and trigger the "Ambassador" to send reminder SMS/emails 24h prior.

### Implementation Prompt
Implement a simple native booking flow for service-based businesses. Allow the business owner to define a service with a duration and price. Provide a mobile-optimized calendar view for customers to select a time slot and pay an initial deposit via Stripe. The Operations Agent should monitor bookings to prevent double-booking.

## Priority
P1

## Estimated Scope
Medium
