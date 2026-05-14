# [Logistics] OHC Tool Integration Research Brief: Shippo Deep Dive

## Title
Frictionless Shipping Operations with Shippo

## Problem Statement
While our general Shipping & Logistics brief identified the need for automated rate calculation and label generation, a deeper look into the workflow reveals specific integration patterns required for a seamless experience within OHC. Business owners shouldn't have to understand shipping complexities; they just want to print a label.

## Research Report
The core value proposition of shipping integrators is abstracting the complexity of dozens of carriers into a single, unified workflow.

**Key Concepts:**
*   **Addresses:** Sender and recipient locations. Must be validated.
*   **Parcels:** Dimensions and weight of the box.
*   **Shipments:** Combines Addresses and a Parcel to generate Rates.
*   **Rates:** Available shipping options and prices from carriers.
*   **Transactions:** The actual purchase of a Rate, generating a shipping label.

**Integration Challenges:**
*   **Rate Shopping vs. Immediate Purchase:** Should OHC show rates to the buyer during checkout, or just charge a flat fee and let the business owner purchase the best label later? Ideally, both.
*   **Address Validation:** Invalid addresses lead to failed shipments. Address validation must be surfaced in the OHC UI.

## Design Doc
**Integration Approach: End-to-End Shipping Flow**

1.  **Address Validation (Pre-shipment):**
    *   When a customer enters an address during checkout in OHC, OHC calls an external address validation service.
    *   If invalid, OHC prompts the user to correct it.

2.  **Rate Fetching (Checkout):**
    *   OHC creates a shipment intent asynchronously during checkout to fetch live rates.
    *   These rates are displayed to the customer to select their shipping speed.

3.  **Label Generation (Fulfillment):**
    *   When the business owner fulfills the order, OHC purchases the selected rate.
    *   The external service returns a label URL. OHC stores this URL and provides a print button in the order dashboard.

## Implementation Prompt
**Objective:** Implement the core integration workflow: Address Validation and Transaction Creation.

**Acceptance Criteria:**
1.  Add an address validation service and integrate it into the checkout flow.
2.  Implement a label generation service that takes an OHC Order, constructs a shipment intent, selects the appropriate rate, and finalizes the transaction.
3.  Store the resulting tracking number and label URL in the OHC `Order` database model.

## Priority
P1

## Estimated Scope
Large
