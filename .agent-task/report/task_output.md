issue_title: "OHC Unified Inventory Lock & AI-Led Graceful Sync"
issue_description: |
  # Mission Queue Protocol: OHC Unified Inventory Lock & AI-Led Graceful Sync

  ## Problem Statement
  OneHumanCorp (OHC) owners like Priya (boutique operator) and Maya (home baker) operate in a "hybrid" mode where sales happen across Instagram DMs, online storefronts, and in-person via Stripe Terminal. Currently, there is no strongly consistent mechanism to prevent "double-selling" when an in-person customer buys the last item while an online customer is at the checkout screen. This leads to owner stress, customer disappointment, and manual refund work.

  ## Research Report
  - **Shopify/Square**: Provide sync but often have a multi-minute lag or require manual refresh. They do not handle the "interrupted checkout" experience gracefully via AI.
  - **Industry Best Practice**: Use Redis Redlock for sub-second distributed locking across multiple API instances.
  - **Persona Fit**: Priya needs to trust that if she taps "Pay" on her phone in the boutique, the online "Buy Now" button for that item instantly disables or triggers a graceful fallback.

  ## Design Doc
  ### Architecture
  The system uses a two-tier locking strategy:
  1.  **Checkout Hold (Soft Lock)**: A 15-minute Redis-backed reservation for online carts.
  2.  **POS/Terminal Hold (Hard Lock)**: A 30-second pre-emptive lock when an item is scanned in-person. This lock pre-empts/invalidates online checkout holds if inventory count is 1.

  ### Sequence Diagram
  ```mermaid
  sequenceDiagram
      participant C as Online Customer
      participant S as Storefront API
      participant R as Redis (Redlock)
      participant O as Owner (POS)
      participant Ag as Operations Agent
      participant CS as Customer Success Agent

      C->>S: Starts Checkout (Last Item)
      S->>R: SET ohc:res:cart:p1 (15m)
      O->>S: Scans Item in Boutique
      S->>R: SET ohc:lock:pos:p1 (30s)
      Note over R: POS Lock Pre-empts Cart Hold
      Ag->>S: Detects Pre-emption
      Ag->>CS: Trigger "Graceful Sell-out"
      CS->>C: Push Notification/Draft: "Sorry, just sold out!"
      O->>S: Completes Payment
      S->>DB: Deduct Inventory
      S->>R: DEL ohc:lock:pos:p1
  ```

  ### Mobile UX Flow (375px)
  - **POS Screen**: Vibrant macOS-style glassmorphic cards. A "Syncing..." status indicator turns green when the Hard Lock is acquired.
  - **Inventory List**: Real-time badges showing "Locked (1)" for items currently in checkout elsewhere.
  - **Touch Targets**: Minimum 44x44px for all increment/decrement and payment buttons.

  ### AI Agent Integration
  - **Operations Agent**: Monitors Redis lock contention. Emits `InventoryConflictEvent` when POS pre-empts an online hold.
  - **Customer Success Agent**: Listens for `InventoryConflictEvent`. Autonomously drafts an apology and a "Consolation Discount" (10%) to the online customer to maintain trust.

  ## Implementation Prompt
  Implement a unified multi-channel inventory locking service (`InventorySyncService`) using Redis Redlock.
  1.  Integrate the lock into `src/server/api/terminal_api.rs` (POS scan) and the online checkout flow.
  2.  Implement "Pre-emptive POS Locking": If inventory is 1, a POS scan should invalidate any existing online checkout holds.
  3.  Integrate with KAIROS: The Operations Agent must detect this pre-emption and signal the Customer Success Agent.
  4.  The Customer Success Agent must draft a proactive apology message to the displaced online customer.
  5.  Verify via Playwright E2E: Simulate a POS sale while an online checkout is pending and assert the online customer receives the graceful sell-out notification.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, inventory, ai-agents]
assignees: []
