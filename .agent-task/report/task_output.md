issue_title: "Implement Real-time Agentic Inventory Synchronization for Hybrid POS Operations"
issue_description: |
  # Research Report: Real-time Agentic Inventory Synchronization for Hybrid POS Operations

  ## Problem Statement
  Small business owners (e.g., Priya the boutique operator) face a critical pain point when operating both online storefronts and physical point-of-sale (POS) systems simultaneously: inventory divergence. Current systems in the market (like basic Shopify or standalone Square) often lack real-time synchronization out-of-the-box, requiring complex third-party integrations or expensive enterprise tiers. When an item is sold via tap-to-pay in-store, the online storefront may still show it as available for several minutes, leading to double-booking, cancelled orders, and frustrated customers. OHC needs a robust, agent-coordinated distributed locking and synchronization mechanism to guarantee strongly consistent inventory counts across all sales channels seamlessly, without the owner needing to configure anything.

  ## Research Report
  - **Market Context:** Competitors like Shopify and Square dominate SME retail, but their inventory sync can be disjointed unless using their full, expensive ecosystem.
  - **The Gap:** OHC currently lacks a real-time, strongly consistent inventory locking mechanism for hybrid (online + offline) merchants.
  - **User Impact (Priya's Journey):** Priya is selling a limited-edition dress. A customer is holding the last one in-store, while an online shopper has it in their cart. If Priya processes the in-store sale via the OHC POS, the online shopper must immediately be notified that the item is no longer available before they complete checkout.
  - **Technical Requirement:** A distributed locking system (e.g., Redis Redlock) integrated into the checkout flow, coordinated by the "Operations Agent" to instantly reconcile and update storefronts.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    sequenceDiagram
        actor User_InStore
        actor User_Online
        participant POS_Client
        participant Web_Storefront
        participant OHC_Operations_Agent
        participant Redis_Lock
        participant PostgreSQL_Ledger

        User_InStore->>POS_Client: Tap-to-Pay for Item X
        POS_Client->>OHC_Operations_Agent: Reserve Item X
        OHC_Operations_Agent->>Redis_Lock: Acquire Lock (tenant_id:inventory:X)
        Redis_Lock-->>OHC_Operations_Agent: Lock Acquired
        OHC_Operations_Agent->>PostgreSQL_Ledger: Decrement Inventory X
        PostgreSQL_Ledger-->>OHC_Operations_Agent: Success
        OHC_Operations_Agent->>Redis_Lock: Release Lock
        OHC_Operations_Agent-->>POS_Client: Sale Complete

        User_Online->>Web_Storefront: Add Item X to Cart (Simultaneous)
        Web_Storefront->>OHC_Operations_Agent: Reserve Item X
        OHC_Operations_Agent->>Redis_Lock: Try Acquire Lock
        Redis_Lock-->>OHC_Operations_Agent: Lock Failed (or Inventory 0)
        OHC_Operations_Agent-->>Web_Storefront: Out of Stock
        Web_Storefront-->>User_Online: "Item no longer available"
    ```

  - **Mobile UX Flow (375px):**
    - The POS view must have large (44x44px minimum) touch targets for rapid checkout.
    - If a conflict occurs (e.g., network latency while an online order processed first), the UI must clearly but gently inform the operator: "This item was just sold online."
    - Optimistic UI updates on the POS should visually reserve the item the moment it's added to the cart, reverting only if the backend lock fails.

  - **AI Agent Integration:**
    - The Operations Agent handles the coordination, catching lock failures and translating them into actionable, plain-language error states for the frontend.
    - The Customer Assistant agent can optionally draft an apology email to the online user if their cart expires.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the real-time inventory synchronization mechanism for hybrid POS operations.
  1. Implement a distributed locking mechanism using Redis (key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`) to handle simultaneous checkout attempts from POS and online storefronts.
  2. Integrate this locking logic into the checkout/order-creation service flow.
  3. Ensure the Operations Agent correctly interprets lock failures (e.g., inventory depleted by another channel) and returns a clear, user-friendly error to the client.
  4. Ensure all database interactions (PostgreSQL) use appropriate row-level locking (`SELECT ... FOR UPDATE` where applicable alongside the Redis lock) to guarantee the central ledger's integrity.
  5. The change must be fully covered by unit tests simulating concurrent purchases, and at least one E2E Playwright test proving the UI handles an out-of-stock scenario gracefully during a simultaneous purchase attempt.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
