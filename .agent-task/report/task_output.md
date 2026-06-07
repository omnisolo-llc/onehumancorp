issue_title: "Implement Multi-Channel Distributed POS & Inventory Sync System"
issue_description: |
  # Mission Queue Protocol: OHC Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with hybrid operations (e.g., Priya the boutique owner) struggle with keeping their online and in-store inventory synchronized. Without real-time synchronization and concurrency control, there is a risk of double-booking and out-of-stock scenarios when simultaneous online and in-person purchases occur. Current tools (like Shopify or simple builders) are either too complex or entirely lack autonomous AI oversight.

  ## Research Report
  - **Competitor Audit**: Traditional systems like Shopify handle POS but often require expensive add-ons and manual syncing between offline and online stores. Other platforms like Square excel in offline but have disjointed online integrations. None provide a fully autonomous "Agentic" backend out of the box.
  - **Findings**: The core missing capability in OHC is a robust concurrency mechanism (Distributed Locks via Redis Redlock) tied to a Central Ledger (PostgreSQL) that serves both mobile-first offline POS flows and edge-cached online storefront checkouts.

  ## Design Doc
  - **Data Model**:
    - Centralized Inventory Ledger residing in PostgreSQL (`inventory` table with optimistic locking or row-level `FOR UPDATE` clauses).
    - Redis Redlock mechanism for temporary reservations during checkout (e.g. 5 minutes for web carts, 15 seconds for rapid Terminal tap-to-pay).
    - Offline POS caching with eventual consistency via `TerminalSession` records to sync and reconcile upon network restoration.
  - **AI Coordination**: The **Operations Agent ("The Manager")** monitors real-time inventory counts and handles conflict resolution, triggers restock notifications, and interfaces with **Customer Success Agent** to immediately update online customers when their cart items become unavailable.
  - **Mobile UX Flow (375px first)**:
    1. Tap-to-Pay POS screen (mobile layout) for instant transaction processing.
    2. Redis locking kicks in visually showing "Processing/Reserving..."
    3. Push notification for low stock sent to the owner after deduction.
  - **Mermaid Flow**:
    ```mermaid
    sequenceDiagram
      participant App as POS App (Priya)
      participant API as API Layer
      participant Redis as Redis (Redlock)
      participant DB as Postgres (Central Ledger)
      participant OpsAgent as Operations Agent
      App->>API: Process in-store tap-to-pay (Item X)
      API->>Redis: Acquire Redlock for Item X (15s)
      Redis-->>API: Lock acquired
      API->>DB: Finalize sale, deduct stock
      DB-->>API: Update successful
      API->>Redis: Release lock
      API-->>App: Success response
      API->>OpsAgent: Emit StockChangedEvent
      OpsAgent->>App: Push Notification: "Stock Low. Restock?"
    ```

  ## Implementation Prompt
  - **Feature**: Build the `InventorySync` service handling Redis Redlocks.
  - **CUJ**: Priya performs a tap-to-pay transaction on the mobile POS. The system immediately applies a Redis lock preventing an online customer from checking out the exact same item. Upon successful payment, PostgreSQL is updated, and the Operations agent sends a low-stock alert if applicable.
  - **Acceptance Criteria**:
    - Must use Redis for concurrency control during checkout.
    - PostgreSQL row updates must be isolated by `tenant_id`.
    - Provide E2E Playwright tests verifying the POS lock behavior simulating concurrent checkouts.
    - Zero mock data; use live local Redis and DB instances.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
