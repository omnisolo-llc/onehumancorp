issue_title: "[Architecture] Offline-First Multi-Currency & Localized Pricing Engine"
issue_description: |
  # Research Report: Offline-First Multi-Currency & Localized Pricing Engine

  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. They lack the time, financial expertise, and technical capability to configure localized pricing, tax routing, and foreign exchange (FX) risk mitigation. Currently, OHC displays a single base currency and does not have robust offline capabilities or auto-conversion that hides the FX risk from merchants.

  ## Research Findings
  - **Competitors:** Shopify Markets requires significant manual setup. Stripe has good APIs but lacks SMB platform integration without code. Wix provides basic conversion but results in non-charming pricing (e.g. $14.82).
  - **Market Dynamics:** Offering Local Payment Methods (LPMs) increases conversion up to 40%. "Charming pricing" (rounding to .99 or .00) is necessary for high conversion. Merchants want payouts in their home currency and zero FX risk exposure.
  - **Gaps in OHC:** The current localization logic (e.g., as seen in the legacy/existing Rust `src/server/api/localization.rs` implementation) fetches FX rates but lacks a comprehensive data model and background agent orchestration for locking FX rates, absorbing variance into charming prices, and ensuring offline-first resilience for mobile clients. This needs to be fully aligned with the primary Go + Bazel backend architecture.

  ## Design Doc
  - **Core Entities:** `FxRateCache` (synced to edge), `LocalizedPricingRule` (determines rounding logic), `TransactionLedger` (records exact FX locks).
  - **Architecture:**
    1. **Edge/Client Cache:** The Flutter app/PWA caches daily FX rates via a new offline-sync payload to enable zero-latency conversion for browsing, falling back to cached rates if offline.
    2. **Charming Price Engine:** A Go service takes the base price, applies the cached FX rate, and rounds to the nearest local charm point (e.g. .99).
    3. **Transaction Lock:** At checkout, the backend locks the live Stripe FX rate for a short window.
    4. **AI Department Coordination:** Finance & Payments agent periodically reconciles expected versus actual payouts and absorbs the minor cent differences into an 'FX Variance' accounting bucket.
  - **Multi-Tenant Isolation:** Pricing rules and base currencies are strictly scoped by `tenant_id` using PostgreSQL RLS.

  ## Implementation Prompt
  Implement the Go + Bazel backend core for the Offline-First Multi-Currency Engine.
  1. Expand the PostgreSQL database schema (via a new migration) to store `tenant_base_currency` and `fx_variance_bucket`, ensuring RLS is enabled using the `tenant_id` column.
  2. Implement an API endpoint `GET /api/v1/localization/pricing-rules` in Go that returns the configured charming price rounding strategies per tenant.
  3. Update the localization logic (migrating concepts from `localization.rs` to Go) to expose a batch FX rate cache endpoint suitable for edge/offline consumption.
  4. Implement an internal utility function in Go that converts a given amount using an FX rate and applies the charming price rounding logic.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
