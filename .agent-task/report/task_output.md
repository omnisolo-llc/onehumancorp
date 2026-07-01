issue_title: "Multi-Currency & Instant Localized Invoicing Architecture"
issue_description: |
  # Research Report: Autonomous Multi-Currency & Instant Localized Invoicing Architecture

  ## 1. Problem Statement
  As OneHumanCorp (OHC) expands to support diverse owners globally (e.g., Leo the online tutor taking international students, or Priya selling boutique items abroad), the lack of a native, automated multi-currency processing and localized invoicing architecture introduces significant friction. Currently, owners must manually calculate exchange rates, manage separate Stripe configurations, and manually draft compliant invoices for different tax jurisdictions, violating the "Zero Technical Hurdle" promise.

  ## 2. Research Report
  - **Codebase & Docs Audit**: OHC currently supports Stripe Checkout, but the billing and invoice data models (`billing.rs`, `invoice.rs`) lack strict multi-currency constraints and automated localization. The `tenant_id` isolation exists, but localized tax logic and currency conversion rely on manual or fragmented implementations.
  - **Competitor Systems Audit**:
    - **Stripe & Shopify**: Both provide robust multi-currency ledgers. Shopify automatically converts display prices based on edge-cached geolocation and handles localized invoicing using deep tax compliance integrations.
    - **Wix & Squarespace**: Offer basic currency conversion but require manual setup for proper localized tax invoicing.
  - **Identify Gaps**: OHC's Finance and Sales AI Departments cannot autonomously generate tax-compliant, localized invoices for cross-border transactions because the underlying data model lacks multi-currency primitive support (e.g., tracking `base_currency` vs `transaction_currency` and exchange rate at the time of transaction).

  ## 3. Design Doc

  **Architecture & Data Model Invariants**:
  - **Financial Ledger Updates**: Introduce `base_currency`, `transaction_currency`, and `exchange_rate` to the core `ORDER`, `INVOICE`, and `PAYMENT` entities.
  - **Multi-Tenant Isolation**: Ensure all financial records strictly enforce `tenant_id` policies via PostgreSQL RLS.
  - **AI Department Coordination**:
    - **Sales Agent**: Autonomously detects the buyer's locale and drafts quotes/invoices in the local currency.
    - **Finance Agent**: Reconciles the foreign currency transaction against the tenant's base currency, applying the timestamped exchange rate, and generates the localized invoice document.

  **Architecture Diagram**:
  ```mermaid
  sequenceDiagram
      actor Buyer
      participant OHC as OHC Mobile UI
      participant Edge as Geolocation / Edge Cache
      participant AI_Sales as Sales Agent
      participant AI_Fin as Finance Agent
      participant DB as OHC PostgreSQL
      participant Stripe as Stripe API

      Buyer->>OHC: Requests Quote/Checkout
      OHC->>Edge: Detect Locale & Currency
      Edge-->>OHC: Returns `EUR` (Base: `USD`)
      OHC->>AI_Sales: Draft Quote in `EUR`
      AI_Sales->>DB: Store Draft (Quote)
      AI_Sales-->>Buyer: Present Quote (EUR)
      Buyer->>Stripe: Approve & Pay (EUR)
      Stripe-->>AI_Fin: Webhook: Payment Success
      AI_Fin->>DB: Record Tx (Base: USD, Tx: EUR, Rate: 1.1)
      AI_Fin->>DB: Generate Localized Invoice (PDF)
      AI_Fin-->>Buyer: Send Localized Invoice
  ```

  **Mobile-First UX Flow**:
  - **Owner View (375px)**: A simple consolidated feed showing the transaction in their base currency with a small indicator of the original transaction currency.
  - **Configuration**: A "Global Sales" toggle in the owner settings. When activated, the Finance Agent autonomously configures Stripe for multi-currency acceptance without exposing API keys or complex settings.

  **Key Design Decisions**:
  - **Immutable Exchange Rates**: The exchange rate is locked and recorded immutably at the time of transaction for accurate accounting.
  - **AI-Driven Localization**: Instead of manual tax rate entry, the Finance Agent utilizes an external tax API (via Stripe Tax or similar) to ensure the generated invoice meets local compliance automatically.

  ## 4. Implementation Prompt
  Implement the Multi-Currency & Localized Invoicing architecture.
  1. Extend the multi-tenant financial data models to support `base_currency`, `transaction_currency`, and `exchange_rate`.
  2. Implement the backend logic for the Finance Agent to autonomously draft localized invoices upon successful cross-border payments.
  3. Create the 375px mobile-first UI for the business owner to view consolidated earnings and enable "Global Sales" via a single toggle.
  4. Ensure all changes enforce strict multi-tenant isolation and pass 100% unit and Playwright E2E test coverage.

  ## 5. Metadata
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
