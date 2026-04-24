# Scout: Social Media Integration (Pinterest)

## Title
Visual Discovery & Social Commerce 🎨 (Pinterest API Integration)

## Problem Statement
Small business owners like Maya (the baker) and Priya (the boutique owner) rely on visual discovery to attract customers. Pinterest is a primary destination for users looking for inspiration for weddings, home repairs, and fashion. Currently, OHC users must manually pin their products, missing out on automatic "Buyable Pins" and real-time catalog synchronization. A direct integration is needed to turn inspiration into instant sales.

## Research Report
- **Goal**: Evaluate Pinterest API as a visual sales channel for OHC's Marketing and Sales departments.
- **Features evaluated**:
  - **Catalog Sync**: Automatic upload and update of product feeds.
  - **Pinterest API for Business**: programmatic Pin creation and analytics.
  - **Enhanced Match**: Improved conversion tracking for ads.
- **Benefits for OHC users (Non-technical)**:
  - Products from the OHC storefront automatically appear as "Product Pins" with price and availability.
  - One-click "Pin this" for new products created in OHC.
  - Access to a high-intent audience looking for purchase inspiration.
- **Integration Risks**:
  - Pinterest API v5 has strict app review processes.
  - Image optimization (aspect ratios) is critical for Pinterest performance.
- **Pricing**: API access is free for developers. Business accounts are free; costs are only incurred if running Pinterest Ads.
- **Cloud vs Standalone**: Native support for Cloud mode. Standalone mode can trigger Pin creation via the user's Pinterest account using the Hybrid MCP tunnel to bridge local product data to the Pinterest API.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via Pinterest Integration |
|---------|------------|-----------------------------------|
| **Maya (Baker)** | Her beautiful cake designs are only on Instagram. | Automatically pin new cake designs to "Wedding Inspiration" boards, driving traffic back to her OHC storefront. |
| **Priya (Boutique)**| Hard to reach customers outside her local area. | Product Pins make her clothing items discoverable globally by users searching for specific styles. |

## Design Doc
- **Component**: `PinterestIntegrationService`
- **Responsibilities**:
  - Handle OAuth2 flow for Pinterest Business accounts.
  - Map OHC Product entities to Pinterest Product Pins.
  - Scheduled synchronization of inventory levels to prevent "sold out" items from being featured.
  - Provide analytics back to the OHC Business Advisor department.
- **User Experience**:
  - A "Connect Pinterest" button in the Marketing department settings.
  - Simple checkbox: "Auto-pin new products to [Board Name]".

## Implementation Prompt
"Implement Pinterest API v5 integration in `src/server/integrations/pinterest/`. Create a service that manages OAuth2 authentication and provides a synchronization worker to push products from the OHC catalog to Pinterest Catalogs. Ensure the integration supports both manual Pinning of individual items and automated catalog-wide sync. Acceptance criteria: A user can link their account and see their OHC products appearing as Product Pins on their Pinterest boards."

## Priority
P2

## Estimated Scope
Medium
