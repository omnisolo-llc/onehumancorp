issue_title: "Implement Distributed Inventory Caching & Redlock Sync for OHC POS"
issue_description: |
  ## Issue Report: Distributed Inventory Caching & POS Architecture

  ### Title: Implement Distributed Inventory Caching & Redlock Sync for OHC POS

  ### Problem Statement
  Currently, the OHC platform lacks a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture. This causes a significant pain point for multi-channel merchants (like Priya the boutique owner) where double-booking and out-of-stock scenarios occur during simultaneous online and in-store purchases because there's no real-time locking and caching mechanism in place.

  ### Research Report
  - **Competitors**: Shopify dominates e-commerce but fails micro-SMEs due to complexity. Their inventory is often out-of-sync without costly third-party tools. Square and Stripe Terminal provide robust hardware but lack integrated, agentic workflow automation.
  - **Persona (Priya - Boutique Owner)**: Sells clothing in-store and online. Requires seamless inventory tracking between web/mobile and in-store tap-to-pay.
  - **The Gap**: Without a real-time, strongly consistent inventory locking and caching mechanism, OHC cannot guarantee that an item won't be double-booked across online and offline channels simultaneously.

  ### Design Doc
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
        LEDGER ||--o{ INVENTORY : maintains
        INVENTORY ||--|{ DISTRIBUTED_LOCK : "secured by"
        POS_CLIENT }|--|| DISTRIBUTED_LOCK : "requests"
        POS_CLIENT }|--o{ LOCAL_CACHE : "reads/writes offline"
        LEDGER {
            uuid tenant_id
            uuid product_id
            int stock_count
        }
        DISTRIBUTED_LOCK {
            string key_pattern "ohc:lock:{tenant_id}:inventory:{product_id}"
            int duration_seconds
        }
    ```

    ```mermaid
    sequenceDiagram
        participant Customer
        participant POS_Client as POS Client (Store)
        participant Redlock as Redis (Redlock)
        participant Ledger as PostgreSQL Ledger
        participant Agent as Operations Agent

        Customer->>POS_Client: Tap to Pay (In-Store)
        POS_Client->>Redlock: Request Lock `ohc:lock:{tenant_id}:inventory:{product_id}` (15s)
        Redlock-->>POS_Client: Lock Acquired
        POS_Client->>Ledger: Update Stock (Decrement)
        Ledger-->>POS_Client: Success
        POS_Client->>Redlock: Release Lock
        Agent->>Ledger: Monitor Stock
        Agent-->>POS_Client: Alert if Low Stock
    ```

  - **UI Wireframes (375px First)**:
    - **Header**: Transparent glass material app bar with standard Back/Menu icons. Title: "Inventory".
    - **Body (List View)**: Vertical list of items. Each item is a card with a thumbnail image (44x44 minimum touch target area), Title, Price, and current Stock Count.
    - **Action**: Tapping a list item opens an optimistic modal bottom sheet to adjust inventory up/down or trigger a restock flow.
    - **State**: A small sync icon at the top right indicates online/offline state.

  - **Architecture**:
    - **Central Ledger**: PostgreSQL as the source of truth with optimistic concurrency control or row-level locking.
    - **Distributed Locks**: Redis Redlock for temporary inventory reservation during checkout (e.g., 15s for rapid tap-to-pay, 5m for online cart). Pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
    - **Offline/Local First POS Client**: Cache catalog data locally. Eventual consistency mechanism to sync finalized sales offline and reconcile asynchronously.

  - **AI Agent Coordination**:
    - **Operations Agent**: Monitors stock levels, handles sync conflicts, suggests restock plans.
    - **Finance Agent**: Processes Terminal splits and correlates online/offline data.
    - **Customer Success Agent**: Updates storefront availability, notifies customers of out-of-stock items.

  - **Mobile UX Flow**: 375px viewport optimized. Optimistic UI updates with rollback on Redis reservation failure. Touch targets ≥ 44x44px.

  ### Implementation Prompt
  - **Objective**: Implement the Redis Redlock inventory reservation service and integrate it into the OHC POS checkout flow.
  - **Requirements**:
    - Build the Redis Redlock reservation logic.
    - Apply a dynamic lock (e.g., 15 seconds for POS) during checkout.
    - Integrate with the existing PostgreSQL inventory ledger.
    - Gracefully handle double-booking (e.g., notify the online customer that the item sold out).
    - Trigger a notification from the Operations Agent for low-stock or sold-out events.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
