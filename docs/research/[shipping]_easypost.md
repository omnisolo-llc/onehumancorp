# Shipping & Logistics: EasyPost

**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking

**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).
- **Pricing**: Free tier available, nominal fee per label thereafter.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo and saves the tracking number.
- OHC automatically emails the customer the tracking number.

**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
- **Priority**: P1
- **Estimated Scope**: Large
- **Acceptance Criteria**:
  - Order fulfillment flow allows generating and printing labels.
  - Tracking numbers are saved and automatically emailed to customers.

**Strategy**: Use EasyPost's unified API to handle multiple shipping carriers natively.
