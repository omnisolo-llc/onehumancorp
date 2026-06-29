issue_title: "Implement Multi-Currency Architecture for Pricing & Billing"
issue_description: |
  # Multi-Currency Architecture Deep Dive

  ## Problem Statement
  OHC currently assumes a single implicit currency (e.g., USD) across its pricing, orders, quotes, and billing tables. As a global platform serving personas like Priya (Boutique owner) or Carlos (Field Service owner), forcing a single currency prevents international expansion, multi-region tap-to-pay functionality, and localized invoicing. OHC needs a robust multi-currency backbone.

  ## Research & Competitive Analysis
  - **Shopify & Stripe:** Both separate the concept of an `amount` from its `currency`. Money is always represented as a tuple/struct: `(amount, currency_code)`. They store amounts in the smallest currency unit (e.g., cents for USD, yen for JPY) to avoid floating-point precision issues.
  - **Multi-Tenant Boundaries:** A tenant typically has a "settlement" currency (the currency they get paid in) but might display prices in "presentment" currencies based on the buyer's location.

  ## Architecture Design

  ### 1. Data Model & Invariants
  We must migrate financial tables to store currency explicitly.
  - **New Domain Type:** Introduce a robust `Money` type in Go that encapsulates `amount` (int64, minor units) and `currency_code` (string, ISO 4217).
  - **Schema Updates:** Update tables like `products`, `orders`, `quotes`, and `billing_invoices` to add a `currency` column (defaulting to 'USD' for existing rows).
  - **Tenant Configuration:** Add `default_currency` to the `tenants` table to define their primary operational currency.

  **Mermaid Diagram:**
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
          string default_currency "e.g. USD, EUR"
      }
      PRODUCT {
          uuid id PK
          uuid tenant_id FK
          numeric price
          string currency "ISO 4217 code"
      }
      ORDER {
          uuid id PK
          uuid tenant_id FK
          numeric total_amount
          string currency "ISO 4217 code"
      }
      TENANT ||--o{ PRODUCT : owns
      TENANT ||--o{ ORDER : processes
  ```

  ### 2. API Boundaries
  - gRPC messages must be updated to include `string currency = X;` wherever an amount is present.
  - JSON payloads (REST/webhooks) should format money consistently, e.g., `{"amount": 1000, "currency": "USD"}`.

  ### 3. AI Agent Coordination
  - **Finance Assistant:** Needs context of the tenant's default currency when generating revenue summaries.
  - **Sales Assistant:** Must draft quotes using the correct currency symbols and formats based on the customer's locale and the tenant's configuration.

  ### 4. Mobile-First UX Flow
  - On the 375px viewport, price inputs must feature a clear, tap-friendly currency selector or explicitly display the tenant's default currency symbol to avoid ambiguity.
  - Analytics dashboards must aggregate totals correctly, ideally converting to the tenant's base currency using a daily exchange rate, or explicitly partitioning charts by currency.

  ## Implementation Prompt
  Implement the foundational multi-currency data structures in the Go backend and apply the necessary PostgreSQL schema migrations.
  1. Define a `Money` struct in a common domain module.
  2. Create a migration to add `default_currency` (VARCHAR, default 'USD') to the `tenants` table, and `currency` columns to `products` and `orders`.
  3. Update the relevant gRPC proto definitions (`pricing`, `orders`) to include currency fields.
  4. Ensure all existing E2E tests pass by updating fixtures to provide the default currency.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
