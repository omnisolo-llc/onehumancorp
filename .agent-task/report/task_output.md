issue_title: "Implement Autonomous Unified Ledger & Multi-Currency Settlement Engine"
issue_description: |
  # Research Report: Autonomous Unified Ledger & Multi-Currency Settlement Engine

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles payments directly via Stripe, but lacks a robust internal financial ledger to coordinate multi-currency transactions, deposits, split-payments (for future marketplace expansion), and offline/cash settlements across the various personas (like Carlos the handyman or Priya the boutique owner). As our user base grows internationally and expands to complex workflows (e.g. partial deposits, cash-on-delivery, and gift cards), a scalable, multi-tenant Double-Entry Ledger System is required to maintain absolute financial truth.

  ## Research & Findings
  Leading platforms (Shopify, Stripe) decouple the payment gateway from the financial state machine. Shopify's "Balance" and Stripe's "Ledger" APIs treat every transaction as a double-entry accounting event. By introducing an internal Ledger, OHC can easily support multi-currency conversion, accurately compute tax liabilities (working with the Legal & Compliance Agent), and provide real-time, trustworthy financial reports via the Finance & Payments Agent ("The Accountant") without directly hammering external API rate limits.

  Competitive Analysis:
  - **Shopify**: Uses Shopify Balance to provide an embedded financial account for merchants.
  - **Stripe**: Provides Stripe Treasury and Ledger APIs for double-entry bookkeeping.
  - **Wix/Squarespace**: Generally rely on third-party integrations (like QuickBooks) rather than offering a native embedded ledger.

  ## Design Doc

  ### Architecture Diagram (Mental Model)
  *   **Ledger Service (Rust)**: Exposes gRPC endpoints: `RecordTransaction`, `GetBalance`, `GetStatement`.
  *   **PostgreSQL**:
      *   Tables: `accounts` (tenant_id, account_id, currency, balance), `transactions` (tenant_id, tx_id, amount, currency, timestamp), and `entries` (tenant_id, entry_id, tx_id, account_id, direction, amount).
      *   Row-Level Security (RLS) enabled on all tables using `tenant_id`.
  *   **Stripe Webhook Handler**: Translates `payment_intent.succeeded` and other relevant Stripe events into Ledger transactions.

  ### AI Agent Integration
  *   **The Accountant (Finance & Payments)**: Continuously reads from the Ledger to generate weekly plain-language SMS/email reports.
  *   **The Advisor (Business Advisory)**: Uses Ledger aggregates to suggest pricing adjustments and identify sales trends.

  ### Mobile UX Flow (375px)
  1.  **Dashboard**: A new "Financials" card on the main dashboard displaying "Total Balance", "Pending Deposits", and "Recent Activity".
  2.  **Detail View**: Tap to view a transaction detail screen (macOS translucent glass style) showing the breakdown of tax, fees, and net revenue.
  3.  **Reporting**: A simplified UI for generating standard reports (Income Statement, Balance Sheet) tailored for non-accountants.

  ## Implementation Prompt
  Implement the core `Ledger Service` in Rust. Create the PostgreSQL schemas for `accounts`, `transactions`, and `entries` ensuring strict multi-tenant isolation via RLS. Expose gRPC endpoints for recording double-entry transactions and querying balances. Ensure 100% unit test coverage and E2E Playwright tests verifying the UI accurately displays the ledger balance. Do NOT hardcode currency conversions; leave hooks for a future exchange rate service.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
