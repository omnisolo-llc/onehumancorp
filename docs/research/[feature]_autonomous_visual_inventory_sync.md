# Title: Autonomous Visual Inventory Sync

## Problem Statement
Small business owners, particularly boutique owners (like Priya) and offline vendors, spend countless hours manually entering inventory data into systems like Shopify. It requires typing descriptions, setting prices, taking photos, and updating stock levels. This friction stops them from quickly listing new items, resulting in stale storefronts and lost sales. They need a system where taking a simple photo from their phone does all the work.

## Research Report
Our deep-dive audit of Shopify and legacy POS systems (like Square) highlights "time to list" as a massive point of friction. Trustpilot reviews consistently feature complaints from non-technical users about variant configuration and UI complexity. Emerging AI competitors (like Dora AI) focus on site generation, but ignore operational data entry. OHC currently lacks an autonomous bridge between physical goods and the digital ledger.

References:
- Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
- Wix App Market Complaints: https://www.reddit.com/r/ecommerce/search.json?q=wix+booking+issues
- OHC Current Gap: Missing direct visual ingestion pipelines.

## Design Doc
**Architecture:**
- **Entry Point:** A new native camera module in the OHC Mobile App.
- **Vision Agent:** Triggers when a photo is submitted. It uses a multimodal LLM to extract object features (type, color, material).
- **Pricing & Description Engine:** Queries live market data and historical shop context to draft SEO-optimized descriptions and suggest retail pricing.
- **Ledger Integration:** Integrates seamlessly with OHC's `Universal Capacity and Inventory Ledger` (as defined in existing architecture).

**UI/UX Flow (Mobile First - 375px):**
1. User taps "Add Item" -> Camera opens.
2. User snaps photo of a dress on a hanger.
3. Loading skeleton (Glassmorphism, 20px blur).
4. A bottom sheet slides up presenting a pre-filled card: "Summer Silk Dress - Red - Suggested Price: $45.00".
5. User taps "Approve & List". Done.

## Implementation Prompt
Implement the Autonomous Visual Inventory feature for the OHC mobile client. When a user uploads or takes a photo of a product, route the image through the Vision Agent. The agent should return a structured proposal containing a generated title, description, suggested price, and inferred tags. Present this to the user in a high-delight, easily editable bottom sheet. Upon approval, automatically commit the new item to the inventory ledger and publish it to the storefront without requiring any manual typing from the user.

## Priority
P1

## Estimated Scope
Medium
