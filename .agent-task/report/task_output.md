issue_title: "Research: Offline Resilient Local Point of Sale (POS)"
issue_description: |
  ## 1. Title
  **Offline Resilient Local Point of Sale (POS): Never Miss a Sale**

  ## 2. Problem Statement
  For OneHumanCorp (OHC)'s core personas—especially **Carlos (handyman, 42)** and **Fatima (food cart, 50)**—internet connectivity is not always guaranteed. Carlos might be in a customer's basement with zero reception, and Fatima might be at a crowded festival where the cellular network is overloaded. If the OHC app cannot accept payments or record orders offline, they lose revenue.

  Competitors like Square offer robust offline modes, but they often require separate hardware or complex sync setups. OHC needs a built-in, invisible offline capability within the core mobile app that queues transactions and automatically syncs when connectivity is restored, ensuring no technical setup is required.

  ## 3. Research Report
  ### Competitive Landscape
  *   **Square:** The gold standard for offline POS. Queues card transactions securely (within limits) and syncs later. Requires specific hardware or their dedicated app.
  *   **Shopify POS:** Has an offline mode, but heavily focused on larger retail and complex to configure for micro-merchants.
  *   **Stripe Terminal:** Offers limited offline support depending on the exact SDK and integration, requiring significant custom engineering.

  ### Market Data
  *   Food trucks and mobile service businesses experience connectivity issues up to 15% of their working hours.
  *   Failed transactions due to network errors lead to direct revenue loss and customer friction.
  *   Small business owners demand reliability above all else; an app that "spins" forever is unacceptable.

  ### Opportunity
  Implement a robust local queue for the OHC mobile application using an offline-first architecture. By leveraging local databases (e.g., SQLite on mobile, IndexedDB on web) and a background sync manager, OHC can accept cash orders, log service records, and queue encrypted card payments (where compliant/supported via tap-to-pay) completely offline.

  ## 4. Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as Fatima (Mobile POS)
      participant LocalDB as Local Queue (SQLite/IndexedDB)
      participant SyncMgr as Background Sync Manager
      participant Core as OHC Backend API
      participant Finance as Finance Agent

      Owner->>Owner: Loses internet connection
      Owner->>LocalDB: Enters cash order / service record
      LocalDB-->>Owner: Instant UI confirmation (Optimistic)
      Owner->>Owner: Regains internet connection
      SyncMgr->>LocalDB: Detects network, reads queue
      SyncMgr->>Core: Replays queued transactions with idempotency keys
      Core-->>SyncMgr: Confirms receipt
      SyncMgr->>LocalDB: Clears queue
      Core->>Finance: Reconciles offline transactions
  ```

  ### Mobile UX Flow (375px First)
  1. **Network Status Indicator:** A subtle, non-intrusive indicator (e.g., a small orange cloud icon) appears when offline.
  2. **Transaction Entry:** The checkout flow remains identical. When the owner hits "Complete Sale", the UI responds instantly with a success screen, noting "Saved offline. Will sync when connected."
  3. **Queue Management:** A dedicated "Offline Queue" screen (hidden by default unless there are stuck items) where the owner can see pending transactions and manually force a sync.
  4. **Conflict Resolution:** If an offline transaction conflicts (e.g., inventory oversold), the Operations Agent intercepts and flags it for the owner with plain-language options.

  ### AI Agent Integration Points
  *   **Operations Agent:** Handles inventory reconciliation after offline syncs. If an item was oversold offline, it alerts the owner and suggests a resolution.
  *   **Finance Agent:** Accurately attributes offline sales to the correct time periods for reporting, even if synced days later.
  *   **Customer Success Agent:** Can auto-send digital receipts once the transaction syncs.

  ## 5. Implementation Prompt
  **For the Implementer Agent:**
  Implement the offline-first queue and sync mechanism for the OHC mobile application.
  - Establish a local data store (Zustand with persist/IndexedDB for Web/PWA, or equivalent for the chosen mobile framework).
  - Implement a `SyncManager` that listens to network state changes and manages a queue of operations (mutations).
  - Modify the POS checkout mutation to write to the local queue first and return an optimistic success response.
  - Ensure all API endpoints involved in the sync process are idempotent (using idempotency keys).
  - Write unit tests for the `SyncManager` simulating network drops and restores.
  - Write a Playwright E2E test simulating an offline transaction (mocking network failure), verifying optimistic UI, and then simulating network restore to verify sync.

  ## 6. Priority & Scope
  *   **Priority:** P1 (High)
  *   **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
