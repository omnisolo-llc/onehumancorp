# [Shipping] Shippo Integration for Automated Multi-Carrier Shipping

## Problem Statement
Small business owners, like Maya who runs a local boutique, spend hours each week manually copying customer addresses into carrier portals (USPS, UPS, FedEx) to calculate shipping rates and print labels. This manual process is error-prone, time-consuming, and prevents them from quickly fulfilling orders during busy periods. Furthermore, comparing rates across carriers to find the cheapest option is tedious, and keeping customers updated with tracking information requires even more manual data entry.

## Research Report
- **Overview**: Shippo is a highly popular, multi-carrier shipping API and web application designed specifically for e-commerce and small businesses.
- **Ease of Use**: For non-technical users, Shippo is highly accessible. If integrated well, business owners don't even need to use the Shippo dashboard; they can manage everything from within their OHC admin panel. Shippo provides default discounted rates (e.g., USPS Commercial Pricing) out of the box without requiring the user to negotiate their own carrier contracts.
- **Pricing**: Shippo is exceptionally friendly for small businesses, offering a "Starter" pay-as-you-go plan with no monthly fees (users just pay a few cents per label plus postage). This zero-fixed-cost entry point is ideal for our standalone and early-stage cloud users.
- **Reputation**: Well-regarded in the e-commerce space, used by major platforms, and known for reliable APIs and solid documentation.
- **SaaS Viability**: Shippo operates seamlessly via REST APIs and Webhooks. It supports OAuth for connecting user accounts (great for multi-tenant Cloud) and allows API key usage (perfect for Standalone/local instances).

## Design Doc
- **Integration Flow**:
  - **Trigger**: When an order is placed and its payment is confirmed in OHC.
  - **Action**: OHC queries Shippo for shipping rates based on the cart's weight/dimensions and the customer's address. The business owner selects a rate and generates a label. OHC downloads the PDF label for the owner to print.
  - **Webhook**: Shippo sends webhook events to OHC when the package is picked up, in transit, and delivered.
- **User Interface**:
  - A "Shipping" configuration page where the owner connects their Shippo account.
  - On the Order Details page, a "Fulfill Order" section showing available rates, a "Buy Label" button, and a link to download the generated PDF.

## Implementation Prompt
- **User-Facing Outcome**: A small business owner should be able to click a single button within their OHC order dashboard to buy and print a shipping label, automatically charging their connected payment method and securing the best available shipping rate.
- **Acceptance Criteria**:
  1. User can authorize a Shippo account from the OHC settings.
  2. For a paid order, the user can input/confirm package weight and dimensions.
  3. The system displays at least three shipping rate options (e.g., cheapest, fastest).
  4. The user can purchase the label, which updates the order status to "Shipped".
  5. The system stores the tracking number and provides a printable PDF link for the label.

## Priority
P1 (high)

## Estimated Scope
Medium
