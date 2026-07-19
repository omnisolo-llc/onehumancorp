issue_title: "Implement Centralized Inventory and Distributed POS Architecture for Hybrid Merchants"
issue_description: |
  # Strategic Feature Issue Dispatch: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement & Gap Analysis (Track 1)
  - **Persona Focus:** Priya (boutique operator). She sells in-store using tap-to-pay and online simultaneously.
  - **The Gap:** OneHumanCorp currently lacks a real-time, strong-consistency inventory locking mechanism (e.g., Redis Redlock) and a distributed offline-first sync protocol for point-of-sale (POS) clients.
  - **Why it matters:** Without this, simultaneous online cart checkouts and in-store purchases lead to double-booking and out-of-stock scenarios. A non-technical owner like Priya shouldn't have to manually reconcile inventory or apologize to customers for phantom stock.
  - **Competitive Insight:** Platforms like Shopify have POS but struggle with complex, tiered inventory that often goes out-of-sync for micro-merchants unless costly 3rd-party apps are used. Square has great hardware but lacks the agentic workflow automation OHC provides.

  ## Architecture & System Design Deep Dive (Track 2)
  - **Central Ledger (PostgreSQL):** The definitive source of truth for inventory. Uses row-level locking (SKIP LOCKED or optimistic concurrency) to handle critical updates safely. Strict row-level security (`tenant_id`) enforced.
  - **Distributed Locks (Redis Redlock):** A short-lived inventory reservation system applied during checkout to prevent double-booking.
    - **Lock key pattern:** `ohc:lock:{tenant_id}:inventory:{product_id}`
    - **TTL:** 5 minutes for online carts, 15 seconds for rapid in-store tap-to-pay.
  - **Offline-Tolerant POS Client:** The Flutter client caches the catalog locally using eventual consistency. If network drops, offline sales are queued and reconciled asynchronously with the central ledger upon reconnection.
  - **AI Department Coordination:**
    - **Operations Agent:** Monitors stock thresholds, handles conflicts between online/offline sales, and suggests restock plans.
    - **Customer Success Agent:** Instantly drafts apologies or offers alternatives if an item goes out of stock mid-cart due to an in-store purchase.
    - **Finance Agent:** Splits payments correctly and ties POS tap-to-pay transactions back to the unified ledger.

  ```mermaid
  erDiagram
    TENANT ||--o{ PRODUCT : has
    PRODUCT ||--o{ INVENTORY : has
    TENANT {
      uuid id PK
      string name
    }
    PRODUCT {
      uuid id PK
      uuid tenant_id FK
      string name
      float price
    }
    INVENTORY {
      uuid id PK
      uuid product_id FK
      uuid tenant_id FK
      int quantity
    }
    REDIS_LOCK ||--|| INVENTORY : reserves
    REDIS_LOCK {
      string key PK
      int ttl
      uuid transaction_id
    }
  ```

  ## Mobile-First & Technical Integrity (Track 3)
  - **Mobile UX Flow (375px First):**
    - The POS interface must render perfectly on a 375px viewport (e.g., iPhone SE) with zero horizontal scrolling.
    - Touch targets for critical POS actions (add to cart, check out, tap-to-pay) must be >= 44x44px.
    - Implement optimistic UI updates with clean, translucent glass visual states and rollback capability if the Redis lock is denied.
  - **Security:** Complete tenant isolation using SPIFFE/SPIRE where applicable, and PostgreSQL RLS on all inventory tables.

  ## Implementation Prompt
  Implement the Centralized Inventory & Distributed POS locking mechanism.
  1. Add Redis Redlock logic to the backend cart/checkout API endpoints.
  2. Implement the database changes in PostgreSQL (if needed) for strong consistency inventory checks.
  3. Update the Flutter POS client to support offline catalog caching and retry queues for network failures.
  4. Build Playwright E2E tests simulating a simultaneous checkout (online and offline) for the same low-stock item to prove the lock prevents double-sales.
  5. Adhere to the translucent UI design system and ensure perfect 375px rendering.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
