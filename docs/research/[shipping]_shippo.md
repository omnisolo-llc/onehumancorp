## [Shipping & Logistics] Issue Brief: Automated Label Generation

**Title**: Scout 🔍: Integrate Shippo for 1-Click Shipping Labels
**Problem Statement**:
Sellers of physical goods (like Maya shipping cookies or Priya shipping clothes) spend too much time calculating postage, going to the post office, and manually entering tracking numbers. They need to print labels directly from their phone.
**Research Report**:
- **Tool**: Shippo API (or EasyPost).
- **Evaluation**: Shippo aggregates multiple carriers (USPS, UPS, FedEx) and provides discounted rates without the user needing to negotiate their own carrier accounts.
- **Ease of Use**: Very easy. On the order details screen, the user clicks "Buy Label", confirms the box weight, and gets a PDF label to print.
- **Pricing**: 5¢ per label + the actual cost of postage.
- **Cloud vs. Standalone**: Cloud-friendly using a master Shippo account with sub-accounts. Standalone would require the user to create their own Shippo account and provide an API key.
**Design Doc**:
- In the "Orders" view, add a "Fulfill & Ship" flow.
- Call Shippo API to get live rates based on the customer's shipping address.
- Business owner selects a rate and purchases the label.
- OHC automatically emails the tracking number to the customer.
**Implementation Prompt**:
Integrate the Shippo API to allow users to purchase and generate shipping labels directly from an order page. The flow should fetch rates, capture payment for the label (either via OHC billing or direct), generate the PDF label, and automatically update the order status with the tracking URL.
**Priority**: P1
**Estimated Scope**: Medium
