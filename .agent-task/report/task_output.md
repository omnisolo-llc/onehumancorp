issue_title: "Architectural Design: Autonomous Dynamic Bundle & Upsell Engine"
issue_description: |
  # [architecture] Autonomous Dynamic Bundle & Upsell Engine

  ## Problem Statement
  Small business owners, such as Maya (custom cakes) and Priya (boutique owner), often leave significant revenue on the table because they lack the time and expertise to configure complex cross-sell, up-sell, or bundling rules (e.g., "People who buy a vegan cake often buy a customized topper"). Legacy platforms require merchants to manually set up explicit rules, which creates operational fatigue and requires constant updating as inventory changes. They need an intelligent, invisible engine that analyzes cart contents and historical data, then dynamically proposes high-converting bundles and 1-tap upsells at checkout, without requiring any manual configuration.

  ## Research Report
  *   **Shopify:** Requires third-party apps for robust bundling and post-purchase upsells. These apps often slow down checkout, conflict with themes, and require complex rule management (e.g., defining specific SKU combinations).
  *   **Wix:** Basic cross-selling features exist, but they are static ("You might also like..."). They do not offer dynamic, AI-driven bundle generation with adjusted pricing at checkout.
  *   **Square/Squarespace:** Similar limitations; upselling is mostly manual configuration.
  *   **OneHumanCorp (OHC) Differentiation - "Zero-Touch Autonomy":** OHC’s Operations and Sales agents monitor catalog and transaction history in the background. When a customer adds an item to their cart or books a service, the engine dynamically generates a hyper-relevant, slightly discounted bundle or upsell offer. This is presented natively during checkout as a single-tap addition, requiring zero merchant configuration and increasing Average Order Value (AOV) invisibly.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Mobile User (375px)
      participant Edge Gateway
      participant Event Mesh
      participant SalesAgent as Sales & Quoting AI
      participant InventoryMesh as Operations AI
      participant PricingEngine as Billing & Quoting Engine

      Customer->>Edge Gateway: Adds "Vegan Chocolate Cake" to Cart
      Edge Gateway->>Event Mesh: Publish `cart.item_added`
      Event Mesh->>SalesAgent: Trigger Upsell Evaluation
      SalesAgent->>InventoryMesh: Query related items in stock (e.g., Candles, Topper)
      InventoryMesh-->>SalesAgent: Returns "Gold Candles (Qty: 12)"
      SalesAgent->>PricingEngine: Request dynamic bundle discount (e.g., 10% off candles)
      PricingEngine-->>SalesAgent: Returns adjusted bundle price
      SalesAgent->>Edge Gateway: Injects dynamic "1-Tap Bundle" offer into Cart State
      Edge Gateway-->>Customer: UI displays "Complete the celebration? Add Gold Candles for $4 (Normally $5)"
      Customer->>Edge Gateway: Taps "Add to Cart"
      Edge Gateway->>Event Mesh: Publish `cart.bundle_accepted`
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). Integrated directly into the native checkout drawer.
  *   **Cart Drawer View:**
      *   Standard cart items listed at the top.
      *   **The Magic Upsell Card:** A distinct card with a subtle iridescent or yellow-tinted glass background (`rgba(255, 220, 100, 0.1)`) to stand out gently.
      *   **Content:** Small thumbnail of the upsell item, a conversational plain-language prompt ("Need candles for the cake?"), and the dynamic price comparison ("+$4, save $1").
      *   **Interaction:** A single, prominent `[ + Add ]` button. No page reloads. The cart total updates optimistically.

  ### Mobile UX Flow
  1. **Add to Cart:** A customer on Maya's OHC storefront adds a cake to their cart.
  2. **Dynamic Injection:** The cart slides up. Instantly, an AI-generated upsell for "Gold Candles" appears at the bottom of the cart.
  3. **1-Tap Action:** The customer taps "Add". The item is added, the total updates, and the button changes to a satisfying green checkmark.
  4. **Merchant View:** In the background, Maya receives a daily briefing later: "✨ Your AI added $45 in upsell revenue today from candles." She did nothing to set this up.

  ### AI Agent Integration Points
  *   **Sales & Quoting Agent:** Responsible for the semantic analysis of the cart (What is the customer trying to achieve? A birthday celebration? A plumbing repair?) and selecting the best complementary item.
  *   **Operations Agent:** Provides the Sales Agent with real-time inventory levels to ensure we only upsell items that are actually in stock and ready to ship/deliver.
  *   **Finance & Pricing Agent:** Calculates a safe, profitable margin for the bundle discount to incentivize the addition without hurting the merchant's bottom line.

  ### Key Design Decisions (Why, not How)
  *   **Dynamic vs. Static Rules:** By utilizing AI to determine relationships between items based on description and past purchase behavior, we eliminate the need for the merchant to maintain a complex graph of "If X, then suggest Y" rules.
  *   **Optimistic UI at Checkout:** The upsell must feel instantaneous. The AI prediction happens asynchronously when the item is added to the cart, so the upsell card is pre-loaded and rendered immediately when the cart drawer opens.
  *   **Zero-Trust Isolation:** Pricing calculations and inventory checks must strictly adhere to the specific merchant's `tenant_id` boundaries. The AI cannot accidentally suggest another merchant's product.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the "Autonomous Dynamic Bundle & Upsell Engine" that intercepts cart additions and natively suggests 1-tap upsells.

  **Customer User Journey (CUJ):**
  1. A customer adds a primary product or service to their cart.
  2. The AI Sales Agent evaluates the cart and inventory, generating a contextually relevant upsell offer with a slight dynamic discount.
  3. The customer views their cart and sees a "1-Tap Add" card for the upsell.
  4. The customer taps "Add," and the cart is updated seamlessly.

  **Acceptance Criteria:**
  *   **Zero Configuration:** The system must work without any explicit linking or configuration by the merchant. The AI must infer relationships.
  *   **Mobile Parity:** The UI must be implemented perfectly for a 375px viewport using Translucent Glass aesthetics inside the cart drawer.
  *   **Performance:** The upsell injection must not block the cart rendering. It should appear instantly or load gracefully in the background.
  *   **Isolation:** Strict multi-tenant isolation must be enforced during inventory and pricing checks.
  *   **Integration:** Must trigger the event mesh to log the upsell acceptance for later analysis by the Business Advisory Agent.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
