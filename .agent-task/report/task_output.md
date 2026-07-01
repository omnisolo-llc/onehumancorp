issue_title: "[Research] Offline-Tolerant Mobile POS & Pre-order System"
issue_description: |
  # Research Report: Offline-Tolerant Mobile POS & Pre-order System

  ## 1. Problem Statement
  Service-based and food-cart small business owners (e.g., Fatima the Food Cart Operator) struggle with unstable internet connections and slow mobile data while operating their daily work. Existing point-of-sale (POS) and pre-order systems fail in low-connectivity environments, resulting in missed orders, double bookings of limited stock, and an inability to process payments on-the-go. Additionally, these users require a simple, offline-capable order list and the ability to operate their business without interruption when network signals drop.

  ## 2. Research Report
  - **Market Context**: Platforms like Square and Shopify offer robust POS solutions, but their full functionality heavily relies on a continuous internet connection. When offline, many features like inventory sync and real-time pre-order notifications are disabled or delayed.
  - **The OHC Opportunity**: By implementing an offline-tolerant architecture with local data caching and asynchronous sync protocols, OHC can capture the market of operators working in low-connectivity environments (e.g., street vendors, remote service workers).
  - **Competitor Gaps**:
    - *Shopify POS*: Offline mode is limited mostly to accepting cash payments; complex inventory syncs often fail or cause conflicts upon reconnection.
    - *Square*: Strong hardware, but the software lacks deep agentic integration that can intelligently merge offline conflicts or automatically draft customer communications about delayed orders.
    - *Wix*: Lacks a robust offline-first mobile POS experience entirely.

  ## 3. Design Doc

  ### High-Level Architecture (Offline-First Sync Protocol)
  - **Local State & Storage (Flutter Client)**: The mobile app utilizes SQLite (or similar local persistence) to cache the daily menu, active pre-orders, and inventory counts. All read operations hit this local cache first.
  - **Queue-Based Sync Engine**: When a transaction (order update, cash payment) occurs offline, it is written to a local SQLite-backed queue. Once the network connection is restored, a background worker systematically flushes this queue to the OHC Backend API.
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth. It uses optimistic concurrency control (versioning) to handle conflicting updates from the offline client and online pre-orders.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Fatima (Mobile App)
      participant Local Storage (SQLite)
      participant Operations Agent
      participant OHC API
      participant Database (PostgreSQL)

      Fatima (Mobile App)->>Local Storage (SQLite): Update order status (Offline)
      Local Storage (SQLite)-->>Fatima (Mobile App): Acknowledge local update
      Note over Fatima (Mobile App), Local Storage (SQLite): Network Restored
      Fatima (Mobile App)->>OHC API: Sync queued local updates
      OHC API->>Database (PostgreSQL): Attempt commit with version check
      alt Conflict Detected (e.g. online pre-order modified same record)
          Database (PostgreSQL)-->>OHC API: Conflict Error
          OHC API->>Operations Agent: Trigger conflict resolution protocol
          Operations Agent->>Database (PostgreSQL): Apply intelligent merge
          Operations Agent-->>Fatima (Mobile App): Send summary notification
      else No Conflict
          Database (PostgreSQL)-->>OHC API: Success
          OHC API-->>Fatima (Mobile App): Sync complete
      end
  ```

  ### Mobile UX Flow (375px)
  1. **Dashboard & Order List**: A simplified, high-contrast list view optimized for outdoor visibility. Features a clear visual indicator (e.g., a colored dot) showing network status (Online vs. Offline Mode).
  2. **Offline Interaction**: When offline, Fatima can still tap an order to mark it "Ready for Pickup" or process a cash transaction. The UI updates instantly (optimistic update) with a subtle "Pending Sync" badge on the item.
  3. **Auto-Recovery**: Upon regaining connectivity, the "Pending Sync" badges disappear sequentially as the background worker flushes the local queue. A toast notification summarizes the successful sync.

  ### AI Agent Integration
  - **Operations Agent**: Monitors the sync process. If an inventory conflict occurs (e.g., Fatima sold the last Falafel offline while an online pre-order claimed it simultaneously), the agent intelligently resolves it based on predefined tenant rules (e.g., prioritize in-person sales) and flags it for review.
  - **Customer Success Agent**: Automatically detects if an online pre-order is delayed or cancelled due to an offline sync conflict, and drafts an apologetic SMS (with a discount offer) for Fatima to approve and send.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Offline-Tolerant POS & Pre-order Engine
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima can seamlessly process orders and manage her daily queue on a low-end Android phone, regardless of cellular connection drops. Her local actions sync gracefully without manual intervention.

  **Critical User Journey (CUJ)**:
  1. Fatima's phone loses signal.
  2. She marks 3 pre-orders as "Completed" and updates inventory for a local cash sale.
  3. The UI reflects these changes immediately.
  4. 10 minutes later, the signal returns. The app silently syncs her actions to the backend.
  5. The Operations Agent resolves any inventory conflicts and updates the online menu.

  **Next Actions**:
  1. Implement the local SQLite-based sync queue in the Flutter client architecture.
  2. Develop the backend API endpoints equipped with optimistic concurrency control (version checks) to receive and process offline batches.
  3. Integrate the Operations Agent to handle conflict resolution when the sync engine encounters a version mismatch.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
