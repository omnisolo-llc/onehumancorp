# [Shipping & Logistics] Automated Shipping Operations via Shippo

**Title**: Implement Automated Shipping Rates and Label Generation with Shippo

**Problem Statement**:
Business owners selling physical goods, like Priya (The Boutique Owner) or Maya (The Home Baker shipping cookies), struggle with calculating accurate shipping costs at checkout. Overcharging loses the customer; undercharging costs the business money. Furthermore, copying and pasting addresses into USPS/FedEx websites to print labels is tedious and error-prone. They need accurate live rates at checkout and a one-click way to print labels from their phone.

**Research Report**:
I evaluated EasyPost, ShipStation API, and Shippo.
- **Shippo**: Extremely developer-friendly API. Excellent pre-negotiated rates for USPS, UPS, and FedEx right out of the box (critical for new small businesses without their own carrier accounts). Great support for international shipping and customs forms. Pricing is highly favorable for platforms.
- **EasyPost**: Also an excellent API, very similar to Shippo, but Shippo's dashboard and pre-negotiated rate structures are slightly more geared toward enabling small merchants quickly.
- **ShipStation**: Powerful, but their API is designed more for integrating into their heavy dashboard rather than letting OHC completely white-label the experience.
- **Conclusion**: Shippo is the best fit. It allows OHC to provide live rates at checkout and lets the AI "Manager" instantly generate a printable PDF label as soon as an order is paid, without the user ever leaving the OHC app.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department.
- **Triggers & Flow**:
  1. User adds package dimensions/weight to their products.
  2. At checkout, OHC pings Shippo to get live carrier rates based on the customer's address.
  3. The customer pays for the order + shipping.
  4. The AI "Manager" immediately calls Shippo to purchase the label and generates a tracking number.
  5. The Tracking number is sent to the customer via the "Ambassador".
  6. The business owner opens the OHC app, taps "Print Label", and the PDF is sent to their AirPrint/Google Print printer.
- **User View**: A clean "Orders to Ship" list. A prominent "Buy & Print Label" button that handles the transaction invisibly.

**Implementation Prompt**:
Integrate the Shippo API to provide end-to-end shipping logistics. Implement a mechanism to fetch and display live, accurate shipping rates during the customer checkout flow based on product weights and destination. Create an "Order Fulfillment" UI in the backend where the business owner can purchase and generate a shipping label with one tap. Ensure the resulting PDF label is easily viewable and printable directly from a mobile device. The system must automatically update the order status to "Shipped" and notify the customer with the tracking link.

**Priority**: P1
**Estimated Scope**: Large
