issue_title: "Implement High-Performance Inventory Cache for Multi-Channel POS"
issue_description: |
  # Research Report: High-Performance Inventory Cache for Multi-Channel POS

  ## Problem Statement
  Small business owners with both online and physical stores face a significant risk of double-selling inventory during traffic spikes (e.g., a viral social media post). When a customer buys the last item in-store via a POS terminal while another customer checks out online, a race condition occurs. Current database-level locks are too slow for instant "tap-to-pay" POS experiences and can bottleneck the central PostgreSQL ledger during high concurrency. We need a high-performance, distributed inventory caching and reservation system to ensure consistency across online and offline channels without sacrificing checkout speed.

  ## Research Report & Gap Analysis
  Competitors like Shopify use edge-based inventory caching and asynchronous ledger reconciliation to handle massive scale. However, configuring robust distributed locks and caching layers often requires enterprise-tier plans or custom engineering.

  For OHC's persona (e.g., Priya the boutique owner), the POS must feel instantaneous (sub-100ms) while guaranteeing that an item sold in-store is immediately unavailable online.

  **The Gap:** OHC currently relies on the central PostgreSQL ledger for inventory counts. While reliable, it lacks a dedicated, high-speed, distributed reservation layer (like Redis Redlock) tailored for hybrid (online + POS) checkout flows. This creates a risk of double-booking under load and slows down the POS experience.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
    A[In-Store POS Terminal] -->|Tap-to-Pay| B(OHC Terminal API)
    C[Online Customer Cart] -->|Checkout| D(OHC Cart API)
    B --> E{Distributed Lock / Redis Redlock}
    D --> E
    E -- Lock Acquired --> F[(PostgreSQL Central Ledger)]
    E -- Lock Denied --> G[Inventory Unavailable Error]
    F -->|Sync| H[Operations Agent]
    H -->|Update Storefront| I(Edge CDN Cache)
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. Priya (boutique owner) scans a "Red Dress" using the OHC mobile POS (375px viewport).
  2. The app displays the item and a "Tap to Pay" button.
  3. When tapped, the system instantly acquires a distributed lock on the inventory ID.
  4. If an online customer is simultaneously viewing their cart with the same item, the Operations Agent pushes a real-time update (via WebSocket/SSE) indicating "Item just sold out in-store."
  5. Priya's POS confirms the sale instantly, and the central ledger is updated asynchronously.

  ### AI Agent Integration
  - The **Operations Agent** monitors the distributed lock state. If a lock is acquired and inventory reaches zero, it immediately triggers the cache invalidator for the storefront edge cache to update the product page to "Sold Out". It also alerts the **Customer Success Agent** to handle any disrupted online carts.

  ## Implementation Prompt
  **Target Persona:** Priya (boutique owner) managing in-store and online sales.

  **Goal:** Build a robust, high-speed distributed locking mechanism for inventory reservations during the checkout process (both online and POS).

  **Acceptance Criteria:**
  1. Implement a distributed lock system (e.g., Redis Redlock) in the OHC backend specifically for inventory items.
  2. Integrate the lock acquisition into both the POS terminal checkout flow and the online cart checkout flow.
  3. The lock duration should be optimized (e.g., 5 minutes for online carts, 15 seconds for rapid POS transactions).
  4. Ensure the lock correctly prevents double-selling when two requests for the last item arrive simultaneously.
  5. Create an E2E Playwright test simulating concurrent online and POS checkouts for the same item to verify the locking mechanism.
  6. The POS mobile UX must provide clear, instant feedback if an item becomes unavailable during the checkout process.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
