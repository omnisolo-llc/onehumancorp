**Title**: Shippo Integration for Shipping & Logistics

**Problem Statement**:
Small business owners who sell physical goods waste hours manually entering addresses into carrier websites to calculate rates and print labels. They need an integrated way to get shipping rates, generate labels, and track packages directly from their OHC orders dashboard.

**Research Report**:
- **Tool**: Shippo (Shipping API).
- **Ease of Use**: High for end-users. Once configured, they just click "Print Label" on an order.
- **Pricing**: Pay-as-you-go (5 cents per label) or $19/month for the professional tier without per-label fees. Excellent carrier discounts.
- **Reputation**: Very reliable API, strong coverage of global carriers (USPS, UPS, FedEx, DHL, etc.).
- **Compatibility**: Compatible with both Cloud and Standalone modes via API keys.

**Design Doc**:
- **Trigger**: Business owner clicks "Fulfill Order" in the OHC dashboard.
- **Action**: OHC requests shipping rates from Shippo API, displays them, and upon selection, generates a shipping label and tracking number.
- **User Interface**: Within an order view, show a "Create Shipping Label" button. Let the user compare rates from different carriers and download the PDF label.
- **Integration Flow**: OAuth connection to Shippo or API key entry in Settings.

**Implementation Prompt**:
Integrate the Shippo API. Add a shipping module to the order management flow. When an order is ready to fulfill, allow the user to request rates, purchase a label via Shippo, and save the tracking URL to the customer's order record.

**Priority**: P2
**Estimated Scope**: Large
