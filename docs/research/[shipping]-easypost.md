# Title: Integrate EasyPost for Automated Shipping Labels and Tracking

## Problem Statement
Small e-commerce businesses waste hours manually calculating shipping rates at the post office and copying tracking numbers into emails. They need a system to automatically buy discounted labels and share tracking info directly from their dashboard.

## Research Report
EasyPost provides a multi-carrier shipping API (USPS, FedEx, UPS, DHL).
- **Ease of use:** High. Once configured, business owners can click a single button to generate a label.
- **Pricing:** Free up to 120,000 shipments per year; access to commercial USPS pricing.
- **Reputation:** Extremely reliable, widely used by major e-commerce platforms.
- **Cloud/Standalone:** Cloud API. Can work in standalone if the user provides their own EasyPost API keys.

## Design Doc
- **Trigger:** Business owner marks an order as "Ready to Ship" and enters package dimensions/weight.
- **Action:** OHC fetches carrier rates via EasyPost, allows user to select the cheapest/fastest option, purchases the label, and saves the tracking URL.
- **User Interface:** An "Order Fulfillment" module. Shows a printable PDF of the label and a button to email the tracking link to the customer.

## Implementation Prompt
Build an order fulfillment flow. Allow the business owner to enter package weight and dimensions for an order. Connect to EasyPost to display shipping rates from USPS and UPS. Provide a "Buy Label" button that purchases the selected rate, downloads a printable PDF of the shipping label, and automatically emails the tracking link to the customer.

## Priority
P2

## Estimated Scope
Large
