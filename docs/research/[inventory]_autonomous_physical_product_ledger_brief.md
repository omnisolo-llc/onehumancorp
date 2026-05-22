# [inventory]_autonomous_physical_product_ledger

## Title
Autonomous Physical Product Ledger: AI-Driven Inventory Management

## Problem Statement
Current AI builders like Durable focus almost exclusively on service-based businesses (landscaping, coaching). For product-based SMBs like Priya (boutique owner) or Maya (baker), managing inventory is a nightmare. Products are sold in-store, online, and via DMs. When a product is sold out, they often forget to update their website, leading to overselling and angry customers. They need a system that tracks inventory autonomously.

## Research Report
Our audit of Durable and 10Web revealed a massive gap: physical product management.
- **Finding 1**: Durable's onboarding flow explicitly asks for "Services" and struggles to generate structured product catalogs.
- **Finding 2**: Traditional platforms like Shopify require tedious manual data entry for every product variant (SKUs, weights, barcodes).
- **Finding 3**: SMB owners do not have the time to sit at a desktop and manage databases. They operate from their phones on the shop floor.

## Design Doc
**Architecture High-Level:**
- **Entities**: `Product`, `Variant`, `InventoryLevel`, `Location` (In-Store, Warehouse).
- **Key Relationships**: A `Product` has many `Variant`s. Each `Variant` has an `InventoryLevel` per `Location`.
- **Integration Points**: POS systems (Square), Unified Inbox (for DM sales).
- **AI Agent Integration**: The `CatalogAgent` allows users to simply snap a photo of a new product. The agent uses computer vision and LLMs to identify the item, write a description, suggest a price, and create the database entry. The `InventoryAgent` monitors sales across all channels (Web, POS, DM) and automatically decrements the `InventoryLevel`.

**Mobile UX Flow (375px first):**
1. The user taps the "+" button in the Inventory tab and selects "Camera."
2. The user takes a picture of a new floral dress.
3. The AI processes the image and presents a draft: Title ("Summer Floral Maxi Dress"), Description, Price ($45), and asks for Initial Quantity.
4. The user inputs "10" and taps "Publish." The product is now live on the storefront and POS.
5. When a customer buys it via the storefront, the inventory drops to 9 globally.

## Implementation Prompt
Implement the Autonomous Physical Product Ledger.
**User-Facing Outcome**: The user can create new products entirely via the mobile camera and natural language, without navigating complex forms. Inventory is synced globally across all sales channels.
**Critical User Journey**:
1. User takes a photo of a product.
2. AI generates title, description, and suggested price.
3. User confirms and the product is added to the active catalog.
4. An order is placed, and the system autonomously reduces the available stock.
**Acceptance Criteria**:
- Product creation via image input (simulated).
- Global inventory ledger that decrements upon an order event.
- Mobile-optimized interface for reviewing AI-drafted products.

## Priority
P1

## Estimated Scope
Medium