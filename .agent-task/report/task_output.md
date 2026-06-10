issue_title: "Implement Real-time Multi-Channel Inventory Sync & POS Reservation"
issue_description: |
  # Research Report: Real-time Multi-Channel Inventory Sync & POS Reservation

  ## Problem Statement
  Small business owners like Priya (boutique operator) manage inventory across multiple channels—online storefronts and in-store point-of-sale (POS) systems. Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism. This leads to critical failures like double-booking (e.g., selling the last item in-store while an online customer simultaneously completes checkout). For non-technical owners, reconciling these errors is a massive pain point that erodes trust in the platform. Existing platforms like Shopify offer this, but at the cost of high complexity and expensive POS hardware add-ons.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers unified inventory, but requires expensive hardware and higher-tier plans for full real-time sync. Complexity is high for micro-merchants.
  - **Square:** Excellent POS, but e-commerce integration (Square Online) often feels bolted on, with lag in inventory syncing.
  - **Wix/Squarespace:** E-commerce focused; POS integration is often handled via third-party apps, leading to sync delays and double-selling risks.
  - **OHC Opportunity:** Leverage our centralized architecture (PostgreSQL + Redis) and AI agents (Operations Agent) to provide an "invisible" real-time sync. When an item is selected in-store (POS mode), it is temporarily locked across all channels. If it sells, the Operations Agent immediately updates the online storefront and can proactively suggest reordering.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Attempt| B(Checkout Service)
      C[Mobile POS App] -->|Select Item| B
      B --> D{Inventory Lock Service Redis}
      D -->|Lock Granted| E[Process Payment Stripe]
      D -->|Lock Denied| F[Return Out of Stock Error]
      E -->|Success| G[Update Central Ledger PostgreSQL]
      G --> H[Operations Agent]
      H -->|Update Storefront| A
      H -->|Low Stock Alert| I[Mobile Feed]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Mobile View:** A clean grid of products. Tapping an item adds it to the cart. During payment processing, the UI shows a "Processing..." state (translucent overlay).
  - **Online Storefront:** If an item is locked by a POS transaction, the online UI should dynamically disable the "Add to Cart" button or show "1 person is buying this" to create urgency. If they try to checkout a locked item, a graceful error: "Sorry, this item was just purchased in-store."
  - **Agent Feed:** A notification card appears if an item sells out: "The last Red Dress was sold in-store. Restock?"

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Subscribes to inventory update events. When an item hits zero, it updates the storefront status and generates a restock suggestion card for the owner's feed.
  - **Customer Success Agent (The Ambassador):** If an online user's cart is invalidated by an in-store sale, the agent can draft an apology email with a discount code for a future purchase.

  ### Key Design Decisions
  - **Redis Redlock for Temporary Reservations:** Use distributed locks (e.g., `ohc:lock:{tenant_id}:inventory:{product_id}`) to hold items during checkout (e.g., 5 mins online, 30 secs in-store POS).
  - **PostgreSQL as Source of Truth:** Final inventory counts and transaction records reside here.
  - **Optimistic Concurrency Control:** For non-critical updates, use version numbers on inventory rows to prevent lost updates without heavy locking.

  ## Implementation Prompt
  **User-Facing Outcome:** As a boutique owner using the OHC mobile app as a POS, when I ring up a customer for the last item in stock, that item is instantly reserved. If someone is simultaneously trying to buy it online, they are prevented from doing so, eliminating double-selling and the need for manual apologies.

  **CUJ & Acceptance Criteria:**
  1. Initialize a product with an inventory count of 1 in the database.
  2. **Simulate Online User:** Start an online checkout process for the product but pause before final payment submission.
  3. **Simulate POS User (Owner):** Using the mobile POS UI (browser/Playwright resized to 375px), select the same product and initiate the checkout flow.
  4. The system MUST acquire a Redis lock for the POS transaction.
  5. The Online User attempts to complete their checkout. The system MUST reject the transaction with a clear "Item just sold out" or "Item currently unavailable" message.
  6. The POS transaction completes successfully, updating the database inventory to 0.
  7. Provide Playwright E2E tests covering this race condition: User A adds to cart and begins checkout, User B (POS) completes checkout, User A attempts final purchase and fails gracefully.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
