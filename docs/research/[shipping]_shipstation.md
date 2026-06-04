## [Shipping] ShipStation Integration
**Title**: Integrate ShipStation for Automated Logistics
**Problem Statement**: Manual entry of shipping addresses and tracking numbers is the largest operational bottleneck for product-based businesses.
**Research Report**:
- **Tool**: ShipStation
- **Target Persona**: E-commerce operators
- **Advantages**: Directly reduces the largest operational bottleneck by automating the flow of orders to fulfillment and returning tracking info.
- **Risks**: API rate limits and ensuring data consistency during sync.
- **Pricing**: Standard ShipStation pricing tiers.
- **Compatibility**: Cloud, Standalone (via API).
**Design Doc**:
- User authenticates via API keys/OAuth to connect ShipStation.
- OHC automatically pushes new orders to ShipStation.
- When an order is fulfilled in ShipStation, ShipStation sends a webhook to OHC.
- OHC updates the order status and stores the tracking number, notifying the customer.
**Implementation Prompt**: Build a two-way sync with ShipStation. Create a mechanism to push paid orders from OHC to ShipStation automatically. Implement webhook endpoints to receive fulfillment notifications and tracking numbers from ShipStation, updating the order state in OHC.
**Priority**: P1
**Estimated Scope**: Large
