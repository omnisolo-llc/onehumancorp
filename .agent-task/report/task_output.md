issue_title: "[Architecture Gap] Real-Time Multi-Channel Inventory Locking & POS Conflict Resolution"
issue_description: |
  ## Title: Real-Time Multi-Channel Inventory Locking & POS Conflict Resolution

  ### Problem Statement
  Boutique owners like Priya operate both a physical retail location and an online storefront simultaneously. Currently, the most agonizing friction point occurs during "Pop-up sales" or busy weekends. If Priya rings up the last available "Summer Midi Dress" via the in-store POS (Tap-to-Pay), but an online customer simultaneously adds the same dress to their cart, the system occasionally double-books the inventory. The owner doesn’t want to be embarrassed by having to refund an online customer because the item was actually sold in-store minutes prior. They need an invisible assistant that strictly guarantees inventory availability and politely manages customer expectations in real-time without requiring the owner to manually reconcile ledgers.

  ### Research Report
  Our competitive analysis indicates that hybrid inventory sync is a notorious weakness in the SMB ecosystem.
  - **Shopify**: Excellent online inventory tracking, but robust multi-location/POS sync often requires higher-tier plans. During high-velocity flash sales, race conditions still occur unless third-party locking apps are employed.
  - **Wix / Squarespace**: Inventory management is heavily delayed between their POS integrations (like Square) and the online storefront. Double-bookings are common, and the resolution process is entirely manual for the owner.
  - **Square**: Strong POS-first inventory, but weak e-commerce integrations when paired with external website builders.
  - **OHC Opportunity**: Unlike traditional systems, OHC can leverage an active Operations Agent to not only enforce strict locking at the database level but also proactively *communicate* with the affected customer (e.g., "I'm so sorry, the last dress was just purchased in-store, would you like a 10% discount on a backorder?").

  ### Design Doc

  #### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C_Online as Online Customer
      participant C_POS as In-Store POS (Priya)
      participant API as OHC API Layer
      participant Cache as Redis (Distributed Lock)
      participant DB as Postgres (Central Ledger)
      participant OpsAgent as Operations Agent

      C_POS->>API: Scan/Add "Dress" to Cart
      API->>Cache: Acquire Redlock (ttl: 15min) `ohc:lock:{tenant_id}:inventory:{item_id}`
      Cache-->>API: Lock Acquired

      C_Online->>API: Attempt Add "Dress" to Cart
      API->>Cache: Try Acquire Lock
      Cache-->>API: Lock Denied
      API-->>C_Online: UI Error "Item currently in another cart"

      C_POS->>API: Complete Transaction (Tap-to-Pay)
      API->>DB: UPDATE products SET inventory_count = inventory_count - 1
      API->>Cache: Release Lock
      API->>OpsAgent: Emit `tenant.inventory.updated` & `LowStockAlert`
      OpsAgent->>C_POS: Push Notification "Dress sold out. Draft restock?"
  ```

  #### Mobile UX Flow & Wireframes (375px)
  1. **POS Screen (Priya)**:
     - A clean, 375px grid of product photos.
     - Priya taps the "Dress" tile (target > 44x44px).
     - The item instantly dims slightly, showing a "Reserved" badge.
  2. **Online Customer Screen**:
     - Browsing on mobile. They tap "Add to Cart".
     - Instead of a generic error, a translucent bottom sheet slides up: "Oops! Someone at our physical store is buying the last one right now. Check back in 15 minutes or browse similar items."
  3. **Operations Agent Feed (Priya's Home Screen)**:
     - Post-transaction, a card appears in Priya's feed: "The Summer Midi Dress is sold out. I've drafted a restock order to your supplier."
     - Button: [Approve & Send] (Massive touch target).

  #### AI Agent Integration Points
  - **The Operations Agent**: Listens to the `InventoryConflictEvent` and `tenant.inventory.updated`. It is responsible for evaluating if the item crossed the low-stock threshold and autonomously drafting the restock purchase order.
  - **The Customer Success Agent**: If an online order *does* slip through due to a delayed offline-sync (e.g., POS lost internet), the CS Agent automatically drafts an apology email to the online customer offering a refund or backorder.

  #### Key Design Decisions and Why
  - **Strict Redlock on Cart Add**: We chose to lock inventory at the "Add to Cart" or "Scan at POS" phase rather than checkout. While this risks cart abandonment locking up stock, it provides a 100% guarantee against double-booking, which is a higher priority for small business brand trust.
  - **Agent-Driven Resolution**: Technical reconciliation logic (handling offline sync conflicts) is kept out of the UI. The UI only shows the *business result* (a drafted restock order or drafted apology).

  ### Implementation Prompt
  **User-Facing Outcome**: When Priya rings up an item in her physical store, the online storefront instantly reflects that item as unavailable or reserved. If an online customer tries to buy it simultaneously, they receive a graceful warning. Post-sale, Priya receives an actionable feed card from her AI assistant suggesting a restock.

  **Critical User Journey (CUJ)**:
  1. Priya opens the OHC mobile app and enters POS mode.
  2. An online customer (in a separate browser session) views the product page for "Unique Vase" (Quantity: 1).
  3. Priya adds "Unique Vase" to her POS cart.
  4. The online customer clicks "Add to Cart". They receive a visual bottom-sheet notification stating the item is currently being purchased in-store.
  5. Priya completes the tap-to-pay transaction.
  6. Priya returns to her OHC Home Feed and sees a new Agent Card: "Unique Vase is sold out. I drafted a restock request."

  **Acceptance Criteria**:
  - Implement a robust distributed locking mechanism that triggers when an item is added to any cart (POS or Web).
  - Ensure the mobile UI gracefully handles the locked state without generic HTTP 500 errors.
  - The Operations Agent must successfully catch the stock depletion event and surface a restock Action Card in the Agent Feed.
  - Implement at least one comprehensive Playwright E2E test covering this exact simultaneous POS vs Web cart scenario.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
