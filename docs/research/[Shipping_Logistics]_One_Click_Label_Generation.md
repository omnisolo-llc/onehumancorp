## [Shipping & Logistics] One-Click Label Generation
**Title**: Integrate Shippo for Multi-Carrier Shipping Labels

**Problem Statement**: E-commerce sellers waste hours copying addresses from orders into carrier websites (USPS, UPS, FedEx, local post) to buy and print shipping labels. They need a way to generate labels instantly when an order comes in.

**Research Report**:
- **Persona Context**: Boutique owners, crafters, and independent e-commerce sellers shipping physical goods.
- **Solution Evaluated**: Shippo and EasyPost. Both aggregate carriers. Shippo has a slightly more forgiving onboarding for small businesses and excellent international coverage.
- **Ease of Use**: Drastically simplifies the shipping process. Instead of 10 clicks across different tabs, it's 2 clicks in one place.
- **Advantages**: Real-time rate comparisons across carriers, discounted rates, automatic tracking updates.
- **Risks**: Label printing requires exact dimensions and weights; user error here can lead to under-postage penalties.
- **Pricing Estimate**: Shippo is free to install, $0.05 per label + carrier postage costs.
- **Cloud/Standalone Support**: Cloud API integration; perfectly suitable for Standalone via user-provided API tokens.

**Design Doc**:
- **Triggers**: An order is marked as "Ready to Ship".
- **Actions**: OHC fetches shipping rates from Shippo, purchases the label upon user confirmation, and retrieves the PDF label and tracking number.
- **User Interface**: An "Orders" view. Clicking on an order shows a "Buy Shipping Label" button. The user enters package weight/dimensions, selects the cheapest carrier rate, and clicks "Print Label". The tracking number is automatically saved.

**Implementation Prompt**:
Build a shipping label generation feature. Users should see pending orders, input package dimensions, compare rates across carriers, and purchase a label. Provide a way to view and print the generated PDF label, and store the tracking number with the order details.

**Priority**: P2
**Estimated Scope**: Medium
