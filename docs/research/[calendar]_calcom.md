# Calendar & Scheduling: Booking Pages via Cal.com

## Title
Enable Custom Booking Pages for Services

## Problem Statement
Service-based small business owners spend too much time emailing clients to find a time to meet or provide a service. They need a simple, professional link they can send clients to book available times automatically.

## Research Report
- **Tool Evaluated:** Cal.com
- **Ease of Use:** Very High. Interface is modern and intuitive.
- **Pricing:** Open source. Free for individuals, $12/mo/user for teams.
- **Reputation:** The leading open-source Calendly alternative. High developer velocity.
- **Cloud/Standalone Compatibility:** Excellent. Designed to be self-hosted or consumed via their SaaS API.

## Design Doc
- **Integration Point:** A new "Scheduling" section under Services/Products.
- **User Experience:** The business owner connects their Google/Outlook calendar. They define their working hours and service durations. OHC provides a branded booking link (e.g., `ohc.page/bobs-plumbing/consultation`). When a client books, it automatically appears on the owner's calendar.
- **System Behavior:** OHC wraps Cal.com's API/embeds, mapping OHC users to Cal.com sub-accounts automatically.

## Implementation Prompt
Implement a "Booking Link" generator for business owners. Create a simple setup wizard to connect a calendar, set working hours, and define one or more bookable services. The final output must be a clean, shareable URL. The client-facing booking page must follow OHC design guidelines and be mobile-optimized.

## Priority
P1

## Estimated Scope
Medium
