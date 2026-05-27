issue_title: "Design: Invisible Accounting & Tax Reconciliation Engine"
issue_description: |
  # [Architecture] Invisible Accounting & Tax Reconciliation Engine

  ## Problem Statement
  For small business owners like Maya (baker) and Carlos (handyman), tax season and monthly bookkeeping are anxiety-inducing nightmares. They collect revenue across different platforms, incur expenses on personal and business cards, and have no idea what their true profit is or how much they owe in local, state, or federal taxes. Traditional accounting software requires them to understand "charts of accounts," "double-entry bookkeeping," and manual bank statement reconciliation—failing the grandmother test entirely. They need an invisible, automated system that continuously tracks income, categorizes expenses, calculates tax liabilities in real-time, and holds the necessary funds aside, so they always know exactly where they stand without ever opening a spreadsheet.

  ## Research Report
  ### Competitive Landscape
  *   **QuickBooks / Xero**: The gold standards for accountants but deeply intimidating for solopreneurs. They require manual categorization, complex setup, and are built around the concept of "reconciliation" that the user must perform manually. They are desktop-first paradigms forced onto mobile.
  *   **Stripe Tax / Shopify Tax**: Excellent at calculating sales tax at checkout based on nexus, but they do not solve the broader problem of income tax, expense categorization, or overall business profitability tracking.
  *   **Catch / Keeper Tax**: Good at estimating taxes for 1099 freelancers by scraping bank accounts, but they exist as separate apps outside of where the actual business operations (invoicing, checkout, inventory) happen.

  ### The OHC Gap
  OneHumanCorp has robust ledger systems for invoicing (`[architecture]_instant_localized_invoicing_ledger.md`) and split payments (`[architecture]_invisible_multi_party_split_payments_ledger.md`), but lacks an overarching, invisible accounting layer that unifies these streams, automatically categorizes expenses, and continuously calculates tax liability (both sales and income). We need an AI Finance Agent that acts as a continuous, invisible bookkeeper.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ LEDGER_ACCOUNT : owns
      TENANT ||--o{ TAX_NEXUS : registered_in
      LEDGER_ACCOUNT ||--o{ LEDGER_ENTRY : contains
      LEDGER_ENTRY }|--|| TRANSACTION_EVENT : triggered_by
      TRANSACTION_EVENT ||--o{ RECEIPT : documented_by

      TENANT {
          string id PK
          string business_type
          string tax_id
      }
      TAX_NEXUS {
          string id PK
          string region "e.g., US-CA, EU-FR"
          float current_sales_tax_liability
          float current_income_tax_liability
      }
      LEDGER_ACCOUNT {
          string id PK
          string type "Revenue | Expense | Liability | Asset"
          float balance
      }
      LEDGER_ENTRY {
          string id PK
          float amount
          string direction "Credit | Debit"
          timestamp posted_at
      }
      TRANSACTION_EVENT {
          string id PK
          string category "e.g., Supplies, Software, Service"
          string source "Invoice | Checkout | BankSync"
      }
  ```
  ```mermaid
  sequenceDiagram
      participant Mobile as Mobile App (375px)
      participant Ops as Ops AI Agent (Receipts)
      participant Finance as Finance AI Agent (Treasurer)
      participant Ledger as Universal Ledger
      participant TaxEngine as Tax Liability Engine

      Mobile->>Ops: Maya uploads photo of flour receipt ($50)
      Ops->>Ops: OCR & Categorization (Category: Supplies)
      Ops->>Finance: Event: Expense logged ($50)
      Finance->>Ledger: Debit Expense Account, Credit Asset (Cash)
      Finance->>TaxEngine: Recalculate Income Tax Liability
      TaxEngine->>Ledger: Adjust Tax Liability Account (lower liability)
      Finance->>Mobile: Update "Safe to Spend" Dashboard Card
  ```

  ### Mobile UX Flow (375px First)
  1.  **The "Safe to Spend" Dashboard (Translucent Glass UI):** The main screen prominently features a single, easy-to-understand number: "Safe to Spend." This is total cash minus calculated tax liabilities and upcoming automated bills. No P&L statements required.
  2.  **Magic Receipt Scanner:** A prominent "+" button allows Maya to quickly snap a photo of a receipt. The Ops AI instantly categorizes it (e.g., "Baking Supplies") and calculates the tax deduction impact invisibly.
  3.  **Tax Vault Auto-Sweep:** A visual card showing the "Tax Vault." When Carlos gets paid $1000 for a job, the UI smoothly animates 20% (or whatever his calculated effective tax rate is) moving into the visually distinct "Tax Vault," ensuring he never accidentally spends money owed to the government.
  4.  **Plain Language Reports:** If the user taps for more detail, they don't see a "Chart of Accounts." They see natural language summaries: "You made $4,000 this month. You spent $800 on supplies. You've set aside $600 for taxes. You're doing great!"

  ### AI Agent Integration Points
  *   **The Treasurer (Finance AI):** Constantly monitors the event mesh for all transactions (invoices paid, split payments executed, receipts uploaded). It maps these to the underlying double-entry ledger without exposing the complexity to the user. It also recalculates tax liabilities on the fly.
  *   **The Vigilant Manager (Ops AI):** Handles the ingestion of offline expenses via receipt OCR and bank feed parsing, categorizing them using historical context.

  ### Key Design Decisions & Integrity
  *   **Continuous Reconciliation:** Traditional accounting is done in batches (monthly reconciliation). OHC accounting is continuous. Every transaction instantly updates the global ledger and tax liability estimates.
  *   **The "Safe to Spend" Paradigm:** We shift the mental model from "Balance" to "Safe to Spend." This prevents the most common SMB failure mode: spending tax money.
  *   **Zero-Trust Multi-Tenancy:** The ledger structure guarantees that Tenant A's financial data is mathematically isolated from Tenant B's, enforced via SPIFFE/SPIRE identity propagation.
  *   **No Accounting Jargon:** The UI must pass the grandmother test. Terms like "Debit", "Credit", "Reconciliation", and "Amortization" are strictly banned from the primary user interface.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the backend infrastructure for the "Invisible Accounting & Tax Reconciliation Engine."

  **Core User Journey (CUJ):**
  Maya snaps a picture of a receipt for baking supplies. The system automatically categorizes the expense, records the transaction in the double-entry ledger, and immediately recalculates her "Safe to Spend" balance and estimated tax liability, updating her mobile dashboard in real-time.

  **Acceptance Criteria:**
  *   **Event-Driven Ledger Updates:** The system must listen for transaction events (both income and expenses) and automatically apply the correct credit/debit entries to the appropriate ledger accounts.
  *   **Real-Time Tax Liability Tracking:** The system must maintain a running estimate of tax liability (both sales and income tax) that updates automatically with every transaction.
  *   **Mobile-First API:** Expose endpoints that power the "Safe to Spend" metric and the plain-language financial summaries, optimized for sub-200ms response times on mobile clients.
  *   **Strict Isolation:** Ensure all ledger queries and updates are rigorously scoped to the specific tenant ID.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

  ## Priority
  P0

  ## Estimated Scope
  Large
