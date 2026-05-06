# OHC Issue Brief: Native Hybrid Booking & Commerce Engine

## Title
Native Hybrid Booking & Commerce Engine

## Problem Statement
Service-based business owners like Carlos (handyman) and Leo (music tutor) don't fit neatly into traditional "e-commerce" or "booking" platforms. They often need to sell physical items (like sheet music) AND book time slots, or they need to quote a job and schedule the service simultaneously. Platforms force them to choose: use Shopify and duct-tape a clunky booking app to it, or use Calendly and struggle to collect complex payments.

## Research Report
*   **Finding:** Many local service businesses remain offline (word-of-mouth only) because no software intuitively maps to their workflow of Quote -> Book -> Pay -> Service.
*   **Competitor Gap:** Squarespace Acuity is powerful but disconnected from their main commerce engine. Shopify's booking apps are often poorly integrated and confusing for customers.
*   **Source:** Competitive feature analysis, YouTube tutorials highlighting complex workarounds for booking on Shopify.

## Design Doc
*   **High-Level Concept:** A unified entity model where "Time" and "Products" are first-class citizens in the same cart.
*   **UI/UX:**
    *   Mobile-first service menu.
    *   Unified checkout experience: A customer can add a "1-hour plumbing consultation" (time) and a "replacement filter" (product) to the same cart and check out once.
*   **AI Agent Integration:**
    *   **Smart Scheduling Assistant:** AI automatically manages calendar buffers, suggests optimal routing for mobile service providers (like Carlos), and auto-sends SMS reminders to reduce no-shows.

## Implementation Prompt
**Critical User Journey:**
1.  Carlos sets up his OHC profile as a Handyman.
2.  He creates a service: "Initial Consultation / Quote" (Booking) and adds common replacement parts (Products) to his catalog.
3.  A customer visits his OHC-powered mobile site, selects a time slot on Tuesday at 2 PM, and adds a note: "Need help fixing a leaky sink."
4.  The OHC system blocks the time on Carlos's calendar and sends a confirmation.
5.  After the job, Carlos opens the OHC app, taps the appointment, adds "1x PVC Pipe" from his product catalog to the final bill, and hits "Send Invoice."

**Acceptance Criteria:**
*   A business owner can define offerings that are purely time-based, purely product-based, or a hybrid.
*   The storefront allows customers to add both services (with specific dates/times) and physical products to a single unified shopping cart.
*   The backend system successfully handles calendar blocking and inventory deduction simultaneously upon checkout.
*   The UI must prioritize simplicity, hiding complex configuration options under an "Advanced Mode" toggle (Progressive Disclosure).

## Priority
P1

## Estimated Scope
Medium
