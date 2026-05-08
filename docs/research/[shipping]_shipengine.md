# Scout: Tool Integration Research [Q2]

## [Shipping] Issue Brief: ShipEngine Integration

**Title**: Multi-Carrier Shipping & Label Generation via ShipEngine

**Problem Statement**:
Priya (Boutique Owner) spends too much time comparing shipping rates between different carriers (UPS, FedEx, DHL) and manually typing addresses to print labels. She needs to see the best rates directly when fulfilling an order and print a shipping label with one click, without leaving her OHC dashboard.

**Research Report**:
- **Tool**: ShipEngine API.
- **Evaluation**: ShipEngine is a multi-carrier shipping API that connects to over 100 carriers. It is the engine behind many major shipping platforms.
- **Ease of Use**: High for the merchant. They just select "Print Label" and OHC does the heavy lifting of rate comparison.
- **Pricing**: Pay-per-label (typically $0.05 per label) plus carrier costs.
- **Reputation**: Highly reliable, industry-standard API.
- **Cloud vs. Standalone**: Works in both. Merchant connects their own carrier accounts.

**Design Doc**:
- When an order is ready for fulfillment, OHC calls ShipEngine to get real-time rates from the merchant's connected carriers.
- "The Manager" AI highlights the cheapest and fastest options.
- The merchant selects a rate and clicks "Buy & Print Label."
- OHC generates a PDF label via ShipEngine and automatically updates the order with the tracking number.
- The customer is automatically notified of the tracking link.

**Implementation Prompt**:
Implement a shipping and fulfillment module powered by the ShipEngine API. Fetch live rates for orders, allow merchants to purchase labels, and generate printable PDFs. Automatically sync tracking numbers back to the OHC order and notify customers.
- **Acceptance Criteria**: Merchant can compare rates from multiple carriers. Merchant can purchase and print a shipping label. Tracking number is automatically attached to the order.
- **Priority**: P2
- **Estimated Scope**: Large
