## [Shipping & Logistics] Issue Brief: Painless Shipping Labels & Tracking

**Title**: Scout 🔍: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**:
Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number natively within OHC without relying on complex external logistics aggregators that break the Radical Simplicity rule.

**Research Report**:
- **Tool**: EasyPost API.
- **Evaluation**: EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL). It abstracts away complex carrier-specific APIs and handles tracking webhooks out of the box.
- **Ease of Use**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC.
- **Advantages**: Great fit for OHC physical product merchants. Consolidates multiple carrier integrations into one.
- **Risks**: Physical label printing can be finicky depending on user's printer setup (e.g., thermal printers).
- **Pricing**: Competitive pricing (free tier for low volume, pennies per label after).
- **Compatibility**: Works well in Cloud. Standalone users would need their own EasyPost API key.

**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view natively in OHC, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.

**Implementation Prompt**:
Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
- **Acceptance Criteria**: Live shipping rates appear at checkout if configured. Merchant can click "Print Label" to generate a PDF label natively. Tracking number is automatically sent to the customer upon shipment.
**Priority**: P1
**Estimated Scope**: Medium
