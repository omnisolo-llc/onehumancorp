issue_title: "[Architecture] Global Offline-First Localization & Currency Engine"
issue_description: |
  **Problem Statement**
  Small business owners operate globally, often in environments with intermittent connectivity, yet their platforms fail to natively handle the complexity of multi-currency transactions, instant local translations, and real-time localized tax compliance without dropping offline capability.

  The platform must natively provide an invisible layer that handles instant localization (UI + conversational AI) and multi-currency ledgers, ensuring this critical functionality is available offline-first.

  **Research Report**
  *   **Codebase & Docs Audit:** Current architectures like `[architecture]_offline_first_mobile_pos.md` exist but lack a unified localization engine that synchronizes across the offline queue, AI agent memory, and core ledger natively.
  *   **Competitor Systems Audit:** Shopify requires constant connectivity for dynamic exchange rates. Wix relies on third-party apps.
  *   **Identify Gaps:** OHC lacks a unified, edge-cached, offline-capable engine for localization and multi-currency handling.

  **Design Doc**
  - **Edge-Cached Exchange Rates:** The app downloads daily/hourly FX rates in the background. If offline, it uses the cached rate to estimate and authorize transactions locally.
  - **Eventual Reconciliation Ledger:** The `MULTI_CURRENCY_LEDGER` handles the complexity. When the local queue syncs, the ledger compares the cached offline rate with the real-time rate, absorbing small differences.
  - **Universal Content Translation Model (UCTM):** The device stores a lightweight translation model or pre-cached UI/common AI strings, allowing the app to switch languages fully offline.

  **Implementation Prompt**
  Implement the Global Offline-First Localization & Currency Engine. The system must consist of a secure, lightweight on-device cache for I18n strings and FX rates, integrated seamlessly with the existing `[architecture]_offline_first_mobile_pos.md` local queue. The backend `MultiCurrencyLedger` must support eventual consistency reconciliation, absorbing safe FX margins when offline transactions are synced. Provide an API for the AI agents to leverage this localized context. Ensure the UI implementation is mobile-first, allowing 1-tap, zero-latency language and currency toggling without requiring a network request. All multi-tenant isolation rules strictly apply.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
