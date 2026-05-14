# [Shipping] OHC Tool Integration Research Brief: Shipping & Logistics

## Title
Automated Shipping Rate Calculation and Label Generation

## Problem Statement
For small business owners selling physical goods, calculating accurate shipping rates at checkout and manually copying customer addresses into carrier websites (like USPS, UPS, or FedEx) to print labels is incredibly time-consuming and prone to errors. They need a way to automate rate calculation and generate shipping labels directly from their OHC dashboard.

## Research Report
The shipping API space is well-established, offering tools that aggregate multiple carriers into a single interface.

**Evaluated Tools:**

1. **Shippo (shippo.com)**
    *   **Focus:** Multi-carrier shipping API and web app for ecommerce.
    *   **Pros:** Excellent API, very easy to integrate. Offers discounted rates out of the box for major carriers. Great documentation.
    *   **Cons:** Can get expensive at high volumes if using their API directly without custom carrier accounts.
    *   **Pricing:** Free tier (pay $0.05 per label) or $10/mo for professional features.
    *   **Modes:** Cloud and Standalone.

2. **EasyPost**
    *   **Focus:** Developer-first shipping API.
    *   **Pros:** Extremely robust API, supports a massive number of carriers globally.
    *   **Cons:** Less focus on a standalone user-friendly dashboard compared to Shippo, requiring OHC to build more UI.
    *   **Pricing:** Developer tier (120,000 shipments free per year).
    *   **Modes:** Cloud and Standalone.

3. **ShipStation**
    *   **Focus:** All-in-one shipping software for ecommerce.
    *   **Pros:** Very popular, integrates with everything.
    *   **Cons:** Their API is older and can be clunky. Often used as a standalone app rather than embedded via API.
    *   **Pricing:** Starts at $9.99/mo.
    *   **Modes:** Cloud and Standalone.

**Recommendation:**
**Shippo** provides the best balance of a modern, developer-friendly API and features that small business owners need (like pre-negotiated discounted rates). It allows OHC to calculate rates during checkout and generate labels within the OHC order management interface smoothly.

## Design Doc
**Integration Approach: Shippo API Integration**

1.  **Setup:**
    *   Business owner enters their Shippo API key in OHC settings.
    *   They configure default package dimensions and weights for their common products.

2.  **Checkout (Rate Calculation):**
    *   During checkout, OHC calls the Shippo API with the cart contents and customer address to fetch real-time shipping rates.
    *   Customer selects their preferred shipping option.

3.  **Fulfillment (Label Generation):**
    *   In the OHC dashboard, the business owner views the order.
    *   They click "Generate Shipping Label".
    *   OHC calls the Shippo API to purchase the label using the pre-selected shipping method.
    *   OHC downloads the PDF label for the user to print and automatically updates the order status to "Shipped" with the tracking number.

## Implementation Prompt
**Objective:** Integrate Shippo to provide real-time shipping rates and label generation.

**Acceptance Criteria:**
1.  Implement a client to interact with the shipping API.
2.  Add a generic interface to support future alternatives.
3.  Update the checkout flow to dynamically fetch shipping rates based on the configured provider.
4.  Add a "Generate Label" action in the order management UI that calls the provider to purchase and retrieve a shipping label URL.

## Priority
P2

## Estimated Scope
Medium
