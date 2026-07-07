issue_title: "OHC Offline-First Multi-Tenant Inventory Sync Protocol for Mobile POS"
issue_description: |
  # Mission Queue Protocol: Offline-First Multi-Tenant Inventory Sync Protocol for Mobile POS

  ## Problem Statement
  Small business owners like Priya (boutique operator) face critical issues when managing inventory across multiple channels, especially during simultaneous online and offline purchases. Legacy platforms often suffer from synchronization delays leading to double-booking or out-of-stock scenarios. OHC needs a robust, real-time, strongly consistent inventory locking and caching mechanism, alongside a distributed sync protocol to handle hybrid merchant operations seamlessly from a mobile device.

  ## Research Report
  - **Market Context**: Competitors like Shopify offer robust POS capabilities but often struggle with real-time inventory sync across online and in-person channels without costly third-party tools or higher-tier plans.
  - **OHC Opportunity**: By implementing an offline-first inventory sync protocol natively within OHC, we can provide non-technical users with a seamless, agent-assisted experience that completely mitigates double-booking and out-of-stock scenarios, setting a new standard for SMB operations.
  - **Competitor Gaps**:
    - *Shopify*: Syncing offline sales can be delayed; requires third-party apps for deep multi-channel inventory management for micro-SMEs.
    - *Square/Stripe Terminal*: Strong POS hardware but lacks integrated agentic workflow automation for unified business operations.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client] -->|Offline Sync/Cache| B(Local Storage)
      B -->|Network Restored| C{Eventual Consistency Sync}
      C --> D[Central Ledger PostgreSQL]
      A -->|Checkout Reservation| E[Distributed Locks Redis Redlock]
      E --> D
      D --> F[Operations Agent]
      D --> G[Finance Agent]
      D --> H[Customer Success Agent]
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL)**: Source of truth for inventory counts, utilizing row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock)**: Temporary inventory reservation system during checkout to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client**: Mobile POS client caches catalog data locally and syncs finalized offline sales asynchronously with the central ledger when the network is restored.

  ### Mobile UX Flow (375px)
  1. **POS Interface**: A clean, touch-friendly interface for inventory adjustment and checkout (touch targets ≥ 44x44px).
  2. **Optimistic UI**: Implement optimistic UI updates for inventory changes, showing truthful pending/error states with rollback capabilities if the Redis reservation fails.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager")**: Monitors stock levels, tracks incoming orders, triggers low-stock alerts, coordinates sync reconciliations, and suggests restock plans.
  - **Finance Agent ("The Accountant")**: Processes splits for Terminal transactions and correlates POS data for unified financial reporting.
  - **Customer Success Agent ("The Ambassador")**: Automatically updates online storefront availability and notifies customers if an item becomes unavailable due to an in-store purchase.

  ## Implementation Prompt
  **Feature Name**: OHC Offline-First Multi-Tenant Inventory Sync Protocol for Mobile POS
  **User Facing Outcome**: Business owners like Priya can seamlessly process in-store and online sales without fear of double-booking or inventory discrepancies. The system handles synchronization automatically in the background, providing peace of mind and accurate stock levels.
  **CUJ (Critical User Journey)**:
  1. Priya opens the OHC mobile app (375px) and navigates to the POS interface.
  2. She selects an item to check out for an in-store customer.
  3. The system immediately reserves the item using Redis Redlock.
  4. If the network is unavailable, the sale is recorded locally.
  5. Upon network restoration, the app asynchronously syncs the sale with the Central Ledger (PostgreSQL), updating the global inventory.
  6. The Customer Success agent updates the online storefront to reflect the new inventory count.
  **Acceptance Criteria**:
  - Implementation of Central Ledger (PostgreSQL) and Distributed Locks (Redis Redlock).
  - Offline-first capabilities in the mobile POS client with eventual consistency sync.
  - Seamless AI Agent coordination for monitoring, financial processing, and customer notification.
  - Mobile-first design adherence (375px viewport, touch targets ≥ 44x44px, optimistic UI updates).
  - 100% unit test coverage for new/modified code.
  - Comprehensive Playwright E2E tests validating the CUJ.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []