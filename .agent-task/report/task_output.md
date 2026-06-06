issue_title: "[Architecture] Offline-First Multi-Currency Synchronization Protocol"
issue_description: |
  # Global Offline-First Localization & Currency Engine

  ## Problem Statement
  Small business owners operate globally, often in environments with intermittent connectivity. They need their platforms to handle multi-currency transactions, instant local translations, and real-time localized pricing without dropping offline capability.

  Currently, OHC requires a network connection for the pricing engine to calculate foreign exchange rates and localize UI responses. This blocks merchants in emerging markets or low-connectivity environments from conducting business locally or cross-border.

  ## Research Report
  - We analyzed the existing `[finance]_instant_localized_invoicing.md` and `[architecture]_offline_first_mobile_pos.md` designs. While the POS queue handles offline transactions, it lacks the ability to lock FX rates locally or apply cosmetic rounding (e.g., .99) when offline.
  - Competitors like Shopify rely on online-only edge functions for currency conversion, breaking the checkout flow when network access is lost.

  ## Design Doc
  We propose a unified **Offline-First Localization & Currency Sync Engine**:
  1.  **On-Device FX Cache:** The mobile app fetches daily exchange rates and cosmetic rounding configurations during the last successful sync.
  2.  **Local Pricing Inference:** When offline, the POS and storefront UI apply the cached FX rate and cosmetic rounding locally (e.g., $54 USD -> €49.99 EUR using yesterday's cached rate).
  3.  **Eventual Consistency Ledger:** Transactions are queued with the applied offline FX rate. Upon network restoration, the backend `MultiCurrencyLedger` compares the offline rate to the real-time rate. The platform absorbs safe FX margins or flags significant discrepancies to the Operations Agent.
  4.  **Instant UI Localization:** A Universal Content Translation Model (UCTM) cache provides zero-latency language toggling without a network request.

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

  **The "Grandmother Test" Mobile Flow:**
  1.  **Action:** Fatima opens the app in a crowded, offline festival. A tourist wants to pay in USD, but her default is local currency.
  2.  **UI:** The 375px view presents a unified "Checkout" card. A small, elegant dropdown allows instant toggling of the display currency. The conversion uses the last-known cached exchange rate.
  3.  **Feedback:** An offline toast notification subtly informs her: "Converted using yesterday's rate. Will finalize on sync."
  4.  **Language Toggle:** A single tap changes the entire UI and AI response generation language (e.g., from Arabic to English) instantly, pulling from the local edge cache on the device.

  ### AI Agent Integration Points
  -   **Operations & Finance Agents:** The Finance Agent handles the FX reconciliation when offline transactions sync, masking the complexity of exchange rate spread from the user.
  -   **Customer Success Agent:** Uses the UCTM to generate localized, culturally appropriate responses to DMs or emails, even when degraded to offline caching (e.g., queuing localized responses).

  ### Implementation Prompt
  Implement the Offline-First Localization & Currency Sync Engine for the mobile POS and UI.
  - Update the edge-cached localization engine to push a daily FX/I18n digest to the client.
  - Integrate the offline FX calculator into the mobile checkout flow, ensuring prices are cosmetically rounded locally.
  - Modify the `MultiCurrencyLedger` backend to process offline-queued transactions, applying eventual consistency reconciliation logic for FX variances.
  - Write E2E Playwright tests simulating a user toggling languages offline and completing an offline multi-currency transaction that successfully reconciles upon sync.
  - Ensure the payload for the edge sync operates under 50kb for low data connectivity.

  ### Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
