issue_title: "Implement Zero-Click Universal Multi-Currency Payout Ledger"
issue_description: |
  # Mission Queue Protocol: Zero-Click Universal Multi-Currency Payout Ledger

  ## Problem Statement
  Platform users (such as creators like Leo or marketplace operators like Jun) struggle immensely with multi-currency reconciliation and reliable payouts. Existing small business platforms either limit users to a single base currency or force them to use complex, expensive third-party tools (like Wise or native Stripe Connect) without providing a unified, plain-language financial assistant. Owners waste hours matching cross-border sales against local bank deposits, often losing track of FX fees and net revenue.

  ## Research Report
  **Market Context & Competitive Analysis:**
  - **Shopify Markets:** Handles multi-currency well for buyers, but the payout reconciliation for merchants remains a complex spreadsheet exercise.
  - **Stripe Connect:** Powerful but highly technical to integrate and manage for non-developers.
  - **Wix:** Basic currency conversion; lacks deep multi-currency ledger capabilities.
  - **The OHC Opportunity:** By introducing an autonomous "Accountant Agent" coupled with a highly scalable, multi-tenant Ledger architecture, OHC can instantly reconcile multi-currency payments, track FX fees, and proactively notify the owner (e.g., Leo) exactly how much will land in his local bank account and when, without him ever opening a spreadsheet.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Multi-Currency Checkout] -->|Payment Intent| B(Payment Gateway)
      B -->|Webhook: payment_succeeded| C{Ledger Ingestion Engine}
      C -->|Record Transaction| D[(PostgreSQL: Ledger & Entries)]
      D --> E[FX Reconciliation Job]
      E --> F[The Accountant Agent]
      F -->|Analyze Payout| G[Generate Plain-Language Summary]
      G --> H[Mobile Feed Card 375px]
      H -->|Tap to View Details| I[Detailed Ledger UI]
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed:** Leo receives a weekly push notification: "Your Payout Summary is ready."
  2. **Summary Card:** Tapping the notification reveals a plain-language summary: "You earned $1,200 USD and €400 EUR this week. After conversion and fees, $1,620 USD will hit your Chase account tomorrow."
  3. **Interaction:** Leo can tap to see a breakdown of individual transactions, clearly separating product revenue, tax collected, and payment processing fees.
  4. **Visuals:** Signature OHC Premium Token styling—clean typography, color-coded positive/negative ledger entries, and clear status indicators (Pending, Paid).

  ### AI Agent Integration Points
  - **The Accountant Agent (Finance Department):** Listens to ledger events. Uses RAG against tenant's historical financial data and current FX rates. Decides when to generate a summary based on payout schedules, providing actionable insights (e.g., "Your European sales are up 20%, consider running a localized ad").

  ### Key Design Decisions
  - **Double-Entry Ledger:** Must implement strict double-entry accounting principles at the database level to ensure zero financial data loss or inconsistency.
  - **Multi-Tenant Isolation:** `tenant_id` RLS policies are critical for financial data security.
  - **Zero-Configuration:** Owners do not set up charts of accounts; the system intelligently maps checkout items (products, shipping, tips, taxes) to appropriate internal ledger categories automatically.

  ## Implementation Prompt
  **User-Facing Outcome:** As Leo, I receive a simple, accurate summary on my phone every week explaining exactly what I earned across all currencies and when it will be deposited, completely eliminating the need for manual financial reconciliation.
  **CUJ & Acceptance Criteria:**
  1. Create `LedgerAccount` and `LedgerEntry` double-entry models in PostgreSQL with RLS.
  2. Implement the webhook listener for `payment_succeeded` that translates raw payment data into balanced ledger entries, including FX conversion handling.
  3. Develop The Accountant Agent capability to parse the ledger and generate a weekly plain-language payout summary.
  4. Build the mobile-first (375px) payout summary UI card.
  5. Provide Playwright E2E tests: Simulate checkouts in USD and EUR, verify the ledger balances correctly, and verify the Accountant Agent drafts the correct summary card in the mobile UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
