issue_title: "[architecture] Universal Embedded Finance & AI Taxation Ledger"
issue_description: |
  # Title: Universal Embedded Finance & AI Taxation Ledger

  ## Problem Statement
  Small business owners like Maya (the Baker) and Carlos (the Handyman) operate in various locations and often collect payments via different channels—cash, bank transfers, online deposits, and in-person tap-to-pay. They are not accountants. Tracking these multi-channel revenue streams, splitting them automatically for taxes, predicting cash flow, and reconciling them into a unified ledger currently requires manual bookkeeping or expensive third-party tools (like QuickBooks or Xero), which is complex and time-consuming. When tax season arrives, these owners face massive anxiety because their data is disjointed. There is a need for an invisible, autonomous ledger embedded deeply into the OHC platform that intercepts every financial event, instantly calculates tax liabilities based on local nexus rules, and sets aside the funds automatically.

  ## Research Report
  *   **Current Architecture Limits:** OHC relies on generic integrations (Stripe) for online payments, but does not provide an internal, double-entry ledger that can reconcile offline transactions, split payments (e.g., instant tax set-asides), or autonomously forecast cash flow.
  *   **Competitor Analysis:**
      *   *Shopify:* Has Shopify Balance, but focuses heavily on eCommerce and lacks seamless local tax splitting for physical/service businesses without heavy app reliance.
      *   *Square:* Offers checking accounts and some tax features, but they are walled gardens and require explicit setup and management.
      *   *QuickBooks/Xero:* Too complex. Uses technical accounting jargon (Accounts Receivable, Chart of Accounts, Reconciliation) that confuses non-technical owners.
  *   **Discovery:** A core architectural gap is an "Autonomous AI Taxation & Financial Ledger." It must provide invisible, real-time tracking of all money movement (online, offline, cash, crypto). The Finance AI Agent needs direct read/write access to this ledger to give plain-language cash flow advice and automatically route tax percentages into a virtual "Tax Vault."

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      FINANCIAL-EVENT ||--o{ EVENT-ROUTER : "Incoming Trx (Stripe, POS, Cash Entry)"
      EVENT-ROUTER ||--o{ TAX-NEXUS-ENGINE : "Calculates Local/State Tax"
      EVENT-ROUTER ||--o{ DOUBLE-ENTRY-LEDGER : "Records Credit/Debit"
      DOUBLE-ENTRY-LEDGER ||--o{ VIRTUAL-WALLETS : "Main Balance & Tax Vault"
      VIRTUAL-WALLETS }|--|| FINANCE-AGENT : "Reads for Cash Flow Reports"
      FINANCE-AGENT ||--o{ MOBILE-DASHBOARD : "Presents Plain-Language Insights"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Mobile Dashboard (375px):**
      *   **Top Card:** "Available Cash: $4,200" (Glassmorphism card, green gradient).
      *   **Tax Vault Card:** "Set Aside for Taxes: $850" (Secured lock icon, reassuring blue).
      *   **Recent Activity:** Not "Invoice Paid" but "Carlos paid his $200 deposit for Tuesday's job."
      *   **Agent Interaction:** A chat bubble from "The Accountant" says: "You made $1,200 this week! I've already moved $240 into your Tax Vault. You're fully covered for Q3."

  ### AI Agent Integration Points
  *   **Finance & Payments Agent:** Constantly monitors the `DOUBLE-ENTRY-LEDGER`. At the end of every day, it uses LLM analysis to detect anomalous spending or outstanding invoices, and drafts a plain-language summary for the owner. It autonomously executes the transfer of funds into the "Tax Vault" virtual account.

  ### Security & Isolation
  *   Strict Row-Level Security (RLS) in PostgreSQL on the `DOUBLE-ENTRY-LEDGER` table, keyed by `tenant_id`. All ledger entries are immutable (append-only) to ensure auditability.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the "Universal Embedded Finance & AI Taxation Ledger." Design the append-only `double_entry_ledger` table with strict `tenant_id` RLS isolation. Create the gRPC services for the Event Router to intercept all checkout and POS events. Implement the Tax Nexus Engine (you can mock the external tax API for now) to calculate tax splits. Develop the Finance Agent's capability to read the ledger, calculate daily summaries, and generate plain-language insights. Expose a mobile-first (375px) API endpoint for the frontend dashboard to display "Available Cash" and "Tax Vault" balances. Write full unit and E2E Playwright tests covering a manual cash transaction entry through to the tax-split calculation and UI display.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
