# Integrate Shippo for Streamlined Shipping & Logistics

## Problem Statement
For small businesses selling physical goods, managing shipping is a major headache. Calculating accurate shipping rates, purchasing labels from different carriers (USPS, UPS, FedEx), and providing tracking numbers to customers requires using multiple disconnected systems. Business owners need a centralized way to handle order fulfillment directly within OHC.

## Research Report
**Findings & Data**: Shippo is an established American e-commerce software company that provides a multi-carrier shipping API.
**Ease of Use**: Shippo aggregates multiple carriers into a single interface/API. For the business owner, it means they only need one Shippo account to access rates and labels from various providers.
**Features**: Real-time rate calculation, discounted label purchasing, and automated tracking updates.
**Pricing**: Offers a pay-as-you-go model with no monthly fees (just a small per-label fee plus the cost of postage), which is perfect for small businesses with variable shipping volumes.
**Reputation**: Highly reliable API with broad carrier coverage globally.

## Design Doc
**Integration flow**:
1.  **Connection**: The user connects their Shippo account via OAuth or API key in OHC settings.
2.  **Order Fulfillment**: When an order is placed or marked ready for shipping in OHC, the user clicks "Create Shipping Label".
3.  **Rate Selection**: OHC fetches real-time rates from Shippo based on package dimensions and destination. The user selects a rate.
4.  **Label Generation**: OHC requests the label from Shippo. The user can download/print the PDF label directly from the OHC interface.
5.  **Tracking**: OHC saves the tracking number provided by Shippo and can automatically email it to the customer.

## Implementation Prompt
**User-Facing Outcome**: When viewing an order in OHC, the business owner can compare shipping rates from multiple carriers, purchase a shipping label, and print it—all without leaving the platform. The customer automatically receives the tracking number.
**Acceptance Criteria**:
- UI to connect Shippo account.
- Integration to fetch and display shipping rates for a given order address and package size.
- Integration to purchase and download the shipping label (PDF) via Shippo API.
- Automatic extraction and storage of the tracking number.
- Functions correctly in both Cloud and Standalone environments.

## Priority
P2

## Estimated Scope
Large
