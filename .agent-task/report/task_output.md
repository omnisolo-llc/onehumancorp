issue_title: "Architect and Implement the Invisible Autonomous Bookkeeping & Tax Mesh"
issue_description: |
  # Issue Brief: Invisible Autonomous Bookkeeping & Tax Mesh

  ## Problem Statement
  Small business owners—whether it's Maya the baker tracking flour costs, Carlos the handyman buying drill bits, or Fatima calculating local food cart tax—spend an average of 15-20 hours a month on bookkeeping. They dread tax season, often commingling personal and business expenses, hoarding receipts in shoeboxes, and struggling to understand their real profit margins. They don't want to learn QuickBooks, categorize transactions, or understand depreciation. They just want to know "how much money did I actually make today?" and "am I safe from an audit?" The cognitive load of financial compliance is a massive barrier to growth.

  ## Research Report
  **Market Gap Analysis:**
  *   **QuickBooks/Xero:** Built for accountants, not micro-business owners. Requires manual reconciliation, chart of accounts setup, and financial literacy. High friction, high cognitive load.
  *   **Shopify/Wix:** Provide basic sales tax calculations and payout reports, but completely ignore the expense side of the equation. Users still have to export data to third-party tools.
  *   **Stripe Tax:** Excellent for calculation, but still requires the merchant to manage the nexus tracking and remittance via external workflows.
  *   **OneHumanCorp Opportunity:** OHC already sees all incoming revenue via the universal ledger. By introducing an invisible expense ingestion layer (via linked bank accounts/cards and AI receipt scanning) and pairing it with an autonomous AI finance agent, we can eliminate bookkeeping entirely. The system categorizes expenses, matches them to revenue, calculates real-time profit, and maintains continuous tax compliance (sales tax and estimated income tax) without the user ever seeing a spreadsheet.

  **Target Personas:**
  *   **Carlos (Handyman):** Buys supplies at Home Depot. Snaps a picture of the receipt; the AI agent instantly logs it as a "Cost of Goods Sold" for his current bathroom remodel job, reducing his taxable income for that project.
  *   **Maya (Baker):** Connects her business debit card. The AI agent auto-categorizes her weekly bulk sugar purchases and automatically sets aside a portion of her daily cake sales into a virtual "Tax Vault" so she's never surprised at year-end.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Interface
          A[OHC Mobile App] -->|Receipt Photo/Upload| B(Expense Intake API)
          A -->|Dashboard View| C(Real-Time Profit/Tax API)
      end

      subgraph Invisible Finance Engine
          B --> D{AI Bookkeeper Agent}
          D -->|Vision/OCR| E[Receipt & Invoice Parser]
          D -->|Categorization| F[Ledger Transaction Mesh]

          G[Plaid/Stripe Issuing] -->|Card Swipes/Bank Sync| D
          H[OHC Universal Revenue Ledger] --> F

          F --> I{AI Tax Agent}
          I -->|Nexus & Rate Calculation| J[TaxJar/Stripe Tax API]
          I -->|Fund Allocation| K[Virtual Tax Vault / Escrow]
      end

      subgraph Data Layer
          F --> L[(Multi-Tenant Financial Ledger)]
          K --> L
      end
  ```

  ### Mobile UX Flow (375px First)
  1.  **The "Pulse" Dashboard:** A single, clean, translucent glass card at the top of the app: "Today's Net Profit: $340.00". No complex P&L statements.
  2.  **Expense Capture (The Shoebox):** A persistent camera button floating action button (FAB). Tap -> Snap receipt -> "Got it. Categorized as Supplies for $45.90."
  3.  **Tax Peace of Mind:** A small badge: "Tax Vault: $1,200 saved. You're on track for Q3."
  4.  **Invisible Interventions:** If an uncategorized bank transaction appears, the AI Assistant sends a chat message: "I saw a $200 charge at Best Buy. Was that for the business?" The user taps "Yes, new printer." The AI handles the rest.

  ### AI Agent Integration Points
  *   **AI Bookkeeper (Finance Dept):** Monitors all incoming bank feeds and receipt uploads. Uses LLMs to infer the business context of a purchase (e.g., "Flour" for a baker is COGS, "Flour" for a handyman is unusual and might trigger a clarifying question).
  *   **AI Tax Agent (Legal/Finance Dept):** Continuously calculates estimated tax liability based on real-time net income. Automatically sweeps funds from the main operational balance into a dedicated Tax Vault sub-account on every payout.
  *   **AI Operations/CS Dept:** Interacts with the user via natural language when clarification is needed, avoiding forms.

  ### Key Design Decisions
  1.  **Zero-Config Chart of Accounts:** The system creates and manages the chart of accounts invisibly based on the user's business type. The user never sees terms like "Asset," "Liability," or "Equity."
  2.  **Proactive Withholding (Tax Vaults):** To prevent end-of-year shock, the system automatically escrows a dynamically calculated percentage of every sale into a tax reserve.
  3.  **Chat-Based Reconciliation:** Fallback for uncategorized expenses is a simple chat prompt, not a spreadsheet reconciliation screen.
  4.  **Strict Multi-Tenancy & Immutability:** The financial ledger (backed by double-entry principles internally) is strictly partitioned by `tenant_id`. All records are append-only to ensure auditability, even though the complexity is hidden.

  ## Implementation Prompt
  **Context:** We need to implement the core ledger structure and AI routing for the Invisible Bookkeeping & Tax Mesh.
  **Outcome:** A backend service that can accept a raw transaction (either a bank feed item or a parsed receipt), automatically categorize it against a hidden standardized chart of accounts, and update the tenant's real-time profit and estimated tax liability.
  **Acceptance Criteria:**
  1.  The system ingests expense events and associates them with the correct `tenant_id`.
  2.  An AI service automatically categorizes the expense with >95% confidence; otherwise, it queues a natural language clarification request for the user.
  3.  The system recalculates the real-time "Net Profit" and updates the "Tax Vault" required balance immediately upon transaction ingestion.
  4.  The financial ledger enforces strict multi-tenant data isolation and append-only immutability.
  5.  Must meet a sub-200ms latency target for dashboard reads.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
