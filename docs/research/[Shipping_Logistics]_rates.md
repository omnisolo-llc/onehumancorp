# Title: Automated Shipping Rate Calculation and Label Generation

## Problem Statement
Small business owners selling physical goods lose significant time manually calculating shipping costs for customers, often undercharging and eating the cost, or overcharging and losing the sale. Once a sale is made, they have to manually copy-paste customer addresses into separate carrier websites (USPS, FedEx, DHL) to buy and print shipping labels, which is tedious and error-prone. They need accurate, real-time rates at checkout and one-click label generation directly from their order dashboard.

## Research Report
We evaluated logistics APIs to handle real-time rates and label generation:
- **EasyPost:** Highly developer-friendly, robust API. Supports over 100 carriers globally. Offers very competitive USPS pricing out of the box. Excellent documentation and reliable webhooks for tracking updates.
- **Shippo:** Very similar to EasyPost in capabilities and carrier support. Slightly more focus on ecommerce platforms, but the API is just as capable. Pricing models are comparable (cents per label).
- **ShipEngine:** The API powering ShipStation. Extremely powerful, but can have a steeper learning curve compared to EasyPost.
- **Cloud vs. Standalone Compatibility:** Fetching rates and generating labels are synchronous outbound API requests, which work perfectly in both **Cloud** and **Standalone** modes. Tracking updates rely on webhooks. For Standalone mode, we can implement a polling mechanism to check the tracking status of active shipments periodically if webhook delivery is not feasible, ensuring parity in the user experience.

**Recommendation:** Integrate EasyPost as the primary logistics provider for OHC. It abstracts the complexity of dealing with individual carriers and provides cheap default rates, which is a huge value-add for small merchants.

## Design Doc
In the OHC dashboard under "Fulfillment Settings," the user can enter their shipping origin address and package dimensions. During the customer checkout flow, OHC pings the logistics API with the cart contents and customer address to display real-time shipping options (e.g., "Standard - 3 Days - $5.00"). Once an order is placed, the merchant sees the order in their dashboard with a "Buy & Print Label" button. Clicking this securely purchases the label via the API and opens a PDF for printing. The system automatically attaches the tracking number to the order and emails it to the customer.

## Implementation Prompt
Implement a shipping and fulfillment module. During customer checkout, dynamically calculate and display shipping rates based on the merchant's origin address, the customer's destination, and standard package dimensions using a logistics API (e.g., EasyPost). Add a feature in the merchant's order management dashboard to purchase and generate a printable shipping label (PDF) with a single click. Automatically capture the resulting tracking number, update the order status to "Shipped," and notify the customer. The UI must hide API complexity from the merchant.

## Priority
P1

## Estimated Scope
Medium
