# Shipping & Logistics Research Brief

## Title
Automated Shipping Rates and Label Generation

## Problem Statement
For product-based small businesses, shipping is a major headache. Calculating accurate shipping rates manually leads to undercharging (losing money) or overcharging (losing sales). Manually typing addresses into carrier websites to generate labels is time-consuming and prone to typos.

## Research Report
### Market Context
Aggregators have revolutionized shipping by providing single APIs that connect to dozens of carriers (USPS, FedEx, DHL) and negotiating discounted rates for small shippers.

### Tool Evaluations

#### 1. Shippo
- **Ease of Use:** High. Great UI and API.
- **Pricing:** Free basic tier, pay per label (usually 5¢) plus postage.
- **Capabilities:** Excellent domestic and international coverage, automatic customs forms.
- **Reputation:** Highly reliable, very popular with small e-commerce sites.

#### 2. ShipStation
- **Ease of Use:** Moderate. Very powerful but complex dashboard.
- **Pricing:** Starts at $9.99/month.
- **Capabilities:** Deep inventory management, multi-channel syncing.
- **Reputation:** The standard for high-volume shippers, but perhaps too complex for a casual seller.

#### 3. EasyPost
- **Ease of Use:** Developer-focused API.
- **Pricing:** Free for up to 120,000 shipments/year.
- **Capabilities:** Pure API, extremely fast, robust tracking webhooks.
- **Reputation:** Best in class for developers building custom shipping workflows.

### Recommended Direction
Integrate with EasyPost or Shippo to abstract away the complexity of individual carriers. Provide a simple interface for the business owner to buy and print labels directly from an order page.

## Design Doc
### Trigger & Action
1. **Trigger:** A customer places an order requiring physical shipping.
2. **Action:** OHC queries the shipping API for rates based on weight/dimensions. The business owner clicks "Generate Label." OHC purchases the postage and retrieves a PDF label and tracking number.
3. **User View:** An "Orders" tab where the owner can input box dimensions, view rates, click "Buy Label," and instantly print the PDF. The tracking number is automatically emailed to the customer.

### Environment Support
- **Cloud Mode:** OHC manages the API connection.
- **Standalone Mode:** User must provide their own EasyPost/Shippo API key to generate labels.

## Implementation Prompt
Implement a "Fulfillment" feature for product orders.
- Integrate with a mock shipping API to calculate rates based on origin, destination, and weight.
- Allow the user to select a shipping rate and click "Generate Label."
- Return a dummy PDF or image representing the shipping label.
- Automatically update the order status to "Shipped" and generate a tracking number.
- Acceptance criteria: A user can take a "Pending" order, generate a label, and see the status change to "Shipped" with a tracking link.

## Priority
P2 (Medium)

## Estimated Scope
Large

### Extended Fulfillment Architecture Analysis
#### Address Validation
Carrier rejection due to invalid addresses incurs financial penalties and delays. The integration must proactively validate addresses using the provider's API before quoting a rate or attempting to purchase postage. Minor typos should trigger a user-friendly prompt suggesting the standardized address format.

#### Dimensional Weight Optimization
Shipping costs are calculated using either actual weight or dimensional weight, whichever is higher. Small business owners frequently overpay because they use unnecessarily large boxes. The system could eventually suggest optimal box sizes based on product dimensions, significantly cutting operational costs for the merchant.

#### International Customs Forms
Cross-border shipping is a massive hurdle. The integration must support generating digital customs declarations (e.g., CN22/CN29). Product descriptions, origin countries, and HS tariff codes must be collected and transmitted seamlessly alongside the label generation request to ensure packages clear customs without friction.

#### Real-time Tracking Webhooks
Customers expect step-by-step visibility into their package's journey. The shipping module should listen for tracking update webhooks and automatically notify the customer via email or SMS when an item is marked "Out for Delivery" or "Delivered."

### User Persona Match
- **Fatima (Boutique Owner):** High value. Shipping dresses nationwide is a core part of her daily operations.
- **Carlos (Consultant):** Non-applicable. His services are entirely digital.

### Conclusion
By automating the tedious process of calculating rates and printing labels, OHC transforms order fulfillment from a dreaded chore into a one-click operation, directly increasing the profit margins of retail-focused business owners.
