## [Shipping] Issue Brief: Real-Time Shipping & Label Generation

**Title**: Scout 🔍: EasyPost Integration for Shipping Labels & Tracking
**Problem Statement**: Businesses selling physical goods spend too much time manually copying addresses into carrier websites to generate shipping labels. They need a simple, one-click way to buy labels and auto-send tracking information to customers.
**Research Report**:
- **Tools Evaluated**: EasyPost, Shippo.
- **Evaluation**: EasyPost offers a unified API for over 100 carriers (USPS, FedEx, UPS). It simplifies the complexities of carrier APIs into a single integration.
- **Ease of Use**: The user simply clicks "Buy Label" on an order page in OHC. The PDF label is generated instantly.
- **Pricing**: EasyPost has a developer tier (free for low volume) and charges per label thereafter.
- **Cloud vs. Standalone**: EasyPost API is RESTful and works well in both modes. Cloud can centralize billing, while Standalone might require the user's own EasyPost API key.
**Design Doc**:
- During checkout, OHC calls EasyPost to fetch real-time shipping rates based on cart dimensions and destination.
- In the order management view, the user clicks "Fulfill" and purchases a shipping label via the EasyPost API.
- OHC stores the generated PDF label and the tracking number.
- Tracking webhooks from EasyPost automatically update the order status and notify the customer.
**Implementation Prompt**: Integrate the EasyPost API. Add real-time rate calculation to the checkout flow. Create a UI in the order dashboard to purchase and print shipping labels. Handle tracking webhooks to automatically email customers when their package ships.
**Priority**: P1
**Estimated Scope**: Medium
