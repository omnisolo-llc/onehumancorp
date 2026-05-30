issue_title: "[Architecture] Hardware-Free Offline-First Mobile POS & Tap-to-Pay Integration"
issue_description: |
  # Architecture Design Document: Offline-First Mobile POS & Tap-to-Pay Engine

  ## Problem Statement
  Small business owners who operate in person—like Priya the boutique owner or Fatima the food cart operator—need a frictionless way to accept in-person payments without purchasing, pairing, and maintaining expensive external POS hardware. In addition, these owners operate in environments with intermittent cellular service. When the network connection drops, the app must remain functional. They need an invisible, highly resilient mobile POS that works seamlessly on their smartphones using native NFC Tap-to-Pay, queuing transactions locally when offline and syncing them automatically when connectivity is restored.

  ## Research Report
  - **Market Context**: Shopify and Square offer POS options, but often rely on expensive external hardware and struggle with true offline resilience. Stripe Terminal offers Tap-to-Pay SDKs allowing merchants to accept payments natively on their mobile devices using NFC without extra hardware.
  - **Discovery**: OHC needs a native, offline-first mobile POS that utilizes smartphone NFC (Apple Tap to Pay / Android Tap to Pay). It requires a robust local queuing system to safely store encrypted transactions offline and a sync engine that intelligently batches and syncs the queue upon reconnection without manual user intervention.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ TERMINAL_SESSION : initiates
      TERMINAL_SESSION ||--|| TRANSACTION : processes
      TRANSACTION }|--|| LOCAL-QUEUE : "Writes Offline Tx"
      LOCAL-QUEUE }|--|| SYNC-ENGINE : "Batches on Reconnect"
      SYNC-ENGINE ||--o{ PAYMENT-ORCHESTRATOR : "Dispatches Txs"
      PAYMENT-ORCHESTRATOR }|--|| LEDGER : "Updates State securely"
      PAYMENT-ORCHESTRATOR }|--o{ INVENTORY : decrements
      PAYMENT-ORCHESTRATOR ||--o{ FINANCE-AGENT : "Triggers Reconciliation"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Home Dashboard**: Primary CTA "Accept Payment" opens amount entry.
  2. **Amount Entry**: Full-screen numeric keypad with high contrast, >44x44px touch targets.
  3. **Checkout Screen (Online/Offline)**: If online, green Wi-Fi icon. If offline, subtle amber "Offline Mode" glassmorphism pill ("Saved offline. Will sync when connected.").
  4. **NFC Interaction**: Native OS Tap-to-Pay overlay appears. "Hold card near phone."
  5. **Success State**: Instant haptic feedback and green checkmark. "Payment Saved!" (Passes Grandmother test).
  6. **Queue Dashboard**: A dashboard card shows "X Payments Pending Sync".

  ### Key Design Decisions
  - **Zero-Hardware Approach**: Strictly leverage native Apple/Android Tap-to-Pay SDKs via Stripe Terminal. No bluetooth readers.
  - **Offline-First Paradigm**: The UI assumes success. Network requests are an asynchronous side-effect. Local queue is encrypted using hardware-backed keystores.
  - **Unified Data & Idempotent Sync**: In-person sales mutate the same Ledger and Inventory as online sales. The sync engine uses strict `idempotency_key`s to ensure no double-charging on network drop during sync.
  - **Zero Trust Security**: Multi-tenant isolation at the terminal session level using SPIFFE/SPIRE-backed identities.

  ### AI Agent Integration Points
  - **Finance Agent**: Reconciles the synced offline transactions. Drafts SMS/Email requests for alternative payment if an offline card is later declined. Includes offline sales data in weekly reports.
  - **Operations Agent**: Deducts sold items. If an offline sale brings inventory below zero, it queues a restock alert.
  - **Business Advisory Agent**: Provides insights combining offline/online sales performance.

  ## Implementation Prompt
  Implement the Offline-First Mobile POS & Tap-to-Pay Engine using Stripe Terminal SDKs. The system must allow native NFC payments on mobile devices without an active internet connection. Build a resilient local queuing mechanism for encrypted offline transactions and an intelligent background sync engine that safely batches and transmits them to the core Ledger using idempotent processing. Ensure strict multi-tenant isolation and seamless 375px UI fallback states (e.g., amber offline indicators) without loading spinners.

  **Acceptance Criteria**:
  - Implement a secure local queue for transaction intents using hardware-backed encryption.
  - Implement an event-driven background sync manager.
  - Build UI fallback states adhering to OHC Glassmorphism standards.
  - Ensure zero lost transactions across device reboots and no double-charges upon sync.
  - Verify multi-tenant isolation for terminal sessions.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
