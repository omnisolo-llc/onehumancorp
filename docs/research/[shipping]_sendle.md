# Native Integration of Sendle for Sustainable Shipping

## Title
Native Integration of Sendle for Sustainable Shipping

## Problem Statement
Small business owners selling physical goods struggle with calculating accurate shipping rates and printing labels efficiently. They want a simple, affordable shipping solution that aligns with their values (e.g., carbon-neutral) without needing a separate logistics platform.

## Research Report
- **Strategy**: Direct integration with Sendle's API for rate calculation and label generation.
- **Target Persona**: Boutique owners, artisans, and small e-commerce merchants primarily in the US, Australia, and Canada.
- **Advantages**: Sendle focuses exclusively on small businesses, offering competitive flat rates, free pickup, and 100% carbon-neutral shipping. This strongly resonates with modern small business owners.
- **Risks**: Sendle's carrier network might not be as expansive globally as major aggregators. Delivery times can sometimes be slightly longer than premium couriers.
- **Pricing**: No subscription fees. Pay per label based on size and destination.
- **Compatibility**: Compatible with both Cloud mode (via centralized account or OAuth) and Standalone mode (via user API key).

## Design Doc
- User goes to Settings > Logistics and connects their Sendle account.
- When configuring physical products in OHC, the user inputs basic dimensions/weight.
- At checkout, OHC queries the Sendle API to provide accurate, real-time shipping quotes to the customer.
- After an order is paid, the user sees a "Fulfill with Sendle" button in the OHC Operations dashboard.
- Clicking the button generates a PDF label natively, books a driver pickup via Sendle, and automatically emails the tracking link to the customer.
- **AI Integration**: The Operations Agent monitors tracking status and alerts the merchant if a package is delayed, drafting a proactive apology email to the customer.

## Implementation Prompt
Integrate Sendle to handle real-time shipping rate calculations during checkout and label generation in the fulfillment dashboard. The system must allow merchants to easily purchase and print Sendle labels directly from OHC and automatically sync tracking information to the customer's order.
- **Acceptance Criteria**: Checkout displays real-time Sendle rates. Merchant can click to purchase and print a Sendle label. Tracking number is saved to the order and emailed to the customer.
- **Priority**: P2
- **Estimated Scope**: Large
