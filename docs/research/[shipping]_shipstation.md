# [shipping] ShipStation Integration

## Problem Statement
Fulfilling orders is a major bottleneck for e-commerce small businesses. Manually copying order details to carrier websites, comparing rates, and printing labels one by one is time-consuming and error-prone. By integrating ShipStation into OHC, business owners can automate rate comparison, label generation, and order tracking from a single interface, significantly reducing fulfillment time.

## Research Report
### Overview
ShipStation is a leading web-based shipping software that integrates with numerous selling channels and carriers (USPS, UPS, FedEx, DHL, etc.). It acts as a centralized hub for managing fulfillment operations.

### Ease of Use
The integration should seamlessly push OHC orders to ShipStation. The business owner will manage the actual fulfillment (printing labels) within the ShipStation interface, which is designed for high-volume processing. Tracking information must then be synced back to OHC.

### Reputation
ShipStation is widely recognized as an industry standard for e-commerce fulfillment, known for its extensive carrier network and automation rules.

### Pricing
ShipStation operates on a subscription model based on shipment volume, starting around $9.99/month. The OHC integration itself should be free.

### Environment
Works in Cloud.

### AI Integration
Low potential for direct AI integration in the core flow, but AI could optimize packaging choices or predict shipping delays based on historical carrier data.

## Design Doc
1.  **Connection:** User connects their ShipStation account via "Integrations" -> "Shipping" -> "Connect ShipStation" using API keys.
2.  **Order Sync:** When an order is marked as "Paid" in OHC, its details (items, shipping address, requested method) are automatically pushed to ShipStation.
3.  **Fulfillment Update:** When the user prints a label in ShipStation, ShipStation sends a webhook to OHC containing the tracking number and carrier info. OHC updates the order status to "Shipped" and notifies the customer.
4.  **Inventory Sync (Optional):** ShipStation can act as the source of truth for inventory counts, syncing back to OHC.

## Implementation Prompt
Implement a bidirectional integration with ShipStation. The business owner should configure the integration using ShipStation API Keys. Create a background worker that automatically exports new OHC orders to ShipStation. Set up a webhook endpoint to receive fulfillment notifications from ShipStation; when received, automatically update the OHC order status to "Shipped," attach the tracking number, and trigger the standard customer notification email.

## Priority
P1 (High) - Essential for any product-based business using the OHC storefront.

## Estimated Scope
Medium
