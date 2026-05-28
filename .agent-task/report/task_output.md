issue_title: "[Architecture] Global Offline-First Localization & Currency Engine"
issue_description: |
  # Title: Global Offline-First Localization & Currency Engine

  ## Problem Statement
  Small business owners operate globally, often in environments with intermittent connectivity, yet their platforms fail to natively handle the complexity of multi-currency transactions, instant local translations, and real-time localized tax compliance without dropping offline capability. Fatima (food cart operator) struggles with the app not fully supporting Arabic and needs to instantly toggle languages to serve English-speaking tourists, all while offline. Leo (music tutor) teaches students globally and needs dynamic multi-currency pricing, but his current setup breaks if he tries to issue an invoice from an area with weak cell reception.

  The platform must natively provide an invisible layer that handles instant localization (UI + conversational AI) and multi-currency ledgers, ensuring this critical functionality is available offline-first. Without this, expanding into high-growth markets like LATAM or the Middle East is bottlenecked by connectivity and language barriers.

  ## Research Report
  *   **Codebase & Docs Audit:** Current architectures like `[architecture]_offline_first_mobile_pos.md` and `[finance]_instant_localized_invoicing.md` exist but lack a unified localization engine that synchronizes across the offline queue, the AI agent memory, and the core ledger natively.
  *   **Competitor Systems Audit:**
      *   *Shopify:* Strong multi-currency and localization (Shopify Markets), but fundamentally requires constant connectivity to apply dynamic exchange rates or switch complex localized rules at checkout.
      *   *Wix:* Relies on third-party apps for robust multi-currency, which breaks offline and adds latency.
  *   **Identify Gaps:** OHC lacks a unified, edge-cached, offline-capable engine for localization and multi-currency handling. The current offline POS can take payments, but cannot dynamically adjust for a localized currency or instantly switch the UI/AI conversational language without network access.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE_APP ||--o{ LOCAL_I18N_CACHE : "Reads UI Strings & Offline Exchange Rates"
      MOBILE_APP ||--o{ LOCAL_QUEUE : "Writes Localized Offline Tx"
      LOCAL_QUEUE }|--|| SYNC_ENGINE : "Batches Txs"
      SYNC_ENGINE ||--o{ MULTI_CURRENCY_LEDGER : "Reconciles Txs based on Sync-time Rates"
      MULTI_CURRENCY_LEDGER ||--o{ EXTERNAL_FX_PROVIDER : "Fetches Daily Rates (Background)"
      AI_AGENTS ||--o{ LOCAL_I18N_CACHE : "Generates Localized Responses"
  ```

  ### Mobile UX Flow (375px First)
  *   **Action:** Fatima opens the app in a crowded, offline festival. A tourist wants to pay in USD, but her default is local currency.
  *   **UI:** The 375px view presents a unified "Checkout" card. A small, elegant dropdown allows instant toggling of the display currency. The conversion uses the last-known cached exchange rate.
  *   **Feedback:** An offline toast notification subtly informs her: "Converted using yesterday's rate. Will finalize on sync."
  *   **Language Toggle:** A single tap changes the entire UI and AI response generation language (e.g., from Arabic to English) instantly, pulling from the local edge cache on the device.

  ### Key Design Decisions
  *   **Edge-Cached Exchange Rates:** The app downloads daily/hourly FX rates in the background. If offline, it uses the cached rate to estimate and authorize transactions locally.
  *   **Eventual Reconciliation Ledger:** The `MULTI_CURRENCY_LEDGER` handles the complexity. When the local queue syncs, the ledger compares the cached offline rate with the real-time rate, absorbing small differences or flagging large discrepancies to the Operations Agent.
  *   **Universal Content Translation Model (UCTM):** The device stores a lightweight translation model or pre-cached UI/common AI strings, allowing the app to switch languages fully offline.

  ### AI Agent Integration Points
  *   **Operations & Finance Agents:** The Finance Agent handles the FX reconciliation when offline transactions sync, masking the complexity of exchange rate spread from the user.
  *   **Customer Success Agent:** Uses the UCTM to generate localized, culturally appropriate responses to DMs or emails, even when degraded to offline caching (e.g., queuing localized responses).

  ## Implementation Prompt
  Implement the Global Offline-First Localization & Currency Engine. The system must consist of a secure, lightweight on-device cache for I18n strings and FX rates, integrated seamlessly with the existing `[architecture]_offline_first_mobile_pos.md` local queue. The backend `MultiCurrencyLedger` must support eventual consistency reconciliation, absorbing safe FX margins when offline transactions are synced. Provide an API for the AI agents to leverage this localized context. Ensure the UI implementation is mobile-first, allowing 1-tap, zero-latency language and currency toggling without requiring a network request. All multi-tenant isolation rules strictly apply.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
