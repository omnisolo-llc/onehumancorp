issue_title: "Implement High-Concurrency Redis Inventory Locking for Hybrid Point-of-Sale (POS)"
issue_description: |
  ## Problem Statement
  Small business owners like Priya (Boutique Operator) run multi-channel sales (in-store POS and online storefront). Currently, OneHumanCorp (OHC) lacks a real-time, strongly consistent inventory locking mechanism during checkout. If an item is purchased in-store via tap-to-pay while an online customer is simultaneously checking out the same item, it leads to double-booking and negative stock levels, severely damaging customer trust and creating manual reconciliation work for the owner. The system must natively prevent overselling across all channels instantly.

  ## Research Report
  - **Competitor Analysis:** Platforms like Shopify handle this via robust centralized databases and optimistic concurrency, but complex POS setups often experience lag unless tied to expensive enterprise tiers. Square and Stripe Terminal provide hardware but lack seamless, autonomous multi-channel inventory sync.
  - **The Gap:** OHC's current architecture does not provide a sub-second, distributed locking mechanism needed for fast in-store (POS) transactions that also respects ongoing, slower online checkout sessions.
  - **Opportunity:** Implement a high-speed Redis-based distributed locking system (Redlock pattern) that reserves inventory momentarily during checkout. This guarantees consistency before writing the final state to the multi-tenant PostgreSQL ledger.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS / Web Checkout] -->|Initiate Checkout| B(API Gateway)
      B -->|Request Lock| C{Redis: Redlock}
      C -- Lock Granted --> D[Process Payment - Stripe]
      C -- Lock Denied --> E[Reject Checkout: Out of Stock]
      D -->|Confirm Payment| F(PostgreSQL: Central Ledger)
      F -->|Release Lock| C
      F -->|Trigger Event| G[Operations Agent]
      G -->|Push Notification| H[Update Storefront / Alert Owner]
  ```

  ### Mobile UX Flow (375px)
  1. **POS Flow (Priya):** Taps "Charge" for the last item. UI shows an instant translucent loading state. Redis lock is acquired (e.g., 15 seconds).
  2. **Online Flow (Customer):** Attempts to checkout the same item simultaneously. API detects the active Redis lock for that `product_id`. The mobile web UI immediately reflects a graceful error: "Item just sold out. 😔"
  3. **Completion:** Priya's transaction completes, DB is updated, and the Operations Agent removes the item from the online storefront completely.

  ### AI Agent Integration
  - **Operations Agent:** Listens for `InventoryDepleted` events. If an item hits zero, it autonomously updates the cached storefront status and can draft a restock order for the owner's review.

  ### Key Design Decisions
  - **Redis as Primary Lock:** Chosen for sub-millisecond latency required by in-person POS transactions.
  - **Lock Expiration:** Locks must have absolute TTLs (e.g., 5 mins for online carts, 30s for POS) to prevent deadlocks if a client crashes mid-checkout.
  - **Multi-Tenant Isolation:** Lock keys must follow the strict pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Operator
  **Goal:** Build the robust, distributed Redis locking mechanism for inventory reservation during the checkout critical user journey.
  **Requirements:**
  1. Implement a distributed locking service utilizing Redis. It must support acquiring, extending, and releasing locks.
  2. Integrate this locking service into the core checkout flow (both API endpoints and internal service layers).
  3. Ensure strict multi-tenant isolation in the Redis key structure (`ohc:lock:{tenant_id}:...`).
  4. Implement robust error handling: if a lock cannot be acquired, the checkout must gracefully fail and return a clear "out of stock / unavailable" response to the client.
  5. **Verification:** 100% unit test coverage for the locking logic. Crucially, implement a Playwright E2E test that simulates a race condition: two concurrent checkout attempts for an item with a stock quantity of 1, verifying that exactly one succeeds and the other receives the correct out-of-stock UI state.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
