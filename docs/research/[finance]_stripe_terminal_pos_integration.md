# [finance] Stripe Terminal POS Integration

## Title
Integrate Stripe Terminal for In-Person Payments via Tap-to-Pay

## Problem Statement
Omnichannel merchants (e.g., Priya the Boutique Owner, Fatima the Food Cart Operator) need to accept in-person payments seamlessly alongside online orders. Requiring a separate POS system breaks inventory sync and reporting.

## Research Report
*   **Competitor Analysis**: Square dominates this space. Shopify has a strong POS offering but requires their specific hardware or higher tier plans.
*   **User Need**: The ability to accept contactless payments directly on the merchant's mobile device (Tap to Pay on iPhone/Android) or via a connected Stripe card reader without leaving the OHC app.

## Design Doc
*   **Architecture**:
    *   Use Stripe Terminal SDK within the Flutter app.
    *   Backend endpoints to create `ConnectionToken` and capture `PaymentIntent`.
*   **UI Wireframes**:
    *   "Charge" button on the mobile dashboard.
    *   Keypad to enter amount or select items from cart.
    *   "Tap to Pay" modal invoking native OS contactless reader.

## Implementation Prompt
Integrate the Stripe Terminal SDK into the Flutter app to support Tap to Pay on mobile devices. Build the necessary backend endpoints to issue connection tokens and process the captured payments. Ensure the transaction is recorded in the OHC unified ledger and inventory is decremented if specific items were selected.

## Priority
P1

## Estimated Scope
Medium
