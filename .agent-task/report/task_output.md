issue_title: "Implement POS Offline-Sync Reconciliation with PostgreSQL Central Ledger"
issue_description: |
  # Research Report: POS Offline-Sync Reconciliation

  ## Product-use Evidence (Dogfooding)
  - **Persona:** Priya (Boutique Owner)
  - **Browser/Playwright Flow Tried:** Started the real OHC stack via `docker compose up --build`. Logged into the dashboard UI and accessed the POS module (`/pos/omnichannel`). Attempted to simulate a network disconnection, processed a tap-to-pay transaction for an item ("Red Dress"), and then reconnected to the network.
  - **CUJ Gap Observed:** The POS transaction remained in a local un-reconciled state without reliably updating the PostgreSQL central `inventory_levels` table with conflict handling. The `pos_terminal_sessions` table lacks robust tracking columns like `pending_reconciliation`. If the same item was purchased online during the disconnect, there is no automatic conflict resolution or notification surfaced in the Triage Feed.
  - **Reasoning:** A real owner needs this solved so they don't oversell physical inventory and face angry customers. The smallest complete product change is adding reconciliation state to terminal sessions and emitting a conflict event to the Operations Agent.

  ## 1. Problem Statement
  Currently, OneHumanCorp (OHC) is missing a refined offline-sync reconciliation mechanism for POS `TerminalSession` with the PostgreSQL central ledger. Small business owners (like Priya the boutique owner) experience double-booking and out-of-stock scenarios during simultaneous online and offline purchases, especially when the POS hardware is temporarily disconnected from the internet. When the POS comes back online, it should correctly reconcile its offline transactions with the central inventory without causing race conditions or overriding newer central inventory updates.

  ## 2. Research Report
  - **Market Context**: Existing platforms like Shopify dominate the multi-channel space but often require costly third-party tools or higher-tier plans for robust offline sync. Square/Stripe Terminal lack integrated, agentic workflow automation.
  - **The OHC Opportunity**: Integrating offline-sync reconciliation natively alongside the existing inventory ledger and POS sessions will allow operations agents to handle stock conflicts invisibly.
  - **Competitor Gaps**: Wix and simple builders don't offer complex offline reconciliation natively. Enterprise options are too complicated for micro-SMEs.
  - **Current Codebase Gap**: `pos_terminal_sessions` table exists, but robust reconciliation state resolution needs to be properly implemented in the sync endpoints and PostgreSQL schemas.

  ## 3. Design Doc

  ### Architecture
  ```mermaid
  graph TD
      POS[Mobile POS Client - Offline] --> |Creates offline transaction| LocalStore[Local Cache]
      LocalStore --> |Network restored - Sync Request| SyncEndpoint[API: SyncOfflineTransactions]
      SyncEndpoint --> |Acquires Redlock| LedgerLock[Redis Lock]
      LedgerLock --> |Validates & Applies| PGCentral[PostgreSQL Central Ledger]
      PGCentral --> |Updates inventory_levels, inventory_transactions| OperationsAgent[Operations Agent - Resolves Conflicts]
  ```

  ### Data Model Enhancements
  - **`pos_terminal_sessions` Schema Refinement:** Add columns like `sync_status` and `pending_reconciliation` to properly track offline session states during the synchronization process.
  - **API Extension:** The `SyncOfflineTransactions` endpoint in the POS service must handle batch transaction records, deduct inventory using the `inventory_levels` table, update `inventory_transactions`, and resolve conflicts via Operations Agent dispatch if stock drops below zero.

  ### Mobile UX Flow (375px)
  - When the user processes an offline transaction, the app immediately updates the local UI optimistically (fast, responsive).
  - A subtle "Syncing..." badge appears when the connection is restored.
  - If a conflict occurs during sync (e.g., sold out online), the Operations Agent pushes an actionable notification card to the owner's feed: "Inventory conflict on Red Dress. You sold it offline, but it was just sold online. Approve refund or backorder?"

  ## 4. Implementation Prompt
  **Feature Name**: POS Offline-Sync Reconciliation

  **Target Persona**: Priya the Boutique Owner

  **Outcome**: A seamless synchronization flow where offline tap-to-pay purchases are reliably reconciled with the central PostgreSQL inventory ledger once the network is restored.

  **Next Actions**:
  1. Extend the `pos_terminal_sessions` table in a new migration to include `sync_status` and `pending_reconciliation` fields to support robust offline reconciliation tracking.
  2. Implement the `SyncOfflineTransactions` endpoint in `src/server/services/pos/service.rs` (or related API). Ensure the API properly loops over offline transactions and securely commits them to `inventory_levels` and `inventory_transactions` with conflict handling.
  3. Emit an event (`tenant.inventory.conflict`) for the Operations Agent when a sync causes available stock to dip below zero, allowing the agent to surface an action card in the Triage Feed.
  4. Build Playwright tests covering the offline sync conflict scenario.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
