# [Integration] Printful / Print-on-Demand Integration

## Title
Zero-Touch Print-on-Demand & Merchandising Engine

## Problem Statement
Small business owners, creative professionals, and influencers (like Leo the Music Tutor) want to monetize their brand by selling merchandise (t-shirts, mugs, posters) to their audience. However, they cannot afford upfront inventory costs, lack warehouse space, and do not have the time to manually pack and ship orders. Existing solutions like Shopify require complex setup, installing third-party apps, manually syncing products, and dealing with fulfillment errors. OHC users need an invisible, zero-touch merchandise engine where they simply upload a design (or have the AI generate one), and everything else—printing, packing, and shipping—is handled automatically without them ever lifting a finger.

## Research Report
- **Strategy**: Deep, native API integration with Printful (or Printify) as a white-labeled fulfillment partner.
- **Target Persona**: Leo (The Music Tutor), Priya (The Boutique Owner), Creative Portfolios, Influencers.
- **Advantages**:
  - **Zero Inventory Risk:** Products are printed only when a customer buys them.
  - **Zero Operations:** Fulfillment, packing, and global shipping are fully handled by the partner.
  - **High Margins:** Users set their own retail price above the base cost.
  - **Brand Continuity:** The white-label dropshipping model means packages arrive with the user's branding, not Printful's.
- **Competitor Landscape**: Shopify relies heavily on Printful/Printify apps. Squarespace has limited native integrations. By building this deeply into OHC, we remove the "app installation" friction entirely. The user just sees a "Merch" tab.
- **Evaluation of Tool (Printful API)**:
  - Printful offers a robust REST API for catalog browsing, mockup generation, order creation, and shipping rate calculation.
  - Supports both Cloud (multi-tenant) and can be accessed via OHC's backend.
  - Generous developer ecosystem and reliable SLA.

## Design Doc
- **Integration with OHC**:
    - **Catalog Sync**: OHC backend pulls the Printful product catalog (t-shirts, hoodies, mugs) and base costs.
    - **Design & Mockup Flow**:
      - The user accesses the "Merchandising" tab in the OHC app.
      - The user uploads a logo/design or asks the "Marketing & Advertising" AI Agent to generate a design.
      - OHC calls Printful's Mockup Generator API to instantly create realistic product photos.
      - The item is immediately listed on the user's OHC storefront.
    - **Order Fulfillment Loop**:
      - A customer buys the t-shirt on the OHC storefront.
      - OHC processes the payment (Stripe).
      - The OHC "Operations" Agent automatically pushes the order details (shipping address, design file, product ID) to the Printful API.
      - Printful prints and ships the item. Webhooks update the order status in OHC, triggering the "Customer Success" Agent to send the customer a tracking link.
- **User View**: A magical "Create Merch" button. They select a product, upload art, and it's live. No inventory screens, no shipping label printers needed.

## Implementation Prompt
Build a native integration with the Printful API. Implement the required data models for `PrintOnDemandProduct`, `Mockup`, and `FulfillmentOrder`.
- Create an API client to fetch the Printful catalog and generate mockups.
- Implement the order routing logic so that when a customer purchases a PrintOnDemand product, the order is automatically submitted to Printful for fulfillment.
- Handle Printful webhooks to update shipping and tracking status in the OHC unified order manager.
- Ensure the AI Agents (Operations, Customer Success) can interact with these endpoints to fully automate the dropshipping lifecycle.

## Priority
P1

## Estimated Scope
Large
