# Issue Brief: DoorDash Drive White-Label Delivery

## Title
[Delivery] DoorDash Drive White-Label Delivery Integration

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman selling parts) want to offer local delivery to their customers to increase sales and convenience. However, listing on major delivery marketplaces like DoorDash or UberEats incurs massive commission fees (often 20-30% per order) which eats away their entire profit margin. Furthermore, they lose control of the customer relationship and data. They need a way to offer "Local Delivery" directly on their own website without hiring their own drivers and without paying exorbitant marketplace percentages.

## Research Report
- **Competitive Audit**:
  - **Shopify / Wix**: Offer local delivery options, but primarily rely on the merchant to fulfill it themselves or integrate with complex 3rd party apps that charge monthly fees on top of per-delivery fees.
  - **Square / Toast**: Have white-label delivery for restaurants, but it's often siloed within their ecosystem and less flexible for non-restaurant retail or service businesses.
  - **DoorDash Drive API**: Offers a white-label fulfillment API. Businesses pay a flat fee per delivery (e.g., $7-$10) rather than a percentage of the cart value. The business keeps the customer data, the checkout happens on the business's OHC storefront, and DoorDash simply dispatches a Dasher to fulfill it.
- **Key Findings**:
  - **Ease of Use**: The API allows for quoting delivery times and costs in real-time, which can be passed onto the customer or absorbed by the business.
  - **Pricing**: Flat per-delivery fee. No marketplace commission. Huge cost saving for high-value orders (e.g., a $100 custom cake delivery).
  - **Reputation**: Reliable, wide coverage network of Dashers.

## Design Doc
When a customer builds a cart on an OHC storefront, they can select "Local Delivery" as a fulfillment method. The OHC engine will ping the DoorDash Drive API with the store's address and the customer's address to get a real-time delivery quote. This cost is added to the cart. Upon checkout completion, OHC autonomously creates a delivery task via the DoorDash Drive API. The business owner gets a notification that a Dasher is on the way to pick up the order, and the customer receives an SMS tracking link (powered by DoorDash or our own SMS agent) to track their delivery in real time.

## Implementation Prompt
Integrate the DoorDash Drive API into the OHC fulfillment engine.
- During checkout, if the customer selects Local Delivery and is within the store's delivery radius, fetch a delivery quote from DoorDash Drive and present the cost to the user.
- Upon successful payment, create the delivery request with DoorDash Drive.
- Expose the DoorDash tracking URL to the customer (via their order status page or SMS).
- Show the business owner the delivery status (Dasher assigned, picked up, delivered) in their OHC dashboard.
- Provide a simple settings page for the business owner to enable/disable DoorDash Drive and choose whether to pass the flat fee to the customer or offer "Free Delivery" and absorb the cost.

## Priority
P1

## Estimated Scope
Medium
