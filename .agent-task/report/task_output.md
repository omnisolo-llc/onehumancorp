issue_title: "Implement Autonomous Dynamic Product Variant & Matrix Inventory Engine"
issue_description: |
  # Autonomous Dynamic Product Variant & Matrix Inventory Engine

  **Problem Statement:**
  Currently, OneHumanCorp (OHC) enables business owners to create single, monolithic products. However, our persona **Priya the Boutique Owner** requires product variants (e.g., a "Summer Dress" in Size S/M/L and Colors Red/Blue). Without a robust Matrix Inventory Engine, she is forced to create separate product listings for every combination, cluttering the storefront, confusing customers, and making inventory tracking across channels nearly impossible.

  **Research Report:**
  - **Shopify:** Provides robust variant support (up to 100 variants and 3 options per product) but relies on complex manual configuration.
  - **Wix / Squarespace:** Offers basic variant handling but lacks intelligent auto-SKU generation.
  - **OHC Opportunity:** Treat variants not as a static data structure, but as an active matrix managed by the Operations Agent. The AI should auto-generate variant combinations, detect when specific sizes/colors are trending, and auto-sync inventory.

  **Design Doc:**
  We decouple the core "Product" from its sellable "Variants". A product is a logical container; a variant is the physical/digital entity that holds inventory and price modifiers.

  *Mobile UX Flow (375px First)*
  1. Creation: User snaps a photo of a dress. AutoDream AI suggests adding sizes/colors.
  2. One-Tap Variants: User taps "Add Sizes" -> selects S/M/L. The AI auto-generates child SKUs and sets default inventory.
  3. Variant Grid: A clean, horizontal scroll or compressed list view shows variants.

  *Architecture*
  ```mermaid
  erDiagram
      PRODUCTS {
          uuid id PK
          string tenant_id FK
          string title
          string description
          string type
      }
      PRODUCT_VARIANTS {
          uuid id PK
          string tenant_id FK
          string product_id FK
          string name
          string sku
          string price_modifier
          int inventory_count
      }
      PRODUCTS ||--o{ PRODUCT_VARIANTS : has
  ```

  *AI Agent Integration Points*
  - Operations Agent: Auto-generates variant combinations and SKUs. Monitors inventory levels and triggers low-stock alerts.

  **Implementation Prompt:**
  Implement the product variants feature in the backend API.
  - Expand product creation API to accept an optional array of variants.
  - Create the `product_variants` database table.
  - Transactionally insert the parent product and all its child variants in the backend Go handlers.
  - Provide a PostgreSQL database migration file to create the schema.

  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
