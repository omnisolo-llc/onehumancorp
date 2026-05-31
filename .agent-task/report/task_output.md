issue_title: "Implement Offline-First Mobile POS Sync Engine"
issue_description: |
  # Research Report & Design Doc
  ## Problem Statement
  Small business owners like Fatima (Food Cart operator) and Priya (Boutique owner) operate in environments with intermittent cellular service. They need an offline-first mobile POS that works seamlessly on smartphones and handles Tap-to-Pay transactions offline with automatic sync.

  ## Research Report
  - Current checkout assumes constant internet connection.
  - Square requires proprietary hardware.
  - Shopify POS is expensive and heavily reliant on connectivity.
  - OHC needs native, offline-first mobile POS leveraging smartphone NFC. Local queuing and background sync are mandatory.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE-APP ||--o{ LOCAL-QUEUE : "Writes Offline Tx"
      MOBILE-APP ||--o{ NFC-HARDWARE : "Interacts (Tap-to-Pay)"
      LOCAL-QUEUE }|--|| SYNC-ENGINE : "Batches on Reconnect"
      SYNC-ENGINE ||--o{ API-GATEWAY : "Dispatches Txs"
      API-GATEWAY ||--o{ PAYMENT-ORCHESTRATOR : "Routes to Processor"
      PAYMENT-ORCHESTRATOR }|--|| CORE-LEDGER : "Updates State securely"
      PAYMENT-ORCHESTRATOR ||--o{ FINANCE-AGENT : "Triggers Reconciliation"
  ```

  ### UI Wireframes
  - Clean Unifi-style screen showing the amount.
  - Large green checkmark upon tap, no spinners.
  - Offline Indicator: Translucent Glass pill at top.
  - Silent background sync.

  ### Key Design Decisions
  - Offline-first paradigm: assume success for reads/writes.
  - Secure local storage for offline transactions.
  - Idempotent Sync: transactions are idempotent to prevent double-charging.
  - Zero Trust & Multi-Tenancy: strict tenant isolation.

  ### AI Agent Integration
  - Finance Agent monitors sync batches.
  - Operations Agent handles recovery if card declined later.

  ## Implementation Prompt
  Implement the Offline-First Mobile POS Sync Engine. Ensure local queuing with secure storage and an intelligent sync engine that automatically batches transactions to the core ledger once connectivity is restored. Use idempotent processing to prevent double-charges and ensure multi-tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
