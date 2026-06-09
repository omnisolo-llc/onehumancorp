issue_title: "Implement Distributed Inventory Lock using Redis Redlock for POS/Online Checkout"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is missing a robust inventory lock mechanism across its online and offline storefronts. When small business operators like Priya (Boutique Owner) ring up a customer using a mobile Point-of-Sale (POS) tap-to-pay while an online customer simultaneously tries to checkout the same item online, a race condition occurs leading to double-booking and out-of-stock scenarios. The lack of a real-time, multi-channel distributed lock prevents a trusted central inventory experience.

  ## Research Findings & Market Gap
  - **Market Context**: Shopify and Square offer POS/Online syncing, but micro-SMEs often run into synchronization delays (eventual consistency) on lower-tier platforms. Small operators need zero double-booking.
  - **Identified Gap in OHC**: There is currently no cross-channel reservation lock in OHC. Our data ledger (PostgreSQL) is the source of truth, but high-velocity concurrent checkout flows (online vs POS tap-to-pay) require transient reservation locks *before* finalizing the PostgreSQL transaction to provide immediate user feedback ("Item just sold out").

  ## Proposed Design Document (Architecture)
  The goal is to implement a robust Redis Redlock reservation system to coordinate inventory locks.

  ### Data Flow & Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant MobilePOS
      participant OnlineWeb
      participant RedisLock
      participant PostgresDB

      MobilePOS->>RedisLock: Acquire lock (15s TTL)
      alt Lock Successful
          RedisLock-->>MobilePOS: Lock Acquired
          MobilePOS->>PostgresDB: Finalize Sale Transaction
          PostgresDB-->>MobilePOS: Transaction Complete
          MobilePOS->>RedisLock: Release Lock
      else Lock Failed
          RedisLock-->>MobilePOS: Failed (Item reserved)
          MobilePOS->>MobilePOS: Show "Out of Stock" UI
      end

      OnlineWeb->>RedisLock: Acquire lock (5m TTL)
      alt Lock Successful
          RedisLock-->>OnlineWeb: Lock Acquired
          OnlineWeb->>PostgresDB: Finalize Web Checkout
          PostgresDB-->>OnlineWeb: Transaction Complete
          OnlineWeb->>RedisLock: Release Lock
      else Lock Failed
          RedisLock-->>OnlineWeb: Failed (Item reserved)
          OnlineWeb->>OnlineWeb: Show "Reserved by another buyer" UI
      end
  ```

  ### Architecture Details
  - **Primary Ledger**: PostgreSQL acts as the master truth for total stock.
  - **Reservation Cache**: Redis acts as a distributed lock manager. When a checkout starts (either via Mobile POS or Online Web Checkout), a temporary Redlock is applied.
    - **Lock Key Pattern**: `ohc:lock:{tenant_id}:inventory:{product_id}`
    - **Lock Duration**: POS transactions (15 seconds, rapid tap-to-pay); Online carts (5 minutes, standard e-commerce reservation).
  - **Failure Handling**: If Redis lock acquisition fails (e.g. item already reserved by another session), the UI immediately gracefully informs the user (e.g., "Item just sold out or reserved by another buyer").
  - **Agent Intervention**: Upon final checkout (Postgres commit) and lock release, the Operations Agent evaluates total remaining stock. If stock drops to 0, it triggers an alert and prompts the user to consider restock workflows.

  ### Mobile UX Flow (375px First)
  - The checkout screens (both POS mode and Web) must reflect real-time inventory state.
  - Optimistic UI: Tapping 'Buy' on the POS immediately attempts a lock. The button transitions to a translucent spinner (`backdrop-filter: blur(20px)` style). If the lock fails, an inline, clean error state appears. Touch targets for recovery actions remain $\ge$ 44x44px.

  ### AI Agent Integration
  - **Operations Agent ("The Manager")**: Needs to consume PostgreSQL row-level updates or listen for specific Redis events to detect when a product hits zero inventory. It should automatically ping the owner: "Red Dress sold out. Would you like to draft a restock order?"

  ## Implementation Prompt (For Implementer Agent)
  1. **Redis Redlock Service**: Implement a distributed locking service utilizing Redis to handle `acquireLock`, `releaseLock`, and `extendLock` operations using the `ohc:lock:{tenant_id}:inventory:{product_id}` pattern. Ensure it handles connection failures gracefully and defaults back to strict PostgreSQL transactions if Redis is unavailable.
  2. **Checkout Integration**: Inject the Redlock service into the checkout flow (both the API used by the web client and the API used by the Flutter POS client). Implement differing TTLs based on context (POS vs Web Cart).
  3. **UI Updates (Flutter/Web)**: Ensure the checkout button explicitly handles "Reservation Failed" or "Out of Stock" lock rejections with clear, non-technical error messages.
  4. **Agent Integration**: Add a hook to notify the Operations Agent when inventory for an item drops to 0 after a successful sale.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
