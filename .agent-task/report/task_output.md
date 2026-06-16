issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  **Mission Queue Protocol & Research Report: Centralized Inventory & Distributed POS Architecture**

  **Problem Statement:**
  Currently, OneHumanCorp (OHC) lacks a real-time, strongly consistent inventory locking mechanism for hybrid merchants who sell both online and in-store. This leads to double-booking and out-of-stock scenarios, confusing users like Priya (boutique owner) who expect seamless sync across channels.

  **Research Report:**
  Based on the competitive landscape (Shopify, Wix, Stripe Terminal) and OHC's target personas, our merchants need a simple yet robust inventory sync system. The gap lies in a distributed lock and caching strategy that reliably handles the "last item sold" scenario in a multi-channel setup, seamlessly integrated with our AI Operations Agent.

  **Design Doc:**
  1.  **Architecture:**
      *   **Central Ledger (PostgreSQL):** Source of truth. Uses row-level locking or optimistic concurrency.
      *   **Distributed Locks (Redis Redlock):** Temporary reservation system during checkout (e.g., 5 min online, 15 sec offline tap-to-pay) to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
      *   **Offline/Local First POS Client:** Mobile POS caches catalog data and uses eventual consistency to sync finalized sales with the central ledger.

      ```mermaid
      sequenceDiagram
          participant POS (Priya)
          participant OnlineCustomer
          participant Redis
          participant OperationsAgent
          participant Postgres

          POS->>Redis: Acquire Redlock for "Red Dress" (15s)
          OnlineCustomer->>Redis: Attempt checkout "Red Dress"
          Redis-->>OnlineCustomer: Deny (Locked)
          OperationsAgent-->>OnlineCustomer: Graceful "Item just sold out"
          POS->>Postgres: Finalize sale & deduct inventory
          OperationsAgent-->>POS: Prompt "Draft restock order?"
      ```
  2.  **Mobile UX Flow:**
      *   Priya uses the mobile POS app (375px optimized). Tap targets > 44x44px.
      *   Real-time inventory counts update via WebSocket/SSE or optimistic UI with Redis fallback.
      *   If connection drops, offline sales are queued and synced later.
  3.  **AI Agent Integration:**
      *   The Operations Agent monitors stock, handles conflicts, and suggests restocks via push notifications.

  **Implementation Prompt for Implementer Agent:**
  *   **Objective:** Implement the Redis Redlock inventory reservation service and integrate it with the PostgreSQL central ledger for robust multi-channel inventory sync.
  *   **CUJ:** Priya completes an in-store sale of the last item via POS. A simultaneous online checkout is gracefully denied. Priya is notified of the out-of-stock status and offered a restock draft by the Operations Agent.
  *   **Acceptance Criteria:**
      *   Redis Redlock is utilized correctly during checkout flows.
      *   PostgreSQL updates handle concurrent inventory adjustments safely.
      *   Operations Agent correctly triggers the low-stock notification.
      *   Full suite of E2E tests covering concurrent online/offline checkout scenarios.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
