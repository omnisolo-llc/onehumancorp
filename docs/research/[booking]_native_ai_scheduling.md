# [Booking] Native AI Scheduling

## Problem Statement
Service-based solopreneurs (like Carlos the handyman or Leo the tutor) lose leads because they manage schedules via text and don't have a dedicated booking page. Existing tools (Calendly) are disjointed from their payment/website platform.

## Research Report
Service businesses are our 'Beachhead Market'. Shopify natively sucks at services (requires clunky apps). Wix has booking, but it's not conversational. The gap is a conversational AI that can negotiate times via SMS and finalize a booking. Source: Trustpilot reviews for Wix Bookings.

## Design Doc
- **High-level architecture:** A calendar entity system linked to services. An AI scheduling agent that can read available slots and hold tentative state until payment/confirmation.
- **UI Wireframes:** A 'Services & Calendar' tab. A public-facing booking widget. A conversational SMS interface for the client.
- **Mobile UX Flow (375px):** Owner sets available hours once. Client texts 'Need a plumber Tuesday'. AI replies 'I have 10 AM or 2 PM available. Which works?' -> Client says '10 AM' -> AI sends confirmation link.
- **AI Integration:** An agent with tool-calling capabilities to check availability and create appointments.

## Implementation Prompt
Build the core scheduling engine and the conversational booking agent. The CUJ is: Owner connects calendar -> Client interacts with AI via web chat or SMS -> AI successfully finds a slot and creates a tentative booking -> Owner receives notification.

## Priority
P1

## Estimated Scope
Large
