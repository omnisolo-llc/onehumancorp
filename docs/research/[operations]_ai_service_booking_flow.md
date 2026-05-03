# Issue Brief: AI Service Booking Flow

## Problem Statement
Service providers like Leo (Tutor) and Carlos (Handyman) struggle with the back-and-forth of scheduling and quoting. Existing tools (like Calendly) don't integrate seamlessly with quoting and deposit collection, forcing owners to stitch tools together.

## Research Report
-   **Finding:** Scheduling friction is the #1 reason service businesses lose online leads.
-   **Competitor Comparison:** Squarespace Acuity is powerful but separated from the core commerce flow. Shopify requires complex apps for service booking.
-   **Source:** E-commerce churn analysis and SMB forum sentiment.

## Design Doc
-   **Entities:** `Service`, `Availability`, `Booking`, `Quote`.
-   **UX Flow:**
    1. Customer visits Carlos's OHC site and describes their problem ("Leaky pipe under sink").
    2. Sales AI generates an estimated quote and provides available time slots.
    3. Customer selects slot, pays deposit.
    4. Carlos receives a single notification: "Booking Confirmed: Leaky Pipe, Deposit Paid."
-   **Mobile UX (375px):** Simple, large calendar date picker. Native numeric keypad for deposit amounts.

## Implementation Prompt
Create an integrated service booking flow where the AI Sales Agent parses natural language service requests to propose an estimated quote and presents an availability calendar. The flow must include taking a deposit via Stripe. The owner dashboard should clearly display upcoming bookings on a mobile-friendly 375px view.

## Priority
P1

## Estimated Scope
Medium
