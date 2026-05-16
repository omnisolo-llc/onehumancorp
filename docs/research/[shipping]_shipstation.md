# Scout: Tool Integration Research

## Shipping & Logistics
**Title**: Integrate ShipStation for High-Volume Multi-Channel E-commerce Fulfillment
**Problem Statement**: As small boutique owners grow, they start selling on OHC, Amazon, and Etsy simultaneously. They are overwhelmed by logging into three different systems to print shipping labels and update tracking numbers.
**Research Report**:
- ShipStation is an industry-standard shipping aggregator. It pulls orders from multiple sales channels into a single dashboard.
- It offers deeply discounted USPS/UPS rates and advanced automation rules (e.g., "always ship items under 1lb via USPS First Class").
- Pricing: Starts at $9.99/month for 50 shipments. No free tier, but the time saved justifies the cost for growing merchants.
- Compatibility: OHC needs to act as a "Custom Store" integration for ShipStation, exposing an XML or REST API for ShipStation to pull orders. Works in both Cloud and Standalone modes.
**Design Doc**:
- OHC implements ShipStation's Custom Store API specification.
- The user connects OHC to their ShipStation account by providing a generated API key and Endpoint URL from their OHC dashboard.
- Orders placed on OHC automatically appear in the user's ShipStation dashboard.
- When the user prints a label in ShipStation, ShipStation sends a webhook back to OHC to mark the order as "Shipped" and attaches the tracking number.
**Implementation Prompt**: Expose an API endpoint compatible with ShipStation's Custom Store format. Allow ShipStation to poll for new, unfulfilled orders. Listen for shipment notification webhooks from ShipStation to update the OHC order status to 'Shipped' and save the tracking URL.
**Priority**: P1
**Estimated Scope**: Large