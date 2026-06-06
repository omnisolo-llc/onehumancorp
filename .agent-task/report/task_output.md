issue_title: "Implement Instant Localized Payouts and Virtual Card Issuing Engine"
issue_description: |
  # Research Report: Instant Localized Payouts and Virtual Card Issuing Engine

  ## Problem Statement
  Small business owners—like Maya the baker or Fatima the food cart operator—often rely on deposits or daily sales to buy the very supplies they need to fulfill orders. Traditional payment processors hold funds for 2-5 business days before paying out to an external bank account. This artificial delay creates a cash flow choke point.
  OneHumanCorp needs to bypass this latency entirely by issuing a platform-native Virtual Wallet and Business Debit Card, providing true instant liquidity the second a transaction clears.

  ## Findings
  - **Ledger Exists**: The backend already has some ledger components (`src/server/services/capital/ledger.rs`, `src/server/services/capital/multi_currency_ledger.rs`).
  - **Wallet/Card Missing**: There is no existing code for `wallet` or `virtual_card` in the Rust backend.
  - **UI/UX Needs**: Need to implement a Wallet Dashboard Card in the UI and a Virtual Card Reveal Flow.

  ## Proposed Next Steps
  1. **Backend API & Service**: Create new Rust modules for the `Wallet` and `VirtualCard` data models and logic in `src/server/domain` and `src/server/services/capital`.
  2. **Data Model**:
     - `OHC_WALLET`: id, tenant_id, available_balance, currency
     - `VIRTUAL_CARD`: id, wallet_id, status, tokenized_pan
  3. **Integration**: Link the existing `Ledger` to the new `Wallet` logic so a transaction (deposit) instantly updates the wallet's available balance.
  4. **AI Finance Agent Integration**: Publish events for balance updates that the AI Finance agent can consume to warn about low balances or suspicious transactions.
  5. **Frontend UI**: Implement the Mobile UI components for the Wallet Dashboard and Virtual Card Reveal Flow in the Tauri desktop/mobile app (`src/ui/tauri`).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
