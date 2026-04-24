# Shipping & Logistics Tools

**Title**: Integrate Shipping Rate Calculation and Label Generation (Shippo, ShipStation)

**Problem Statement**:
Product-based businesses (like Priya the Boutique Owner) struggle with calculating accurate shipping costs at checkout and manually writing shipping labels. They need an automated way to charge customers the right amount and print USPS/UPS labels with one click.

**Research Report**:
Evaluated Shippo and ShipStation.
- **Shippo**: API-first shipping platform.
  - *Ease of Use*: Built for platforms. OHC can completely white-label the label generation process.
  - *Pricing*: Pay-as-you-go model is perfect for small businesses. Provides discounted USPS rates out of the box.
  - *Reputation*: Highly reliable API.
- **ShipStation**: Very popular, but UI-heavy.
  - *Ease of Use*: Usually expects the merchant to log into the ShipStation dashboard to print labels, which violates OHC's "one unified app" principle.
  - *Pricing*: Monthly subscription required.
- **Recommendation**: Integrate Shippo's API. It allows us to keep the user entirely inside the OHC app while providing enterprise-grade shipping features.

**Design Doc**:
- **Trigger**: A customer adds an item to their cart and enters their address.
- **Action**: OHC backend pings Shippo API to get real-time rates and displays them. When the order is paid, the business owner clicks "Generate Label" in the OHC app. OHC buys the label via Shippo and displays the PDF for printing. The "Customer Success" agent auto-emails the tracking number.
- **User Experience**: Business owner views an order, taps "Print Shipping Label", and a PDF opens. Tracking updates are handled automatically.

**Implementation Prompt**:
Integrate the Shippo API to fetch real-time shipping rates during the checkout flow. Build a backend endpoint to purchase and retrieve shipping labels (PDF/ZPL format). Add a "Fulfill Order" UI in the Flutter app that allows the user to generate and download the label. Automate tracking status updates via webhooks.

**Priority**: P1
**Estimated Scope**: Medium
