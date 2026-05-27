issue_title: "[Architecture] Autonomous Dynamic Bundling & Upsell Engine"
issue_description: |
  # Issue Brief: Autonomous Dynamic Bundling & Upsell Engine

  ## Problem Statement
  Small business owners (like Priya the boutique owner or Maya the baker) frequently miss out on increased Average Order Value (AOV) because creating, pricing, and managing product bundles or post-purchase upsells is too complex. They have to manually select items, calculate discounts, track inventory for each component, and write promotional copy. Customers checking out often abandon carts if presented with confusing pop-ups, but are highly receptive to personalized, one-click additions that make sense. OHC currently lacks an invisible engine that automatically identifies optimal bundling opportunities based on cart contents, purchase history, and inventory levels, and presents them seamlessly to the buyer.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify**: Bundling requires installing third-party apps, which often conflict with theme code, slow down page loads, and require complex configuration. Upsell popups are often obtrusive.
    - **Wix / Squarespace**: Offer very limited manual "related products" widgets. No dynamic pricing or inventory-aware bundling.
    - **OHC Advantage**: By leveraging internal AI Operations and Sales Agents, OHC can analyze the merchant's catalog and automatically suggest logical bundles. The AI can dynamically generate 1-tap post-checkout upsells that are context-aware, abstracting all inventory and pricing math away from the merchant.
  - **Key Findings**:
    - Post-purchase 1-click upsells convert at a significantly higher rate than pre-checkout pop-ups.
    - Bundling increases AOV by 15-30%, but 80% of small merchants don't use it because of inventory management headaches.

  ## Design Doc
  ### Key Design Decisions
  1. **Zero-Config Activation**: The system defaults to active. The AI Agents analyze the existing catalog and inventory to generate offers on the fly.
  2. **Margin Protection**: The pricing engine strictly enforces a minimum margin threshold.
  3. **1-Tap Atomic Cart Updates**: Upsells are presented as clean, native UI cards. A single tap adds the item and applies the bundle discount atomically.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CATALOG_ITEM : "owns"
      CATALOG_ITEM ||--o{ INVENTORY_LEDGER : "tracked_by"
      CART ||--o{ CART_ITEM : "contains"
      CART ||--o| DYNAMIC_BUNDLE_OFFER : "receives"
      DYNAMIC_BUNDLE_OFFER {
          uuid id
          uuid trigger_item_id
          uuid upsell_item_id
          float proposed_discount
          string ai_generated_pitch
      }
      DYNAMIC_BUNDLE_OFFER ||--o| CHECKOUT_SESSION : "converted_in"
  ```

  ### Mobile UX Flow (375px First)
  1. **Buyer View**: A clean, translucent glass card sits just below the "Checkout" button. A single "Add to Cart" button instantly updates the total without a page reload.
  2. **Merchant View**: Priya receives a push notification: "Your AI Sales Agent just bundled 3 items for a $45 sale! 🚀". No configuration was needed on her part.

  ### AI Agent Integration Points
  - **AI Sales Agent**: Monitors cart composition and infers buyer intent to select the highest-converting upsell item. It also generates the short, localized pitch text.
  - **AI Operations Agent**: Serves as the guardian of inventory and margins.

  ## Implementation Prompt
  Implement the Autonomous Dynamic Bundling & Upsell Engine.
  - **Outcome:** The checkout system must be capable of requesting and displaying dynamically generated, 1-tap upsell or bundle offers based on cart contents. Merchants should not have to manually configure these bundles.
  - **CUJ:** A buyer adds a custom cake to their cart. The engine verifies inventory and proposes adding a set of candles for 20% off. The buyer taps "Add", the cart total instantly recalculates, and the inventory ledger atomically reserves both.
  - **Acceptance Criteria:**
    - Offer generation completes in < 100ms.
    - Inventory availability is strictly enforced.
    - Dynamic price calculation respects tenant's margin threshold.
    - Multi-tenant data isolation ensures data privacy.
    - UX must be 100% mobile-first (Translucent Glass, 44px touch targets).

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
