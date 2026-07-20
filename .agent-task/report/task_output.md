issue_title: "Implement Multi-Currency Auto-Reconciliation Engine"
issue_description: |
  ## Title
  Implement Multi-Currency Auto-Reconciliation Engine

  ## Problem Statement
  As international users like Priya scale, handling sales in multiple currencies and settling them in a base currency creates massive accounting overhead. Small business owners currently export Stripe logs and manually reconcile exchange rates in spreadsheets. OHC needs an invisible financial agent that automatically matches payouts to multi-currency sales, calculates FX variance, and presents a clean daily ledger without the owner touching a spreadsheet.

  ## Research Report
  - **Market Gaps**: Shopify handles multi-currency at checkout but buries the reconciliation in complex finance reports. Xero/QuickBooks require manual bank feed matching.
  - **OHC Advantage**: OHC already processes transactions. By introducing an internal double-entry ledger tightly coupled to the AI Finance Agent, OHC can instantly present "Realized Revenue" vs "Pending FX variance" on a 375px mobile screen.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Multi-Currency Sale] --> B[Stripe Webhook]
      B --> C[Ledger Engine]
      C --> D[Double Entry: AR & FX Suspense]
      E[Stripe Payout Webhook] --> C
      C --> F[Finance Agent: Auto-match & clear Suspense]
      F --> G[Mobile Dashboard Update]
  ```

  ### Mobile UX Flow
  - **Home Dashboard (375px):** A unified "Net Cash Today" card showing the base currency amount, abstracting away the multi-currency complexity.
  - **Tap for details:** A slide-over panel shows exactly which international sales made up the payout, with any FX gains/losses clearly labeled in plain English (e.g., "Currency shift since sale: +$4.20").

  ### AI Agent Integration
  - **Finance Agent**: Subscribes to Stripe `payout.paid` webhooks. It queries the local ledger for open Accounts Receivable, matches the payout amount considering exchange rate timestamps, and commits the reconciling ledger entries.

  ## Implementation Prompt
  Implement the core `LedgerTransaction` and `LedgerLine` models with multi-tenant row-level security. Expose an internal gRPC service for the Finance Agent to record double-entry transactions (Debit/Credit must equal zero). Build the mobile-first UI component that displays a daily financial summary card translating these ledger entries into plain-English "Cash In" and "Pending" states, hiding the double-entry mechanics from the user. Ensure full E2E testing using Playwright to verify the dashboard accurately reflects the underlying ledger state.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
