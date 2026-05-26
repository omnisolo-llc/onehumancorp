issue_title: "[Architecture] Invisible Autonomous Bookkeeping & Tax Reconciliation Engine"
issue_description: |
  # Title: Invisible Autonomous Bookkeeping & Tax Reconciliation Engine

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) despise accounting, bookkeeping, and tax preparation. They often mix personal and business finances, lose track of receipts, and are blindsided by tax bills at the end of the year. Current accounting tools (QuickBooks, Xero) are built for accountants, not micro-business owners, requiring manual categorization and double-entry bookkeeping knowledge. OHC users need an invisible engine that automatically reconciles transactions, estimates taxes, and prepares books in the background, requiring zero accounting knowledge.

  ## Research Report
  *   **Current Capabilities:** OHC handles payments and basic invoicing, but leaves ledger balancing, expense tracking, and tax estimation to third-party tools.
  *   **Competitor Analysis:**
      *   *QuickBooks / Xero:* Powerful but incredibly complex. High learning curve. Requires manual reconciliation or expensive accountant integration.
      *   *Shopify / Stripe:* Provides gross revenue and basic sales tax, but doesn't handle business expenses or income tax estimations.
      *   *Found / Novo:* Modern business banking that does some tax estimation, but disconnected from the core business operating system (inventory, CRM, bookings).
  *   **Gap Identified:** A deeply integrated, AI-driven bookkeeping engine that categorizes every inbound and outbound transaction natively within the OHC platform, estimating tax liability in real-time.
  *   **Strategic Advantage:** By solving the "tax time panic," OHC becomes indispensable. If the AI handles bookkeeping invisibly, churn drops significantly because the business owner relies on OHC for financial compliance and peace of mind.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ TRANSACTION : generates
      TRANSACTION ||--o{ CATEGORY : assigned_to
      TENANT ||--o{ LEDGER_ACCOUNT : has
      TRANSACTION ||--|{ LEDGER_ENTRY : creates
      TENANT ||--o{ TAX_ESTIMATE : tracks

      TENANT {
          string id PK
          string name
          string tax_region
      }
      TRANSACTION {
          string id PK
          string tenant_id FK
          float amount
          datetime timestamp
          string type "Income | Expense"
          string source "OHC Payment | Plaid Sync"
      }
      CATEGORY {
          string id PK
          string name "Supplies | Services | Meals"
          boolean tax_deductible
      }
      LEDGER_ENTRY {
          string id PK
          string transaction_id FK
          string account_id FK
          float credit
          float debit
      }
      TAX_ESTIMATE {
          string id PK
          string tenant_id FK
          float estimated_liability
          string period "Q1 | Q2 | Annual"
          datetime updated_at
      }
  ```

  ### Mobile UX Flow (375px First)
  1.  **Passive Monitoring:** The AI Finance Agent continuously monitors OHC payments and connected bank feeds (via Plaid) in the background.
  2.  **Daily Briefing Alert:** If a transaction is ambiguous (e.g., an unusually large Home Depot purchase), the daily mobile briefing asks: "Was this $500 Home Depot charge for a customer job or new equipment?"
  3.  **1-Tap Categorization:** The user taps "Customer Job (Materials)" on a native mobile card. The AI instantly updates the ledger and adjusts the tax deduction estimate.
  4.  **Financial Health Dashboard:** A simple "Money" tab in the app shows 3 numbers: "Cash on Hand", "Set Aside for Taxes", and "Profit This Month". A single button says "Generate Year-End Tax Report for Accountant".
  5.  **Receipt Capture:** User snaps a photo of a receipt. The AI Agent extracts the total, vendor, and date, auto-matching it to a bank feed transaction and throwing away the physical paper.

  ### AI Agent Integration Points
  *   **The Controller (Finance Dept):** Ingests raw transaction data, uses LLMs to probabilistically categorize expenses (e.g., "Uber" -> "Travel"), and balances the double-entry ledger without exposing the complexity to the user.
  *   **The Auditor (Legal/Compliance Dept):** Continuously calculates estimated local and federal tax liabilities based on categorized income and expenses, ensuring the "Set Aside for Taxes" number is accurate.
  *   **The Business Advisor:** Proactively suggests actions: "You have high cash reserves. Consider buying that new mixer before Dec 31 to lower your tax bill."

  ### Performance & Security Integrity
  *   **Zero-Trust Isolation:** Financial ledgers are strictly partitioned by `tenant_id` with immutable, append-only logs for auditability.
  *   **Mobile-First Performance:** The financial dashboard must load instantly (< 500ms). Heavy categorization algorithms run entirely asynchronously via a background job queue.
  *   **Compliance:** SOC2 and PCI compliance baselines applied to all ledger entries.

  ## Implementation Prompt
  Implement the Invisible Autonomous Bookkeeping & Tax Reconciliation Engine.
  The system must process inbound and outbound transactions, automatically applying ledger entries and categorization using the AI Finance Agent. It should maintain a real-time estimate of tax liabilities.
  The UI must completely hide standard accounting terminology (debits, credits, chart of accounts). Instead, present a mobile-first dashboard showing clear, actionable financial health metrics (Cash, Tax Liability, Profit) and 1-tap categorization prompts for ambiguous transactions. Acceptance criteria include: successful ingestion of a transaction, AI categorization, ledger balancing, and real-time update of the tax estimate on the mobile dashboard.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
