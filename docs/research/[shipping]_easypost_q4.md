# [Shipping] Automated Shipping with EasyPost

**Title**: Implement EasyPost for Seamless Label Generation

**Problem Statement**:
E-commerce small businesses spend hours manually calculating shipping rates and copy-pasting addresses into different carrier websites to print labels. They need a single button to buy and print a shipping label directly from an order.

**Research Report**:
- **Evaluated Tools**: EasyPost, Shippo, ShipStation.
- **Findings**: EasyPost offers a highly robust developer API that supports dozens of carriers globally (USPS, FedEx, UPS, DHL, etc.) and is perfect for building a white-labeled shipping experience directly inside OHC. Shippo is also good but EasyPost's reliability and API design are superior for this use case. ShipStation is more of a standalone product rather than an API-first tool.
- **Ease of Use**: By integrating the EasyPost API, the complexity of dealing with individual carriers is completely hidden from the business owner.
- **Pricing**: EasyPost is free for up to 120,000 shipments per year, which covers almost all small businesses, charging only the actual postage costs.
- **Cloud vs Standalone**: Perfect for both. OHC can act as the intermediary or the user can provide their own EasyPost API key in Standalone mode.

**Design Doc**:
- **Trigger**: The user views an "Order" in OHC and clicks "Create Shipping Label".
- **Action**: OHC sends the package dimensions, weight, and destination address to the EasyPost API to fetch rates.
- **User View**: A simple modal appears showing shipping options (e.g., "USPS Priority - $8.50"). The user clicks "Buy Label", and a PDF of the shipping label is immediately presented for printing. Tracking information is automatically saved to the order.

**Implementation Prompt**:
Integrate a shipping API (like EasyPost) to allow users to generate shipping labels directly from an order page. The interface must allow the user to enter package weight/dimensions, select a shipping rate from available carriers, purchase the label, and download the resulting PDF. Provide a tracking link that is saved with the order details.

**Priority**: P2
**Estimated Scope**: Large
