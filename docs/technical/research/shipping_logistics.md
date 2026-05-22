# [Shipping & Logistics] Automated Shipping Rates and Labels

## Problem Statement
Sellers of physical products need to charge customers the correct amount for shipping and easily print shipping labels. Calculating this manually is error-prone, and going to the post office to buy labels is time-consuming. They need shipping costs calculated automatically at checkout and one-click label printing.

## Research Report
- **Target Tools**: Shippo API or EasyPost API.
- **Competitive Analysis**: Shopify has robust native shipping. OHC needs a simple, comparable alternative without the complexity of carrier negotiation.
- **Ease of Use**: Shippo/EasyPost abstract multiple carriers (USPS, FedEx, UPS) into a single API. OHC users don't need their own carrier accounts.
- **Pricing**: Pay-as-you-go per label (cents per label + postage). Can be passed to the user or absorbed in OHC premium tiers.
- **Reputation**: Highly reliable APIs used by many e-commerce platforms.
- **Advantages and Risks**: Massive time-saver for physical product sellers. Risk involves miscalculating package weights resulting in undercharging for shipping.
- **Cloud vs Standalone**: Cloud integrates directly. Standalone could work if the API calls are client-side, but might still rely on a Cloud proxy to handle billing.

## Design Doc
- **Integration Flow**: In "Operations", the user sets up their shipping origin address and default box sizes.
- **Actions**: During customer checkout, the system calls the API to get real-time rates based on the cart contents and destination. Post-purchase, the user can click "Generate Label" to get a printable PDF and automatically email tracking info to the customer.
- **User Experience**: A "Print Shipping Label" button appears on physical product orders. The system handles the payment for the postage in the background (deducted from their payout or charged to their card on file).

## Implementation Prompt
Integrate a shipping aggregation API (like EasyPost or Shippo) to provide real-time shipping rate calculation at checkout for physical products. Additionally, build a feature allowing the business owner to generate and print shipping labels directly from the order details screen in the OHC app. Ensure tracking numbers are automatically generated and attached to the order.

## Priority
P1

## Estimated Scope
Large
