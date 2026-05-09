# Scout 🔍: Integrate Sendle for Simple and Green Shipping

## Problem Statement
Priya (Boutique Owner) finds traditional shipping rates and "zones" confusing. She also wants to align her business with her values of sustainability. She needs a straightforward way to ship her products that is easy to understand, has flat rates, and is good for the planet.

## Research Report
- **Tool**: Sendle
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker), Eco-conscious SMBs.
- **Evaluation**: Sendle is a 100% carbon-neutral shipping service specifically designed for small businesses. It offers simple, flat-rate pricing based on weight and size rather than complex postal zones.
- **Ease of Use**: High. They focus on making shipping as easy as possible for non-experts.
- **Pricing**: Flat rates. No hidden fees or complex fuel surcharges. Competitive with major carriers for small to medium parcels.
- **Reputation**: High. Well-loved by small Etsy/Shopify sellers for its simplicity and green credentials.
- **Cloud vs. Standalone**: Compatible with both via API.

## Design Doc
- **Checkout Integration**: OHC calculates Sendle flat rates during checkout based on the product's weight class.
- **One-Click Fulfillment**: Label generation happens with one click in the OHC "Operations" dashboard.
- **Tracking**: Tracking numbers are automatically pulled from Sendle and shared with the customer.
- **Brand Value**: The "Carbon Neutral Shipping" badge is prominently displayed to the customer during checkout.

## Implementation Prompt
Integrate the Sendle API for shipping rate calculation and label generation. Implement a simple "Ready to Ship" workflow where labels are generated and tracking is updated automatically. Highlight the carbon-neutral aspect in the checkout UI.
- **Acceptance Criteria**: Merchant can calculate Sendle rates. Merchant can print labels from OHC. Tracking info is automatically sent to the customer.
- **Priority**: P2
- **Estimated Scope**: Medium
