issue_title: "Architectural Implementation: Autonomous Omnichannel Inventory Synchronization"
issue_description: |
  # Mission Brief: Autonomous Omnichannel Inventory Synchronization

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) operate across both physical and digital spaces simultaneously. Currently, when Priya sells a limited-edition dress in-store using her phone's Tap-to-Pay, her online Shopify/OHC digital storefront inventory does not immediately reflect this drop. This forces her to manually reconcile inventory across platforms to prevent double-selling, violating the OHC promise of "keeping momentum without needing technical or operational expertise." They need an invisible bridge that autonomously synchronizes in-person and digital inventory in real-time.

  ## Research Report
  **Market & Competitive Gap:**
  - **Shopify POS:** Offers excellent sync, but treats the POS as a separate application rather than a unified owner feed. It limits hardware choices.
  - **Square:** The physical POS market leader, but digital integration often feels bolted on. Inventory reconciliation can sometimes be delayed.
  - **Current OHC State:** We have mobile Tap-to-Pay and digital storefronts, but they rely on isolated ledgers. There is no real-time event-driven bridge connecting terminal SDK events to the global multi-tenant inventory cache.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> TapSDK[Native Tap-to-Pay SDK];
          App --> LocalCache[(Offline-First CRDT Cache)];
          TapSDK --> TransactionEvent[Transaction Authorized Event];
      end

      TransactionEvent -- Real-time / Queued --> API[OHC Gateway / gRPC];

      subgraph Cloud Backend
          API --> EventBus[Redis Pub/Sub & Dead-letter Queue];
          EventBus --> OpsAgent[Ops Agent Swarm (Go workers)];
          OpsAgent --> GlobalDB[(PostgreSQL Tenant DB)];
          OpsAgent --> StorefrontAPI[Online Storefront Webhooks];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** Priya opens the OHC app. "New Sale" is prominent.
  2. **In-Store Checkout:** Priya rings up the limited-edition dress. The OS-native Tap-to-Pay UI slides up.
  3. **Payment Completion:** The customer taps their phone. A clean, translucent success modal appears.
  4. **Invisible Sync:** In the background, the app immediately decrements the local CRDT inventory and fires a synchronization event to the OHC backend. The digital storefront is updated in milliseconds.
  5. **Operations Feed:** A subtle notification appears in Priya's work feed: "Dress sold in-store. Online inventory updated (1 remaining)."

  ### AI Agent Integration Points
  - **Operations Assistant:** Listens to the transaction event bus, validates the inventory deduction against the global ledger, and updates the digital storefront cache.
  - **Decision & Reporting Assistant:** Monitors for "low stock" thresholds after the sync and proactively drafts a reorder proposal if the item is popular.

  ### Key Design Decisions
  - **Event-Driven Architecture:** Use an asynchronous event bus (Redis Pub/Sub) to decouple the Tap-to-Pay SDK from the global inventory database, ensuring terminal transactions are never blocked by database latency.
  - **Optimistic UI Updates:** The mobile app updates its local inventory immediately (via CRDTs) while the cloud sync happens in the background.
  - **Idempotency:** All synchronization events must use unique idempotency keys to prevent double-deductions during network retries.

  ## Implementation Prompt
  Implement the Omnichannel Sync Engine to bridge the mobile Tap-to-Pay module and the global Inventory DB.
  - **User-Facing Outcome:** When an owner processes an in-person payment via the mobile app, the corresponding item's inventory is instantly and invisibly updated across all their digital storefronts without manual intervention.
  - **Critical User Journey (CUJ):**
    1. Owner logs into OHC app and adds a physical product to the in-store cart.
    2. Owner completes checkout using Tap-to-Pay.
    3. App displays success and optimistically updates local inventory.
    4. The backend Operations Agent receives the transaction event and deducts the global inventory.
    5. A customer viewing the digital storefront sees the updated (decremented) inventory immediately.
  - **Acceptance Criteria:**
    - A completed Tap-to-Pay transaction triggers an event that updates the PostgreSQL multi-tenant inventory ledger.
    - Network unreliability is handled gracefully (events are queued locally and retried).
    - Digital storefronts (web) reflect the inventory change via real-time update (e.g., SSE or polling if cached).
    - Full test coverage (100% unit test coverage for the Go synchronization logic).
    - An E2E Playwright test must simulate the full flow from terminal authorization to digital storefront update.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
