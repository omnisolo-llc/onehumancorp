issue_title: "[Architecture] Offline-First Unified Tap-to-Pay & Inventory Sync Pipeline"
issue_description: |
  ## Title
  Offline-First Unified Tap-to-Pay & Inventory Sync Pipeline

  ## Problem Statement
  Maya (baker) and Fatima (food cart) rely heavily on mobile devices in environments with unreliable internet connections (e.g., crowded farmers markets, basements, or remote events). Currently, processing transactions or updating inventory requires a persistent, low-latency connection. If the connection drops, Maya can't accept card payments via tap-to-pay or sync her sold-out cake variants, resulting in lost sales and frustrated customers. We need a zero-friction, offline-first architecture for the OneHumanCorp mobile app to seamlessly queue tap-to-pay payments, cache local inventory states, and intelligently sync with our multi-tenant backend once a connection is re-established.

  ## Research Report
  Shopify and Square offer robust Point-of-Sale (POS) systems with offline transaction queuing. Square's offline mode allows merchants to swipe/dip cards and queues them for up to 24 hours. However, their systems are heavy and require complex configurations that fail the "grandmother test." Our research across SMB platforms reveals a gap in providing a truly invisible, zero-config offline-first architecture that seamlessly bridges physical tap-to-pay (using native iOS/Android NFC) with cloud inventory without the merchant managing "offline mode" toggles. Using local-first CRDTs (Conflict-free Replicated Data Types) combined with a resilient background job queue and optimistic UI updates, we can provide continuous service.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Client - 375px UI] -->|Tap-to-Pay NFC| B(Local SQLite/CRDT Store)
      A -->|Optimistic UI Update| A
      B -->|Connection Restored| C{Sync Orchestrator Agent}
      C -->|Validates Transaction & Inventory| D[OHC Core API Gateway]
      D -->|Multi-Tenant Routing| E[(Tenant Ledger DB)]
      C -->|Conflict Resolution| F[Operations Agent]
      F -->|Notification| A
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  1. **Checkout Screen**: Clean, macOS glass-morphic card showing order total and a large "Tap to Pay" button.
  2. **Offline State**: Invisible to the user. The app behaves normally. If offline, the transaction is approved instantly locally, and a small green checkmark appears with a subtle "Synced when online" microcopy hidden in an info tooltip.
  3. **Inventory Screen**: Toggles for "Sold Out" react instantly, applying a local optimistic state change.
  4. No error modals interrupting the user if offline.

  ### AI Agent Integration Points
  - **Sync Orchestrator Agent**: Monitors background connectivity and flushes the SQLite/CRDT queues to the backend securely.
  - **Operations Agent (CS & Finance)**: If a queued payment fails (e.g., declined after syncing), this agent automatically generates a polite follow-up SMS or email to the customer with a secure payment link, and notifies the merchant (e.g., Fatima) without requiring manual reconciliation.

  ### Key Design Decisions & Why
  - **Local-First CRDTs**: Ensures inventory and ledger mutations never conflict, even if multiple devices are used at a busy food cart.
  - **Invisible Offline Mode**: Passes the grandmother test. No manual toggles.
  - **Zero Trust Multi-Tenancy**: The Sync Orchestrator attaches tenant IDs securely from the edge client's JWT to ensure cross-tenant data leakage is structurally impossible.

  ## Implementation Prompt
  Build the offline-first syncing engine and UI. Implement a seamless Tap-to-Pay checkout flow on mobile that defaults to optimistic local updates using a local cache. Ensure the checkout UX feels premium and instant, adopting translucent glass materials. Implement a background sync manager that securely reconciles queued transactions and inventory changes with the backend once connectivity is restored. Do not block the user with loading spinners during checkout if the network is flaky. The Operations Agent must handle any downstream reconciliation failures automatically.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
