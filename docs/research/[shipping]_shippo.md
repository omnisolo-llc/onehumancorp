# Integration Issue Brief: Shipping & Logistics (Shippo)

## Title
Automated Shipping Label Generation & Tracking: Shippo

## Problem Statement
E-commerce small business owners spend hours manually copying addresses, comparing carrier rates, and printing labels. This manual process is error-prone and scales poorly. They need an automated way to get the best shipping rates and generate labels directly from their order dashboard.

## Research Report
*   **Tool Evaluated**: Shippo
*   **Ease of Use**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL, etc.) into a single, easy-to-use API and dashboard, simplifying rate comparison and label generation.
*   **Market Position & Reputation**: Highly respected in the shipping API space, known for providing deep discounts on USPS and UPS rates.
*   **Pricing**:
    *   **Starter (Free)**: Pay $0.05 per label (if using own carrier accounts) or free if using Shippo's discounted rates.
    *   **Pro**: Starts at $17/month for up to 200 labels, no per-label fees.
*   **Cloud vs. Standalone Compatibility**: API-based service. OHC Cloud and Standalone will securely communicate with Shippo's REST API.

## Design Doc
*   **Integration Trigger**: User connects their Shippo account (or OHC provisions a white-labeled Shippo account) via API key.
*   **Action Flow**:
    1.  When an order is marked "Ready to Ship" in OHC, OHC sends package dimensions, weight, and addresses to Shippo.
    2.  Shippo returns a list of rates across carriers.
    3.  User selects a rate; OHC purchases the label via API.
    4.  OHC downloads the PDF label for printing and stores the tracking number.
*   **User Experience**: The business owner views an order, clicks "Buy Shipping Label", sees the cheapest options, clicks "Purchase", and immediately gets a printable PDF and tracking link—all without leaving OHC.

## Implementation Prompt
Integrate Shippo to enable seamless shipping label generation within OHC's order management view. Build a UI flow that takes order details, queries the Shippo API for rates, and presents the options to the user. Upon selection, execute the transaction to purchase the label, display the printable PDF to the user, and automatically update the OHC order record with the tracking number and a "Shipped" status.

## Priority
P2

## Estimated Scope
Medium
