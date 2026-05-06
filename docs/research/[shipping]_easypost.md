# Scout: Tool Integration Research Q2 - EasyPost

**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**: Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number.
**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).
- Competitive pricing (free tier for low volume, pennies per label after).
- Abstracts away complex carrier-specific APIs and handles tracking webhooks.
- Great fit for OHC physical product merchants.
- Works securely in both Cloud mode (via an overarching OHC key) and Standalone mode (users bring their own API keys).
**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
**Priority**: P1
**Estimated Scope**: Medium
