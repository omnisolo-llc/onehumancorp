# [Calendar & Scheduling] Integrate Cal.com for Seamless Booking

## Problem Statement
Service-based businesses (like freelance handymen or music tutors) lose clients due to the friction of back-and-forth emails to find a meeting time. They need a simple, professional way for customers to view their availability and book appointments directly, automatically syncing with the owner's personal calendar to avoid double-booking.

## Research Report
**Tool Analyzed:** Cal.com (Open Source Scheduling Infrastructure)

*   **Capabilities:** Comprehensive scheduling, calendar syncing (Google, Outlook, Apple), automated reminders, routing, and integrated video conferencing links.
*   **Ease of Use (for Non-Technical Users):** Extremely easy. The business owner shares a simple link or embeds a widget. The customer sees a calendar and picks a time.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Robust API and webhooks for seamless SaaS integration.
    *   *Standalone:* Fully open-source and Docker-ready for self-hosting in local environments.
*   **Pricing:** Free for individuals. Team plans start at $12/user/month. API/Platform pricing available for white-labeling.
*   **Reputation:** Highly respected developer-friendly alternative to Calendly. Open core model aligns perfectly with OHC's architecture.

## Design Doc
**Integration with OHC:**
*   **Trigger:** User enables the "Booking" feature on their OHC public profile or service listing.
*   **Action:** OHC provisions a Cal.com event type via API behind the scenes. The business owner connects their Google/Outlook calendar via an OHC-managed OAuth flow.
*   **User Interface:** Customers see an embedded booking widget on the OHC storefront. When they book, the business owner receives a notification in the OHC app, and the event appears on their connected calendar.
*   **AI Agent Synergy:** "The Manager" (Operations) tracks upcoming bookings and sends automated SMS/email reminders. "The Salesperson" can auto-send booking links to leads.

## Implementation Prompt
Implement a booking and scheduling system using Cal.com as the backend engine.
1.  Add a UI flow for business owners to connect their existing calendar (Google/Outlook).
2.  Allow owners to define "Service" items with specific durations (e.g., "1 Hour Plumbing Fix").
3.  Display a date/time picker on the customer-facing storefront for these services, reflecting the owner's actual real-time availability.
4.  Upon booking, automatically generate calendar invites for both the customer and the owner.

## Priority
P0 (Critical) - Core requirement for all service-based personas.

## Estimated Scope
Large
