issue_title: "Implement Offline-First Mobile POS and Sync Reconciliation Engine"
issue_description: |
  # Research Report: Offline-First Mobile POS & Sync Reconciliation

  ## 1. Problem Statement
  Small business owners operating in challenging network conditions (e.g., Fatima managing her food cart in areas with patchy 4G, or Priya selling at a weekend pop-up market) face critical friction when their POS or inventory tools fail offline. Existing platforms either prevent actions completely without a connection or show generic errors, leading to lost sales. OHC needs a robust, offline-first mobile architecture that allows users to seamlessly manage inventory, process local cash/tap-to-pay offline operations, and securely sync state back to the backend once connectivity is restored.

  ## 2. Research & Competitive Analysis
  - **Square POS:** The industry leader in offline mode, allowing merchants to process cash and offline card payments seamlessly when the network drops. Transactions are queued locally and synced later.
  - **Shopify POS:** Offers offline capabilities primarily for cash. Card payments require a connection. Inventory is cached locally but true optimistic sync for complex catalog updates is limited.
  - **Wix/Squarespace:** Primarily online-dependent. Robust offline-first POS and inventory management are not deeply integrated.

  **OHC Gap:** OHC currently lacks an offline-first POS experience. We need an Optimistic Mutation Engine coupled with a local-first datastore and a robust sync mechanism.

  ## 3. Design Doc

  ### Architecture
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> LocalDB[(Local Cache / CRDT)];
          App --> TapToPay[Native Tap-to-Pay SDK];
          LocalDB --> SyncEngine[Offline Sync Engine];
          TapToPay --> LocalDB: Record Encrypted Payment Intent;
      end

      SyncEngine -- Network Restored --> Gateway[OHC API Gateway];
      Gateway --> MainDB[(Cloud Ledger)];
      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Update Inventory & Reconcile Conflicts];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** Features a "New Sale" FAB using macOS-style Translucent Glass materials.
  2. **Offline Status:** A subtle pill at the top indicates "Offline Mode" when the network drops.
  3. **Cart & Checkout:** Users add items and select "Tap to Pay" or "Cash". Actions are optimistic and instantly update the local UI.
  4. **Syncing:** When back online, the background worker syncs intents. If conflicts occur (e.g., item sold out online), the Operations Agent resolves them and notifies the user gracefully.

  ## 4. Implementation Prompt
  **Feature:** Offline-First Mobile POS & Optimistic Sync Engine
  **Target Personas:** Fatima (Food Cart), Priya (Boutique)

  **User-Facing Outcome:** Users can operate the mobile POS seamlessly offline. They can add items to a cart, record cash sales, and initiate Tap-to-Pay. The app remains responsive. Once online, the app syncs automatically.

  **Critical User Journey (CUJ):**
  1. Fatima opens the app while offline (network indicator shows "Offline Mode").
  2. She adds a "Falafel" to the cart and records a cash sale.
  3. The inventory decreases optimistically on her screen.
  4. Later, when connected to Wi-Fi, the app syncs the transaction to the OHC backend.
  5. The backend reconciles the POS session, updating the central ledger and resolving any inventory conflicts.

  **Next Actions for Engineering:**
  1. **Frontend (Mobile App):** Implement the local outbox queue and the offline mutation engine. Update the dashboard UI to show the "Offline Mode" pill and handle optimistic updates for inventory decrement and sale recording.
  2. **Backend (Server):** Implement idempotent sync endpoints to process batched offline operation payloads.
  3. **Reconciliation:** Ensure the backend properly updates POS session states and inventory counts, invoking the Operations Agent on state drift.

  **Acceptance Criteria:**
  - App functions offline without blocking user actions.
  - UI strictly adheres to mobile-first 375px design and glassmorphism.
  - Background sync successfully reconciles with the backend.
  - Comprehensive unit and Playwright E2E tests for the offline flow.

  ## 5. Priority & Scope
  - **Priority:** P0
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
