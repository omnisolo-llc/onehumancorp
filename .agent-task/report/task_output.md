issue_title: "Architecture: Multi-Currency & Instant Localized Invoicing Engine"
issue_description: |
  ## Problem Statement
  Nora (Agency Principal) and Priya (Boutique Operator) frequently serve international clients and customers. Currently, OneHumanCorp (OHC) lacks a native, unified architecture for multi-currency pricing, real-time FX rate caching, and instant localized invoicing. Owners are forced to manually calculate conversion rates, rely on external accounting tools to draft cross-border invoices, and navigate complex regional tax compliance (e.g., EU VAT) alone. This friction directly violates the OHC promise of "keeping advanced setup hidden until needed."

  ## Research Report
  - **Competitor Analysis:**
    - *Shopify Markets:* Handles multi-currency seamlessly by caching daily exchange rates and applying intelligent rounding rules.
    - *Stripe Billing:* Provides localized invoicing out of the box, but directly exposing Stripe's configuration dashboard overwhelms non-technical SMEs.
  - **Market Need:** Approximately 40% of modern digital and service SMEs operate cross-border. To maintain the "Assistant-First" experience, the system must automatically detect client locales, apply up-to-date currency conversions, ensure local tax compliance, and draft invoices in the appropriate language—all with a single tap of approval from the owner.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Invoice : issues
      Invoice ||--|{ InvoiceLineItem : contains
      Invoice {
          uuid id
          uuid tenant_id
          uuid customer_id
          string base_currency
          string target_currency
          decimal exchange_rate_applied
          string status
      }
      ExchangeRateCache {
          string base_currency
          string target_currency
          decimal rate
          timestamp updated_at
      }
      Agent_Finance ||--o{ Invoice : drafts_and_reconciles
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage (Home Screen):** Nora views her priority feed. A card displays: "Client approved project 'Brand Refresh'."
  2. **Agent Proposal:** The Finance Assistant surfaces a sub-card: "Client is in London. Draft invoice for £2,000 (approx. $2,550 USD)?"
  3. **Interaction:** Nora taps the primary "Approve & Send" button. The button is full-width (≥44px touch target) anchored at the bottom of the card, styled with OHC Premium translucent glass materials.
  4. **Confirmation:** The UI optimistically updates to "Sending...", followed by a clear success token state. No complex FX settings or tax forms are shown unless Nora taps "Advanced Edit".

  ### AI Agent Integration
  - **Finance Assistant ("The Accountant"):** Equipped with a new tool `draft_localized_invoice(customer_id, base_amount, target_currency)`. It queries the Redis-backed `ExchangeRateCache` for current rates and interfaces with Stripe Billing to generate the PDF.
  - **Customer Assistant:** Detects the recipient's locale and drafts the accompanying email or WhatsApp message in their native language, linking to the localized invoice.

  ### Technical Integrity & Multi-Tenancy
  - **Data Isolation:** Both `Invoice` and `ExchangeRateCache` are secured via row-level security (`ENABLE ROW LEVEL SECURITY`) keyed on `tenant_id`.
  - **FX Updates:** A background worker running via PostgreSQL `SKIP LOCKED` job queue fetches updated rates daily and caches them in Redis (`ohc:fx:{base}:{target}`).
  - **Zero Trust:** The mobile client never performs currency calculations. It merely renders the localized strings and amounts computed by the secure backend.

  ## Implementation Prompt
  **Implementer Agent Task:**
  Design and implement the localized invoicing engine. Begin by extending the PostgreSQL schema with `Invoice` and `ExchangeRateCache` tables, strictly enforcing `tenant_id` RLS. Implement the backend service (Go) to fetch and cache daily FX rates in Redis. Create the `draft_localized_invoice` capability for the Finance Agent. Finally, build the Flutter mobile-first UI components to display the agent's invoice proposal and the "Approve & Send" action on a 375px viewport. You MUST add a Playwright E2E test that starts from the owner's feed, triggers the invoice approval, and verifies the localized output state.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
