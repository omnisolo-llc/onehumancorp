issue_title: "Implement Unified Multi-Channel Inventory Sync & Distributed POS"
issue_description: |
  ## Title
  Implement Unified Multi-Channel Inventory Sync & Distributed POS Architecture

  ## Problem Statement
  For multi-channel merchants like Priya the Boutique Owner, managing inventory across both online storefronts and in-person tap-to-pay sales is currently disjointed. Without real-time synchronization and reliable distributed locks, double-booking occurs—selling the same "last item" simultaneously online and in-store. Non-technical owners need an invisible, highly consistent inventory system that naturally handles network drops, prevents overselling, and automatically coordinates across channels, without requiring them to install complex third-party syncing apps.

  ## Research Report
  ### Findings & Competitive Analysis
  - **Shopify**: Excellent at e-commerce inventory but requires higher-tier plans or expensive third-party tools to achieve true omnichannel POS integration that guarantees lock-level consistency during traffic spikes.
  - **Wix & Squarespace**: Offer basic inventory tracking, but their POS integrations often experience synchronization delays, leading to overselling during concurrent online and in-person purchases.
  - **Square**: Dominates physical POS and hardware but its online store integration is less capable for advanced e-commerce logic, often relying on "eventual consistency" that fails high-velocity drops.
  - **GoDaddy**: Simplistic inventory that breaks down for merchants managing real-time physical store traffic alongside online visitors.
  - **OHC Opportunity**: OHC can differentiate by embedding agentic workflows natively into the POS and inventory layer. Our Operations Agent ("The Manager") can automatically manage real-time stock levels, apply distributed locks transparently during checkout, and reconcile offline sales.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Online)
      participant Priya (POS/In-Store)
      participant OHC API
      participant Redis (Redlock)
      participant DB (Central Ledger)
      participant Operations Agent

      Priya (POS/In-Store)->>OHC API: Initiates checkout for "Red Dress"
      OHC API->>Redis (Redlock): Acquire 15s lock for ProductID:123
      Redis (Redlock)-->>OHC API: Lock granted
      User (Online)->>OHC API: Attempts checkout for "Red Dress"
      OHC API->>Redis (Redlock): Attempt lock
      Redis (Redlock)-->>OHC API: Lock denied
      OHC API-->>User (Online): "Item just sold out in-store!"
      OHC API->>DB (Central Ledger): Finalize POS Sale & Deduct Stock
      OHC API->>Redis (Redlock): Release Lock
      OHC API->>Operations Agent: Trigger "Stock Update" event
      Operations Agent->>Priya (POS/In-Store): Push notification: "Red Dress sold out. Draft restock order?"
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **POS Main Screen**: Large touch targets (≥ 44x44px). A grid of top products. The current cart slides up from the bottom.
  2. **Checkout Flow**: Tap "Charge $X.XX". If network is available, it acquires the real-time lock. A translucent loading spinner overlays the screen.
  3. **Inventory Conflict State**: If an online order completes a fraction of a second before the POS tap, a clear modal pops up: "Item sold online just now. Only X left."
  4. **Offline Mode**: A subtle top banner indicates "Offline Mode - Syncing Paused". Transactions are queued locally and the UI reflects optimistic updates.

  ### Mobile UX Flow
  - Layout is built around one-handed use.
  - Uses macOS-style Translucent Glass materials for the cart overlay to maintain context of the underlying catalog.
  - Actions use UniFi-style modular dashboard cards for clear, high-contrast readability.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the central ledger. When stock hits zero due to a POS sale, it immediately broadcasts to all active online sessions to grey out the "Add to Cart" button. It also drafts a restock email to the supplier.
  - **Customer Success Agent**: If an online order must be cancelled due to an offline sync race condition, it drafts a personalized apology and discount code.

  ### Key Design Decisions
  - **Distributed Redis Redlocks over pure DB locking**: To ensure low-latency checkouts across globally distributed edge nodes and prevent DB contention during flash sales.
  - **Local-First POS Client Cache**: To allow the boutique to keep processing transactions even if the internet drops, reconciling with the central ledger later.
  - **Agentic Recovery**: Instead of just throwing an error when an item is oversold, the AI agents proactively manage the fallout, turning a negative experience into an owner-approved resolution.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the Unified Multi-Channel Inventory Sync & Distributed POS feature.

  **User-Facing Outcome:**
  Priya the boutique owner can process a sale on her mobile POS (375px view) for the last unit of an item. Simultaneously, an online customer trying to buy the exact same item will be gracefully prevented from completing the checkout. The entire process must be seamless, with "The Manager" AI agent notifying Priya that the item sold out and offering a restock action.

  **Critical User Journey (CUJ):**
  1. Login to the mobile POS interface and view the catalog.
  2. Add the last available item to the POS cart and tap "Charge".
  3. In a separate browser window, simulate an online checkout for the same item. The online checkout must fail with a clear "Item just sold out" message due to the lock.
  4. Complete the POS sale. The inventory is permanently deducted.
  5. The POS dashboard displays an agent notification card suggesting a restock.

  **Acceptance Criteria:**
  - Introduce a distributed locking mechanism (e.g., using Valkey/Redis) for inventory reservation during checkout.
  - Ensure the POS UI is mobile-first, utilizing Translucent Glass and modular cards, with touch targets ≥ 44x44px.
  - The Operations Agent must detect the stock-out and generate an actionable feed item for the owner.
  - Provide complete E2E tests using Playwright that simulate the race condition between online and POS checkouts.
  - Zero mock data in the UI; all catalog and inventory state must flow from the real backend.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
