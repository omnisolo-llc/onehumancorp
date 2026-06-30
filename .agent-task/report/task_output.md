issue_title: "Implement POS Backend Inventory Sync and Distributed Redis Lock"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya, a boutique owner, needs a way to seamlessly track and sync inventory between her online storefront and physical in-store operations (using Stripe Terminal tap-to-pay).
  Currently, there is a risk of double-booking because the OHC backend does not reliably lock inventory in real-time when simultaneous purchases occur across channels.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers excellent online-offline synchronization, but requires higher-tier plans for true real-time distributed locking and is often too complex for micro-SMEs to configure out of the box.
  - **Wix POS / Square:** Good synchronization, but their ecosystems are fragmented and rely on manual intervention for stock discrepancy resolutions rather than autonomous agents handling the fallouts.
  - **OHC Opportunity:** Leverage our multi-tenant PostgreSQL (SKIP LOCKED) and Redis (Redlock) architecture within our Go backend to create a completely seamless, real-time locking mechanism that prevents double-booking entirely. Our Operations Agent can then autonomously handle any conflicts or restock recommendations.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[In-Store POS Terminal] -->|Checkout Initiated| B(Go API Backend)
      C[Online Storefront] -->|Add to Cart| B
      B --> D{Redis Distributed Lock}
      D -->|Lock Acquired| E[Reserve Inventory]
      D -->|Lock Denied| F[Graceful 'Sold Out' Error]
      E --> G[PostgreSQL Ledger]
      G --> H[Operations Agent]
      H -->|Low Stock Alert| I[Mobile Push Notification]
  ```

  ### Mobile UX Flow (375px First)
  - **Checkout Flow:** When Priya initiates an in-store transaction, a visual indicator on the 375px POS interface shows "Reserving item...".
  - **Conflict Handling:** If the item was just purchased online, the interface immediately shows a friendly alert: "Item just sold out online!"
  - **Resolution:** A one-tap button appears to "Notify Operations Agent to Re-order". Touch targets are exactly 44x44px.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Redis Redlock:** Used to enforce distributed locks across multi-channel environments to avoid database row-lock congestion on high-velocity items. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Go Backend Resilience:** The Go API layer will handle the lock acquisition and release, guaranteeing that temporary network partitions do not result in permanent inventory locks.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya can ring up a customer in-store while an online shopper is browsing. If the in-store transaction starts, the online shopper is gracefully prevented from buying the last item.
  **CUJ & Acceptance Criteria:**
  1. Add a Redis connection capability for distributed locking (Redlock) in the Go backend.
  2. Implement the locking function (acquire, release) during the checkout process.
  3. Modify the POS Terminal and online checkout endpoints to enforce the lock before completing a transaction.
  4. Ensure a robust fallback if the lock cannot be acquired (returning a graceful error).
  5. Provide Playwright E2E tests: A user logs in, attempts simultaneous checkout on two simulated channels, and one succeeds while the other fails gracefully with the correct UI state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
