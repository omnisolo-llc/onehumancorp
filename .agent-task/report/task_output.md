issue_title: "Architectural Gap: Invisible Global Multi-Currency & Cross-Border Commerce Engine"
issue_description: |
  **Research Report Mode Output**

  *Findings:*
  Currently, OneHumanCorp (OHC) lacks native multi-currency support and seamless cross-border commerce handling. If merchants want to sell their inventory globally, they have to manually calculate exchange rates or rely on expensive third-party apps to handle localized checkout. Competitors like Shopify use Shopify Markets, which requires explicit configuration. A review of the codebase (`src/server/db.rs` and active research docs) reveals that there is no centralized, real-time FX (foreign exchange) cache or multi-tenant currency ledger that allows transparent real-time conversion at the edge. The current `tenant_id` boundaries do not natively embed a "base currency vs. active display currency" mapping for instant edge caching.

  *Proposed Next Steps:*
  Implement the Invisible Global Multi-Currency Engine within the OHC platform. The edge layer must auto-detect locale and fetch realtime FX rates without blocking catalog load. The core database must maintain strict multi-tenant isolation (`tenant_id`) and store all core ledgers in the merchant's base currency, while checkout events must immutably record the captured local currency, base currency, and exact FX rate used. Finally, integrate the Finance Agent to alert on high FX volatility.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
