# Issue Brief: Real-time Shipping Rates and Label Generation

**Category**: Shipping & Logistics

## Problem Statement
E-commerce business owners waste time manually calculating shipping costs and copying addresses into carrier websites to print labels.

## Research Report

### Tool Evaluations

**1. EasyPost**
- **Capabilities**: Connects to 100+ carriers (USPS, FedEx, UPS, DHL) via a single API.
- **Pricing**: Pay-per-label model (e.g., 1¢ per label). Very affordable for small businesses.
- **Feature Set**: Offers real-time rate calculation, address verification, label generation (PDF/ZPL), and tracking webhooks.
- **Mode Compatibility**: Fully API driven. OHC Cloud can manage a master EasyPost account, or Standalone users can plug in their own API keys.

**2. Shippo**
- **Capabilities**: Similar to EasyPost, strong multi-carrier support.
- **Pricing**: Has a subscription tier as well as a pay-as-you-go tier.
- **User Experience**: Shippo also offers a web UI for users, but our goal is to keep the user inside OHC. We only need their API.

**3. Sendle**
- **Capabilities**: Great for small businesses focusing on carbon-neutral shipping and simple flat rates.
- **Pricing**: Flat rates based on size, often cheaper than standard post for specific routes.

**Summary Recommendation**: Integrate EasyPost. Its API is highly robust for generating shipping labels and tracking packages. OHC will store the package dimensions for products, send them to EasyPost at checkout, and display live rates to the buyer.


## Design Doc
Integrate EasyPost or Shippo API. Auto-calculate rates based on cart weight/dimensions. Generate PDF shipping labels directly in OHC. Provide automated tracking updates.

## Implementation Prompt
Implement a shipping module that fetches real-time rates based on package dimensions and allows the user to generate and print a shipping label with one click.

## Priority
P2

## Estimated Scope
Medium
