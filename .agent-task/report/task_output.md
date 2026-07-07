issue_title: "Implement AI-Driven Multi-Currency & Localized Instant Invoicing Architecture"
issue_description: |
  ## Title
  AI-Driven Multi-Currency & Localized Instant Invoicing Architecture

  ## Problem Statement
  Small business operators like Nora (Agency Principal) and Priya (Boutique Operator) serve international clients or deal with tourists. Currently, they struggle with manual currency conversions, localized tax compliance, and reconciling foreign payments. Existing tools like Shopify require complex multi-market setups, and manual invoicing platforms leave the burden of localization and tax on the owner. OHC needs an invisible, agent-driven localized invoicing system where the owner enters a price in their base currency, and the system autonomously handles the rest.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix:** Require explicit and complex setup for "Markets" or regional settings. It's often overwhelming for a non-technical solopreneur.
  - **Stripe / QuickBooks:** Provide multi-currency features, but require manual API integration for localized tax rules or expect the user to understand complex accounting ledgers.
  - **OHC Opportunity:** Leverage "The Accountant" (Finance Agent) to intercept draft invoices, automatically apply real-time exchange rates (cached locally), determine the buyer's locale to format the invoice (date formats, currency symbols, VAT/GST rules, line-item translation), and output a localized, compliant payment link. The owner simply reviews an AI-generated card in their feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner/Agent Drafts Invoice] --> B[Invoice Domain Service]
      B --> C{The Accountant Agent}
      C -->|Fetch Rates| D[Redis FX Cache]
      C -->|Buyer Locale Lookup| E[Customer Graph DB]
      C --> F[Tax & Compliance Rules Engine]
      F --> G[Localized Invoice Artifact]
      G --> H[Stripe Checkout Session]
      H --> I[Customer]
      I -->|Pays via Stripe| J[Stripe Webhook]
      J --> K[Ledger Reconciliation]
      K --> L[Update Owner Mobile Dashboard]
  ```

  ### Mobile UX Flow (375px First)
  - **Feed UI (Mobile):** Owner taps "+" to create an invoice for "$1,000 USD" to a saved "Client in London".
  - **Agent Intervention:** The feed shows an action card: "Drafted Invoice for £790. Includes UK VAT. [Preview] [Send to Client]".
  - **Preview Screen:** A clean, translucent glass-styled invoice preview showing translated French/English line items (if applicable) and localized formatting, perfectly readable without horizontal scrolling on a 375px screen.
  - **Action:** Tapping "Send to Client" dispatches the localized Stripe Payment Link via Omnichannel.

  ### AI Agent Integration Points
  - **The Accountant (Finance Agent):** Intercepts the invoice creation event via the event mesh. Looks up the customer's region, applies the correct currency conversion, and determines necessary tax compliance based on tenant origin and destination.
  - **The Ambassador (Customer Success Agent):** Optionally translates the invoice line items into the customer's primary language.

  ### Key Design Decisions
  - **Base Currency Anchor:** All tenant ledger entries in the PostgreSQL DB are anchored to the tenant's base currency. Display amounts and localized invoices are computed at runtime.
  - **Idempotency & FX Fluctuations:** The Stripe checkout session locks the FX rate at the time of invoice approval. The webhook strictly reconciles the final settled amount to prevent ledger drift.
  - **Zero-Trust Isolation:** Multi-tenant PostgreSQL RLS ensures that `tenant_id` scopes all fx_rates and ledger entries.

  ## Implementation Prompt
  **User-Facing Outcome:** As Nora, I can bill a client in France $5,000. OHC automatically converts this to EUR, adds applicable EU VAT, and translates the invoice line items to French. I just tap "Approve and Send" from my phone.

  **CUJ & Acceptance Criteria:**
  1. Owner drafts an invoice to an international customer via the UI.
  2. "The Accountant" agent intercepts the request, looks up the customer's region, and applies currency conversion & tax rules.
  3. A localized invoice record and Stripe payment link are generated.
  4. The webhook handler successfully receives a simulated payment and reconciles it in the ledger using the tenant's base currency.
  5. **Automated Verification:** Provide a Playwright E2E test where an owner creates an invoice for a foreign customer, reviews the localized AI-drafted card on a 375px viewport, and verifies the final ledger state.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
