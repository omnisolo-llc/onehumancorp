# 🔍 Scout: ShipStation (Shipping & Logistics)

## Title
Integrate ShipStation for Advanced E-commerce Fulfillment

## Problem Statement
For business owners selling physical goods at a higher volume (e.g., Priya the Boutique Owner), manually copying order data to buy shipping labels is a massive bottleneck. They need a system that aggregates orders, calculates accurate shipping rates, and prints labels in bulk across multiple carriers (USPS, UPS, FedEx).

## Research Report
**ShipStation** is a leading web-based e-commerce shipping solution. It acts as a middleware between e-commerce platforms (like OHC) and shipping carriers. It offers a very mature API for order sync and label creation.

**Pros for Non-Technical Users:**
- Best-in-class UI for bulk label printing and order management.
- Pre-negotiated discounted shipping rates for USPS, UPS, etc.
- Handles complex tasks like return labels and international customs forms automatically.

**Integration Risks:**
- Pushing order data reliably is critical; missed webhooks mean lost orders.
- It pushes the user out of the OHC UI into the ShipStation UI for fulfillment, which breaks the "all-in-one" single-pane-of-glass experience.
- The API rate limits can be restrictive during high-volume periods (like Black Friday).

**Pricing:**
- Starts at $9.99/month. No free tier.

**Environment Support:**
- Cloud-only. Requires an active ShipStation account.

## Design Doc
- **Integration:** The user connects their ShipStation account via API Keys in the OHC settings.
- **Data Flow:** OHC acts as a "Custom Store" in ShipStation. When an order is placed in OHC, it is pushed via API to ShipStation. The user fulfills the order in ShipStation, which then sends a webhook back to OHC with the tracking number and carrier info. OHC then marks the order as fulfilled and notifies the customer.
- **Action:** The "Operations" agent monitors ShipStation webhooks, updates the OHC database, and triggers the "Customer Success" agent to send the shipping confirmation email.

## Implementation Prompt
Build a two-way integration with the ShipStation API. First, implement a reliable background worker that pushes new physical product orders from OHC to ShipStation. Second, create a secure webhook endpoint to receive fulfillment notifications from ShipStation (containing tracking numbers) and update the corresponding order status in OHC. Ensure robust retry logic for API calls and clear error states in the UI if an order fails to sync.

## Priority
P2

## Estimated Scope
Large
