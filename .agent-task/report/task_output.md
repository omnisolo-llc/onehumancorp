issue_title: "Implement Centralized Inventory & Distributed POS Sync"
issue_description: |
  ## Mission Queue Protocol
  **Title**: Implement Centralized Inventory & Distributed POS Sync

  **Problem Statement**:
  Priya, a boutique owner, requires seamless inventory tracking between her online web/mobile storefront and her in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism and a robust distributed sync protocol for hybrid merchants. Without this, simultaneous online and offline purchases lead to double-booking and out-of-stock scenarios, causing lost revenue, poor customer experience, and manual reconciliation headaches.

  **Research Report**:
  Based on the competitive landscape (Shopify, Wix, Square), competitors offer POS solutions but often fail micro-SMEs due to complexity and the need for expensive third-party tools to synchronize online and offline inventory effectively. Square/Stripe provide robust hardware but lack integrated, agentic workflow automation. OHC can leapfrog incumbents by building a native, agent-driven centralized inventory system. Our Operations Agent ("The Manager") will autonomously monitor stock across all channels, triggering low-stock alerts, and resolving conflicts automatically.

  **Design Doc**:

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Process| C(Distributed Lock - Redis)
      B[Mobile POS - 375px] -->|Tap-to-Pay| C
      C -->|Acquire Lock| D[Central Ledger - PostgreSQL]
      D -->|Update inventory_levels| D
      D -->|Log inventory_transactions| D
      D -->|Release Lock| C
      D -->|Trigger Events| E[Operations Agent]
      E -->|Low Stock Alert| F[Owner Mobile Dashboard]
      E -->|Sync Conflicts| G[Customer Success Agent]
  ```

  ### Mobile UX Flow (375px First)
  - **POS Interface (Offline/Local First):** Touch targets for inventory adjustment and checkout are ≥ 44x44px.
  - **Optimistic UI:** When a sale is processed offline, the UI immediately updates inventory counts optimistically.
  - **Asynchronous Sync:** When network is restored, the POS client syncs finalized sales with the central ledger. If the Redis reservation fails (e.g. out of stock), a clear rollback and error state is presented to the user.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors `inventory_levels`. Upon hitting a low-stock threshold, it generates an alert for the owner. It also coordinates with the Redis locking mechanism to reconcile conflicts and handles timeouts (e.g., cart abandonment).
  - **Finance Agent ("The Accountant"):** Processes payment splits and correlates POS transaction data with online sales for unified reporting.
  - **Customer Success Agent ("The Ambassador"):** Updates online storefront availability dynamically. If an item in a cart becomes unavailable due to a faster in-store purchase, it notifies the customer politely and suggests alternatives.

  ### Key Design Decisions
  - **Central Ledger:** Utilize existing PostgreSQL tables (`products`, `product_variants`, `inventory_levels`, `inventory_transactions`) as the absolute source of truth.
  - **Distributed Locks:** Implement Redis (Valkey) Redlock for temporary inventory reservation during checkout (e.g., `ohc:lock:{tenant_id}:inventory:{product_id}`).
  - **Eventual Consistency for Offline POS:** The mobile POS caches catalog data locally to support offline sales, with robust conflict resolution handled by the Operations Agent upon reconnection.

  **Implementation Prompt**:
  Implement the backend synchronization logic and distributed locking mechanism for Centralized Inventory & Distributed POS.
  - The feature must allow the mobile POS client (operating in potentially flaky network conditions) to finalize a transaction and eventually sync with the central database.
  - Integrate Redis (Valkey) locks (pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`) to prevent double-booking during concurrent checkouts from the online store and the physical POS.
  - Ensure the Operations Agent is triggered upon inventory changes to manage low-stock alerts and conflict resolution.
  - Create the necessary API endpoints to acquire/release locks and commit inventory changes to `inventory_levels` and `inventory_transactions`.
  - Validate the flow using end-to-end tests ensuring strict multi-tenant isolation.

  **Acceptance Criteria:**
  - A user (Priya) can ring up a sale on the mobile POS, and the inventory count is accurately and safely decremented on the backend without race conditions from a simultaneous online purchase.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []