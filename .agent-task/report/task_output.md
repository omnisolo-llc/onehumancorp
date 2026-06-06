issue_title: "[Feature] Offline Edge Inventory Sync Engine"
issue_description: |
  ## Problem Statement
  For mobile-first businesses operating in constrained environments (e.g., Fatima the food cart owner at a crowded street festival, Carlos out of cellular range in a client's basement), network reliability is a major bottleneck. Currently, when they make a sale using Tap-to-Pay, OHC requires an active internet connection to reliably decrement inventory. If the connection drops, they risk either losing sales or unknowingly overselling limited stock. They need a system where inventory continues to function accurately on their device even when fully offline, syncing back seamlessly when they reconnect.

  ## Research Report
  Our deep dive into offline synchronization mechanisms (`[architecture]_offline_edge_inventory_sync.md` and related documents) reveals the following:
  *   **Competitor Landscape:** Square/Stripe Terminal handles offline payments well but often disconnects from complex real-time inventory sync. Shopify POS syncs later but lacks deep, autonomous AI resolution for complex variant oversells without manual merchant intervention.
  *   **Current State:** OHC successfully queues POS transactions offline. However, inventory deduction relies on direct decrement queries on the PostgreSQL backend upon synchronization.
  *   **The Gap:** There is no dedicated edge-to-cloud ledger for tracking offline inventory *modifications*. Relying on absolute decrements from concurrent offline edge devices can cause drift or unpredictable overrides when they eventually reconnect.

  ## Design Doc
  We propose a high-performance offline-first edge inventory sync architecture based on an event-sourcing and CRDT (Conflict-free Replicated Data Type) model.

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge Device
          UI[Mobile UI / 375px] --> LocalStore[(Local Edge Store)]
          LocalStore --> EventLog[Local Event Log]
          LocalStore --> Outbox[Sync Outbox]
      end
      Outbox -- Network Reconnection --> API[Backend Sync API]
      API --> BackendDB[(PostgreSQL / CRDT Ledger)]
      BackendDB --> AI_Agents[AI Operations Agent]
      AI_Agents -. Resolves Oversell Conflicts .-> API
  ```

  ### UI Wireframes & Mobile UX Flow (375px first)
  1.  **Catalog Screen:** The inventory counts are cached locally. If the user marks an item as sold out or completes a sale, the UI updates optimistically.
  2.  **Offline State:** A subtle, premium frosted glass "Offline Mode" indicator appears.
  3.  **Syncing:** Upon reconnection, the outbox flushes. A toast notification confirms sync.
  4.  **Conflict Resolution:** If the backend detects an oversell (e.g., item sold out online while the device was offline), the UI does not show complex error states. The Operations AI agent handles the conflict in the background.

  ### AI Agent Integration Points
  *   **Operations Agent ("The Manager"):** Actively monitors the CRDT sync queue for oversell conflicts. If an item oversells due to concurrent offline operations, it triggers a workflow: notifies the owner via the Business Advisory department and automatically drafts an apology/refund request via the Customer Success agent.

  ### Key Design Decisions
  *   **Event-Sourced Inventory:** Instead of directly updating an `inventory_count`, edge devices push structured deltas/events (e.g., "Sold 2 units of Item A") to a ledger.
  *   **Optimistic UI:** The mobile app relies on local edge storage for instant, zero-latency interactions, masking network latency entirely.

  ## Implementation Prompt
  Implement the edge-sync architecture for inventory modifications.
  1.  Create the backend sync endpoints and conflict resolution logic based on CRDT or event-sourcing principles for inventory items.
  2.  Define the required PostgreSQL schema additions to support the sync outbox and conflict resolution ledger (do not rely on simple integer decrements).
  3.  Implement the local edge storage abstraction on the Flutter client, ensuring offline-first reads for catalog data.
  4.  Add comprehensive automated E2E tests validating the offline-to-online transition and the AI-driven conflict resolution.
  5.  Ensure all UI updates are instant and properly reflect the network state without blocking spinners.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
