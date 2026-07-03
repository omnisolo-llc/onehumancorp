issue_title: "Architectural Design: Offline-First Zero-Trust Mobile POS & Tap-to-Pay Synchronization"
issue_description: |
  ## Title
  Architectural Design: Offline-First Zero-Trust Mobile POS & Tap-to-Pay Synchronization

  ## Problem Statement
  For physical business operators and location managers (like Fatima, the Food Cart Operator, and Priya, the Boutique Operator), internet connectivity is often unreliable. Slow mobile data, temporary dead zones in a crowded market, or building interference can bring sales to a halt. Existing systems (like Square or basic Shopify POS) often fail or delay transactions when offline, frustrating the owner and the customer. They need an offline-tolerant, mobile-first Point-of-Sale (POS) flow that securely caches product catalogs and prices locally, gracefully captures tap-to-pay or cash transactions offline, and automatically synchronizes to the cloud ledger via a reliable background queue when the network is restored.

  ## Research Report
  - **Market Context**: Square dominates the small physical merchant space with robust offline mode, but it does not integrate with an overarching AI assistant that anticipates inventory shortages or handles omnichannel customer interactions. Shopify POS handles online-offline convergence well but requires expensive hardware and premium plans for advanced features.
  - **The OHC Opportunity**: By building an offline-first mobile POS architecture directly into the OHC Flutter app and backing it with an Edge-Sync queue, OHC can capture the physical point of sale. The AI agents (Finance and Operations) can then use this data to trigger inventory reorders, daily summaries, and customer follow-ups without the owner managing a separate POS tool.
  - **Competitor Gaps**:
    - *Square*: Powerful POS but lacks an AI work-assistant layer; owner still has to analyze the data manually.
    - *Shopify POS*: Complex multi-app ecosystem; weak offline resilience without specific hardware.
    - *Stripe Terminal*: Great SDKs, but requires the platform to implement offline queuing and sync mechanics themselves.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter)
      participant EdgeQueue as Local SQLite Queue
      participant Terminal as Stripe Terminal SDK
      participant API as OHC API Gateway
      participant DB as Postgres (Tenant Ledger)
      participant Agent as Agent Event Bus

      App->>App: Load cached catalog
      App->>Terminal: Initiate Tap-to-Pay (Offline)
      Terminal-->>App: Payment token / Auth
      App->>EdgeQueue: Store pending transaction & intent
      App-->>App: Show "Payment Saved" UI

      rect rgb(240, 248, 255)
      Note over App, DB: Network Restored (Background Sync)
      EdgeQueue->>API: Sync pending transaction
      API->>DB: Commit to Ledger (Idempotent)
      DB-->>API: Success (Tx ID)
      API->>Agent: Publish "OfflineSaleSynced" event
      Agent->>Agent: Finance Agent updates daily summary
      API-->>App: Acknowledge Sync
      End
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **POS Home (Cached)**: A grid of top-selling items with large touch targets. Network status indicator ("Offline" in subtle amber) at the top.
  2. **Cart Drawer**: Slides up from the bottom. Shows total, taxes, and a massive "Tap to Pay" / "Cash" action button.
  3. **Payment State**: Full-screen modal showing a pulsing card reader icon. If offline, instantly transitions to a green "Saved for Sync" checkmark.
  4. **Sync Center**: A hidden "Advanced" drawer that shows pending offline items, avoiding cognitive load unless an error occurs.

  ### Mobile UX Flow
  - **Zero-Friction Checkout**: Checkout flow requires no loading spinners when offline. Validations happen strictly against the local cache.
  - **Truthful UI**: The owner is informed that the sale is captured locally, establishing trust.
  - **Haptic Feedback**: Success and failure states leverage native haptics to confirm actions in loud environments (e.g., a food cart).

  ### AI Agent Integration Points
  - **Operations Agent**: Automatically calculates total daily physical sales when offline queues flush and cross-references with online sales.
  - **Finance Assistant**: Detects offline sync failures (e.g., declined offline card) and drafts a plain-language summary for the owner.
  - **Inventory Agent**: Decrements local inventory counts speculatively and reconciles upon sync.

  ### Key Design Decisions and Why
  - **Local SQLite over SharedPreferences**: Required for robust, transactional offline queuing that won't corrupt on crash.
  - **Idempotency Keys**: Every offline transaction is assigned a UUIDv4 on the device. The backend ensures exact-once processing during sync retries.
  - **Optimistic UI Updates**: The app assumes the transaction will eventually succeed, keeping the line moving for the operator.

  ## Implementation Prompt
  **Feature Name**: OHC Offline-First POS & Sync Queue
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima can accept pre-orders and physical payments rapidly during a rush hour, even if her cell signal drops. The app caches the menu, queues the payments locally, and syncs them automatically when signal is restored without interrupting her workflow.

  **Next Actions**:
  1. Implement a local SQLite repository in the Flutter app to cache the product catalog and queue pending transactions.
  2. Build the background sync worker that monitors network status and drains the local queue using idempotency keys.
  3. Design the 375px mobile POS screen with large touch targets, optimistic UI state, and a clear network/sync status indicator.
  4. Implement the backend API endpoint to receive batched offline transactions and publish `OfflineSaleSynced` events to the message bus for the Finance Agent.

  **Acceptance Criteria**:
  - App can display catalog and process a cart to "Pending Sync" state with network disabled.
  - Background sync successfully commits queued transactions to the backend exactly once when network is restored.
  - The UI accurately reflects "Offline" and "Synced" states.
  - Playwright/E2E test verifies the offline cart flow and sync mechanism.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
