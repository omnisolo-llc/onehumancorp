issue_title: "Implement Multi-Tenant Zero-Trust Edge Terminal Payment Flow for Offline OHC Operators"
issue_description: |
  # Research Report: Implement Multi-Tenant Zero-Trust Edge Terminal Payment Flow for Offline OHC Operators

  ## Title
  Implement Multi-Tenant Zero-Trust Edge Terminal Payment Flow for Offline OHC Operators

  ## Problem Statement
  Operators like Priya (Boutique Operator) and Fatima (Food Cart Operator) frequently experience network drops while transacting with customers in person. They need a resilient, offline-first tap-to-pay interface that can capture payment intents, queue them locally using an edge-cache architecture, and robustly sync to the centralized multi-tenant Postgres backend upon reconnection without losing any revenue or creating double-charges. Existing OHC payment interfaces assume a perfectly stable internet connection and fail silently or present technical "Network Error" alerts to the owner during crucial point-of-sale moments.

  ## Research Report
  - **Stripe Terminal SDK / Square:** Leading point-of-sale systems utilize smart edge caching where the local device functions as an offline mini-ledger. Transactions are cryptographically signed locally and queued.
  - **Current OHC Platform Gap:** OHC currently lacks an edge-caching layer for its Point-of-Sale workflows. The frontend immediately calls the centralized Go/Postgres backend for all state mutations.
  - **Proposed Architecture Alignment:** The new capability requires building a `TerminalSession` edge-cache schema in SQLite/PWA local storage, coupled with an idempotent Zero-Trust multi-tenant sync protocol to `src/server/services/payments/` using gRPC/REST.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Operator as OHC Mobile PWA
      participant EdgeCache as Local Edge DB (SQLite/IndexedDB)
      participant GoServer as OHC API Backend
      participant PaymentGateway as Stripe/Provider

      Operator->>EdgeCache: Record offline Tap-to-Pay intent
      EdgeCache-->>Operator: Render local success UI (Pending Sync state)
      Note over EdgeCache, GoServer: Network Reconnects
      EdgeCache->>GoServer: Sync batch intents (Idempotent, Zero-Trust)
      GoServer->>PaymentGateway: Process auth & capture
      PaymentGateway-->>GoServer: Confirm Capture
      GoServer-->>EdgeCache: Update Ledger Status (Settled)
      EdgeCache-->>Operator: Update UI to "Completed"
  ```

  ### Mobile UX Flow
  1. **Checkout Screen (375px):** Clean Translucent Glass keypad UI. The operator taps "Charge $15.00".
  2. **Payment Processing:** If offline, the UI shows a "Saved Offline - Will sync when connected" toast using OHC Premium Tokens. The operator can immediately take the next order.
  3. **Queue Visibility:** A small indicator icon on the Home Shell shows pending offline transactions.
  4. **Auto-Recovery:** Upon network restoration, the indicator spins and turns to a green checkmark, reflecting successful background sync by the AI Operations Agent.

  ### Key design decisions and why
  - **Local edge schema for intents:** This provides the resilience required by mobile offline operators (Fatima) without breaking downstream operations.
  - **Idempotent sync endpoint:** A must to avoid double charges on network reconnects.

  ### AI Agent Integration
  - **Finance & Decision Assistant:** Monitors the sync queue. If an offline transaction fails upon sync (e.g., card declined post-auth), the agent drafts an owner-ready summary identifying the failed transaction and prepares a follow-up link (SMS/Email) to request payment from the customer.

  ## Implementation Prompt
  Implement an offline-capable tap-to-pay component for the OHC Flutter/PWA application. Design the local storage schema for caching `TerminalSession` intents. Build the multi-tenant idempotent sync endpoint on the Go backend that processes these queued transactions securely via Stripe. Ensure the UI gracefully handles offline states with clear, non-technical indicators for the operator. The feature must be completely functional and verified via Playwright E2E tests simulating offline mode. Do not dictate specific SQL schemas or Go function signatures.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
