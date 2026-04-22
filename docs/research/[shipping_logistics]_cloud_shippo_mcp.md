# Shippo Integration
## Problem Statement
Small business owners, such as Priya the boutique owner, need an efficient way to generate shipping labels, compare rates across carriers (USPS, UPS, FedEx, DHL), and track packages for physical product deliveries without managing multiple carrier accounts.

## Research Report
**Tool**: Shippo
**Ease of use**: High. Known for a developer-friendly API and a user-friendly dashboard for merchants.
**Pricing**: Free tier available (pay-as-you-go, just pay for postage). Professional plans start at $19/month for advanced features and higher volumes.
**Reputation**: Highly reputable, powers shipping for major platforms (e.g., Shopify, Wix, Square) and trusted by thousands of SMBs.

## Design Doc
**Cloud Mode**: Integrate via Shippo REST API. When an order is placed on the OHC platform, the details are sent to Shippo to fetch shipping rates. The business owner selects a rate and generates a label. OHC stores the tracking number and updates the order status.
**Standalone Mode**: Since shipping labels require real-time API access to carriers, the core label generation must happen online. However, OHC can queue shipping requests offline and process them in bulk once internet connectivity is restored.
**Triggers**: Customer completes checkout for physical goods, business owner fulfills an order.
**User Experience**: Business owner views the order in OHC, clicks "Fulfill", sees a list of shipping rates from different carriers, purchases the label, and prints it directly from the OHC interface. Customers automatically receive an email with the tracking link.

## Implementation Prompt
Integrate Shippo into the OHC platform to provide multi-carrier shipping and label generation.
**Acceptance Criteria**:
1. Business owners can connect a Shippo account to OHC via API keys or OAuth.
2. During order fulfillment in the OHC dashboard, users can input package dimensions and weight to retrieve real-time shipping rates from Shippo.
3. Users can purchase and generate a shipping label (PDF/ZPL) directly within OHC.
4. OHC automatically saves the tracking number to the order and updates the order status to "Shipped".

## Priority
P2

## Estimated Scope
Medium
