issue_title: "Implement Autonomous AI Tax and Financial Ledger Architecture for SMB Owners"
issue_description: |
  # Research Report: Embedded Finance and AI-Driven Taxation Ledger Architecture

  ## Problem Statement
  SMB owners (like Maya the baker or Carlos the handyman) lose countless hours managing fragmented financial tools, leading to missed tax deductions, poor cash flow visibility, and compliance risks. Current solutions require manual reconciliation and complex configuration. The gap is an invisible, autonomous financial layer within OHC that securely tracks cross-channel revenue, auto-categorizes expenses, estimates tax obligations, and reserves funds, seamlessly operating in the background.

  ## Research Report
  - **Market Context**: Traditional SMB accounting tools (QuickBooks, Xero) require financial literacy. Next-gen AI-native tools are starting to emerge, but lack deep integration with operational workflows (bookings, POS).
  - **Competitor Insights**: Shopify provides basic financial reports but relies on apps for tax and deep ledger functions. Platforms like Mercury and Novo offer banking, but are disconnected from the primary workflow system.
  - **User Need**: Owners need to know their exact financial position, tax liabilities, and have "safe-to-spend" balances, without manual tagging.

  ## Design Doc
  ### Architectural Principles
  - **Immutable Ledger**: Double-entry accounting system tracking all transactions (payments, refunds, internal transfers) with strict immutability.
  - **Multi-Tenant Isolation**: RLS (Row Level Security) at the database level ensuring strict separation between tenants.
  - **AI Categorization Engine**: Asynchronous job queue processing new transactions to categorize them (e.g., Supplies, Software, Taxable Revenue) using LLMs.
  - **Tax Sub-Ledger**: Dedicated accounts tracking estimated tax liabilities.

  ### Database Schema (High-Level)
  - `Tenant` -> `Account` (e.g., Operating, Tax Reserve)
  - `Account` -> `Transaction` (id, amount, timestamp, type, category)
  - `Transaction` -> `LedgerEntry` (debit_account, credit_account, amount)

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Sales[Sales/Bookings System] --> PaymentGateway[Payment Gateway webhook];
      PaymentGateway --> CoreAPI[OHC Core API];
      CoreAPI --> LedgerDB[(Immutable Ledger DB)];
      LedgerDB --> JobQueue[AI Categorization Queue];
      JobQueue --> AIWorker[AI Finance Worker];
      AIWorker --> LLM[LLM API];
      LLM --> AIWorker;
      AIWorker --> LedgerDB;
      LedgerDB --> DashboardAPI[Dashboard API];
      DashboardAPI --> MobileApp[Mobile App UI (375px)];
  ```

  ### Mobile UX Flow (375px)
  1. **Home Feed**: Owner sees a simplified "Safe to Spend" balance.
  2. **Transaction Tap**: Tapping a recent transaction shows its AI-assigned category and the estimated tax withheld.
  3. **Insights Screen**: A plain-language summary (e.g., "You have $1,200 set aside for quarterly taxes").

  ### AI Agent Integration
  - **Trigger**: New settled transaction.
  - **Context**: Transaction metadata, merchant name, amount.
  - **Action**: Assign category, estimate tax liability, and generate a plain-language summary for the owner.

  ## Implementation Prompt
  Implement the backend database schema, API endpoints, and mobile UI components for the Autonomous Financial Ledger.
  1. Create the immutable ledger schema with strict multi-tenant isolation.
  2. Implement the core API to record transactions and query account balances.
  3. Create an AI worker job that categorizes transactions and calculates estimated tax liabilities based on basic rules.
  4. Develop the mobile-first (375px) dashboard components displaying the "Safe to Spend" balance and a plain-language financial summary.
  Ensure comprehensive unit and Playwright E2E tests are included.

  Estimated Scope: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]

assignees: []
