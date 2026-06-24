issue_title: "Implement Invisible AI Autonomous Ledger & Embedded Taxation Engine"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Small business owners (Priya the boutique owner, Carlos the handyman) operate across multiple channels (online, in-store, offline services) and struggle profoundly with end-of-month financial reconciliation, tax calculation, and tracking cross-channel revenue. They use "Franken-stacks" of tools to figure out their taxes, frequently miss deductions, and often end up owing large lump sums at tax time because they didn't reserve enough.

  ## Research Report
  - **Competitor Gap:** Shopify and Square have basic reporting, but tax automation usually requires complex third-party app integrations (e.g., Avalara) or expensive accountants. GoDaddy has basic categorization but no proactive tax saving.
  - **User Sentiment:** SMB users on Reddit and Trustpilot constantly complain about the anxiety of tax season and the difficulty of tracking what they actually owe versus what they've made.
  - **OHC Opportunity:** An *invisible*, autonomous ledger that seamlessly calculates taxes on every transaction, tracks revenue across all channels (Stripe Terminal, online checkout, manual invoices), and proactively segments a "tax savings reserve" invisibly, requiring zero configuration from the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      LEDGER_ENTRY {
          uuid id PK
          uuid tenant_id FK
          string channel "online, pos, invoice"
          decimal amount
          decimal tax_amount
          string tax_region
          timestamp created_at
      }
      TAX_RESERVE {
          uuid id PK
          uuid tenant_id FK
          decimal total_reserved
          timestamp last_updated
      }
      FINANCE_AGENT_JOB {
          uuid id PK
          uuid tenant_id FK
          string status "pending, completed"
          timestamp scheduled_for
      }
      LEDGER_ENTRY ||--o{ TAX_RESERVE : "funds"
      TAX_RESERVE ||--o{ FINANCE_AGENT_JOB : "monitored_by"
  ```

  ### Mobile UX Flow
  1. The user logs into the OHC app (375px view).
  2. The Home feed displays a unified "Total Balance" and an invisible "Tax Reserve Balance" (hidden by default to avoid anxiety, but accessible via a tap).
  3. A new transaction occurs (e.g., in-store sale).
  4. The Finance Agent calculates the tax automatically in the background.
  5. An Action Card appears: "We've safely set aside $4.50 for taxes from your last 5 sales. You're on track for Q3!"

  ### AI Agent Integration Points
  - **The Finance Agent:** Continuously monitors the `LEDGER_ENTRY` table. It classifies transactions, computes estimated tax liabilities based on the `tax_region`, and updates the `TAX_RESERVE`.
  - **The Customer Success Agent:** Can use ledger data to offer dynamic discounts (e.g., if a user is highly profitable this week, it might suggest offering a 10% discount to VIP customers).

  ### Key Design Decisions
  - **Zero-Config Taxation:** Taxes are calculated implicitly based on the merchant's and customer's location. The user does not need to configure tax rates.
  - **Tenant Isolation:** All ledger entries MUST enforce strict RLS based on `tenant_id`.

  ## Implementation Prompt
  Implement the backend infrastructure and mobile-first UI for the Invisible AI Autonomous Ledger.

  **CUJ (Critical User Journey):**
  1. An online or offline transaction is recorded.
  2. The backend ledger securely stores the transaction.
  3. The Finance Agent background job detects the new transaction, calculates the estimated tax liability, and updates the tenant's tax reserve.
  4. The user sees a clean, simple summary on their mobile dashboard indicating their total revenue and the amount safely reserved for taxes.

  **Acceptance Criteria:**
  - Create the `LEDGER_ENTRY` and `TAX_RESERVE` database tables with strict RLS (`tenant_id`).
  - Implement a background worker (simulated Finance Agent) that processes new ledger entries and updates the tax reserve.
  - Build a mobile-first (375px) React/Flutter UI component to display the financial summary.
  - 100% test coverage for the ledger logic.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
