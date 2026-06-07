issue_title: "Robust CRDT-based Offline-First POS Synchronization & AI Conflict Resolution"
issue_description: |
  # Mission Queue Protocol: Robust CRDT-based Offline-First POS Synchronization & AI Conflict Resolution

  ## Problem Statement
  Priya the Boutique Owner occasionally relies on her mobile device to run transactions at pop-up stores or farmer's markets where internet access is spotty. When she goes offline, she can currently queue POS transactions (as seen in `src/server/services/pos/service.rs`), but when returning online, there is no robust conflict resolution for inventory levels (e.g. if an item was sold both online and offline concurrently). If inventory drops below zero due to offline sync conflicts, Priya is forced to manually audit the ledger, causing severe non-technical user friction.

  ## Research Report
  - **Current State:** The backend currently handles offline sync using `OfflineSyncRequest` via `offline_sync_handler` and `sync_offline_transactions`. These simply decrement inventory using `GREATEST(0, inventory_count - $1)` and insert into a `pos_offline_transactions` queue. There is no idempotent vector-clock or CRDT mechanism to gracefully merge concurrent offline/online inventory changes.
  - **Competitor Analysis:** Shopify POS employs a sophisticated conflict-resolution queue where offline actions are timestamped and replayed safely. If there's an oversell, Shopify warns the user. Square uses an eventual consistency model with clear visual indicators to the merchant when a conflict arises.
  - **Opportunity:** OneHumanCorp can introduce an AI-driven resolution mechanism. Instead of silently failing or dropping to zero, the "Operations" AI department can review the conflict, automatically issue an alert to Priya, and even draft an email to the affected customer if an oversell occurred, maintaining our "AI does the work" promise.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant App as OHC Mobile App (Priya)
      participant API as PosService (Backend)
      participant DB as PostgreSQL (Row-Level Security)
      participant Queue as Job Queue (ohc_job_queue)
      participant AI as Operations & CS AI Agents

      App->>App: Priya sells item offline
      App->>App: Store transaction locally
      App->>API: Connection Restored -> Sync Offline Transactions
      API->>DB: Insert into pos_offline_transactions & ohc_job_queue
      Queue->>API: Worker processes pos_offline_sync job
      API->>DB: Check Inventory & Merge CRDT
      alt Conflict / Oversold
          API->>DB: Set transaction status to CONFLICT
          API->>AI: Trigger Operations Agent Event
          AI->>App: Send UI Alert (Dashboard)
          AI->>DB: Draft refund/apology via CS Agent
      else Success
          API->>DB: Update Ledger & Inventory
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Mode Indicator:** The dashboard shows a subtle, non-intrusive "Working Offline" banner (Translucent Glass material) at the top of the screen.
  2. **Transaction Sync:** When connection is restored, a micro-animation shows "Syncing X sales...".
  3. **Conflict UI:** If a conflict occurs (e.g., oversold), an action card appears on the UniFi-style dashboard: "Heads up! A pop-up sale overlapped with an online order for the Blue Dress. Operations has drafted an email to the online customer."

  ### AI Agent Integration Points
  - **Operations ("The Manager"):** Detects the sync conflict from the `ohc_job_queue` worker. Analyzes the discrepancy.
  - **Customer Success ("The Ambassador"):** Automatically drafts an apology email or discount offer to the customer who needs to be refunded due to the oversell.

  ### Key Design Decisions
  - **Data Invariants:** Transition `pos_offline_transactions` processing to use Logical Clocks (or timestamp + client_id) to ensure idempotency.
  - **Zero Trust:** `tenant_id` boundaries must be strictly enforced during the job queue processing.
  - **No Technical Jargon:** The user never sees words like "CRDT", "sync collision", or "vector clock". They only see "We caught a double-booking".

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the conflict resolution engine for offline POS transactions.
  - **Goal:** Update the background worker that processes `pos_offline_sync` jobs (and the `offline_sync_handler` in `src/server/api/offline_sync.rs`) to detect when an offline deduction causes an inventory oversell.
  - **Action:** Instead of silently dropping inventory to 0, if an oversell is detected, trigger an event to the "Operations" AI department.
  - **Acceptance Criteria (CUJ):**
    1. A merchant completes a sale offline.
    2. Concurrently, the same item is sold online, reducing inventory to 0.
    3. The device comes online and syncs the transaction.
    4. The backend detects the oversell, marks the sync as conflicted, and the AI agent automatically creates a task/alert on the dashboard proposing a resolution.
  - **Verification:** You MUST implement a full E2E Playwright test simulating this offline-online overlap and verify that the dashboard alert appears without exposing technical terms. Run `bazel test //...` to ensure 100% coverage. Do NOT prescribe specific DB schema changes; design what is needed to fulfill the acceptance criteria.

  ## Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []