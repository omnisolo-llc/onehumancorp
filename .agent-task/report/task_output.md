issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with hybrid sales models (e.g., Priya the boutique owner) face severe inventory management pain points. When operating both an online storefront and an in-store Point-of-Sale (POS) system, lack of real-time inventory synchronization leads to double-booking and out-of-stock scenarios. Current solutions either require expensive, complex integrations (like advanced Shopify tiers or third-party apps) or fail entirely, disrupting the customer experience.

  ## Research Report
  - **The Gap**: OHC currently lacks a real-time, strongly consistent inventory reservation mechanism and a distributed sync protocol.
  - **Persona Focus**: Priya (boutique owner) requires a system where an in-store tap-to-pay transaction instantly reserves and deducts stock, preventing an online customer from purchasing the same item simultaneously.
  - **Proposed Solution**: A centralized inventory ledger (PostgreSQL) paired with a distributed locking mechanism (Redis Redlock) for temporary inventory reservations during checkout. This system must also support eventual consistency for offline POS clients.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      OnlineStore[Online Storefront] --> Checkout
      POS[Mobile POS Client] --> Terminal[Stripe Terminal]
      Terminal --> Checkout
      Checkout --> Redlock[Redis Redlock: Reserve Inventory]
      Redlock --> |Lock Acquired| Ledger[PostgreSQL: Central Ledger]
      Ledger --> OperationsAgent[The Manager Agent]
      OperationsAgent --> |Low Stock Alert / Restock Suggestion| OwnerDashboard[Owner Dashboard]
  ```

  ### Mobile UX Flow (375px)
  1.  **Online Customer**: Browses the store, adds an item to the cart. If the item is locked (reserved by a POS transaction), the UI gracefully displays an "Item just sold out" or "Item currently unavailable" message.
  2.  **In-Store POS (Priya)**: Uses the mobile app to scan/select an item. The checkout process initiates a 15-second Redis lock on the item's inventory. Upon successful payment (Stripe Terminal), the inventory is permanently deducted in the PostgreSQL ledger.
  3.  **Owner Notifications**: The Operations Agent monitors the ledger and pushes low-stock notifications or restock suggestions to Priya's mobile feed.

  ### AI Agent Integration
  -   **Operations Agent ("The Manager")**: Monitors stock levels, handles sync conflicts, and triggers low-stock push notifications or drafts restock orders.
  -   **Finance Agent ("The Accountant")**: Correlates POS data with online sales for unified reporting.
  -   **Customer Success Agent ("The Ambassador")**: Updates online storefront availability based on real-time inventory.

  ### Key Design Decisions
  -   **Redis Redlock**: Chosen for its robust distributed locking capabilities, essential for preventing race conditions between concurrent online and offline transactions.
  -   **Offline-First POS**: The POS client caches data and utilizes eventual consistency to sync offline sales once the network is restored.

  ## Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: A robust system where inventory is flawlessly synchronized between online and in-store channels, managed invisibly by the Operations Agent.

  **Acceptance Criteria**:
  1.  **Redis Redlock Integration**: Implement a Redis-backed reservation service that locks inventory items during the checkout flow (both online and POS).
  2.  **Central Ledger Updates**: Ensure the PostgreSQL ledger is accurately updated upon transaction completion and that locks are released.
  3.  **Offline POS Sync**: Define the `TerminalSession` data schema to support offline-sync reconciliation.
  4.  **Operations Agent Capabilities**: Extend the Operations Agent to track real-time stock and trigger notifications when inventory thresholds are met.
  5.  **E2E Playwright Tests**: Create a test simulating simultaneous online and POS checkout attempts for the same limited-stock item, verifying the Redis lock prevents double-booking.

  ## Priority
  P1

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
