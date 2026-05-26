issue_title: "Architectural Gap: Global Multi-Currency and Localization Engine"
issue_description: |
  **Research Report**:
  OneHumanCorp currently lacks a native, zero-configuration engine to handle cross-border commerce seamlessly. Competitors like Shopify and Stripe offer these capabilities but often require developer or merchant configuration (e.g., managing "Markets").

  **Findings**:
  - Small business owners lose international sales due to unfamiliar checkout currencies and lack of local payment methods.
  - Manual configuration of foreign exchange rates, localized pricing, and cross-border taxes is too complex for OHC's target personas.
  - OHC needs an invisible layer that intercepts buyer sessions, determines the locale, applies smart-rounded FX conversions (via the Finance AI Agent), surfaces local payment methods, and collects necessary taxes (via the Legal/Tax AI Agent), all while keeping the merchant's ledger strictly in their base currency.

  **Proposed Next Steps**:
  1. Implement the `LocalizationContext` middleware to parse buyer locales.
  2. Develop the `DYNAMIC_PRICE_ENGINE` to interact with the Finance AI Agent for smart rounding and FX buffering.
  3. Update the Checkout flow to surface localized payment methods and dynamic tax injections based on the localization context.
  4. Ensure multi-tenant isolation so localization data does not leak across sessions.

  See the full design document at `docs/research/[architecture]_global_multi_currency_and_localization_engine.md` for architectural diagrams, 375px mobile UI/UX flow, and detailed acceptance criteria.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []