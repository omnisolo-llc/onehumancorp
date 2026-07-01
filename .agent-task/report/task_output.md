issue_title: "Implement Global Offline-First Localization & Multi-Currency Engine"
issue_description: |
  # Mission Queue Protocol: Global Offline-First Localization & Multi-Currency Engine

  ## 1. Problem Statement
  Small business owners operate globally, often in environments with intermittent connectivity, yet their platforms fail to natively handle the complexity of multi-currency transactions, instant local translations, and real-time localized tax compliance without dropping offline capability. For instance, Fatima (food cart operator) struggles with the app not fully supporting Arabic and needs to instantly toggle languages to serve English-speaking tourists, all while offline. Leo (music tutor) teaches students globally and needs dynamic multi-currency pricing, but his current setup breaks if he tries to issue an invoice from an area with weak cell reception.

  ## 2. Research Report
  - **Market Context**: Leading platforms (Shopify, Stripe) decouple the payment gateway from the financial state machine. Shopify's "Balance" and Stripe's "Ledger" APIs treat every transaction as a double-entry accounting event. Shopify has strong multi-currency and localization (Shopify Markets), but fundamentally requires constant connectivity to apply dynamic exchange rates or switch complex localized rules at checkout. Wix relies on third-party apps for robust multi-currency, which breaks offline and adds latency.
  - **The OHC Opportunity**: By introducing an internal offline-first Ledger, OHC can easily support multi-currency conversion, accurately compute tax liabilities (working with the Legal & Compliance Agent), and provide real-time, trustworthy financial reports via the Finance & Payments Agent ("The Accountant") without directly hammering external API rate limits. Edge caching of dynamic catalog states, localized strings, and multi-currency exchange rates will allow the app to function flawlessly offline.
  - **Identify Gaps**: OHC lacks a unified, edge-cached, offline-capable engine for localization and multi-currency handling. The current offline POS can take payments, but cannot dynamically adjust for a localized currency or instantly switch the UI/AI conversational language without network access.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Mobile App)
      participant Edge Cache (Local DB)
      participant Sync Worker
      participant OHC Ledger (Backend)
      participant FX/Localization API

      User->>Edge Cache: Request UI in Arabic & Price in AED
      Edge Cache-->>User: Serve cached translations & FX rate (Offline)
      User->>Edge Cache: Complete Sale (Offline, 50 AED)
      Edge Cache->>Edge Cache: Store OperationIntent in Outbox

      Note over Sync Worker: Network restored

      Sync Worker->>OHC Ledger: Sync localized transaction (50 AED)
      OHC Ledger->>FX/Localization API: Verify/Update FX rates
      OHC Ledger-->>Sync Worker: Acknowledge sync, update Edge Cache
  ```

  ### Mobile UX Flow (375px)
  1. **Language/Currency Toggle**: A simple, accessible toggle in the POS or booking interface to switch between supported languages (e.g., English to Arabic) and currencies (e.g., USD to AED).
  2. **Offline Indicator**: A subtle UI pill indicating offline mode, assuring the owner that transactions are still securely logged and will sync later.
  3. **Seamless Transactions**: Customers see prices and receipts in their preferred language/currency, even when offline.

  ### AI Agent Integration Points
  - **Finance & Payments Agent**: Reconciles offline multi-currency transactions, flags significant FX rate changes, and generates plain-language financial summaries for the owner.
  - **Legal & Compliance Agent**: Ensures localized tax rules are correctly applied to offline transactions once synced.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Global Multi-Currency Engine
  **Target Personas**: Fatima (Food Cart Operator), Leo (Music Tutor), Priya (Boutique Owner)
  **Outcome**: Owners can seamlessly transact in multiple currencies and languages, even offline. The system automatically handles exchange rates, localized pricing, and synchronization, providing a flawless experience for both the owner and their global customers.

  **Next Actions**:
  1.  **Data Model**: Implement the `LedgerEntry` and `Currency` models in PostgreSQL with multi-tenant isolation, supporting double-entry accounting principles.
  2.  **Edge Caching**: Extend the local SQLite/Hive outbox pattern to cache exchange rates, localized product strings, and tax rules.
  3.  **UI Implementation**: Add language and currency toggles to the mobile POS interface (375px), ensuring they work seamlessly when the device is offline.
  4.  **Sync Worker**: Enhance the background sync worker to handle multi-currency `OperationIntents`, interacting with the internal ledger and resolving any FX drift.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
