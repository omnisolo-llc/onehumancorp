issue_title: "Architectural Gap: Offline-First Mobile Sync & Tap-to-Pay Architecture"
issue_description: |
  # Title: Offline-First Mobile Sync & Tap-to-Pay Architecture

  ## Problem Statement
  For non-technical owner/operators operating in physical or variable-network environments (like Fatima at her food cart or Carlos on a field service job), network flakiness is a constant threat to operations. Currently, if the connection drops during a checkout or while recording a service completion, the transaction fails, leading to lost revenue or manual reconciliation headaches later. Priya also needs reliable in-person tap-to-pay capabilities that instantly sync with her boutique's centralized inventory. We lack a unified, offline-tolerant multi-tenant sync engine that pairs with tap-to-pay SDKs and guarantees state consistency once the network is restored.

  ## Research Report
  Our research into leading mobile POS systems (Square, Shopify POS) shows that offline capability is a primary driver of adoption for physical merchants. They use local SQLite/IndexedDB caches combined with optimistic UI updates and a durable background sync queue.
  - **Shopify POS**: Uses a robust local storage mechanism to queue offline sales and syncs them automatically upon reconnection.
  - **Square**: Allows offline payments with encrypted card data stored locally, processed when back online.
  - **OHC Gap**: OHC currently lacks an edge-level data synchronization layer that seamlessly handles offline writes (like queueing a deposit or updating inventory) and secure tap-to-pay integrations (like Stripe Terminal SDK) with automatic AI-led reconciliation when back online.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter)
      participant LocalDB as Local Store (SQLite)
      participant SDK as Stripe Terminal SDK
      participant API as OHC API (Go/gRPC)
      participant AI as AI Finance Agent

      App->>LocalDB: Read cached inventory/offers (Offline)
      App->>SDK: Initiate Tap-to-Pay (Offline/Online)
      SDK-->>App: Payment Auth Token
      App->>LocalDB: Store Transaction & Sync Intent (Optimistic)
      App-->>User: Show Success UI (Immediate)

      Note over App, API: Network Restored
      App->>API: Background Sync (Flush Sync Queue)
      API->>LocalDB: Acknowledge & Resolve Conflicts
      API->>AI: Trigger Auto-Reconciliation Event
      AI->>API: Update Ledgers & Owner Dashboards
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **Checkout Screen**: A clean, full-bleed numeric keypad and "Tap to Pay" button. Uses Translucent Glass materials.
  2. **Offline Indicator**: A subtle, non-intrusive pill badge at the top reading "Offline - Saving safely" (No technical jargon).
  3. **Success State**: Immediate green checkmark, regardless of network status.
  4. **Sync Center (Hidden in Settings)**: For advanced users, showing pending syncs.

  ### Mobile UX Flow
  - **Action**: User taps "Charge $45.00".
  - **Process**: App calls Stripe Terminal SDK. If offline, it stores the encrypted intent locally.
  - **Feedback**: UI optimistically updates inventory and marks order as "Pending Network".
  - **Resolution**: Once back online, the Flutter background isolate flushes the queue to the Go backend.

  ### AI Agent Integration Points
  - **Operations Agent**: Adjusts local inventory optimistically. Upon sync, resolves any global inventory conflicts.
  - **Finance Assistant**: Automatically reconciles the delayed batch payments and adds a plain-language summary to the owner's daily feed (e.g., "3 offline payments from yesterday successfully processed").

  ### Key Design Decisions
  - **Optimistic UI**: Never block the owner from completing a sale.
  - **Conflict Resolution**: Server wins on inventory count, but the Finance Agent alerts the owner of discrepancies rather than silent failures.
  - **Security**: Local sensitive data (like encrypted tap-to-pay tokens) stored using secure enclaves, never plaintext SQLite.

  ## Implementation Prompt
  **Target**: Implementer Agent
  **Goal**: Build the `OfflineSyncService` in Flutter and the corresponding `SyncResolution` gRPC endpoints in the Go backend.
  **CUJ**:
  1. Carlos marks a repair job as "Complete" and taps to collect a $50 deposit while in a basement with no cellular signal.
  2. The app shows a success screen immediately.
  3. Carlos walks outside, regains signal.
  4. The app automatically syncs the deposit and job status to the backend.
  5. The AI Finance Agent processes the update and it appears in his work feed.
  **Acceptance Criteria**:
  - The Flutter app must include a local SQLite/Hive database for queueing mutations.
  - The Go backend must accept batch sync payloads with idempotent handling.
  - Zero mock data in the UI; empty states must reflect the real local queue.
  - Playwright E2E test must simulate going offline, performing a transaction, going online, and verifying the backend state update.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
