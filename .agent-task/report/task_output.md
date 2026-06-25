issue_title: "Centralized Inventory & Distributed POS Architecture"
issue_description: |
  ## Problem Statement
  Small business owners running multi-channel operations (like Priya the boutique owner) suffer from disjointed inventory management. Online inventory often falls out-of-sync with in-person sales (tap-to-pay/POS) unless they use expensive, complex third-party tools. Without real-time, strongly consistent synchronization, double-booking and out-of-stock scenarios frequently occur when simultaneous online and offline purchases happen. Existing platforms provide passive sync, not proactive management.

  ## Research Report
  - **Market Context**: Platforms like Shopify have extensive POS, but they fail micro-SMEs due to complexity and often require higher-tier plans or apps to achieve flawless real-time sync. Square provides robust POS hardware but lacks agentic workflow automation.
  - **The OHC Opportunity**: OHC can differentiate by embedding real-time, distributed inventory locking directly into the checkout flow (both web and POS) and using the Operations Agent ("The Manager") to proactively handle stock levels, sync conflicts, and restock alerts.
  - **The Gap**: OHC currently lacks a real-time, strongly consistent inventory locking mechanism (like Redis Redlock) and a robust distributed sync protocol for handling offline/local-first POS transactions that reconcile with the central PostgreSQL ledger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Attempt| B{Distributed Lock Engine}
      C[POS/Terminal] -->|Tap-to-Pay| B
      B -->|Lock Acquired| D[Central Ledger / PostgreSQL]
      B -->|Lock Denied| E[Graceful 'Item Sold Out' UX]
      C -.->|Offline Mode| F[Local Cache & Queue]
      F -.->|Network Restored| D
      D --> G[Operations Agent]
      G -->|Low Stock Alert| H[Mobile Feed Notification]
  ```

  ### Mobile UX Flow (375px)
  1. **POS Client (In-Store)**: Fast, touch-optimized (44x44px targets) catalog view. When an item is added to the cart and checkout begins, a temporary (e.g., 15s) Redis lock is acquired.
  2. **Online Storefront**: A customer attempting to buy the exact same item sees optimistic availability, but if the POS acquires the lock first, the online checkout gracefully handles the failure, showing an "Item just sold out" message provided by the Customer Success Agent.
  3. **Owner Dashboard**: After the sale completes and inventory drops below the threshold, Priya receives a mobile push notification: "Red Dress sold out. Would you like to draft a restock order?" with a 1-tap "Approve" button.

  ### AI Agent Integration
  - **Operations Agent ("The Manager")**: Monitors stock levels globally. Coordinates with the sync engine to handle conflicts. Suggests restock plans and alerts the owner.
  - **Finance Agent ("The Accountant")**: Processes splits for Terminal transactions and unified reporting.
  - **Customer Success Agent ("The Ambassador")**: Automatically updates online storefront availability and can notify customers if cart items become unavailable.

  ## Implementation Prompt
  **User-Facing Outcome:** A seamless multi-channel inventory system. An in-store tap-to-pay purchase instantly reserves stock, preventing an online customer from double-booking. The Operations Agent invisibly manages the sync and notifies the owner to restock.

  **CUJ & Acceptance Criteria:**
  1. Priya logs into the OHC POS mobile interface. An online customer is browsing her store.
  2. Priya starts an in-store sale for the last "Red Dress". The system applies a Redis Redlock for the `product_id`.
  3. The online customer attempts to checkout the same item. The system denies the purchase gracefully due to the lock.
  4. The POS transaction finalizes via Stripe Terminal, updating the PostgreSQL ledger.
  5. The Operations Agent identifies the zero-stock state and pushes a restock suggestion card to Priya's triage feed.
  6. Provide E2E Playwright tests verifying the concurrent checkout lock mechanism and the resulting UI states.

  **Next Actions for Engineering:**
  1. Implement Redis Redlock service for `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. Refine `TerminalSession` data schema to handle offline-sync reconciliation.
  3. Extend Operations Agent to monitor real-time stock levels and trigger push notifications.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []