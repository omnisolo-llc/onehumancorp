issue_title: "Unified Real-Time AI Tax & Compliance Engine"
issue_description: |
  **Research Report Mode Output**

  **Problem Statement:**
  Small business owners like Fatima (Food Cart operator) and Carlos (Handyman) operate across multiple jurisdictions. Currently, configuring complex, hyper-local tax rules is a massive cognitive burden. They need an invisible, AI-driven compliance engine that natively understands location-based tax rules, product-type exemptions, and multi-currency conversions, calculating the exact correct amount instantly.

  **Research Findings:**
  Platforms like Shopify or Wix typically rely on manual tax zone configurations or paid third-party integrations (like Avalara or TaxJar). OHC needs a native, real-time Tax & Compliance Engine that leverages device location or buyer address combined with AI catalog understanding to automatically classify items and calculate exact liabilities.

  **Proposed Next Steps:**
  1. Implement a Real-Time Tax Engine that intercepts checkout/invoicing requests.
  2. Query AI Catalog for product tax classifications.
  3. Integrate with an external Tax API (e.g., Stripe Tax).
  4. Create an Immutable Tax Ledger to securely record liabilities per tenant.
  5. Provide AI Agent integration (Finance and Legal) to handle end-of-month reporting and nexus threshold alerts.
  Detailed architectural diagram, UX flow, and implementation prompt have been added to `docs/research/[architecture]_unified_realtime_ai_tax_and_compliance_engine.md`.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
