# [Shipping & Logistics] Integrate Shippo for Automated Fulfillment

## Problem Statement
Sellers of physical products (like Priya's Boutique) struggle with calculating accurate shipping rates at checkout and manually copying addresses to print labels. They need an automated way to charge customers the right shipping fee and print labels with one click.

## Research Report
**Evaluated Tool:** Shippo API
**Alternatives Considered:** EasyPost, ShipEngine
**Pros:** Excellent API design, strong network of global carriers, built-in address validation. Often provides discounted USPS/UPS rates out of the box without requiring the user to negotiate their own carrier accounts.
**Cons:** Customer support can be slow on lower tiers.
**Ease of Use for Non-technical Users:** The user enters the weight of their product. When an order arrives, they click "Buy Label", and a printable PDF appears. Shippo's default discounted rates mean the user doesn't need to configure carrier accounts.
**Pricing:** Pay-as-you-go (per label fee) or monthly subscriptions.
**Deployment:** Cloud-native.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer enters their shipping address at checkout (for live rates), or the business owner clicks "Fulfill Order".
- **Action:** OHC queries Shippo for shipping rates, or generates a shipping label transaction.
- **AI Agent Interaction:** "The Operations Manager" automatically fetches the tracking number from Shippo and triggers "The Ambassador" to email the customer the tracking link.
- **User View:** A "Fulfillment" screen on the order details page showing a generated label PDF and tracking status.

## Implementation Prompt
Integrate the Shippo API to provide real-time shipping rate calculation at checkout and shipping label generation in the order management dashboard. Ensure tracking webhooks are processed to update order statuses and trigger customer notifications automatically.

## Priority
P1

## Estimated Scope
Medium
