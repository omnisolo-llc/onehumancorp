# Title: Automatic Shipping Rates and Label Printing
## Problem Statement
E-commerce businesses spend hours manually copying addresses to carrier websites, guessing shipping costs, and printing labels one by one.

## Research Report
Shippo or EasyPost provide aggregated APIs for dozens of carriers (USPS, UPS, FedEx, local carriers).
- **Ease of Use**: The business owner connects their Shippo/EasyPost account. OHC handles the rest.
- **Pricing**: Pay-as-you-go per label (usually a few cents + carrier cost).
- **Reputation**: Both are industry standards for multi-carrier shipping aggregation.

## Design Doc
- **Trigger**: During customer checkout, OHC queries the shipping API for live rates. After purchase, the business owner views the order in OHC.
- **Action**: OHC displays a "Buy Shipping Label" button. Clicking it purchases the label via the API and downloads a PDF.
- **User View**: Live shipping rates at checkout for the customer. A simple one-click "Print Label" button for the business owner on the order details page.

## Implementation Prompt
Integrate Shippo (or EasyPost) to provide real-time shipping rates during the checkout process based on product weight and customer address. In the OHC order management dashboard, add a feature to purchase and download shipping labels directly. Automatically update the order status to "Shipped" and email the tracking number to the customer once a label is generated.

## Priority
P1

## Estimated Scope
Large

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported via the OHC cloud backend.
- **Standalone Mode**: Fully supported as these are synchronous outbound API requests to generate labels.
- **Risks**: Address validation failures, label printing errors, and unexpected carrier surcharges.
