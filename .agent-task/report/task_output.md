issue_title: "[Architecture] Offline-First Tap-to-Pay POS Synchronization Mesh"
issue_description: |
  # [Architecture] Offline-First Tap-to-Pay POS Synchronization Mesh

  ## Problem Statement
  Small business owners like Fatima (food cart) and Priya (boutique/market stall) operate in unpredictable physical environments where cellular networks are often congested or unavailable. If OHC loses connection during an in-person transaction, the entire business halts—Fatima can't process payments, check her inventory, or manage her pre-orders. While digital storefronts are robust, the in-person physical sales experience lacks the resilience provided by an edge-cached, offline-first data synchronization mesh. Small businesses need the ability to process seamless "Tap-to-Pay" transactions directly on their smartphones and manage their operations completely offline, with the app transparently caching the data and queueing actions to sync when the network returns.

  ## Research Report
  - **Competitive Landscape**: Both Square and Shopify offer offline modes for their Point-of-Sale apps. However, these solutions often trap merchants into buying proprietary dongles or paying for expensive POS add-ons. Wix also lacks robust local-first caching for its mobile apps.
  - **The Missing Capability**: OneHumanCorp is missing a unified Offline-First Synchronization Mesh built on local CRDTs (Conflict-Free Replicated Data Types) that treats the smartphone's local database as the primary source of truth, enabling immediate UX feedback and zero downtime, while intelligently pushing to the centralized Postgres datastore.
  - **Hardware Independence**: By combining local-first sync with native Apple and Google "Tap to Pay" SDKs, OHC can completely eliminate the need for POS dongles while matching or exceeding Square’s offline reliability.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> LocalDB[(Local SQLite / CRDT)];
          App --> NativeSDK[Apple/Google Tap-to-Pay SDK];
          LocalDB --> SyncQueue[Offline Sync Engine];
          NativeSDK --> LocalDB: Record Payment Intent;
      end

      SyncQueue -- Network Restored --> Gateway[OHC API Gateway];

      subgraph OHC Cloud (Multi-Tenant)
          Gateway --> SyncManager[CRDT Sync Manager];
          SyncManager --> MainDB[(Cloud Postgres)];
          SyncManager --> Agents[AI Agent Swarm];
      end

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Reconcile Inventory Conflicts];
          Agents --> FinanceAgent[Finance: Ledger Sync & Tap-to-Pay Settlement];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Instant Loading**: App loads the catalog and current orders instantly (<50ms) from the local SQLite CRDT store.
  2. **Cart & Checkout**: Fatima adds items to the cart. If offline, the UI shows a subtle "Offline - Syncing later" glassmorphic pill. She taps "Charge".
  3. **Payment Processing**: The native Tap-to-Pay interface appears. The customer taps their card. The encrypted payment intent and updated inventory are written to the `LocalDB`.
  4. **Background Sync**: Once the phone regains connectivity, the `SyncEngine` batches local transactions and pushes them to the `SyncManager` without blocking the UI.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the synchronization manager. If Fatima sold a cake offline that was simultaneously purchased online (conflict), the agent instantly flags the order, holds the funds, and messages Fatima via the Omnichannel Inbox to resolve the double-sell.
  - **Finance Agent**: Automatically reconciles the delayed Tap-to-Pay captures with the main ledger once they sync, generating an accurate daily P&L.

  ### Key Design Decisions
  - **CRDT-Based SQLite**: We must use local-first databases with conflict-free replicated data types to ensure all multi-tenant state changes (inventory, orders) can happen deterministically while offline.
  - **Native-Only Tap-to-Pay**: Absolute zero reliance on third-party hardware dongles; all physical payments go through the smartphone's built-in NFC securely.

  ## Implementation Prompt
  Implement the Offline-First Tap-to-Pay POS Synchronization Mesh.
  - **User-Facing Outcome**: The mobile app functions with sub-50ms latency regardless of network state. Users can add products to cart, update inventory, and process "Tap-to-Pay" transactions offline. Data syncs invisibly in the background when connectivity is restored.
  - **CUJ**:
    1. User opens the OHC app in airplane mode.
    2. User adds items to the cart and taps "Charge".
    3. Customer taps card against the phone (native SDK logs intent).
    4. Transaction completes locally; inventory decrements immediately in the UI.
    5. Network is restored; changes automatically sync to OHC Cloud and agents reconcile.
  - **Acceptance Criteria**:
    - The client-side application uses an embedded SQLite database configured for CRDTs.
    - A background sync service reliably transfers the local queue to the cloud API upon reconnection.
    - No synchronization jargon or spinners block the user; "offline" states are handled with subtle glassmorphic indicators.
    - Apple/Google Tap-to-Pay flows integrate directly with the local queue.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
