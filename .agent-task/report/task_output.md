issue_title: "Architecture Design: Multi-Currency & Localized Pricing Engine"
issue_description: |
  # Multi-Currency & Localized Pricing Engine

  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. Priya (boutique owner) ships globally but struggles to manually calculate exchange rates and update prices for her international customers; she loses sales when Canadian or European buyers see USD prices and abandon their carts. Leo (music tutor) teaches students in the UK and Australia but invoicing them in USD causes confusion and hidden conversion fees for his students. Currently, OHC displays a single base currency for all storefronts and invoices. Business owners lack the time, financial expertise, and technical capability to configure localized pricing, tax routing, and foreign exchange (FX) risk mitigation. They need a system that invisibly handles multi-currency pricing, local payment methods, and automated FX reconciliation without any manual configuration.

  ## Research Report
  - **Competitor Analysis:**
    - *Shopify:* Offers strong multi-currency and localization (Shopify Markets), but fundamentally requires constant connectivity to apply dynamic exchange rates or switch complex localized rules at checkout. It is also complex for non-technical users to set up correctly.
    - *Wix:* Relies on third-party apps for robust multi-currency, which breaks offline and adds latency.
    - *Stripe Billing/Invoicing:* Powerful and scalable, with multi-currency and tax localization capabilities. However, it is heavily developer-focused and the dashboard can be overwhelming for non-technical users.
  - **OHC Gap:** OHC lacks a unified, edge-cached, offline-capable engine for localization and multi-currency handling. The current system and POS can take payments, but cannot dynamically adjust for a localized currency or instantly switch the UI/AI conversational language without network access.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Central Ledger] --> B[Multi-Currency Exchange Cache]
      B --> C[Edge Caching Storefront]
      B --> D[Offline POS App]
      C --> E[Customer Local UI]
      D --> F[Local Tap to Pay]
      E --> G[Agent: The Ambassador]
      F --> H[Agent: The Accountant]
      G --> I[Stripe Intents/Sessions]
      H --> I
      I --> A
  ```

  ### Mobile UX Flow
  1. **Global Sales Toggle (375px):** In Settings > Payments, the owner toggles "Enable Global Multi-Currency" with a single tap. There is no complicated FX spread setup.
  2. **Automated Conversion Display:** When enabled, storefront visitors (or invoice recipients) automatically see prices in their local currency based on IP or browser locale. The UI cleanly shows a "Prices in EUR" indicator.
  3. **Invoice Generation:** When Leo creates an invoice, he selects the student. The system automatically detects the student's location (UK) and proposes the invoice in GBP, showing the estimated USD payout to Leo.
  4. **POS Offline Mode:** The POS app caches the last known exchange rates. If Fatima takes an order from a tourist paying in a different accepted currency (if configured), the app uses the cached rate and syncs the exact conversion when reconnected.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success):** Automatically detects the customer's language and currency context. When drafting replies to inquiries, it quotes prices in the customer's local currency.
  - **The Accountant (Finance & Operations):** Reconciles the multi-currency payouts from Stripe into the single base currency of the business. It provides a simple summary to the owner: "You made $500 this week, including 5 sales in Euros which were converted automatically."

  ### Key Design Decisions
  - **Zero-Config Activation:** The feature must be a simple toggle. OHC handles the complexity of syncing rates and configuring Stripe under the hood.
  - **Offline-First Caching:** Exchange rates are pushed to the edge and cached on mobile clients to ensure POS and storefronts remain functional during connectivity drops.
  - **Unified Ledger Integration:** All multi-currency transactions must map back to a robust double-entry ledger to ensure perfect financial reconciliation.

  ## Implementation Prompt
  Implement the "Enable Global Multi-Currency" toggle in the merchant settings. Update the storefront and invoice generation flows to automatically localize prices based on the viewer's context (mocking the locale detection for E2E purposes if needed). Integrate this with the underlying double-entry ledger to track the original currency amount and the settled base currency amount. Ensure the Finance Agent's daily summary correctly aggregates these multi-currency sales into a single plain-language total for the owner. Ensure the mobile settings view fits perfectly on a 375px screen.

  *Note: A UI audit should be performed to ensure the "Global Sales" toggle and related UI elements are present and functional without relying on mock data.*

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []