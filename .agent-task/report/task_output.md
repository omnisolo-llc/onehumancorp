issue_title: "[architecture] Universal Agentic Offline-First POS & Tap-to-Pay Sync Engine"
issue_description: |
  # Title: Universal Agentic Offline-First POS & Tap-to-Pay Sync Engine

  ## Problem Statement
  Small business operators like **Priya (Boutique Owner)** and **Fatima (Food Cart Operator)** operate in physical environments where network connectivity is often flaky, slow, or drops completely. They need to process in-person transactions seamlessly via Tap-to-Pay, update inventory instantly, and rely on the AI agent to summarize the day, even when offline. Existing solutions (like Shopify POS or Square) either fail gracefully but block critical agentic features offline, or require complex setups to sync with an online catalog. The owner needs a transparent, offline-tolerant ledger that feels like magic—it just works, and syncs to the cloud securely the moment a connection is re-established.

  ## Research Report
  - **Market Gap**: Competitors (Square, Lightspeed, Wix) offer offline payments, but their AI assistants rely entirely on cloud-based LLM inference. If the network drops, the assistant goes down.
  - **OHC Advantage**: By utilizing the "Local First" Standalone wrapper capability and CRDTs (Conflict-free Replicated Data Types) combined with a local SQLite database and edge-based lightweight ML models, OHC can process inventory updates, queue Stripe Terminal Tap-to-Pay intents idempotently, and keep the agentic feed active.
  - **Core Component**: The Offline Sync Engine (`SyncDaemon`) acts as a background queue. Mutations (sales, inventory adjustments) are written locally to the SQLite `local_ledger` table.
  - **Agentic Integration**: The Operations Assistant locally queues "intent" logs. Once online, these intents are flushed to the cloud Teammate Mesh, enabling the backend to reconcile and trigger follow-up actions (e.g., reorder low stock, send digital receipts).

  ## Design Doc
  **Architecture Diagram:**
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile UI (Priya)
      participant OperationsAgent as Operations Assistant (Local)
      participant SQLite as Local SQLite Ledger (CRDT)
      participant StripeTerminal as Stripe Tap-to-Pay SDK
      participant SyncDaemon as Background Sync Daemon
      participant OHCCloud as OHC Cloud Backend

      Owner->>OperationsAgent: "Ring up 2 Blue Silk Scarves"
      OperationsAgent->>SQLite: Write Pending Invoice (Local)
      OperationsAgent->>StripeTerminal: Trigger Tap-to-Pay Intent
      StripeTerminal-->>OperationsAgent: Payment Authorized (Offline Token)
      OperationsAgent->>SQLite: Update Invoice Status (Paid) & Decrement Inventory (CRDT)
      Owner->>OperationsAgent: "Done" (UI shows Success instantly)

      Note over SyncDaemon, OHCCloud: When network connection restores...
      SyncDaemon->>SQLite: Read Unsynced Mutations
      SyncDaemon->>OHCCloud: Sync Payload (Idempotent keys)
      OHCCloud-->>SyncDaemon: ACK & Merge
      OHCCloud->>OperationsAgent: Trigger Finance/Inventory Agents for Cloud-side actions
  ```

  **Mobile UX Flow (375px):**
  1. **POS Dashboard:** Big, tap-friendly grid of quick items + search bar. Translucent glass effect header indicating online/offline status (green vs. amber dot).
  2. **Checkout Modal:** Slides up from bottom. Big total, "Tap to Pay" primary button. 44x44px minimum touch targets.
  3. **Success State:** Instant visual confirmation with haptic feedback. Even if offline, a subtle "Synced locally" toast appears, ensuring Priya knows the data is safe.

  **AI Agent Integration Points:**
  - **Operations Assistant:** Intercepts natural language commands locally (if small on-device model is present) or provides fallback UI actions to record sales.
  - **Decision Assistant (Cloud):** Once the sync happens, it analyzes the influx of data and updates Priya's daily summary.

  ## Implementation Prompt
  Implement the Local-First POS Data layer and Sync Daemon.
  1. Create a `local_ledger` SQLite schema (or equivalent Powersync/wa-sqlite setup) supporting offline mutation queues and CRDT for inventory counts.
  2. Implement the `SyncDaemon` background worker that watches the local queue and flushes mutations to the cloud REST/gRPC endpoints using exponential backoff and strict idempotency keys.
  3. Build the mobile-first POS checkout screen adhering to OHC Premium CSS tokens (translucent glass, large 44x44px touch targets). Ensure the UI does NOT block on cloud network calls; it should update locally instantly.
  4. Integrate Stripe Terminal JS SDK hooks to capture payment intents gracefully.
  5. The UI MUST demonstrate truthful online/offline indicators.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
