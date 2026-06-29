issue_title: "Implement High-Scale Distributed Redis Redlock for Inventory POS sync"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (like Priya the boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  **Findings & Competitive Analysis:**
  - Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed.
  - Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **OHC Opportunity:** Leverage our "Operations Agent" philosophy. The system requires a real-time reservation protocol utilizing Redis Redlock to coordinate online and offline (POS) purchases and ensure strong consistency, combined with an agentic workflow that manages out-of-stock and restocking states invisibly to the user.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Attempt| B(Inventory Sync Gateway)
      C[Mobile POS / Tap-to-Pay] -->|In-store Sale| B
      B --> D{Redis Redlock distributed locking}
      D -->|Lock acquired| E[PostgreSQL Central Ledger]
      D -->|Lock failed / Wait| F[Operations Agent]
      E --> G[Update Stock]
      G --> F
      F -->|Notify customer| A
      F -->|Alert low stock| H[Action Required Queue - Owner App]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Screen (Mobile):** Shows quick-tap items with real-time stock levels.
  - **Interaction:** When a POS transaction is initiated, the UI immediately shows a subtle "Processing..." state while the Redis lock is acquired.
  - **Action:** If an online order beats the POS tap, the POS instantly displays an "Item Sold Out Online" alert before payment completes.
  - **Visual Design:** Glassmorphism cards, fast snappy transitions, native touch target size (44x44px).

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Monitors Redis locks and PostgreSQL inventory levels. If an item drops to zero during concurrent checkouts, it drafts a message for the online user ("Sorry, this item just sold out in-store!") and creates a feed item for the owner ("Draft restock order for Item X").

  ### Key Design Decisions
  - **Redis Redlock:** Essential for cross-channel locking to prevent race conditions between independent online and POS workers.
  - **Optimistic Concurrency:** Rely on Redis for fast path locks and PostgreSQL for the durable source of truth.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya is running her boutique. An online customer tries to buy the last "Red Dress" at the exact moment an in-store customer taps to pay for it. The system flawlessly resolves this: the in-store customer gets it, the online customer receives an apology, and Priya gets a notification to restock—all without her doing any manual inventory math.
  **CUJ & Acceptance Criteria:**
  1. The backend must implement a Redis Redlock mechanism in the checkout/POS flow using a key like `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. Implement a `POSCheckout` API route that attempts to acquire this lock before deducting inventory in PostgreSQL.
  3. If the lock is held (simulating concurrent online purchase), the POS route must gracefully handle the failure and return a clear out-of-stock response.
  4. The Operations Agent should be triggered upon out-of-stock events to draft a restock action card.
  5. Provide Playwright E2E tests: Simulate concurrent requests to the checkout and POS endpoints and verify that only one succeeds and inventory doesn't drop below zero.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
