issue_title: "[finance] Native QuickBooks Online Integration for Automated Accounting Sync"
issue_description: |
  Small business owners like Priya (Boutique) and Carlos (Handyman) struggle with bookkeeping and tax preparation. They rely on manually exporting sales data, refunds, and tax collections into CSV files to send to their accountant or manually enter into QuickBooks. This error-prone process takes hours each month and often results in miscategorized expenses or missing revenue records. They need an automated "set-it-and-forget-it" way to sync OHC transactions directly into their QuickBooks Online account.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Offers Quickbooks integration but often requires paid third-party apps like "Sync to QuickBooks". Complex mapping required.
  - **Wix:** Has an integration with QuickBooks but limited sync on advanced products or variants.
  - **Square:** Good native sync with QuickBooks but limits business owners to the Square ecosystem.
  - **Current OHC State:** Transactions remain siloed within OHC's ledger.

  **Market Needs:**
  Business owners want their tax prep and accounting fully automated without manual reconciliation. Direct API integration with the QuickBooks Online Accounting API removes one of the biggest administrative burdens, reinforcing OHC's value proposition of "Radical Simplicity".

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph OHC Backend
          Ledger[OHC Transaction Ledger];
          FinanceAgent[Finance AI Agent];
          SyncWorker[QuickBooks Sync Background Worker];
          OAuthConfig[OAuth Secure Vault];
      end

      subgraph Intuit
          QBAPI[QuickBooks Online API];
      end

      Ledger -- Triggers on new Tx/Refund --> SyncWorker;
      OAuthConfig -- Provides Tokens --> SyncWorker;
      SyncWorker -- Map to SalesReceipt/RefundReceipt --> QBAPI;
      SyncWorker -- Error/Success Status --> FinanceAgent;
      FinanceAgent -- Notify User --> OHCApp[OHC Mobile App];
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** User opens "Finance & Payments" settings. A new card displays "Connect QuickBooks".
  2. **Connection:** User taps and is redirected to the native Intuit OAuth flow.
  3. **Configuration:** The Finance AI Agent asks a few plain-language conversational questions to map OHC data to their QuickBooks chart of accounts (e.g., "Where should we record sales income?", "Which account holds your collected taxes?").
  4. **Visibility:** On the home dashboard, the Finance AI Agent leaves an activity card: "Synced 14 transactions to QuickBooks today."

  ### AI Agent Integration Points
  - **Finance AI Agent:** Monitors the sync status and alerts the business owner in plain language if an error occurs (e.g., "Hey Priya, your QuickBooks sync paused because your subscription expired"). It manages mapping questions.

  ### Key Design Decisions
  - **Direct API Mapping:** Leverage the QuickBooks `SalesReceipt`, `RefundReceipt`, and `Payment` endpoints.
  - **Background Queuing:** Utilize the existing high-performance job queue to decouple the sync process from checkout latency.

  ## Implementation Prompt
  Implement a secure OAuth 2.0 flow for Intuit/QuickBooks Online. Create the necessary data synchronization workers to listen to OHC's order and payment events and push corresponding SalesReceipts or Invoices to the QuickBooks API.

  - **User-Facing Outcome:** User can successfully authenticate and connect their QuickBooks Online account via "Finance & Payments". New sales/refunds processed in OHC automatically appear as matching transactions in the connected QuickBooks account.
  - **CUJ:** Priya completes an order. The background worker maps it to a QuickBooks `SalesReceipt`. Priya sees the updated balance in her QuickBooks app.
  - **Acceptance Criteria:**
    - OAuth flow securely saves access/refresh tokens.
    - Sales mapping logic gracefully handles discounts and taxes.
    - Failing syncs retry with exponential backoff and alert the Finance Agent.

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
