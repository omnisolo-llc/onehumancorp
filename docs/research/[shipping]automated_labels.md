### Title
`[shipping]automated_labels`: Implement Shipping Label Generation via Shippo

### Problem Statement
For product-based businesses, manually calculating shipping rates and copying addresses to carrier websites to buy labels is a massive bottleneck. They need a way to automatically offer accurate shipping rates at checkout and print labels with one click after an order is placed.

### Research Report
- **Tool**: Shippo API
- **Pros**: Aggregates dozens of carriers (USPS, UPS, FedEx, international) into a single API. Excellent rate calculation.
- **Cons**: Support can be slow for complex carrier account issues.
- **Reputation**: Very strong, widely used by major e-commerce platforms.
- **Pricing**: Pay-as-you-go (5¢ per label) or flat monthly fees for high volume.
- **Ease of Use for Non-Technical Users**: Users enter package dimensions and click "Buy Label". Rates are shown instantly.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: An order is placed, and the business owner clicks "Fulfill Order".
- **Action**: The OHC API server requests label generation from Shippo using the saved package dimensions and customer address.
- **User View**: A "Fulfillment" page showing order details, a "Buy Shipping Label" button, and a tracking link generator.

### Implementation Prompt
Integrate Shippo to provide real-time shipping rates at checkout and automated label generation for the business owner. The system should allow the owner to define standard package sizes. Upon order completion, provide a one-click flow to purchase and print the shipping label, and automatically email the tracking number to the customer.

### Priority
P2

### Estimated Scope
Large
