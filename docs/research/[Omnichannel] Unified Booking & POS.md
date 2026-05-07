# [Omnichannel] Unified Booking & POS

## Problem Statement
Service providers like Carlos (handyman) and Leo (music tutor) struggle with fragmented tools. They use one app for their website, another for booking, and a third (like Square) for taking payments in person. This causes double-booking and accounting headaches.

## Research Report
- **Competitor Audit:** Square is strong in POS and booking but weak in website generation. Shopify is terrible for service booking. Wix is okay but disjointed.
- **Pain Point:** "I forgot to block off my calendar when a walk-in client showed up, and now I'm double-booked."
- **Opportunity:** A natively unified system where the calendar, the website, and the physical POS are a single source of truth.

## Design Doc
- **Architecture:** A unified entity model where a 'Service' acts like a 'Product' but has temporal constraints. The booking engine and the checkout engine must write to the same ledger.
- **UX Flow:** The mobile app has a "Take Payment" tab that instantly checks out a booked appointment or allows ad-hoc charges.
- **AI Integration:** The AI agent can look at the unified calendar and automatically text waitlisted clients if a cancellation occurs.

## Implementation Prompt
**Outcome:** A seamless system where an online booking instantly updates the in-person availability, and an in-person POS transaction can be tied directly to a client's historical booking record.
**Critical User Journey:** Client books online -> Owner sees booking in OHC app -> Client arrives -> Owner taps "Checkout" on the booking to process payment via OHC POS.
**Acceptance Criteria:**
- The booking calendar must sync in real-time with checkout actions.
- The UI must clearly differentiate between products and services, yet allow both in a single cart if needed.

## Priority
P1

## Estimated Scope
Large
