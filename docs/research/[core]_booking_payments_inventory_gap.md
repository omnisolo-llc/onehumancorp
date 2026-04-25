# Issue Brief: Core Transacting Capabilities Gap (Bookings, Payments, Inventory)

## Problem Statement
Small business owners rely on transactions to survive. Our target personas, such as Carlos the Handyman and Priya the Boutique Owner, cannot use OHC if they cannot accept bookings or process payments. Currently, the OHC platform lacks the foundational infrastructure for core transacting capabilities, specifically Stripe payment integration, service booking flows, and inventory management. This gap prevents users from moving beyond a conceptual storefront to a functional business.

## Research Report
- **Competitor Baseline:** Shopify and Wix both offer robust built-in payment processing (Shopify Payments, Wix Payments) and inventory tracking out-of-the-box.
- **User Pain Points:** "I have a site for my portfolio but use a separate messy tool for bookings" is a frequent complaint among service-based businesses (e.g., Leo the Tutor).
- **Current State:** A codebase audit confirms that while UI elements exist for business setup (mentioning Stripe), the backend API and domain models for processing real transactions or tracking physical inventory limits are absent.

## Design Doc
### High-Level Architecture
- **Entity Types:** Needs new entities for `Product`, `InventoryItem`, `BookingSlot`, `Order`, and `PaymentIntent`.
- **Integration Points:** Deep integration with Stripe API (Checkout Sessions, Webhooks for status updates).
- **Relationships:** A `Booking` or `Order` must reserve an `InventoryItem` or `BookingSlot` and be linked to a `PaymentIntent`.

### Mobile UX Flow (375px First)
- **Product/Service Creation:** A simple, native-feeling form to define what is being sold, including quantity/availability limits.
- **Checkout Flow:** A frictionless, mobile-optimized checkout screen using Stripe Elements or Apple/Google Pay.
- **Dashboard:** A clear "Orders & Bookings" view showing upcoming commitments and low inventory warnings.

## Implementation Prompt
Implement the backend domain models, database schema updates (with RLS), and API endpoints to support Products, Inventory, Bookings, and Stripe Payments.
The system must allow a tenant to create a bookable service or physical product, track its availability, and process a payment via Stripe.
Create the corresponding Flutter frontend views to manage these entities and execute a checkout flow, ensuring perfect rendering on a 375px screen.
Ensure all critical paths are covered by E2E tests simulating a real user purchase.

## Priority
P0

## Estimated Scope
Large
