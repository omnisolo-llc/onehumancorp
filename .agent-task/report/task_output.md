issue_title: "Implement Standalone In-Person POS Terminal & Cash Ledger Capabilities"
issue_description: |
  # Research Report: Standalone In-Person POS Terminal & Cash Ledger Capabilities

  ## 1. Problem Statement
  Service-based and physical retail small business owners (e.g., Priya the Boutique Operator, Carlos the Field Service Owner) require offline-first in-person point of sale (POS) capabilities tightly integrated with an accurate cash ledger. Existing solutions rely on fragmented, expensive third-party hardware or tablet apps (like Square or Shopify POS) that act as separate silos of data and add up to 2.5-3% + 15¢ per transaction in fees, while failing to offer a unified, agent-driven view of physical inventory and local cash flow. When local networks fail, operations halt entirely.

  ## 2. Research Report
  - **Market Context**: Micro-SMEs often run multi-channel operations (online sales + physical storefront/field service). Bridging the gap requires dedicated POS software which adds complexity.
  - **Competitor Gaps**:
    - *Shopify POS*: Powerful but requires Shopify infrastructure. Offline mode is rudimentary, lacking intelligent sync for multi-terminal inventory.
    - *Square*: Excellent hardware integration but poor CRM/agentic follow-up capabilities. Disconnected from proactive scheduling.
    - *Wix Point of Sale*: Dependent on continuous connectivity; rigid ledger logic.
  - **The OHC Opportunity**: By building an offline-tolerant, native "Terminal Mode" within the OHC mobile app, powered by a local SQLite sync engine and synchronized via KAIROS distributed state machine, OHC can eliminate the need for distinct POS software. Integrating the Finance Agent allows proactive cash ledger reconciliation.

  ## 3. Design Doc
  ### Data Model & Invariants (PostgreSQL / Local SQLite)
  - `TerminalSession`: Represents a continuous physical shift or session for a device (`device_id`, `status: ACTIVE | CLOSED`, `offline_changes_count`).
  - `PosTransaction`: Represents a local checkout (`terminal_session_id`, `amount`, `currency`, `payment_method: CASH | CARD_OFFLINE | CARD_ONLINE`, `sync_status: PENDING | SYNCED`).
  - `CashLedgerEntry`: Immutable append-only ledger tracking physical cash movements (sales, drawer opens, drops).
  - **Multi-Tenancy Rule**: Every record MUST enforce strict row-level security (`tenant_id`). The local SQLite replica only receives the specific tenant's active catalog and settings.

  ### AI Agent Coordination
  - **Finance Agent**: Automatically flags discrepancies between expected cash (from `PosTransaction`s) and the final `TerminalSession` closeout amounts.
  - **Operations Agent**: Monitors inventory levels locally and queues low-stock alerts when the terminal regains connectivity.

  ### Mobile UX Flow (375px)
  1. **Terminal Mode Toggle**: The owner switches the app into "Terminal Mode" (distinct from the Owner Dashboard). UI simplifies to a high-contrast, large-button catalog view (touch targets >= 44x44px).
  2. **Offline Checkout**: Items are added to the cart. If the network drops, the app seamlessly switches to a distinct "Offline Mode" banner. The user selects "Cash" or "Save Card for Sync".
  3. **End of Day / Sync**: When back online, an intuitive progress indicator shows local transactions syncing to the OHC cloud.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Offline-First POS Terminal
  **Target Persona**: Priya the Boutique Operator

  **Outcome**: Priya can run her physical boutique checkouts entirely through the OHC mobile app. If the store's Wi-Fi drops, she can continue accepting cash and securely storing card transactions for later sync. The Finance Agent automatically reviews the day's cash ledger upon session closure.

  **Next Actions**:
  1. Define the PostgreSQL data models for `TerminalSession`, `PosTransaction`, and `CashLedgerEntry`, ensuring strict `tenant_id` multi-tenancy.
  2. Design the local SQLite schema and the synchronization logic to queue offline actions and securely sync them when network connectivity is restored (e.g., implementing an intelligent sync conflict resolution strategy).
  3. Develop the "Terminal Mode" mobile-first UI with large touch targets and distinct online/offline visual states.
  4. Implement the Finance Agent capability to analyze `CashLedgerEntry` records against `TerminalSession` totals to detect anomalies.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
