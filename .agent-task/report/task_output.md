issue_title: "[Architecture] Autonomous Global Multi-Currency Engine Integration"
issue_description: |
  # Research Report: Autonomous Global Multi-Currency Engine Integration

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles multi-currency rates via `ohc_fx_rates` and `ohc_multi_currency_ledger`, but the `LedgerRepository` which handles Invoices operates strictly on a single currency model (`Invoice` has a `currency` column but `LedgerEntry` doesn't track presentment vs settlement currency or FX rate). To deliver "Zero-Config" localized pricing, we need to bridge the core Invoice generation and Ledger processing with the MultiCurrencyLedger.

  ## Codebase Audit
  - **Ledger Entries:** `src/server/domain/repository/models.rs` has `LedgerEntry` which only stores `credit` and `debit`. It does not store currency details or FX rates.
  - **Invoices:** The `Invoice` struct tracks a single `currency` and `total_amount`, missing a distinction between presentment (what the buyer sees) and settlement (what the merchant gets).
  - **MultiCurrencyLedger:** The `MultiCurrencyLedger::record_transaction` in `src/server/services/capital/multi_currency_ledger.rs` correctly applies FX and rounding, but it's isolated. It writes to `ohc_multi_currency_ledger` but doesn't integrate with `payment_events` or `ledger_entries` in `LedgerRepository`.

  ## Proposed Architecture & Design Doc

  ### Multi-Currency Flow
  1. **Invoice Creation:** Modify `CreateInvoiceDraftRequest` to optionally accept `presentment_currency`. If provided, it should determine the `settlement_currency` (the merchant's default) and calculate the localized `total_amount` using the FX rates in `MultiCurrencyLedger`.
  2. **Payment Application:** Enhance `ApplyPaymentRequest` to include `presentment_currency` and `cached_rate` (for offline support).
  3. **Ledger Update:** The `LedgerRepository::apply_payment_event` function must call `MultiCurrencyLedger::record_transaction` or duplicate its logic to ensure double-entry accounting records the exact FX rate, presentment amount, and settlement amount within the same transaction block as `ledger_entries`.

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant App (Mobile UX)
      participant Ledger API
      participant LedgerRepository
      participant MultiCurrencyLedger
      participant Database

      App (Mobile UX)->>Ledger API: POST /api/ledger/invoice/:id/pay
      Note over App (Mobile UX), Ledger API: Includes presentment_currency
      Ledger API->>LedgerRepository: apply_payment_event()
      LedgerRepository->>MultiCurrencyLedger: record_transaction(presentment, settlement)
      MultiCurrencyLedger->>Database: Fetch FX Rate
      MultiCurrencyLedger-->>LedgerRepository: Return locked FX Rate
      LedgerRepository->>Database: Insert PaymentEvent, LedgerEntry, & MultiCurrencyLedger
      Database-->>LedgerRepository: Success
      LedgerRepository-->>Ledger API: Success
      Ledger API-->>App (Mobile UX): 200 OK
  ```

  ### Mobile UX Integrity (375px)
  - Ensure the invoice creation UI on mobile displays a toggle to set the target customer's currency.
  - **Dashboard:** Show merchant balances strictly in home currency with a frosted-glass tooltip detailing "Cross-border fees absorbed".

  ### AI Agent Integration Points
  - **The Accountant (Finance Agent):** Monitors `ohc_multi_currency_ledger` for FX rate fluctuations and automatically notifies the merchant with plain-language insights on how exchange rate trends affect their net settlements.
  - **The Ambassador (Customer Success Agent):** Drafts cross-border follow-ups in the buyer's localized language, explicitly quoting their paid presentment amount for clarity.

  ## Implementation Prompt
  - Update `src/server/api/ledger.rs` to support multi-currency fields in its requests.
  - Integrate `MultiCurrencyLedger` into `LedgerRepository` or the `ledger.rs` API handlers.
  - Ensure transactions seamlessly write to both `ledger_entries` (for core accounting) and `ohc_multi_currency_ledger` (for FX tracking).
  - Add comprehensive unit testing for the integration flow in `src/server/domain/repository/ledger_repo.rs`.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
