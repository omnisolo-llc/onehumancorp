<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [Frontend] Mobile-First Storefront & Catalog Editor

## Problem Statement
Non-technical small business owners like Maya (baker) and Priya (boutique owner) struggle with complex product management interfaces in platforms like Shopify. They need a way to manage their "Catalog" entirely from a phone, with AI doing the heavy lifting of writing descriptions, organizing categories, and optimizing images. Current OHC implementation lacks any product/catalog management UI.

## Research Report
- **Shopify:** Mobile app is secondary; complex variant management is painful on small screens. Many features (like Shopify Magic) are not yet optimized for mobile ([Source](https://help.shopify.com/en/manual/shopify-admin/productivity-tools/shopify-magic)).
- **Wix:** AI website builder is strong, but catalog management remains a traditional form-heavy experience requiring manual entry ([Source](https://www.wix.com/ai)).
- **Durable:** Generates a site but lacks deep inventory/variant management for retail ([Source](https://durable.co/)).
- **OHC Opportunity:** A "Catalog Feed" experience where the user just snaps a photo, and the Marketing (Promoter) department auto-generates the product page, tags, and SEO metadata.

### User Journey Comparison
```mermaid
journey
    title Maya's Photo-to-Live Journey
    section Capture
      Snap photo: 5: Maya
      AI Vision extracts features: 3: Promoter Agent
    section Refine
      Draft Review: 4: Maya
      Auto-SEO/Tags: 2: Promoter Agent
    section Publish
      Live on Store: 5: Maya
```

## Design Doc
- **Entity Types:** `Product`, `Category`, `Variant`, `Inventory`.
- **UI Flow (375px):**
  1. **Quick-Add:** Floating action button (FAB) triggers camera.
  2. **AI Processing:** Image uploaded → Vision AI extracts features → Promoter agent drafts name/description/price.
  3. **Review Card:** Glassmorphic card showing the draft. User taps "Publish" or "Edit".
  4. **Catalog Grid:** Visual grid with "Sold Out" toggles accessible via thumb-reach.

## Implementation Prompt
**Outcome:** Implement a mobile-first Catalog Editor.
**CUJ:** Maya takes a photo of a new "Vegan Chocolate Cake". The AI automatically populates the product details. Maya approves, and it immediately appears on her live storefront.
**Acceptance Criteria:**
- 375px optimized layout with touch targets ≥ 44px.
- Integration with Marketing department for auto-description.
- Optimistic UI updates for "Sold Out" toggles.

## Priority
P0

## Estimated Scope
Medium

</div>
