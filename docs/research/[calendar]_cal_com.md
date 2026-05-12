# Issue Brief: Cal.com Booking Infrastructure

## Title
Implement Cal.com Booking Infrastructure for Small Business Owners

## Problem Statement
Setting up a booking page that handles different timezones for international clients, sends reminders, and prevents double-booking is too complicated for a user to configure manually.

## Research Report
Cal.com provides the underlying logic for advanced scheduling pages.

**Persona Impact:** A consultant can send a single link. The client clicks it, sees the times localized to their own timezone, and books instantly.

**Advantages:** Provides a world-class, high-converting booking experience out of the box.

**Risks:** Relying on a third-party for the core booking experience means we must ensure their design matches the OHC aesthetic seamlessly.

**Pricing Estimate:** Generous free tier for individuals. Highly SMB friendly.

**Environment:** Works well in Cloud mode and Standalone mode.

## Design Doc
1.  **Service Setup:** User defines a service ('1 Hour Consultation') and sets a price.
2.  **Public Widget:** OHC generates a beautiful, shareable public page that handles the entire booking flow.

## Implementation Prompt
Integrate the Cal.com engine to power the OHC public storefront's booking widget, providing a flawless scheduling experience for the end-customer.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
The Cal.com integration must strictly enforce OHC's payment gateway preferences. If the business owner has required a 50% deposit for a booking, the Cal.com widget must utilize the OHC Stripe/Mercado Pago connection to collect that deposit before finalizing the calendar slot.
