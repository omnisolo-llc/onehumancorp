# Real-time Shipping Labels and Rates

**Problem Statement**: Calculating shipping costs manually and copying addresses to carrier sites leads to errors and lost money.

**Research Report**: EasyPost or Shippo APIs aggregate carriers (USPS, UPS, FedEx, DHL). Essential for physical product sellers. Need to handle package dimensions and weights. Cloud mode simplifies API key management.

**Design Doc**: Order fulfillment screen. Input package weight/dims. Fetch rates via EasyPost. Generate and print label. Auto-update tracking.

**Implementation Prompt**: Add a feature to fetch shipping rates and generate printable labels for orders.

**Priority**: P1
**Estimated Scope**: Large
