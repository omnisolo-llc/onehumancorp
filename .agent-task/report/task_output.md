issue_title: "Implement Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. Target Persona: Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).

  ## Research Report
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      CentralLedger[Central Inventory Ledger] --> Inventory[Inventory Tracking];
      DistributedLock[Distributed Lock Service] --> Lock[Temporary Reservation];
      POSClient[Mobile POS Client] --> OfflineCache[Offline Catalog Cache];
      POSClient --> Checkout[Checkout Flow];
      Checkout -.-> Lock;
      Checkout -.-> Inventory;
  ```

  ### UI Wireframes & Mobile UX Flow
  - **Mobile-First UX Flow:** The POS interface operates strictly on a 375px viewport (mobile-first). The checkout process starts with a grid of products. Tapping a product adds it to the cart. During the tap-to-pay flow, the cart is locked.
  - **Touch Targets:** All inventory adjustment and checkout buttons are ≥ 44x44px.
  - **Offline/Optimistic States:** Implement optimistic UI updates for inventory changes. If the reservation fails, the UI gracefully rolls back and shows a translucent glass notification to the user.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - Use a distributed locking mechanism to reserve items temporarily during checkout to prevent double booking.
  - Use a central ledger as the ultimate source of truth, enforcing strict locking or optimistic concurrency.
  - Mobile client must operate offline-first for POS, syncing eventual consistency when network restores.
  - All complex integration is hidden behind a clean, translucent glass UI, and AI agents handle conflict resolution.

  ## Implementation Prompt
  Implement a robust, distributed inventory synchronization protocol for OneHumanCorp's Point-of-Sale (POS) and online storefront.

  **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system temporarily reserves the item during the transaction.
  4. The online customer sees the item is reserved or out of stock before they can complete checkout.
  5. The POS system finalizes the transaction, updating the central inventory and triggering AI workflows.

  **Acceptance Criteria:**
  - Implement a temporary inventory reservation mechanism during checkout (e.g., locking).
  - Establish a central inventory ledger with concurrency control for critical updates.
  - Build offline-first POS client sync capabilities that eventually reconcile with the central ledger upon network restoration.
  - Ensure all updates seamlessly trigger AI Agent (Manager, Accountant, Ambassador) workflows without blocking the primary transaction path.

  **Estimated Scope:** Large
  **Priority:** P0

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
