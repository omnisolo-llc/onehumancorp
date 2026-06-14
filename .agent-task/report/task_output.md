issue_title: "Centralized Inventory & Distributed POS Synchronization Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Synchronization Architecture

  ## Problem Statement
  Small business owners (like Priya, the boutique owner) need to manage inventory across multiple channels—online stores and physical storefronts. Currently, OHC lacks a robust, real-time inventory locking mechanism and a reliable distributed sync protocol. Without these, merchants face out-of-stock scenarios or double-bookings when online and offline transactions occur simultaneously. Conventional POS systems often fail to seamlessly integrate with online inventory without expensive third-party tools, causing operational friction for micro-SMEs.

  ## Research Report
  - **Market Landscape:** Competitors like Shopify and Wix offer POS integrations but often require higher-tier plans or complex setups that overwhelm small operators. Square and Stripe Terminal provide excellent hardware but lack the integrated AI-driven workflow that OHC envisions.
  - **Identified Gap:** OHC's current architecture does not support optimistic concurrency for inventory or provide a resilient distributed locking mechanism necessary for high-frequency offline transactions (e.g., tap-to-pay) synchronizing with a central online ledger.
  - **Proposed Paradigm:** A centralized ledger (PostgreSQL) paired with a high-speed distributed lock (Redis Redlock) for checkout sessions, complemented by an offline-first POS client that syncs eventual consistency updates.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client - Offline Capable] -->|Syncs| B(Omnichannel Gateway API)
      C[Online Storefront Web] -->|Reserves| B
      B --> D{Distributed Lock Engine - Redis Redlock}
      D -->|Grants Lock| B
      B --> E[Central Ledger - PostgreSQL]
      A -->|Local Cache| F(IndexedDB / SQLite)
      E -->|Replication / Sync| A
      B --> G[Operations AI Agent]
      G -->|Resolves Conflicts| E
  ```

  ### Mobile UX Flow (375px First)
  - **Scenario:** Priya checks out an in-store customer using the OHC mobile app.
  - **UI:** A clean, Unifi-style catalog grid. When an item is selected, an immediate "Hold" is placed on the inventory.
  - **Offline State:** If network connectivity drops, the app switches to an "Offline Mode" indicator. The transaction proceeds, logging locally.
  - **Reconnection:** Upon network restoration, a background sync seamlessly updates the central ledger. If a conflict occurs (e.g., the item was sold online concurrently), an Action Card is generated for Priya to review and resolve via the Operations Agent.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Monitors inventory levels and sync conflicts. If an item is oversold due to an offline sync collision, The Manager drafts an Action Card suggesting a refund or an alternative item offer, minimizing manual intervention.
  - **Customer Success Agent:** Can automatically draft a conciliatory message to the online customer if their order is affected by an in-store out-of-stock event.

  ### Key Design Decisions
  - **Centralized Source of Truth:** PostgreSQL serves as the definitive inventory ledger using row-level locking for critical write paths.
  - **Distributed Reservation:** Implement Redis Redlock to handle temporary inventory holds during checkout flows (e.g., a 15-second hold for tap-to-pay, a 5-minute hold for online carts).
  - **Eventual Consistency for POS:** Mobile clients must be able to finalize sales locally. The system accepts eventual consistency for offline transactions, utilizing AI agents to manage edge-case conflicts rather than blocking the sale.

  ## Implementation Prompt
  **User-Facing Outcome:** Ensure the platform can seamlessly track and lock inventory across online and offline (POS) channels. Establish a Redis-backed distributed lock for active checkout sessions and define the API endpoints for the mobile POS to sync offline transactions.

  **Acceptance Criteria:**
  1. Implement Redis-based distributed locking for inventory items during checkout.
  2. Create API endpoints for the POS client to report offline transactions and sync state.
  3. Ensure PostgreSQL schema supports inventory holds and transaction logs with multi-tenant row-level security.
  4. Provide a mechanism for the Operations Agent to be triggered on inventory conflicts.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
