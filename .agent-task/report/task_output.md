issue_title: "Implement CRDT-Based Offline-First Mobile Tap-to-Pay POS with AI Operations Sync"
issue_description: |
  ## Title
  Implement CRDT-Based Offline-First Mobile Tap-to-Pay POS with AI Operations Sync

  ## Problem Statement
  Non-technical SMB owners like Priya (boutique owner) and Fatima (food cart) sell products in-person and online. Currently, if they lose network connectivity (common for food carts or pop-up shops), they cannot process payments, view active orders, or log sales, halting their operations. Furthermore, physical in-store tap-to-pay sales on their phone are isolated from the global online inventory ledger, leading to double-selling or manual reconciliation. They need an offline-first POS that accepts tap-to-pay directly on their smartphones without dongles, logging transactions locally via CRDTs, with an AI Operations Agent resolving inventory conflicts invisibly in the background when network connectivity is restored.

  ## Research Report
  - **Shopify**: Requires a separate POS app for robust offline functionality. Limited offline modes based on the payment terminal used, adding hardware friction.
  - **Square**: Dominant for physical POS and tap-to-pay, but requires ecosystem lock-in and its e-commerce builder lacks the depth of pure digital platforms.
  - **Wix**: Offers a POS app with tap-to-pay, but offline capabilities are extremely limited.
  - **Data**: Mobile checkout completion drops by over 30% when network latency exceeds 3 seconds. Merchants at pop-up markets report connectivity as a top 3 operational hurdle.
  - **OHC Opportunity**: OHC can differentiate by deeply integrating native Apple/Google Tap-to-Pay SDKs with an optimistic mutation engine powered by CRDTs (e.g., local SQLite). The AI Operations department handles reconciliation, making it an "invisible" enterprise-grade feature for everyday SMBs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device (375px App)
          UI[OHC Mobile App] --> LocalDB[(Local SQLite / CRDT)];
          UI --> TapToPay[Native Tap-to-Pay SDK];
          LocalDB --> SyncEngine[Offline Sync Engine];
          TapToPay --> LocalDB: Record Encrypted Intent / Cash Sale;
      end
      SyncEngine -- Network Restored --> Gateway[OHC API Gateway];
      Gateway --> Stripe[Stripe Terminal API];
      Gateway --> MainDB[(Cloud Postgres)];
      Gateway --> Agents[AI Agent Swarm];
      subgraph Agent Departments
          Agents --> OpsAgent[Operations: Resolve Inventory Conflicts];
          Agents --> FinanceAgent[Finance: Reconcile Offline Ledger];
      end
  ```
  ### Mobile UX Flow (375px First)
  1. Priya opens the OHC app dashboard. A macOS-style Translucent Glass "New Sale" button is immediately accessible at the bottom.
  2. She selects items from her visual catalog. Interaction is sub-50ms because it reads from local CRDT SQLite.
  3. She taps "Charge $45.00". A bottom sheet (glassmorphism UI) offers "Tap to Pay" or "Cash".
  4. She chooses "Tap to Pay". The native iOS/Android system UI appears for the customer to tap their card directly on her phone.
  5. In airplane mode or weak signal, a subtle "Offline - Syncing later" badge appears. Cash and recorded pre-orders are logged locally.
  ### AI Agent Integration Points
  - **Operations Agent**: Listens to sync events from the local CRDT store. If an item sold out offline was simultaneously sold online, it resolves the conflict by prioritizing the physical sale and emailing the online customer with an automated apology and alternative offer.
  - **Finance Agent**: Reconciles batched offline transactions to ensure daily ledger accuracy and generates plain-language summaries for the owner.
  ### Key Design Decisions
  - **Offline-First CRDTs**: Use a local-first database (like SQLite with CRDTs) to ensure the app never blocks on network requests.
  - **Dongle-less Payments**: Rely exclusively on native Apple/Google Tap-to-Pay SDKs to remove hardware friction.
  - **Unified Inventory**: Physical and digital sales hit the same underlying ledger without a separate POS system.

  ## Implementation Prompt
  **User-Facing Outcome**: Merchants can open the OHC app, build a cart, and process sales via Tap-to-Pay on their phone. The system remains blazingly fast and functional even in airplane mode, syncing securely in the background when connectivity returns.

  **CUJ (Critical User Journey)**:
  1. User adds items to the cart in the 375px mobile app.
  2. User selects "Tap to Pay" and customer taps their card.
  3. Payment succeeds and local inventory decrements immediately.
  4. User goes offline (airplane mode), logs a cash sale, and app continues functioning without blocking.
  5. User goes online, and the app invisibly syncs the offline CRDT state to the cloud.

  **Acceptance Criteria**:
  - Native Tap-to-Pay flows triggered correctly on supported devices.
  - App state (orders, inventory) fully readable and writable offline.
  - Background AI Operations agent accurately syncs restored offline data to the global PostgreSQL ledger.
  - Playwright E2E tests verify the offline-to-online transition, local mutations, and inventory resolution.
  - The UI strictly adheres to the glassmorphism and card-based design system on a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
