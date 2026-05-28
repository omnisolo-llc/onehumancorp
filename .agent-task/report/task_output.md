issue_title: "Architecture: Zero-Touch Bookkeeping & Cashflow Engine"
issue_description: |
  # Title: Zero-Touch Bookkeeping & Cashflow Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Carlos (handyman) spend late nights manually reconciling receipts, matching bank feeds, and categorizing expenses for taxes. They use fragmented tools like QuickBooks or Xero, which are built for professional accountants, not mobile-first entrepreneurs. This manual effort leads to delayed cashflow insights, missed tax deductions, and immense stress. They need a system that automatically categorizes every transaction in real-time and provides a single, unified view of cashflow without ever having to touch a ledger.

  ## Research Report
  *   **Competitor Analysis**:
      *   **QuickBooks Online**: Powerful but overly complex. Assumes basic accounting knowledge. Not natively integrated with the primary point of sale or booking system, requiring third-party connectors.
      *   **Xero**: Similar to QuickBooks, built for accountants. The mobile app is secondary to the desktop experience.
      *   **Wave Accounting**: Simpler, but lacks deep automation and still requires manual reconciliation of many bank feed items.
  *   **The OHC Differentiator**: OHC must provide "Zero-Touch Bookkeeping". Because OHC handles the entire lifecycle—from the initial quote/booking deposit to the final invoice and payment—we can automatically record both sides of every transaction. Our AI Finance Agent will categorize expenses from connected bank feeds and receipt scans invisibly.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      BankFeed[Plaid/Stripe Bank Feeds] --> EdgeGateway[Zero-Trust Edge Gateway];
      ReceiptScan[Mobile Camera Receipt Scan] --> EdgeGateway;
      OHC_Tx[OHC Internal Transactions] --> KAIROS[KAIROS Orchestration Hub];
      EdgeGateway --> KAIROS;

      KAIROS --> AIFinanceAgent[AI Finance Agent];

      AIFinanceAgent -->|Auto-Categorization & Match| UniversalLedger[(Universal Multi-Tenant Ledger)];

      UniversalLedger --> CashflowEngine[Real-Time Cashflow Engine];

      CashflowEngine --> Dashboard[OHC App: Cashflow Dashboard];
      CashflowEngine --> TaxPrep[Tax Prep & Export Module];
  ```

  ### Key Design Decisions & Invariants
  *   **Mobile-First UX Flow**: A 375px viewport dashboard displaying "Cash In" vs. "Cash Out" with large, easy-to-read typography (macOS translucent glass style). A single tap on "Needs Review" shows any uncategorized expenses for a swipe-to-categorize action (Tinder-style for expenses).
  *   **AI Auto-Categorization**: The AI Finance Agent uses historical data and OCR on receipts to categorize 95%+ of expenses automatically. Only low-confidence matches are bubbled up for user review.
  *   **Zero-Trust Multi-Tenancy**: Financial data is the most sensitive data. Strict tenant isolation ensures that Priya's transactions are cryptographically segregated from Carlos's. SPIFFE/SPIRE identity guarantees only the authorized Finance Agent can write to the ledger for a specific tenant.
  *   **Real-Time Sub-Second Latency**: The Cashflow Dashboard must render in under 300ms by querying a highly optimized read-replica or materialized view, even on slow 3G networks. Offline mode supports reviewing cached data.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the core logic for the Zero-Touch Bookkeeping & Cashflow Engine.
  1. Build the data ingestion pipeline that listens for incoming transactions from connected bank feeds and OHC's internal payment processor.
  2. Develop the `AIFinanceAgent` handler that receives unmapped transactions, uses the LLM/Categorization service to assign a standard chart of accounts category, and writes the reconciled entry to the `UniversalLedger`.
  3. Create the API endpoint for the mobile app to fetch the real-time cashflow summary (Cash In, Cash Out, Net) for a given time period (e.g., this month).
  4. **Acceptance Criteria**:
     - 90%+ of simulated test transactions must be auto-categorized correctly.
     - The cashflow summary endpoint must respond in under 300ms.
     - The solution must include unit tests verifying multi-tenant data isolation (Tenant A cannot see Tenant B's ledger).
     - Ensure the API response format maps directly to the UI components described in the design doc.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
