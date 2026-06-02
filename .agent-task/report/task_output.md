issue_title: "[Architecture] Global Offline-First Localization & Currency Engine"
issue_description: |
  # Global Offline-First Localization & Currency Engine

  ## Problem Statement
  Small business owners like Fatima, operating a food cart with spotty internet connection in diverse neighborhoods, need the ability to seamlessly handle multiple currencies and languages. Currently, tools like Shopify require constant connectivity for dynamic exchange rates or language switching, rendering them useless in offline scenarios. OHC needs a robust, edge-cached, offline-first localization and currency engine.

  ## Research Report
  - **Competitive Landscape**: Shopify offers strong multi-currency and localization (Shopify Markets), but fundamentally requires constant connectivity to apply dynamic exchange rates or switch complex localized rules at checkout.
  - **OHC Ecosystem Gap**: Existing designs like `[architecture]_offline_first_mobile_pos.md` support offline POS, but lack unified localization engine that synchronizes across the offline queue, AI agent memory, and core ledger natively.
  - **The Opportunity**: Build a global offline-first localization & currency engine that provides an invisible layer for instant localization (UI + conversational AI) and multi-currency ledgers, ensuring this critical functionality is available offline-first.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App / POS] -->|Offline Sync| B(Local SQLite/CRDT)
      B -->|FX Rates & I18n Cache| C{Offline Decision Engine}
      C -->|Process Transaction| D[Local Queue]
      D -->|Network Restored| E[OHC API Gateway]
      E -->|Reconcile & Apply Margins| F[MultiCurrencyLedger]
      E -->|Fetch Updates| B
  ```

  ### Mobile UX Flow
  1. **Settings / General Tab:**
     - User navigates to Settings and sets primary language and secondary language.
  2. **Dashboard / Cash Register:**
     - User can toggle language or currency using a localized toggle switch.
  3. **Offline mode:**
     - A UI indicator shows "Offline", however toggles still function, and exchange rates continue working using the cached margin rates.

  ### AI Agent Integration Points
  - **Finance Agent**: Automatically calculates safe FX margins for offline transactions and reconciles them when online.
  - **Customer Success / Ops**: AI agents use the locally cached language preferences to instantly communicate with the customer in their preferred language, even when offline.

  ### Key Design Decisions
  - **Offline FX Margins**: The system stores conservative exchange rate buffers locally to ensure transactions made offline do not result in losses upon synchronization.
  - **Zero-Latency Toggling**: Language and currency toggling in the UI must happen instantly without network requests, relying on the local CRDT store.
  - **Strict Multi-Tenant Isolation**: FX rates and language dictionaries are isolated securely per tenant.

  ## Implementation Prompt
  Implement the Global Offline-First Localization & Currency Engine.
  - The system must consist of a secure, lightweight on-device cache for I18n strings and FX rates, integrated seamlessly with the existing `[architecture]_offline_first_mobile_pos.md` local queue.
  - The backend `MultiCurrencyLedger` must support eventual consistency reconciliation, absorbing safe FX margins when offline transactions are synced.
  - Provide an API for the AI agents to leverage this localized context.
  - Ensure the UI implementation is mobile-first, allowing 1-tap, zero-latency language and currency toggling without requiring a network request. All multi-tenant isolation rules strictly apply.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
