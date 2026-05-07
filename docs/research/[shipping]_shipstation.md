# Streamline Order Fulfillment with ShipStation

## Problem Statement
Product-based small businesses spend hours copying and pasting customer addresses into carrier websites (USPS, FedEx, UPS) to buy shipping labels. This manual process is error-prone, slow, and makes it difficult to provide tracking numbers back to the customer quickly.

## Research Report
**Tool Evaluated:** ShipStation API
- **Ease of Use:** ShipStation acts as an aggregator. The business owner connects their OHC store to ShipStation, and manages all their shipping labels in one place.
- **Pricing:** Plans start at $9.99/month for 50 shipments. They also offer discounted USPS/UPS rates, which often pays for the subscription itself.
- **Reputation:** Extremely popular among SMBs. Robust API, wide carrier support, and reliable uptime.
- **Deployment:** Cloud mode fully supported via API. Standalone mode works perfectly as it just pushes order data out to ShipStation's cloud.

## Design Doc
- **Trigger:** Business owner generates API keys in ShipStation and pastes them into OHC Integrations.
- **Action:** When an order is marked as "Paid" in OHC, it is automatically pushed to ShipStation. When a label is generated in ShipStation, a webhook sends the tracking number and carrier back to OHC, marking the order "Shipped".
- **User View:** They continue to fulfill orders via ShipStation, but their OHC store automatically updates customers with tracking links without manual data entry.

## Implementation Prompt
Implement a ShipStation integration that automatically syncs paid OHC orders to a user's ShipStation account. Listen for shipment notification webhooks from ShipStation to automatically update the OHC order status to "Shipped" and attach the relevant tracking number and carrier information, triggering an automated email to the customer.

## Priority
P2

## Estimated Scope
Medium