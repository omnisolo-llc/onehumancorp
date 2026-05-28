issue_title: "Autonomous Hyperlocal Collaborative Commerce Network"
issue_description: |
  # Autonomous Hyperlocal Collaborative Commerce Network

  ## Problem Statement
  Small business owners frequently collaborate in the physical world but lack tools to do so digitally. For example, Maya (Baker) often partners with a local florist and a coffee roaster to create "Weekend Brunch Boxes." Currently, doing this online is a nightmare: they have to manually coordinate inventory, figure out how to split payments securely, and calculate combined shipping or local delivery routes. Existing platforms like Shopify or Wix are strictly single-tenant; they do not allow two independent merchants to easily bundle products, share a checkout, or automatically split the revenue and tax liabilities without expensive custom development or clunky third-party apps.

  ## Research Report
  *   **Current Architecture Limits:** OHC, like most eCommerce platforms, is fundamentally designed around isolated single-tenant boundaries. A checkout session belongs to one merchant, and revenue is routed to one ledger.
  *   **Competitor Analysis:**
      *   *Shopify:* Merchants can use basic affiliate links or complex apps like Collabs, but true cross-store product bundling with atomic, multi-party checkout is virtually impossible out-of-the-box.
      *   *Wix/Squarespace:* No native support for multi-merchant carts or revenue splitting.
      *   *Stripe Connect:* Has the backend capability for multi-party splits (Destination Charges), but requires a custom-built frontend and order management system to orchestrate.
  *   **Discovery:** We need an architectural evolution: a multi-tenant capability that allows OHC merchants to securely expose specific inventory items to a "Collaborative Network." This network must support a unified cart where a customer can buy from Maya, the florist, and the coffee roaster in a single tap, with the platform autonomously managing the complex background orchestration of inventory reservation, payment splitting, and unified local delivery routing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER ||--o{ UNIFIED-CART : "Creates"
      UNIFIED-CART ||--|{ CART-ITEM : "Contains"
      CART-ITEM }|--|| INVENTORY-MESH : "Reserves"
      INVENTORY-MESH ||--o{ TENANT-A-LEDGER : "Belongs to Maya"
      INVENTORY-MESH ||--o{ TENANT-B-LEDGER : "Belongs to Florist"

      UNIFIED-CART ||--|| CHECKOUT-ENGINE : "Processes"
      CHECKOUT-ENGINE ||--|| SPLIT-PAYMENT-ROUTER : "Authorizes"
      SPLIT-PAYMENT-ROUTER }|--|| STRIPE-CONNECT : "Executes Destination Charges"

      CHECKOUT-ENGINE ||--|| DELIVERY-DISPATCH : "Routes"
      DELIVERY-DISPATCH ||--o{ LOCAL-COURIER-API : "Schedules Pickup/Dropoff"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Merchant View (Collaboration Setup - 375px):**
      *   Maya opens the "Network" tab (a clean, Unifi-style card layout).
      *   She taps "New Bundle" and searches for local OHC merchants. She selects the Florist.
      *   A simple visual selector allows her to pick her "Vegan Chocolate Cake" and the Florist's "Spring Bouquet".
      *   She sets a bundle price. The UI transparently shows: "You get $40, Florist gets $30." (Passes the Grandmother Test).
  *   **Customer View (Unified Checkout - 375px):**
      *   The customer sees the "Weekend Brunch Box" on Maya's storefront.
      *   They tap "Buy with Apple Pay".
      *   The transaction is atomic. A single receipt is generated, clearly indicating fulfillment by multiple local partners.

  ### Key Design Decisions
  *   **Zero-Trust Multi-Tenancy:** The Collaboration Engine must act as an isolated intermediary. It temporarily assumes a delegated role to read inventory from Tenant A and Tenant B, without giving Tenant A direct access to Tenant B's database.
  *   **Atomic Transactions (Saga Pattern):** The checkout must employ a distributed transaction model (Saga). If the payment succeeds but Tenant B's inventory reservation fails at the last millisecond, the system must autonomously roll back the payment and Tenant A's reservation.
  *   **Dynamic Split Routing:** The Payment Router must dynamically calculate tax liabilities per tenant based on their individual nexus and automatically route the net payouts directly to their respective bank accounts via Stripe Connect/Mercado Pago, keeping OHC out of the flow of funds where possible to reduce regulatory burden.

  ### AI Agent Integration Points
  *   **Operations Agent:** Monitors the collaborative order. If the Florist is delayed in preparing their portion, the Operations Agent automatically alerts Maya and adjusts the courier dispatch time.
  *   **Marketing Agent:** Proactively suggests high-converting bundles based on local purchasing trends (e.g., "Maya, 40% of your customers also buy coffee nearby. Want to partner with Joe's Roastery?").
  *   **Finance Agent:** Generates clear, plain-language summaries of collaborative revenue, ensuring Maya's bookkeeping stays clean without manual reconciliation.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  Implement the backend architecture for the Autonomous Hyperlocal Collaborative Commerce Network. This requires creating a new `CollaborationEngine` service that sits above the isolated tenant ledgers. The primary user journey allows two independent OHC merchants to create a "Bundle Product" consisting of items from both inventories. You must implement the `UnifiedCart` and `SplitPaymentRouter` capable of handling atomic checkouts via Stripe Connect Destination Charges. Crucially, design the `InventoryMesh` to handle distributed locks (reserving a cake and a bouquet simultaneously) and implement a Saga pattern for failure recovery during checkout. The outcome must be entirely invisible to the end customer (they experience a single 1-tap checkout) and require zero technical configuration from the merchants beyond selecting the products and agreeing on the split. Acceptance criteria include successful end-to-end multi-tenant checkout, accurate fund routing to two distinct merchant accounts, and robust rollback on simulated inventory failure. Do not prescribe specific database schemas, but ensure strict multi-tenant data isolation is maintained.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
