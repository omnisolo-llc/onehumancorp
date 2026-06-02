issue_title: "[architecture]_autonomous_unified_ledger_and_multi_currency_settlement_engine.md"
issue_description: |
  **Problem Statement**
  Currently, OneHumanCorp (OHC) handles payments directly via Stripe, but lacks a robust internal financial ledger to coordinate multi-currency transactions, deposits, split-payments (for future marketplace expansion), and offline/cash settlements across the various personas (like Carlos the handyman or Priya the boutique owner). As our user base grows internationally and expands to complex workflows (e.g. partial deposits, cash-on-delivery, and gift cards), a scalable, multi-tenant Double-Entry Ledger System is required to maintain absolute financial truth.

  **Research Report**
  Leading platforms (Shopify, Stripe) decouple the payment gateway from the financial state machine. Shopify's "Balance" and Stripe's "Ledger" APIs treat every transaction as a double-entry accounting event. By introducing an internal Ledger, OHC can easily support multi-currency conversion, accurately compute tax liabilities (working with the Legal & Compliance Agent), and provide real-time, trustworthy financial reports via the Finance & Payments Agent ("The Accountant") without directly hammering external API rate limits.

  **Design Doc**
  *   **Architecture Diagram (Mental Model)**:
      *   `Ledger Service (Rust)` exposes gRPC endpoints: `RecordTransaction`, `GetBalance`, `GetStatement`.
      *   `PostgreSQL`: Tables `accounts` (tenant_id, account_id, currency, balance), `transactions` (tenant_id, tx_id, amount, currency, timestamp), and `entries` (tenant_id, entry_id, tx_id, account_id, direction, amount) with Row-Level Security.
      *   `Stripe Webhook Handler`: Translates `payment_intent.succeeded` into a Ledger transaction.
  *   **UI Wireframes/Mobile UX Flow (375px)**:
      *   A new "Financials" dashboard card displaying "Total Balance", "Pending Deposits", and "Recent Activity".
      *   Tap to view a transaction detail screen (macOS translucent glass style) showing the breakdown of tax, fees, and net revenue.
  *   **AI Agent Integration**:
      *   "The Accountant" agent continuously reads from the Ledger to generate weekly plain-language SMS/email reports.
      *   "The Advisor" uses Ledger aggregates to suggest pricing adjustments.

  **Implementation Prompt**
  Implement the core `Ledger Service` in Rust. Create the PostgreSQL schemas for `accounts`, `transactions`, and `entries` ensuring strict multi-tenant isolation. Expose gRPC endpoints for recording double-entry transactions and querying balances. Ensure 100% unit test coverage and E2E Playwright tests verifying the UI accurately displays the ledger balance. Do NOT hardcode currency conversions; leave hooks for a future exchange rate service.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
