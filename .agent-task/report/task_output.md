issue_title: "Implement Multi-Channel Inventory Sync & POS Capabilities"
issue_description: |
  # Mission Queue Protocol: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) operate seamlessly across online storefronts and physical retail spaces. Currently, OHC lacks a real-time, highly-consistent inventory synchronization engine and a mobile Point-of-Sale (POS) experience that reliably bridges the gap. Without robust inventory locking during checkout flows, hybrid merchants suffer from double-booking and out-of-stock scenarios. An AI-managed, automated solution is required to unify the inventory ledger and seamlessly recover from network disconnections.

  ## Research Report
  Our competitive analysis indicates that legacy platforms (like Shopify) solve this through complex third-party apps or high-tier plans. Systems like Square/Stripe Terminal offer great hardware but lack the agentic orchestration to connect in-store actions with online state dynamically.
  We require a system using PostgreSQL as the central ledger and Redis Redlock for short-lived, distributed inventory reservations during checkout, supervised by the "Operations Agent".

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Attempt| B(Redis: Distributed Lock)
      C[Mobile POS] -->|Tap-to-pay| B
      B -->|Lock Acquired| D[PostgreSQL: Central Ledger]
      B -->|Lock Failed| E[Operations Agent: Alert Customer/Owner]
      D --> F[Operations Agent: Trigger Restock/Update UI]
  ```

  ### Mobile UX Flow (375px First)
  - **POS Terminal Screen:** Clean, grid-based catalog using macOS Translucent Glass styles. Touch targets >44x44px for rapid selection.
  - **Offline Resilience:** If connection drops, the POS client allows queueing of sales locally, syncing to PostgreSQL when connection returns (eventual consistency).
  - **Cart Resolution:** If an online user tries to check out an item locked by the POS, they receive a graceful "Item just sold out" message, initiated by the Customer Success agent.

  ### AI Agent Integration
  - **The Manager (Operations Agent):** Tracks overall stock velocity, resolves any sync conflicts that couldn't be automatically handled, and proposes restocking drafts for the owner.
  - **The Ambassador (Customer Agent):** Handles communications when stock conflicts arise, offering alternatives or backorders.

  ## Implementation Prompt
  Implement the Redis Redlock inventory reservation service and integrate it into the checkout flows (both web cart and API POS endpoints). Ensure PostgreSQL acts as the single source of truth and handles multi-tenant row-level security (`tenant_id`). Construct the base API routes for the POS client to fetch catalog state and submit offline-queued transactions. Do NOT prescribe specific DB table column layouts; implement the minimal, scalable version that closes the double-booking gap. Must include E2E Playwright coverage verifying the locking mechanism (e.g. concurrent checkout attempts).

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
