issue_title: "[Architecture] Omni-Channel Multi-Currency Ledgers with Zero-Trust Isolation"
issue_description: |
  # Omni-Channel Multi-Currency Ledgers with Zero-Trust Isolation

  ## Problem Statement
  Currently, OneHumanCorp (OHC) platform targets personas like Priya (boutique owner) and Carlos (handyman) who increasingly rely on multi-channel commerce (in-store tap-to-pay, online invoicing, custom deposit links). As these non-technical owners expand their physical and digital footprints, they need a unified, resilient financial ledger that can handle multi-currency payments, automatic reconciliation, and cross-channel visibility without requiring them to act as accountants. The existing multi-tenancy implementation lacks a dedicated, zero-trust isolated financial ledger schema and coordination layer capable of supporting offline-first, locally-cached transaction logging that syncs reliably upon reconnection.

  ## Research Report
  - **Market Analysis**: Competitors like Shopify and Stripe heavily invest in unified, multi-currency ledger systems. Shopify's balance sheet model and Stripe's Financial Connections prove that SMBs abandon platforms when they have to manually reconcile offline POS transactions with online orders.
  - **Codebase Audit**: The current database models handle basic transactions but lack an immutable, append-only, cryptographically verified ledger structure necessary for true multi-currency financial records. Go microservices and multi-tenant Postgres architecture are in place, but need the schema mapping.
  - **Persona Fit**: Priya needs to see her daily tap-to-pay and online sales unified. Carlos needs a reliable deposit record that isn't lost if he goes offline at a job site.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ LedgerAccount : owns
      LedgerAccount {
          uuid id
          uuid tenant_id
          string currency
          decimal current_balance
          timestamp last_synced
      }
      LedgerAccount ||--o{ LedgerEntry : contains
      LedgerEntry {
          uuid id
          uuid ledger_account_id
          decimal amount
          string type
          string idempotency_key
          timestamp created_at
      }
  ```
  ### AI Agent Integration
  - **Finance Assistant**: Automatically monitors `LedgerEntry` anomalies and drafts plain-language daily revenue summaries for the owner.
  - **Operations Assistant**: Uses `LedgerAccount` balance to trigger automated vendor payment alerts.

  ### Mobile UX Flow (375px)
  - **Dashboard Card**: A unified "Today's Revenue" card showing combined online/offline sales.
  - **Transaction List**: A clean, chronological feed of `LedgerEntry` items with clear status badges (Pending, Cleared, Offline Syncing).
  - **Tap-to-Pay**: A seamless transition into a native mobile tap-to-pay interface that caches transactions locally before pushing to the `LedgerEntry` table.

  ## Implementation Prompt
  **Goal**: Implement the core `LedgerAccount` and `LedgerEntry` database schemas, Go API endpoints, and a mobile-first unified financial dashboard for the non-technical owner.
  **CUJ**: Priya opens the OHC app, sees her total daily revenue across all currencies and channels, and taps to view a unified list of online orders and offline tap-to-pay transactions.
  **Requirements**:
  - Implement immutable, append-only ledger tables with strict `tenant_id` RLS isolation in PostgreSQL.
  - Create Go gRPC/REST APIs for idempotent transaction recording.
  - Build a responsive Flutter UI (starting at 375px) that displays the unified revenue and transaction list without exposing complex accounting terms.
  - Ensure 100% unit and E2E test coverage, with a Playwright test simulating an offline-to-online transaction sync.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
