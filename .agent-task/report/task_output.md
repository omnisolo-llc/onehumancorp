issue_title: "Implement OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  **Title**: Implement OHC Unified Multi-Channel Inventory Sync & POS

  **Problem Statement**
  Boutique owners like Priya need seamless inventory tracking between their online store and in-person POS. Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism, leading to double-booking when simultaneous online and offline purchases happen. This forces SMBs to use expensive third-party tools.

  **Research Report**
  Traditional platforms like Shopify provide extensive POS capabilities but are overly complex for micro-SMBs, often requiring expensive third-party apps to keep online and offline inventory perfectly synced. Competitors like Wix and Squarespace have limited native POS. Square is strong in POS but weak in automated online agent workflows. OHC must provide a unified, agentic solution where inventory sync happens invisibly. We propose a centralized inventory model using PostgreSQL as the source of truth, backed by Redis Redlock for temporary reservations during checkout to prevent double-booking. The Operations Agent will handle monitoring, conflict resolution, and restock alerts.

  **Design Doc**
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
        participant Customer (Online)
        participant Mobile POS (In-Store)
        participant API Gateway
        participant Inventory Service
        participant Redis (Redlock)
        participant DB (Central Ledger)
        participant Operations Agent
        Mobile POS (In-Store)->>API Gateway: Initiate Checkout (Tap to Pay)
        API Gateway->>Inventory Service: Request Lock
        Inventory Service->>Redis (Redlock): Acquire 15s Lock (ohc:lock:tenant_id:inventory:product_id)
        Redis (Redlock)-->>Inventory Service: Lock Acquired
        Customer (Online)->>API Gateway: Initiate Checkout
        API Gateway->>Inventory Service: Request Lock
        Inventory Service->>Redis (Redlock): Acquire 5m Lock
        Redis (Redlock)-->>Inventory Service: Lock Denied
        Inventory Service-->>Customer (Online): Graceful Error: "Item just sold out"
        Mobile POS (In-Store)->>API Gateway: Payment Success
        API Gateway->>DB (Central Ledger): Update Inventory
        DB (Central Ledger)-->>Operations Agent: Event: Item out of stock
        Operations Agent-->>Mobile POS (In-Store): Action Card: Draft restock order
    ```
  - **Mobile UX Flow (375px first)**:
    - POS interface shows available stock with large (min 44x44px) touch targets. Tapping an item adds it to the cart.
    - Initiating checkout triggers the fast Redis lock visually with a subtle glassmorphism loading state.
    - If another channel attempts to buy the locked item, the UI gracefully falls back to "Item just sold out" with an option to notify when restocked.
  - **AI Agent Integration Points**:
    - The Operations Agent monitors the ledger asynchronously. When an item sells out, it drafts a restock notification or updates the online catalog visibility, pushing an Action Card to the Unified Agent Feed.
  - **Key Decisions**:
    - Using Redis Redlock ensures cross-node safety for reservations without locking the entire Postgres row for long periods, which is crucial for high-throughput, multi-channel sales.
    - Graceful degradation on lock failure ensures users are informed transparently.

  **Implementation Prompt**
  Implement the Unified Multi-Channel Inventory Sync & POS feature. Create the necessary Redis-backed inventory reservation service that temporarily locks items during checkout (both online and POS). Integrate the reservation check into both checkout flows and update the POS UI to handle reservation failures gracefully. Connect the Operations Agent to listen for "out of stock" events to generate restock action cards in the feed. Verify the implementation fully supports 375px mobile screens and includes E2E Playwright tests covering the multi-channel conflict scenario.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
