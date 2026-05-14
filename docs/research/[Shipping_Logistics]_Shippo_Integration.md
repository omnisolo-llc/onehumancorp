## 5. Shipping & Logistics
**Title**: Integrate Shippo for Multi-Carrier Label Generation
**Problem Statement**: E-commerce micro-businesses waste hours standing in line at the post office and guessing shipping costs. They need to print discounted shipping labels from home automatically when an order is placed.
**Research Report**:
- **Tool**: Shippo API
- **Problem it solves for which persona**: Helps independent makers, crafters, and boutique shops fulfill physical orders quickly from their home or small warehouse.
- **Ease of Use**: Owner connects Shippo, enters box dimensions, and clicks "Print Label".
- **Pricing**: Pay-as-you-go ($0.05 per label) or $10/mo for the pro tier. Excellent USPS/UPS discounts.
- **Key Advantages**: Aggregates many carriers (USPS, UPS, FedEx, DHL) behind one clean API.
- **Integration Risks**: Handling edge cases like rural addresses, customs forms for international shipping.
- **Environment**: Cloud and Standalone supported.
**Design Doc**:
- **Trigger**: Owner clicks "Fulfill Order" in OHC.
- **Action**: OHC fetches rates from Shippo, creates a transaction, and downloads the PDF label.
- **User Interface**: Order details page has a "Buy Shipping Label" button, showing rates from different carriers.
**Implementation Prompt**: Build an integration with the Shippo API to fetch shipping rates for a given order, purchase a shipping label, and retrieve the PDF label and tracking number for the business owner to print.
**Priority**: P2
**Estimated Scope**: Medium
