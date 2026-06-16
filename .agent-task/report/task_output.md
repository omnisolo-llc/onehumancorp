issue_title: "Implement Offline-First Mobile POS with Redis Distributed Locking for Inventory Sync"
issue_description: |
  # Problem Statement
  Priya, a boutique operator, runs a clothing shop and wants online demand without losing control of in-store operations. She needs inventory-aware offers, product variants, and tap-to-pay visibility. Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism, leading to potential overselling and out-of-sync scenarios between her online storefront and in-person POS. Without offline-tolerant flows, Fatima (food cart operator) also struggles to handle pre-orders and pickup timing smoothly during slow mobile data connections. We need a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp.

  # Research Report
  Our analysis of existing solutions (Shopify, Square, Stripe Terminal) reveals that while they offer POS hardware, their systems often fail micro-SMEs due to complexity or disjointed inventory management unless costly higher-tier plans are employed. The gap in OHC is the lack of a real-time, strongly consistent inventory locking mechanism to handle concurrent online and offline (tap-to-pay) purchases. The system must also gracefully handle offline scenarios with eventual consistency, a feature often lacking in basic POS systems.

  # Design Doc
  **Architecture:**
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally using IndexedDB/SQLite on the mobile app. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  **Mobile UX Flow:**
  - **375px First:** Designed for a 375px viewport with >= 44x44px touch targets. No horizontal scrolling.
  - **Translucent Glass UI:** Adopt macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts.
  - **Optimistic UI:** Updates for inventory changes occur instantly, with clear rollback indicators and toast notifications if a reservation fails upon network sync.
  - **Offline Mode Indicator:** Clear visual cue when operating offline, and a queue count for pending syncs.

  **AI Agent Integration:**
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.

  # Implementation Prompt
  Implement a Redis-backed inventory locking mechanism during the checkout process for concurrent online and POS transactions.
  1. Use Redis Redlock for cross-agent coordination with the lock key pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. Develop the offline-first mobile POS client (Flutter/Tauri) capable of caching catalog data and queueing transactions locally when offline.
  3. Establish a background sync queue that uses eventual consistency to update the PostgreSQL central ledger when network connectivity is restored.
  4. Integrate the Operations Agent to trigger low-stock alerts when inventory thresholds are reached.
  5. The UI must strictly support a 375px viewport, utilize Translucent Glass styling, and provide clear user feedback during synchronization and offline states.

  # Priority
  P1

  # Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []
