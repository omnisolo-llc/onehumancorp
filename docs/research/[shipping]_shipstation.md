# Scout: Tool Integration Research

## [Shipping] Issue Brief: ShipStation for Native Fulfillment
**Problem Statement**:
Maya (Home Baker) spends hours manually copying customer addresses into carrier websites to print shipping labels for her cookies. She needs to print labels with one click from her OHC dashboard to save time.

**Research Report**:
- **Tool**: ShipStation API.
- **Evaluation**:
  - **Ease of Use**: High once configured.
  - **Pricing**: Subscription-based; OHC can negotiate partner rates.
  - **Reputation**: Industry standard for SMB shipping.
  - **Cloud vs. Standalone**: Both supported.
- **Key Advantages**: Discounted USPS/UPS rates out of the box. Automated tracking number updates.
- **Risks**: Requires accurate product weight/dimensions.

**Design Doc**:
- **User Flow**: In "Orders", user clicks "Buy Label". OHC fetches rates from ShipStation.
- **Integration**: `POST /orders` to ShipStation. Generate and download PDF label.
- **User Experience**: Label prints directly; tracking is automatically sent to the customer.

**Implementation Prompt**:
Implement a native shipping label generation flow using the ShipStation API. Fetch live shipping rates based on order weight and destination. Allow merchants to purchase and print labels directly from the OHC order management screen.

**Priority**: P1
**Estimated Scope**: Medium
