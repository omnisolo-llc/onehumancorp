# 🔍 Scout: Native Integration Architecture & Strategy

## Shipping & Logistics Integration

### Title
Integrate Shippo for Automated Label Generation and Live Rates

### Problem Statement
Priya (Boutique Owner) and Maya (Home Baker) spend hours copying and pasting customer addresses into different carrier websites to print shipping labels. They need to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

### Research Report
- **Strategy**: Build a native fulfillment interface powered by the Shippo API in the backend.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Shippo provides a unified API for aggregating rates from USPS, UPS, FedEx, DHL. It is simple to integrate and offers a pay-as-you-go model (no monthly fee). By integrating it natively, the user can just click 'Buy Label & Print' without leaving OHC.
- **Risks**: Reliance on carrier APIs which can occasionally be slow or down. International shipping might require complex customs declarations, which should be deferred for future iterations.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth) and Standalone (API Key) compatible via API.

### Design Doc
- **Trigger:** A new order is placed with a physical shipping address.
- **Actions:**
  1. OHC fetches live shipping rates from Shippo based on order weight, dimensions, and destination natively during checkout.
  2. The user navigates to the specific order details page within OHC and selects "Fulfill Order".
  3. The Operations agent shows the cheapest shipping option.
  4. The user clicks a native 'Buy Label & Print' button, and OHC purchases the label via Shippo and downloads the PDF label for printing.
  5. OHC automatically marks the order as "Shipped" and emails the tracking number to the customer.
  6. **AI Integration**: The Customer Success Agent monitors tracking numbers natively and proactively notifies the customer if a delivery is delayed.

### Implementation Prompt
Implement a native shipping and fulfillment module powered by Shippo. Connect the Shippo API to fetch shipping rates based on order weight/dimensions. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
- **Acceptance Criteria**: Live shipping rates appear at checkout. Merchant can click "Print Label" to generate a PDF label. Tracking number is automatically sent to the customer.
- **Priority**: P1
- **Estimated Scope**: Large
