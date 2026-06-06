issue_title: "Implement Offline Tap-to-Pay Architecture"
issue_description: |
  **Problem Statement:** Non-technical business owners often operate in areas with poor or zero internet connectivity. Currently, OHC requires an active internet connection to process payments, causing lost revenue.

  **Research Findings:**
  - Competitors like Square POS offer offline mode.
  - Stripe Terminal supports offline mode, allowing Tap-to-Pay on iPhone/Android to function without immediate internet access.
  - OHC's current architecture lacks a local queue for transactions and relies on synchronous API calls.

  **Design Doc:**
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter PWA/Mobile App] -->|navigator.onLine == false| B[Local Transaction Queue (SQLite/Isar)]
      A -->|navigator.onLine == true| C[src/server/services/sync/offline_pos.rs]
      B -->|Connectivity Restored| C
      C -->|Batch Sync| D[Stripe Terminal SDK]
      D -->|Success/Failure| E[Operations AI Agent]
      E -->|Alerts| F[Business Owner]
  ```
  ### UI Wireframes/Screen Flow
  1. **Checkout Screen:** Standard Tap-to-Pay UI. If offline, display a subtle "Offline Mode" indicator.
  2. **Payment Confirmation:** Show "Payment Saved Offline" instead of "Payment Successful".
  3. **Dashboard (Reconnected):** Toast notification: "Syncing 3 offline payments...".
  4. **Dashboard (Sync Error):** Alert card: "1 payment failed to sync. Tap to resolve."

  ### Mobile UX Flow
  - 375px viewport optimized.
  - Touch targets >= 44x44px.
  - Use native mobile keyboard for manual entry if needed.
  - Optimistic UI updates with rollback on failure during sync.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the background sync process. If a transaction is declined upon sync, the agent generates a plain-language alert for the owner and drafts a follow-up message to the customer.

  ### Key Design Decisions
  - **CRDT:** Essential for handling offline modifications and syncing without data loss.
  - **Local Storage:** SQLite/Isar chosen for robust, secure local storage.

  **Implementation Prompt:**
  Implement the offline Tap-to-Pay architecture. Create the `offline_pos.rs` sync handler in the backend to process batched offline transactions using a CRDT approach. Update the Flutter client to queue transactions locally (using SQLite/Isar) when `navigator.onLine` is false. The user should see a "Payment Saved Offline" notification, and a background sync should automatically occur when connectivity is restored. The Operations AI agent must be integrated to handle sync failures gracefully.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
