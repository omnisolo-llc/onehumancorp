# Research Report: Shippo Integration for Small Business Shipping

## Track 1: Dynamic Integration & Market Need Discovery
**Ecosystem Scraping & Community Mining:**
Based on reviews from r/smallbusiness and e-commerce forums, one of the biggest pain points for non-technical small business owners (like Maya, who runs an artisanal craft shop, or Carlos, managing a local electronics store) is handling shipping logistics. Managing multiple carrier rates (USPS, FedEx, UPS, DHL), printing labels, and syncing tracking numbers manually often leads to lost time, expensive shipping errors, and poor customer experience. While basic website builders offer simple rate calculations, they lack comprehensive, multi-carrier label printing and automated tracking sync for local, omnichannel, or omnichannel sales.
**Discovered Integration Target:** Shippo (Shipping & Fulfillment)

## Track 2: Selected Tool Deep-Dive Evaluation (Shippo)
**User-First Value Mapping:**
Shippo allows business owners to instantly access deeply discounted shipping rates across 85+ global carriers without needing to negotiate their own contracts. For a user like Maya, the OHC integration would mean that when an order is placed, she simply clicks "Fulfill" in her OHC dashboard, compares rates instantly, prints a shipping label with her thermal printer, and the tracking information is automatically emailed to her customer.

**Capabilities & Limits:**
- **Developer Docs & API:** Shippo offers a robust REST API with clean documentation, strong OAuth 2.0 flows, and reliable webhooks for tracking updates.
- **Webhooks:** Excellent support for `transaction.created`, `track.updated`, which is perfect for keeping OHC order states in sync.
- **SaaS Viability:** Shippo provides a "Pay-as-you-go" free tier (no monthly fee, just per-label costs, or free for their default carriers). This makes it highly viable for standalone, multi-tenant cloud, and offline-first setups because there are no upfront subscription costs to block our small business users.

## Track 3: Strategic Integration Dispatch (Issue Brief)
*(This section is documented as an issue brief for engineering)*

### Title: Integrate Shippo for Automated Multi-Carrier Shipping & Label Printing

### Problem Statement:
Small business owners currently spend hours manually copying order details into carrier websites (USPS, UPS, etc.) to buy shipping labels and then copying the tracking numbers back into their systems to notify customers. This manual workflow is error-prone and scales poorly. Non-technical users need a seamless way to compare carrier rates, buy labels, and notify customers without ever leaving the OHC dashboard.

### Research Report:
- **Target:** Shippo
- **Market Need:** Extremely high. Shipping logistics is routinely cited as a top 3 operational hurdle for independent ecommerce sellers.
- **Ease of Use for Non-Technical Users:** Shippo masks the complexity of carrier APIs. Users just need to input package dimensions and weight.
- **Pricing:** Very SMB-friendly. The Pay-as-you-go tier has no monthly fee (only $0.05 per label using your own carrier accounts, or free when using Shippo's default carrier accounts).
- **Reputation:** Highly rated among Shopify, Wix, and standalone ecommerce operators.

### Design Doc:
- **Trigger:** When an order transitions to a "Ready for Fulfillment" state in OHC, the Shippo integration is triggered.
- **Actions Taken:**
  1. OHC sends the destination address and item weights to Shippo to fetch real-time carrier rates.
  2. The user is presented with a simplified UI to select a rate and purchase the label.
  3. Upon purchase, OHC receives the tracking number and label PDF.
  4. OHC automatically emails the tracking information to the customer and updates the order status to "Shipped."
- **User View:** A "Shipping" module on the Order Details page showing live rate options, a "Buy Label" button, and a link to print the generated label.

### Implementation Prompt:
Integrate Shippo to enable seamless shipping workflows for OHC merchants. The user must be able to securely connect their Shippo account. Once connected, the OHC order management UI should display a "Fulfill Order" flow that fetches available shipping rates based on order weight/dimensions and the customer's address. The user must be able to select a rate, generate a shipping label (available for download/printing), and automatically sync the tracking number back to the order, triggering a customer notification. Ensure the flow gracefully handles errors such as invalid addresses or missing package dimensions.

### Priority: P1 (High)
### Estimated Scope: Medium
