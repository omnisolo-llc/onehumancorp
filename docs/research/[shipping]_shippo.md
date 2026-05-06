### Title
Native Shipping Rate Calculation and Label Generation (Shippo)

### Problem Statement
Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

### Research Report
- **Strategy**: Build a native fulfillment interface powered by the Shippo API in the backend.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier available, nominal fee per label thereafter.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo and saves the tracking number.
- OHC automatically emails the customer the tracking number.

### Implementation Prompt
Implement a native shipping and fulfillment module powered by Shippo. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
- **Acceptance Criteria**: Live shipping rates appear at checkout. Merchant can click "Print Label" to generate a PDF label. Tracking number is automatically sent to the customer.
- **Priority**: P1
- **Estimated Scope**: Large
