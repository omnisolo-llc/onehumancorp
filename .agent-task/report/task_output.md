issue_title: "[Research] AI-Driven Centralized Inventory & Tap-to-Pay POS Sync"
issue_description: |
  # Research Report: AI-Driven Centralized Inventory & Tap-to-Pay POS Sync

  ## Problem Statement
  Small business owners like Priya (boutique operator) manage inventory across multiple channels—online stores and in-person sales. Current tools like Shopify or Square require complex manual syncing or expensive third-party integrations, leading to double-selling and inaccurate stock levels. OHC lacks a real-time, strongly consistent inventory locking mechanism and a robust distributed sync protocol that works offline for hybrid merchants.

  ## Research Report
  - **Shopify:** Excellent POS and online store, but multi-channel sync can be delayed or requires complex setup for micro-merchants.
  - **Square:** Strong hardware POS, but lacks integrated AI agentic workflows to automate reordering, notify customers of stock changes, or unify online/offline data.
  - **Wix/Squarespace:** Basic inventory management that struggles with high-volume or rapid in-person vs online contention.
  - **Opportunity:** OHC can differentiate by integrating the "Operations Agent" to automatically manage stock levels, reconcile conflicts, and guide the owner with plain-language summaries and actions, all while ensuring strong consistency using Redis distributed locks.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Intent| B(Redis: Lock Inventory)
      C[Mobile POS / Tap-to-Pay] -->|Checkout Intent| B
      B -->|Success| D{PostgreSQL: Update Ledger}
      B -->|Failure: Conflict| E[Operations Agent: Resolve/Alert]
      C -->|Offline Sale| F[Local Cache]
      F -->|Network Restore| G[Async Sync Queue]
      G --> D
      D --> H[Operations Agent: Low Stock Alert]
  ```

  ### Mobile UX Flow (375px first)
  1.  **Home Screen:** Unified dashboard showing today's sales (online + in-store) and urgent alerts ("3 items low on stock").
  2.  **POS Mode:** Large, touch-friendly product grid (44x44px min targets). Tap to add to cart.
  3.  **Checkout:** Integrated tap-to-pay interface. Immediate visual confirmation.
  4.  **Inventory Management:** Simple list view with +/- buttons to adjust stock. Agent suggests restock quantities.

  ### AI Agent Integration
  - **Operations Agent:** Monitors inventory levels. If an item sells out in-store while in a user's online cart, the agent triggers a polite, automated email/notification apologizing and offering an alternative or waitlist.
  - **Finance Agent:** Consolidates POS and online revenue into a single daily summary for the owner.

  ### Key Design Decisions
  - **Redis Redlock:** Used for distributed locking during checkout to prevent double-booking across online and in-store channels.
  - **Eventual Consistency for POS:** The mobile POS must work offline (Fatima persona). It writes to a local cache and syncs to PostgreSQL when the network returns.
  - **Agent-First Conflict Resolution:** Instead of showing the owner a raw database conflict, the Operations Agent handles minor discrepancies or presents a plain-language summary for review.

  ## Implementation Prompt
  **Outcome:** Implement the core inventory locking and sync mechanism, along with the Operations Agent's ability to monitor and alert on stock levels.
  **CUJ:** Priya completes an in-store sale via the mobile POS while a customer simultaneously tries to buy the same last item online. The system correctly grants the lock to one, completes the sale, prevents the other, and the Operations Agent notifies Priya of the stock-out and (if applicable) sends a drafted apology to the online customer.
  **Acceptance Criteria:**
  - Distributed lock mechanism (Redis) is implemented for inventory items.
  - Mobile POS interface supports offline caching and async syncing.
  - Operations Agent can detect stock-outs and generate alerts/drafts.
  - 100% test coverage for the locking logic.
  - E2E Playwright test verifies the concurrent checkout scenario and agent notification.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
