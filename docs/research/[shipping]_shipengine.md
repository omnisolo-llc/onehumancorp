# Title: Automate Shipping Rates and Labels with ShipEngine

## Problem Statement
Small business owners selling physical goods struggle with shipping. They manually guess shipping costs at checkout, often losing money, and then have to manually copy-paste customer addresses into carrier websites to print labels. They need a way to charge the exact right amount for shipping and print the label with one click.

## Research Report
ShipEngine (by Stamps.com) is a powerful multi-carrier shipping API.
- **Ease of Use**: While the API is for developers, it allows OHC to build a magical, one-click experience for the user. They don't need to negotiate with carriers; they get discounted rates out of the box.
- **Pricing**: Generally pay-as-you-go based on label volume, with very low per-label fees. This is ideal for small businesses without huge predictable volumes.
- **Reputation**: Highly reliable, powering major e-commerce platforms. Excellent support for USPS, UPS, FedEx, and international carriers.
- **Comparison**: Shippo and EasyPost are very similar. ShipEngine often has a slight edge in raw discounted USPS rates out-of-the-box, which is crucial for US-based small sellers.
- **Cloud vs Standalone**: Generating labels and fetching rates relies entirely on outbound API requests, meaning this integration works perfectly in both Cloud and Standalone modes without any network modifications.

## Design Doc
- **Triggers & Actions**: At checkout on an OHC storefront, ShipEngine calculates the live shipping rate based on the cart weight and customer address. When the owner wants to fulfill the order, they click "Buy Label" in OHC, which generates a PDF via ShipEngine and provides a tracking number.
- **User Experience**: On the order details page in OHC, there is a prominent "Print Shipping Label" button. Clicking it shows the cost, confirms the package size, and immediately downloads a PDF label to print. The customer automatically gets an email with the tracking link.

## Implementation Prompt
Integrate a shipping label generation and rate calculation system.
- **User-Facing Outcome**: The business owner can click a single "Print Label" button on an order to get a ready-to-print shipping label and automatically send tracking info to the customer.
- **Acceptance Criteria**:
  - The storefront provides real-time shipping cost estimates during checkout based on destination.
  - The business owner can generate and download a shipping label PDF from the order details screen.
  - Generating a label automatically attaches a tracking number to the order.

## Priority
P2

## Estimated Scope
Large
