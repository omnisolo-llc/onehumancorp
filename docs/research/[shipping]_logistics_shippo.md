**Title**: Real-time Shipping Rates & Labels with Shippo
**Problem Statement**: E-commerce users spend too much time manually calculating shipping costs for physical goods and buying labels at the post office. This eats into their margins and slows down fulfillment. They need automatic rate calculations at checkout and one-click label generation.
**Research Report**:
- **Shippo**: An American e-commerce software company that provides a multi-carrier shipping API and web application.
- **Ease of Use**: Shippo aggregates dozens of carriers (USPS, UPS, FedEx, DHL, etc.) behind a single API. This saves the developer from integrating with each carrier individually. For the merchant, it provides a unified dashboard to compare rates and print labels.
- **Pricing**: Very SMB friendly. Often free to use the API (pay per label) with deeply discounted USPS/UPS rates compared to retail.
- **Reputation**: Highly reputable, used by many major e-commerce platforms.
- **Cloud/Standalone**: Fully API-driven. OHC Cloud can easily make backend calls to Shippo. Standalone users would just need to provide their Shippo API key.
**Design Doc**:
- **Trigger**: A customer enters their address during checkout on an OHC storefront.
- **Action**: OHC queries the Shippo API with the cart weight/dimensions and destination address to fetch real-time shipping options.
- **UI**: During checkout, the customer sees accurate shipping tiers (e.g., Standard vs. Express). In the OHC admin dashboard, under an "Orders" view, the merchant sees a "Buy Label" button. Clicking it generates a PDF label via Shippo.
**Implementation Prompt**: Integrate the Shippo API. In the checkout flow, dynamically fetch and display shipping rates based on the cart contents and customer address. In the order management view, allow the merchant to purchase and generate a printable shipping label for an order with one click.
**Priority**: P1
**Estimated Scope**: Medium
