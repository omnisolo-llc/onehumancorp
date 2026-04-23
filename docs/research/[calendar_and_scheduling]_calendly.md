# Calendar & Scheduling - Calendly

## Problem Statement
Service providers (tutors, handymen) struggle with back-and-forth emails to schedule appointments. They need a simple link where customers can book a time that automatically syncs with their personal calendar.

## Research Report
Calendly is the industry standard for scheduling.
- **Ease of Use**: Very intuitive for both the business owner and the customer booking the time.
- **Pricing**: Basic is free (1 event type). Essentials is $8/month.
- **Reputation**: Ubiquitous and reliable.
- **Cloud/Standalone**: Cloud-based SaaS.

## Design Doc
- **Trigger**: User creates a "Service" product in OHC.
- **Action**: OHC provisions a Calendly event type via API or uses Calendly's OAuth to connect the user's existing account.
- **User View**: OHC embeds the Calendly booking widget on the user's public storefront. The business owner manages availability through OHC, which syncs to Calendly.

## Implementation Prompt
Integrate Calendly for scheduling services. Allow users to connect their Calendly account or create a new one via OHC. Embed the Calendly booking flow on the public storefront for service-type products.
- Acceptance Criteria: User can link Calendly. Storefront shows available times. Booking an appointment updates the user's connected calendar.

## Priority
P0

## Estimated Scope
Small

---
