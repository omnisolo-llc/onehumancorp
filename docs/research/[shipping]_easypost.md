# [Shipping] EasyPost Integration

**Title**: Integrate EasyPost for Streamlined Label Generation & Tracking

**Problem Statement**: Retailers selling physical goods struggle with calculating shipping rates and generating labels. They need an automated way to get shipping rates and buy labels directly from their OHC dashboard.

**Research Report**:
- **Tool**: EasyPost
- **Target Persona**: Retail business owners shipping physical goods.
- **Advantages**: Unified API for 100+ carriers (USPS, FedEx, UPS, international). High reliability.
- **Risks**: Requires handling complex shipping logic (box sizes, weights).
- **Pricing**: Free tier up to 120,000 shipments/year.
- **Compatibility**: Cloud. Standalone.

**Design Doc**:
- Users enter product weights and dimensions in their inventory.
- During checkout, EasyPost API is called to present real-time shipping rates.
- Upon order fulfillment, the user clicks "Generate Label", which uses EasyPost to purchase and print the label.
- Tracking numbers are automatically synced and emailed to the customer.

**Implementation Prompt**: Integrate EasyPost API to fetch real-time shipping rates during checkout and allow business owners to generate and print shipping labels from the order management dashboard.

**Priority**: P1

**Estimated Scope**: Large
