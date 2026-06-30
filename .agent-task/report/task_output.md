issue_title: "[Architecture] Implement Multi-Tenant Multi-Currency Distributed Ledger"
issue_description: |
  # Research Report & Design Doc: Autonomous Unified Ledger and Multi-Currency Settlement Engine

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles payments directly via external gateways (like Stripe), but lacks a robust internal financial ledger to coordinate multi-currency transactions, deposits, split-payments (for future marketplace expansion), and offline/cash settlements across the various personas (like Carlos the handyman or Priya the boutique owner). As our user base grows internationally and expands to complex workflows (e.g. partial deposits, cash-on-delivery, and gift cards), a scalable, multi-tenant Double-Entry Ledger System is required to maintain absolute financial truth. Without this, the system is brittle to webhook failures, cannot support non-Stripe payments (like cash or local bank transfers), and provides no internal audit trail.

  ## Research Report
  - **Competitive Analysis:** Platforms like Shopify and Stripe use immutable double-entry ledgers as the core of their financial systems. A simple "balance" column is insufficient for a platform handling other people's money.
  - **Market Need:** Small business owners need to track offline payments (cash, checks) alongside online payments. They need to handle multi-currency scenarios (e.g., a boutique owner in Mexico charging in MXN but paying suppliers in USD).
  - **Technical Gap:** OHC currently lacks a centralized, multi-tenant ledger service.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ ACCOUNT : owns
      ACCOUNT ||--o{ ENTRY : has
      TRANSACTION ||--o{ ENTRY : contains
      TRANSACTION {
          uuid id
          uuid tenant_id
          string description
          timestamp created_at
      }
      ACCOUNT {
          uuid id
          uuid tenant_id
          string name
          string currency
          string type
      }
      ENTRY {
          uuid id
          uuid transaction_id
          uuid account_id
          uuid tenant_id
          decimal amount
          string direction
      }
  ```

  ### System Design Decisions
  - **Double-Entry:** Every transaction must have at least two entries (debits and credits) that sum to zero.
  - **Multi-Tenant:** Every row in `accounts`, `transactions`, and `entries` must have a `tenant_id` and enforce PostgreSQL Row-Level Security (RLS).
  - **Immutability:** Once a transaction is committed, its entries cannot be modified or deleted. Corrections must be made via a new reversing transaction.
  - **Multi-Currency:** Each account is tied to a specific currency. Transactions must enforce that entries balance per currency.

  ### Mobile UX Flow (375px first)
  - **Home Screen (Financial Summary):** A clean card showing "Total Balance" with a breakdown by currency if applicable.
  - **Transaction List:** A simple feed of recent transactions (online and offline) with clear debit/credit indicators.
  - **Add Offline Payment:** A form to manually record a cash or check payment, selecting the appropriate customer and currency.

  ### AI Agent Integration Points
  - **Finance Agent:** Can query the ledger to generate weekly financial summaries.
  - **Operations Agent:** Can automatically record a deposit transaction when a booking is confirmed.

  ## Implementation Prompt
  Implement the core `Ledger Service` in Go.
  - Create the PostgreSQL schemas for `accounts`, `transactions`, and `entries` ensuring strict multi-tenant isolation via RLS.
  - Expose gRPC endpoints for recording double-entry transactions and querying balances.
  - Ensure 100% unit test coverage.
  - Implement E2E Playwright tests verifying the UI accurately displays the ledger balance.
  - Do NOT hardcode currency conversions; leave hooks for a future exchange rate service.

  **Acceptance Criteria:**
  - The database schema enforces RLS and the double-entry invariant (sum of entries = 0).
  - The gRPC API allows creating accounts and recording transactions.
  - The UI accurately displays the balance for a given tenant.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
