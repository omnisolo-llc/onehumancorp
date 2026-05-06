# [Shipping and Logistics] Shippo Integration

**Title**: Integrate Shippo for easy shipping rate calculation and label generation

**Problem Statement**: Small e-commerce sellers like Priya struggle with calculating accurate shipping rates, finding the cheapest carrier, and printing shipping labels. Manually entering addresses into different carrier websites takes too much time and risks errors, resulting in lost margins.

**Research Report**: Shippo is an e-commerce shipping software company that provides a multi-carrier shipping API.
- **Ease of use**: Very high. End-users do not interact directly with complex carrier APIs. Shippo provides a unified interface and API to access discounted rates.
- **Pricing**: Pay-as-you-go (per label) and subscription tiers available. Affordable for small businesses.
- **Reputation**: Reliable, integrates with many major platforms (eBay, Shopify, Stripe) and major carriers (USPS, UPS, FedEx).
- **Cloud/Standalone**: API driven. Requires internet access to fetch live rates and generate labels, so Standalone mode requires an active connection.

**Design Doc**:
- **Trigger**: An order is placed and the business owner marks it "Ready to Ship" in OHC.
- **Action**: OHC queries the Shippo API with the package weight/dimensions and destination to get live rates. Once the user selects a rate, OHC generates the label via Shippo.
- **User Experience**: The business owner sees a "Print Shipping Label" button on the order page. They select the cheapest rate from a dropdown and instantly get a PDF label to print. Tracking info is automatically saved.

**Implementation Prompt**: Integrate the Shippo API to provide a seamless shipping experience. Add a "Generate Label" workflow on the order details page that fetches live rates, purchases the selected label, and provides a printable PDF and tracking number.

**Priority**: P1
**Estimated Scope**: Medium