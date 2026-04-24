# Shippo API Integration

**Title**: Implement Automated Shipping Rates and Labels via Shippo
**Problem Statement**: Businesses selling physical goods need to calculate accurate shipping rates at checkout and easily print shipping labels. Doing this manually per order is a massive time sink.
**Research Report**:
- **Tool**: Shippo API.
- **Ease of Use (End User)**: Highly automated. Rates appear automatically at checkout. The owner clicks "Print Label" in the OHC dashboard.
- **Pricing**: Pay-as-you-go ($0.05 per label) or monthly subscription for volume. Often provides discounted carrier rates.
- **Cloud vs. Standalone**: Cloud API. Works in both modes, though Standalone requires internet access to fetch rates and generate labels.
**Design Doc**:
- **Trigger**: Customer reaches checkout (fetches rates). Business owner clicks "Fulfill Order" (generates label).
- **Action**: OHC queries Shippo for rates based on package weight/dimensions and destination. Upon fulfillment, OHC purchases the label via Shippo and provides a printable PDF link.
- **UI**: "Shipping" settings to define box sizes. Real-time rate display at checkout. "Generate Label" button on the order detail page.
**Implementation Prompt**: Integrate the Shippo API to provide real-time shipping rates at checkout based on product weight and customer address. Add functionality in the order management UI for the business owner to purchase and download shipping labels for fulfilled orders.
**Priority**: P1
**Estimated Scope**: Medium
