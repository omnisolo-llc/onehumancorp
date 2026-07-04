issue_title: "Implement Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Mission Brief: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with hybrid operations (e.g., Priya the boutique owner) struggle with disjointed inventory management. When selling online (web/mobile) and in-store (tap-to-pay), inventory counts frequently fall out of sync. This leads to double-booking, out-of-stock scenarios, and frustrated customers. Existing platforms like Shopify or Square often require complex, expensive integrations to unify these channels. OHC needs a seamless, real-time inventory locking and synchronization mechanism powered by AI agents to handle this complexity invisibly.

  ## Research Report
  - **Competitor Analysis:** Shopify dominates e-commerce but requires costly higher-tier plans or third-party apps for robust POS/inventory sync. Square provides excellent POS hardware but lacks integrated agentic workflow automation.
  - **The Gap:** OHC lacks a real-time, strongly consistent inventory locking system and a robust distributed sync protocol.
  - **Persona Need:** Priya needs an in-store tap-to-pay purchase to instantly reserve stock, preventing an online customer from checking out the exact same item simultaneously, without any manual intervention.

  ## Design Doc
  ### Architectural Overview
  - **Central Ledger (PostgreSQL):** The ultimate source of truth, utilizing row-level locking or optimistic concurrency control (`tenant_id` isolated).
  - **Distributed Locks (Redis Redlock):** A reservation system applied during checkout. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** Mobile POS caches catalog data and employs eventual consistency to sync finalized offline sales when the network is restored.

  ### AI Agent Integration
  - **Operations Agent (The Manager):** Monitors stock levels, handles sync conflicts, triggers low-stock alerts, and suggests restock plans.
  - **Finance Agent (The Accountant):** Correlates POS data with online purchases for unified reporting.
  - **Customer Success Agent (The Ambassador):** Updates online storefront availability and notifies customers if a cart item becomes unavailable.

  ### Mobile UX Flow (375px)
  - **POS Interface:** Optimized for 375px viewport with touch targets ≥ 44x44px.
  - **Optimistic UI:** Instant visual feedback on inventory changes, with rollback capabilities if the Redis reservation fails.

  ```mermaid
  graph TD
      subgraph Frontend "Flutter App (Mobile-First POS)"
          UI[POS UI]
          Cache[Local Catalog Cache]
      end

      subgraph Backend "Go + Bazel Backend"
          API[Checkout & Sync API]
          LockMgr[Redis Redlock Service]
          OpsAgent[Operations Agent]
      end

      subgraph Data
          DB[(PostgreSQL Central Ledger)]
          Redis[(Redis)]
      end

      UI -->|Transaction| API
      API -->|Acquire Lock| LockMgr
      LockMgr --> Redis
      API -->|Update| DB
      OpsAgent -.->|Monitor & Alert| DB
  ```

  ## Implementation Prompt
  **Outcome:** Create a seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item.
  **CUJ & Acceptance Criteria:**
  1. Priya processes an in-store sale for a product using the POS interface.
  2. The backend applies a short-lived Redis Redlock (e.g., 15 seconds) to reserve the item.
  3. A simulated online customer attempts to checkout the same product but receives a graceful "Item just sold out" message.
  4. The POS transaction finalizes, updating the PostgreSQL ledger.
  5. The Operations Agent sends Priya a notification (e.g., "Product sold out. Draft restock order?").
  **Tasks:**
  - Implement the Redis Redlock inventory reservation service.
  - Refine the `TerminalSession` schema for offline-sync reconciliation.
  - Extend the Operations Agent to handle low-stock monitoring and notifications.
  - Add Playwright E2E tests for the conflict scenario.

  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
