issue_title: "Design: Zero-Configuration Global Tax & Currency Mesh"
issue_description: |
  # Zero-Configuration Global Tax & Currency Mesh

  ## Research Findings
  Small business owners face massive friction when expanding globally due to complex sales tax, VAT, and multi-currency pricing configurations. Current platforms like Shopify and Wix require manual "Tax Zone" setups, third-party apps (TaxJar, Avalara), or expensive enterprise tiers. This breaks the OHC promise of "zero to live in under 10 minutes without manuals."

  ## Proposed Next Steps
  We have designed the **Global Tax & Currency Mesh (GTCM)** to solve this. The system will feature:
  1. **Edge Pricing Converter:** Serves localized, cleanly rounded prices based on buyer locale with sub-50ms latency using edge-caching.
  2. **Transaction Interceptor:** Dynamically calculates precise tax at checkout without merchant configuration.
  3. **Invisible Nexus Monitor:** An AI agent in the Finance department that monitors sales volume across jurisdictions and prompts the merchant when they approach economic nexus thresholds.
  4. **Multi-Tenant GTCM Ledger:** An isolated, zero-trust ledger for recording currency, FX rates, and collected taxes.

  An issue brief has been submitted at `docs/research/[architecture]_zero_configuration_global_tax_and_currency_mesh.md`. The implementer swarm should begin designing the core data models, edge routing, and AI triggers.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []