issue_title: "Implement Distributed POS Sync & Locking Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g., Priya the boutique owner) need a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture that works flawlessly. They need their online and in-store operations (tap-to-pay or card reader) to always stay in sync in real-time, to avoid double-booking and out-of-stock scenarios. The current OHC system does not have strong enough consistency controls (inventory locking/caching) for distributed merchants, nor does it have an robust distributed sync protocol for hybrid merchants.

  ## Research Report
  - **Shopify:** Provides robust POS, but complexity limits usage for micro-SMEs, often leading to out-of-sync online and offline inventories without expensive tier subscriptions.
  - **Square/Stripe Terminal:** Great hardware, but lack the agentic workflows built into OHC.
  - **OHC Opportunity:** Utilize AI agents and our distributed architecture (PostgreSQL row-level locking + Redis distributed locks + offline POS client caching) to provide a seamless, non-technical, and highly accurate centralized inventory management experience.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client - Offline Capable] -->|Sync Protocol| B(API Gateway)
      C[Online Storefront] -->|Checkout| B
      B --> D[Central Ledger / PostgreSQL]
      B --> E[Redis Redlock - Dist. Locks]
      D -->|Locking/Concurrency Control| D
      E -->|Short-term Reservation| E
      B --> F[Operations Agent]
      F -->|Monitor & Reconcile| D
  ```

  ### Mobile UX Flow (375px First)
  - **Tap-to-Pay / Checkout:** A highly responsive (glassmorphism/UniFi styled) UI on mobile. The cashier selects items, initiates payment, and the backend acquires a rapid lock (e.g., 15s).
  - **Offline Mode:** If offline, the POS client records the transaction using a locally cached catalog and syncs to the central ledger when the network is restored, triggering any conflict resolutions via the Operations Agent.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors for inventory conflicts, handles asynchronous reconciliation after offline syncs, and proactively alerts the owner if inventory levels drop critically low due to simultaneous sales.

  ### Key Design Decisions
  - **Redis Redlock:** Used for distributed locking to handle concurrent checkout attempts.
  - **Row-level locking/Optimistic Concurrency (PostgreSQL):** For the Central Ledger.
  - **Local-first mobile architecture:** Cache product catalog locally for offline POS capabilities, utilizing Eventual Consistency for reconciliation.

  ## Implementation Prompt
  **Feature Name:** OHC Distributed POS & Centralized Inventory Sync
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** Priya can sell the last of a specific dress either online or in-store without fear of double-selling. The system handles the complex distributed locking and offline reconciliation invisibly.

  **Next Actions:**
  1. Implement Redis Redlock mechanism (`ohc:lock:{tenant_id}:inventory:{product_id}`) for checkout processes.
  2. Implement strict row-level locking or optimistic concurrency for the Inventory Ledger in PostgreSQL.
  3. Ensure the mobile POS client UI properly caches data for offline scenarios and handles eventual consistency syncing.
  4. Integrate the Operations Agent to handle conflict resolution when offline transactions clash with online transactions upon sync.
  5. Add appropriate unit and Playwright E2E tests validating the locking mechanism under concurrent scenarios.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
